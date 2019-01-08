// This example demonstrates how to use the gstreamer crate in conjunction
// with the future trait. The example waits for either an error to occur,
// or for an EOS message. When a message notifying about either of both
// is received, the future is resolved.

extern crate gstreamer as gst;
use gst::prelude::*;

extern crate futures;
use futures::executor::block_on;
use futures::prelude::*;

use std::env;

#[path = "../examples-common.rs"]
mod examples_common;

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

    // BusStream implements the Stream trait, but Stream::for_each is
    // calling a closure for each item and returns a Future that resolves
    // when the stream is done or an error has happened
    let messages = gst::BusStream::new(&bus)
        .for_each(|msg| {
            use gst::MessageView;

            // Determine whether we want to resolve the future, or we still have
            // to wait. The future is resolved when either an error occurs, or the
            // pipeline succeeded execution (got an EOS event).
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
                Err(()) // This resolves the future that is returned by for_each
                        // FIXME: At the moment, EOS messages also result in the future to be resolved
                        // by an error. This should probably be changed in the future.
            } else {
                Ok(()) // Continue - do not resolve the future yet.
            }
        })
        .and_then(|_| Ok(()));

    // Synchronously wait on the future we created above.
    let _ = block_on(messages);

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}

fn main() {
    // tutorials_common::run is only required to set up the application environent on macOS
    // (but not necessary in normal Cocoa applications where this is set up autmatically)
    examples_common::run(example_main);
}
