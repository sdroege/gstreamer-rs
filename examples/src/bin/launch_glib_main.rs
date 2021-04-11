// This is a simplified rust-reimplementation of the gst-launch-<version>
// cli tool. It has no own parameters and simply parses the cli arguments
// as launch syntax.
// When the parsing succeeded, the pipeline is run until it exits.
// Main difference between this example and the launch example is the use of
// GLib's main loop to operate GStreamer's bus. This allows to also do other
// things from the main loop (timeouts, UI events, socket events, ...) instead
// of just handling messages from GStreamer's bus.

use gst::prelude::*;

use std::env;

#[path = "../examples-common.rs"]
mod examples_common;

fn example_main() {
    // Get a string containing the passed pipeline launch syntax
    let pipeline_str = env::args().collect::<Vec<String>>()[1..].join(" ");

    gst::init().unwrap();

    // Like teasered above, we use GLib's main loop to operate GStreamer's bus.
    let main_loop = glib::MainLoop::new(None, false);

    // Let GStreamer create a pipeline from the parsed launch syntax on the cli.
    let pipeline = gst::parse_launch(&pipeline_str).unwrap();
    let bus = pipeline.bus().unwrap();

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    let main_loop_clone = main_loop.clone();

    //bus.add_signal_watch();
    //bus.connect_message(None, move |_, msg| {
    bus.add_watch(move |_, msg| {
        use gst::MessageView;

        let main_loop = &main_loop_clone;
        match msg.view() {
            MessageView::Eos(..) => main_loop.quit(),
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                main_loop.quit();
            }
            _ => (),
        };

        glib::Continue(true)
    })
    .expect("Failed to add bus watch");

    main_loop.run();

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");

    // Here we remove the bus watch we added above. This avoids a memory leak, that might
    // otherwise happen because we moved a strong reference (clone of main_loop) into the
    // callback closure above.
    bus.remove_watch().unwrap();
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    examples_common::run(example_main);
}
