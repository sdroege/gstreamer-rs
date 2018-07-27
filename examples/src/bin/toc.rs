extern crate gstreamer as gst;
use gst::prelude::*;

extern crate glib;

use std::env;

#[path = "../examples-common.rs"]
mod examples_common;

fn example_main() {
    gst::init().unwrap();

    let args: Vec<_> = env::args().collect();
    let uri: &str = if args.len() == 2 {
        args[1].as_ref()
    } else {
        println!("Usage: toc file_path");
        std::process::exit(-1)
    };

    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("filesrc", None).unwrap();
    let decodebin = gst::ElementFactory::make("decodebin", None).unwrap();

    src.set_property("location", &glib::Value::from(uri))
        .unwrap();

    pipeline.add_many(&[&src, &decodebin]).unwrap();
    gst::Element::link_many(&[&src, &decodebin]).unwrap();

    // Need to move a new reference into the closure
    let pipeline_weak = pipeline.downgrade();
    decodebin.connect_pad_added(move |_, src_pad| {
        let pipeline = match pipeline_weak.upgrade() {
            Some(pipeline) => pipeline,
            None => return,
        };
        let queue = gst::ElementFactory::make("queue", None).unwrap();
        let sink = gst::ElementFactory::make("fakesink", None).unwrap();

        let elements = &[&queue, &sink];
        pipeline.add_many(elements).unwrap();
        gst::Element::link_many(elements).unwrap();

        for e in elements {
            e.sync_state_with_parent().unwrap();
        }

        let sink_pad = queue.get_static_pad("sink").unwrap();
        assert_eq!(src_pad.link(&sink_pad), gst::PadLinkReturn::Ok);
    });

    assert_ne!(
        pipeline.set_state(gst::State::Paused),
        gst::StateChangeReturn::Failure
    );

    let bus = pipeline.get_bus().unwrap();

    while let Some(msg) = bus.timed_pop(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(_) | MessageView::AsyncDone(_) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                break;
            }
            MessageView::Toc(msg_toc) => {
                let (toc, updated) = msg_toc.get_toc();
                println!(
                    "\nReceived toc: {:?} - updated: {}",
                    toc.get_scope(),
                    updated
                );
                if let Some(tags) = toc.get_tags() {
                    println!("- tags: {}", tags.to_string());
                }
                for toc_entry in toc.get_entries() {
                    println!(
                        "\t{:?} - {}",
                        toc_entry.get_entry_type(),
                        toc_entry.get_uid()
                    );
                    if let Some((start, stop)) = toc_entry.get_start_stop_times() {
                        println!("\t- start: {}, stop: {}", start, stop);
                    }
                    if let Some(tags) = toc_entry.get_tags() {
                        println!("\t- tags: {}", tags.to_string());
                    }
                    for toc_sub_entry in toc_entry.get_sub_entries() {
                        println!(
                            "\n\t\t{:?} - {}",
                            toc_sub_entry.get_entry_type(),
                            toc_sub_entry.get_uid()
                        );
                        if let Some((start, stop)) = toc_sub_entry.get_start_stop_times() {
                            println!("\t\t- start: {}, stop: {}", start, stop);
                        }
                        if let Some(tags) = toc_sub_entry.get_tags() {
                            println!("\t\t- tags: {:?}", tags.to_string());
                        }
                    }
                }
            }
            _ => (),
        }
    }

    assert_ne!(
        pipeline.set_state(gst::State::Null),
        gst::StateChangeReturn::Failure
    );
}

fn main() {
    // tutorials_common::run is only required to set up the application environent on macOS
    // (but not necessary in normal Cocoa applications where this is set up autmatically)
    examples_common::run(example_main);
}
