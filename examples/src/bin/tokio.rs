extern crate gstreamer as gst;
use gst::prelude::*;

extern crate futures;
use futures::stream::Stream;
extern crate tokio_core;
use tokio_core::reactor::Core;

use std::env;

#[path = "../examples-common.rs"]
mod examples_common;

fn example_main() {
    let pipeline_str = env::args().collect::<Vec<String>>()[1..].join(" ");

    gst::init().unwrap();

    let mut core = Core::new().unwrap();

    let pipeline = gst::parse_launch(&pipeline_str).unwrap();
    let bus = pipeline.get_bus().unwrap();

    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    let messages = gst::BusStream::new(&bus).for_each(|msg| {
        use gst::MessageView;

        let quit = match msg.view() {
            MessageView::Eos(..) => true,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                true
            }
            _ => false,
        };

        if quit {
            Err(())
        } else {
            Ok(())
        }
    });

    let _ = core.run(messages);

    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}

fn main() {
    // tutorials_common::run is only required to set up the application environent on macOS
    // (but not necessary in normal Cocoa applications where this is set up autmatically)
    examples_common::run(example_main);
}
