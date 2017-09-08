extern crate gstreamer as gst;
use gst::prelude::*;

use std::u64;

fn main() {
    // Initialize GStreamer
    gst::init().unwrap();

    // Build the pipeline
    let uri = "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm";
    let pipeline = gst::parse_launch(&format!("playbin uri={}", uri)).unwrap();

    // Start playing
    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    // Wait until error or EOS
    let bus = pipeline.get_bus().unwrap();
    loop {
        use gst::MessageView;

        let msg = match bus.timed_pop(u64::MAX) {
            None => break,
            Some(msg) => msg,
        };

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {}: {} ({:?})",
                    msg.get_src().get_path_string(),
                    err.get_error(),
                    err.get_debug()
                );
                break;
            }
            _ => (),
        }
    }

    // Shutdown pipeline
    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}
