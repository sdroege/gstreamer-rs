// This example demonstrates how to use GStreamer's query functionality.
// These are a way to query information from either elements or pads.
// Such information could for example be the current position within
// the stream (i.e. the playing time). Queries can traverse the pipeline
// (both up and downstream). This functionality is essential, since most
// queries can only answered by specific elements in a pipeline (such as the
// stream's duration, which often can only be answered by the demuxer).
// Since gstreamer has many elements that itself contain other elements that
// we don't know of, we can simply send a query for the duration into the
// pipeline and the query is passed along until an element feels capable
// of answering.
// For convenience, the API has a set of pre-defined queries, but also
// allows custom queries (which can be defined and used by your own elements).

extern crate gstreamer as gst;
use gst::prelude::*;

extern crate glib;

use std::convert::TryInto;
use std::env;

#[path = "../examples-common.rs"]
mod examples_common;

fn example_main() {
    // Get a string containing the passed pipeline launch syntax
    let pipeline_str = env::args().collect::<Vec<String>>()[1..].join(" ");

    gst::init().unwrap();

    let main_loop = glib::MainLoop::new(None, false);

    // Let GStreamer create a pipeline from the parsed launch syntax on the cli.
    let pipeline = gst::parse_launch(&pipeline_str).unwrap();
    let bus = pipeline.get_bus().unwrap();

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    // Need to move a new reference into the closure.
    // !!ATTENTION!!:
    // It might seem appealing to use pipeline.clone() here, because that greatly
    // simplifies the code within the callback. What this actually dose, however, is creating
    // a memory leak. The clone of a pipeline is a new strong reference on the pipeline.
    // Storing this strong reference of the pipeline within the callback (we are moving it in!),
    // which is in turn stored in another strong reference on the pipeline is creating a
    // reference cycle.
    // DO NOT USE pipeline.clone() TO USE THE PIPELINE WITHIN A CALLBACK
    let pipeline_weak = pipeline.downgrade();
    // Add a timeout to the main loop. This closure will be executed
    // in an interval of 1 second.
    let timeout_id = glib::timeout_add_seconds(1, move || {
        // Here we temporarily retrieve a strong reference on the pipeline from the weak one
        // we moved into this callback.
        let pipeline = match pipeline_weak.upgrade() {
            Some(pipeline) => pipeline,
            None => return glib::Continue(true),
        };

        //let pos = pipeline.query_position(gst::Format::Time).unwrap_or(-1);
        //let dur = pipeline.query_duration(gst::Format::Time).unwrap_or(-1);
        let pos: gst::ClockTime = {
            // Create a new position query and send it to the pipeline.
            // This will traverse all elements in the pipeline, until one feels
            // capable of answering the query.
            let mut q = gst::Query::new_position(gst::Format::Time);
            if pipeline.query(&mut q) {
                Some(q.get_result())
            } else {
                None
            }
        }
        .and_then(|pos| pos.try_into().ok())
        .unwrap();

        let dur: gst::ClockTime = {
            // Create a new duration query and send it to the pipeline.
            // This will traverse all elements in the pipeline, until one feels
            // capable of answering the query.
            let mut q = gst::Query::new_duration(gst::Format::Time);
            if pipeline.query(&mut q) {
                Some(q.get_result())
            } else {
                None
            }
        }
        .and_then(|dur| dur.try_into().ok())
        .unwrap();

        println!("{} / {}", pos, dur);

        glib::Continue(true)
    });

    // Need to move a new reference into the closure.
    let main_loop_clone = main_loop.clone();
    //bus.add_signal_watch();
    //bus.connect_message(move |_, msg| {
    bus.add_watch(move |_, msg| {
        use gst::MessageView;

        let main_loop = &main_loop_clone;
        match msg.view() {
            MessageView::Eos(..) => main_loop.quit(),
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                main_loop.quit();
            }
            _ => (),
        };

        glib::Continue(true)
    });

    main_loop.run();

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");

    bus.remove_watch().unwrap();
    glib::source_remove(timeout_id);
}

fn main() {
    // tutorials_common::run is only required to set up the application environent on macOS
    // (but not necessary in normal Cocoa applications where this is set up autmatically)
    examples_common::run(example_main);
}
