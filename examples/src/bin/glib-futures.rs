use gst::prelude::*;

use futures::prelude::*;

use std::env;

#[path = "../examples-common.rs"]
mod examples_common;

async fn message_handler(loop_: glib::MainLoop, bus: gst::Bus) {
    let mut messages = bus.stream();

    while let Some(msg) = messages.next().await {
        use gst::MessageView;

        // Determine whether we want to quit: on EOS or error message
        // we quit, otherwise simply continue.
        match msg.view() {
            MessageView::Eos(..) => loop_.quit(),
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                loop_.quit();
            }
            _ => (),
        }
    }
}

fn example_main() {
    // Get the default main context and make it also the thread default, then create
    // a main loop for it
    let ctx = glib::MainContext::default();
    ctx.push_thread_default();
    let loop_ = glib::MainLoop::new(Some(&ctx), false);

    // Read the pipeline to launch from the commandline, using the launch syntax.
    let pipeline_str = env::args().collect::<Vec<String>>()[1..].join(" ");

    gst::init().unwrap();

    // Create a pipeline from the launch-syntax given on the cli.
    let pipeline = gst::parse_launch(&pipeline_str).unwrap();
    let bus = pipeline.get_bus().unwrap();

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    // Spawn our message handling stream
    ctx.spawn_local(message_handler(loop_.clone(), bus));

    // And run until something is quitting the loop, i.e. an EOS
    // or error message is received above
    loop_.run();

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");

    ctx.pop_thread_default();
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    examples_common::run(example_main);
}
