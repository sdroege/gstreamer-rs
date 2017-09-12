extern crate gstreamer as gst;

use gst::prelude::*;

fn main() {
    // Initialize GStreamer
    gst::init().unwrap();

    // Create the elements
    let source = gst::ElementFactory::make("videotestsrc", "source")
        .expect("Could not create source element.");
    let sink =
        gst::ElementFactory::make("autovideosink", "sink").expect("Could not create sink element");

    // Create the empty pipeline
    let pipeline = gst::Pipeline::new("test-pipeline");

    // Build the pipeline
    pipeline.add_many(&[&source, &sink]).unwrap();
    source.link(&sink).expect("Elements could not be linked.");

    // Modify the source's properties
    source.set_property_from_str("pattern", "smpte");

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
