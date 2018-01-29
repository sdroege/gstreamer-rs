extern crate gstreamer as gst;
use gst::prelude::*;

#[path = "../tutorials-common.rs"]
mod tutorials_common;

fn tutorial_main() {
    // Initialize GStreamer
    if let Err(err) = gst::init() {
        eprintln!("Failed to initialize Gst: {}", err);
        return;
    }

    let audio_source = gst::ElementFactory::make("audiotestsrc", "audio_source").unwrap();
    let tee = gst::ElementFactory::make("tee", "tee").unwrap();
    let audio_queue = gst::ElementFactory::make("queue", "audio_queue").unwrap();
    let audio_convert = gst::ElementFactory::make("audioconvert", "audio_convert").unwrap();
    let audio_resample = gst::ElementFactory::make("audioresample", "audio_resample").unwrap();
    let audio_sink = gst::ElementFactory::make("autoaudiosink", "audio_sink").unwrap();
    let video_queue = gst::ElementFactory::make("queue", "video_queue").unwrap();
    let visual = gst::ElementFactory::make("wavescope", "visual").unwrap();
    let video_convert = gst::ElementFactory::make("videoconvert", "video_convert").unwrap();
    let video_sink = gst::ElementFactory::make("autovideosink", "video_sink").unwrap();

    let pipeline = gst::Pipeline::new("test-pipeline");

    audio_source.set_property("freq", &215.0).unwrap();
    visual.set_property_from_str("shader", "none");
    visual.set_property_from_str("style", "lines");

    pipeline
        .add_many(&[
            &audio_source,
            &tee,
            &audio_queue,
            &audio_convert,
            &audio_resample,
            &audio_sink,
            &video_queue,
            &visual,
            &video_convert,
            &video_sink,
        ])
        .unwrap();

    gst::Element::link_many(&[&audio_source, &tee]).unwrap();
    gst::Element::link_many(&[&audio_queue, &audio_convert, &audio_resample, &audio_sink]).unwrap();
    gst::Element::link_many(&[&video_queue, &visual, &video_convert, &video_sink]).unwrap();

    let tee_audio_pad = tee.get_request_pad("src_%u").unwrap();
    println!(
        "Obtained request pad {} for audio branch",
        tee_audio_pad.get_name()
    );
    let queue_audio_pad = audio_queue.get_static_pad("sink").unwrap();
    tee_audio_pad.link(&queue_audio_pad).into_result().unwrap();

    let tee_video_pad = tee.get_request_pad("src_%u").unwrap();
    println!(
        "Obtained request pad {} for video branch",
        tee_video_pad.get_name()
    );
    let queue_video_pad = video_queue.get_static_pad("sink").unwrap();
    tee_video_pad.link(&queue_video_pad).into_result().unwrap();

    pipeline
        .set_state(gst::State::Playing)
        .into_result()
        .expect("Unable to set the pipeline to the Playing state.");
    let bus = pipeline.get_bus().unwrap();
    while let Some(msg) = bus.timed_pop(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;
        match msg.view() {
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?}: {}",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error()
                );
                eprintln!("Debugging information: {:?}", err.get_debug());
                break;
            }
            MessageView::Eos(..) => break,
            _ => (),
        }
    }

    pipeline
        .set_state(gst::State::Null)
        .into_result()
        .expect("Unable to set the pipeline to the Null state.");
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    tutorials_common::run(tutorial_main);
}
