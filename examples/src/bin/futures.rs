// This example demonstrates how to use the gstreamer crate in conjunction
// with the future trait. The example waits for either an error to occur,
// or for an EOS message. When a message notifying about either of both
// is received, the future is resolved.

use gst::prelude::*;

use futures::executor::LocalPool;
use futures::prelude::*;

use std::env;

#[path = "../examples-common.rs"]
mod examples_common;

async fn message_loop(bus: gst::Bus) {
    let mut messages = bus.stream();

    while let Some(msg) = messages.next().await {
        use gst::MessageView;

        // Determine whether we want to quit: on EOS or error message
        // we quit, otherwise simply continue.
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
        };
    }
}

fn example_main() {
    // Read the pipeline to launch from the commandline, using the launch syntax.
    let pipeline_str = env::args().collect::<Vec<String>>()[1..].join(" ");

    gst::init().unwrap();

    // Create a pipeline from the launch-syntax given on the cli.
    let pipeline = gst::parse_launch(&pipeline_str).unwrap();
    let bus = pipeline.get_bus().unwrap();

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    // Use a LocalPool as executor. This runs single threaded on this very thread.
    let mut pool = LocalPool::new();

    // Run until our message loop finishes, e.g. EOS/error happens
    pool.run_until(message_loop(bus));

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    examples_common::run(example_main);
}
