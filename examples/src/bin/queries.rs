extern crate gstreamer as gst;
use gst::prelude::*;

extern crate glib;

use std::env;

#[path = "../examples-common.rs"]
mod examples_common;

fn example_main() {
    let pipeline_str = env::args().collect::<Vec<String>>()[1..].join(" ");

    gst::init().unwrap();

    let main_loop = glib::MainLoop::new(None, false);

    let pipeline = gst::parse_launch(&pipeline_str).unwrap();
    let bus = pipeline.get_bus().unwrap();

    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    let main_loop_clone = main_loop.clone();

    let pipeline_clone = pipeline.clone();
    glib::timeout_add_seconds(1, move || {
        use gst::QueryView;

        let pipeline = &pipeline_clone;

        //let pos = pipeline.query_position(gst::Format::Time).unwrap_or(-1);
        //let dur = pipeline.query_duration(gst::Format::Time).unwrap_or(-1);
        let pos = {
            let mut q = gst::Query::new_position(gst::Format::Time);
            if pipeline.query(q.get_mut().unwrap()) {
                match q.view() {
                    QueryView::Position(ref p) => Some(p.get_result()),
                    _ => None,
                }
            } else {
                None
            }
        }.and_then(|pos| pos.try_to_time())
            .unwrap();

        let dur = {
            let mut q = gst::Query::new_duration(gst::Format::Time);
            if pipeline.query(q.get_mut().unwrap()) {
                match q.view() {
                    QueryView::Duration(ref p) => Some(p.get_result()),
                    _ => None,
                }
            } else {
                None
            }
        }.and_then(|dur| dur.try_to_time())
            .unwrap();

        println!("{} / {}", pos, dur);

        glib::Continue(true)
    });

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
                    msg.get_src().map(|s| s.get_path_string()),
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

    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}

fn main() {
    // tutorials_common::run is only required to set up the application environent on macOS
    // (but not necessary in normal Cocoa applications where this is set up autmatically)
    examples_common::run(example_main);
}
