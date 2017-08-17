extern crate gstreamer as gst;
use gst::prelude::*;

use std::u64;
use std::env;
use std::process;

fn main() {
    let pipeline_str = env::args().collect::<Vec<String>>()[1..].join(" ");

    gst::init().unwrap();

    let mut context = gst::ParseContext::new();
    let pipeline =
        match gst::parse_launch_full(&pipeline_str, Some(&mut context), gst::PARSE_FLAG_NONE) {
            Ok(pipeline) => pipeline,
            Err(err) => {
                if let Some(gst::ParseError::NoSuchElement) = err.kind::<gst::ParseError>() {
                    println!("Missing element(s): {:?}", context.get_missing_elements());
                } else {
                    println!("Failed to parse pipeline: {}", err);
                }

                process::exit(-1);
            }
        };
    let bus = pipeline.get_bus().unwrap();

    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

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

    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}
