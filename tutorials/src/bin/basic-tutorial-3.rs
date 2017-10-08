extern crate gstreamer as gst;

use gst::prelude::*;

fn main() {
    // Initialize GStreamer
    gst::init().unwrap();

    // Create the elements
    let source = gst::ElementFactory::make("uridecodebin", "source")
        .expect("Could not create uridecodebin element.");
    let convert = gst::ElementFactory::make("audioconvert", "convert")
        .expect("Could not create convert element.");
    let sink =
        gst::ElementFactory::make("autoaudiosink", "sink").expect("Could not create sink element.");

    // Create the empty pipeline
    let pipeline = gst::Pipeline::new("test-pipeline");

    // Build the pipeline Note that we are NOT linking the source at this
    // point. We will do it later.
    pipeline.add_many(&[&source, &convert, &sink]).unwrap();
    convert.link(&sink).expect("Elements could not be linked.");

    // Set the URI to play
    let uri = "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm";
    source
        .set_property("uri", &uri)
        .expect("Can't set uri property on uridecodebin");

    // Connect the pad-added signal
    let pipeline_clone = pipeline.clone();
    let convert_clone = convert.clone();
    source.connect_pad_added(move |_, src_pad| {
        let pipeline = &pipeline_clone;
        let convert = &convert_clone;

        println!(
            "Received new pad {} from {}",
            src_pad.get_name(),
            pipeline.get_name()
        );

        let sink_pad = convert
            .get_static_pad("sink")
            .expect("Failed to get static sink pad from convert");
        if sink_pad.is_linked() {
            println!("We are already linked. Ignoring.");
            return;
        }

        let new_pad_caps = src_pad
            .get_current_caps()
            .expect("Failed to get caps of new pad.");
        let new_pad_struct = new_pad_caps
            .get_structure(0)
            .expect("Failed to get first structure of caps.");
        let new_pad_type = new_pad_struct.get_name();

        let is_audio = new_pad_type.starts_with("audio/x-raw");
        if !is_audio {
            println!(
                "It has type {} which is not raw audio. Ignoring.",
                new_pad_type
            );
            return;
        }

        let ret = src_pad.link(&sink_pad);
        if ret != gst::PadLinkReturn::Ok {
            println!("Type is {} but link failed.", new_pad_type);
        } else {
            println!("Link succeeded (type {}).", new_pad_type);
        }
    });

    // Start playing
    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(
        ret,
        gst::StateChangeReturn::Failure,
        "Unable to set the pipeline to the Playing state."
    );

    // Wait until error or EOS
    let bus = pipeline.get_bus().unwrap();
    while let Some(msg) = bus.timed_pop(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;
        match msg.view() {
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {}: {}",
                    msg.get_src().get_path_string(),
                    err.get_error()
                );
                eprintln!("Debugging information: {:?}", err.get_debug());
                break;
            }
            MessageView::StateChanged(s) => if msg.get_src() == pipeline {
                println!(
                    "Pipeline state changed from {:?} to {:?}",
                    s.get_old(),
                    s.get_current()
                );
            },
            MessageView::Eos(..) => break,
            _ => (),
        }
    }

    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(
        ret,
        gst::StateChangeReturn::Failure,
        "Unable to set the pipeline to the Null state."
    );
}
