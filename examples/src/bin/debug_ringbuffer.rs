// This example shows how to use the debug ringbuffer.
//
// It runs a simple GStreamer pipeline for a short time,
// and on EOS it dumps the last few KB of debug logs.
//
// It's possible to dump the logs at any time in an application,
// not just on exit like is done here.
use std::process;

use gst::prelude::*;

#[path = "../examples-common.rs"]
mod examples_common;

fn example_main() {
    let pipeline_str = "videotestsrc num-buffers=100 ! autovideosink";

    gst::init().unwrap();

    /* Disable stdout debug, then configure the debug ringbuffer and enable
     * all debug */
    gst::log::remove_default_log_function();
    /* Keep 1KB of logs per thread, removing old threads after 10 seconds */
    gst::log::add_ring_buffer_logger(1024, 10);
    /* Enable all debug categories */
    gst::log::set_default_threshold(gst::DebugLevel::Log);

    let mut context = gst::ParseContext::new();
    let pipeline =
        match gst::parse::launch_full(pipeline_str, Some(&mut context), gst::ParseFlags::empty()) {
            Ok(pipeline) => pipeline,
            Err(err) => {
                if let Some(gst::ParseError::NoSuchElement) = err.kind::<gst::ParseError>() {
                    println!("Missing element(s): {:?}", context.missing_elements());
                } else {
                    println!("Failed to parse pipeline: {err}");
                }

                process::exit(-1)
            }
        };
    let bus = pipeline.bus().unwrap();

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => {
                break;
            }
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
    }

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");

    /* Insert a message into the debug log */
    gst::error!(gst::CAT_DEFAULT, "Hi from the debug log ringbuffer example");

    println!("Dumping debug logs\n");
    for s in gst::log::ring_buffer_logger_get_logs().iter() {
        println!("{s}\n------------------");
    }
}

fn main() {
    // examples_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    examples_common::run(example_main);
}
