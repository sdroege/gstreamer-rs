// This is a simplified rust-reimplementation of the gst-launch-<version>
// cli tool. It has no own parameters and simply parses the cli arguments
// as launch syntax.
// When the parsing succeeded, the pipeline is run until the stream ends or an error happens.

extern crate gstreamer as gst;
use gst::prelude::*;

use std::env;
use std::process;

#[path = "../examples-common.rs"]
mod examples_common;

fn example_main() {
    // Get a string containing the passed pipeline launch syntax
    let pipeline_str = env::args().collect::<Vec<String>>()[1..].join(" ");

    gst::init().unwrap();

    // Let GStreamer create a pipeline from the parsed launch syntax on the cli.
    // In comparision to the launch_glib_main example, this is using the advanced launch syntax
    // parsing API of GStreamer. The function returns a Result, handing us the pipeline if
    // parsing and creating succeeded, and hands us detailed error information if something
    // went wrong. The error is passed as gst::ParseError. In this example, we separately
    // handle the NoSuchElement error, that GStreamer uses to notify us about elements
    // used within the launch syntax, that are not available (not installed).
    // Especially GUIs should probably handle this case, to tell users that they need to
    // install the corresponding gstreamer plugins.
    let mut context = gst::ParseContext::new();
    let pipeline =
        match gst::parse_launch_full(&pipeline_str, Some(&mut context), gst::ParseFlags::NONE) {
            Ok(pipeline) => pipeline,
            Err(err) => {
                if let Some(gst::ParseError::NoSuchElement) = err.kind::<gst::ParseError>() {
                    println!("Missing element(s): {:?}", context.get_missing_elements());
                } else {
                    println!("Failed to parse pipeline: {}", err);
                }

                process::exit(-1)
            }
        };
    let bus = pipeline.get_bus().unwrap();

    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                break;
            }
            _ => (),
        }
    }

    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}

fn main() {
    // tutorials_common::run is only required to set up the application environent on macOS
    // (but not necessary in normal Cocoa applications where this is set up autmatically)
    examples_common::run(example_main);
}
