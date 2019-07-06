// This example demonstrates how to use the gstreamer crate in conjunction
// with the future trait. The example waits for either an error to occur,
// or for an EOS message. When a message notifying about either of both
// is received, the future is resolved.

extern crate gstreamer as gst;
use gst::prelude::*;

extern crate futures;
use futures::executor::LocalPool;
use futures::future;
use futures::prelude::*;
use futures::task::SpawnExt;

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

    // Use a LocalPool as executor. This runs single threaded on this very thread.
    let mut pool = LocalPool::new();

    // We use an AbortHandle for having a Future that runs forever
    // until we call handle.abort() to quit our event loop
    let (quit_handle, quit_registration) = future::AbortHandle::new_pair();
    let quit_future =
        future::Abortable::new(future::pending::<()>(), quit_registration).map(|_| ());

    // BusStream implements the Stream trait. Stream::for_each is calling a closure for each item
    // and returns a Future that resolves when the stream is done
    let messages = gst::BusStream::new(&bus).for_each(move |msg| {
        use gst::MessageView;

        // Determine whether we want to quit: on EOS or error message
        // we quit, otherwise simply continue.
        match msg.view() {
            MessageView::Eos(..) => quit_handle.abort(),
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                quit_handle.abort();
            }
            _ => (),
        };

        // New future to resolve for each message: nothing here
        future::ready(())
    });

    // Spawn our message handling stream
    pool.spawner().spawn(messages).unwrap();

    // And run until something is quitting the loop, i.e. an EOS
    // or error message is received above
    pool.run_until(quit_future);

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}

fn main() {
    // tutorials_common::run is only required to set up the application environent on macOS
    // (but not necessary in normal Cocoa applications where this is set up autmatically)
    examples_common::run(example_main);
}
