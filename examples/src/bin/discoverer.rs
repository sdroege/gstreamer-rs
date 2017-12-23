extern crate gstreamer as gst;
use gst::prelude::*;

extern crate gstreamer_pbutils as pbutils;
use pbutils::DiscovererExt;
use pbutils::DiscovererInfo;
use pbutils::DiscovererInfoExt;
use pbutils::DiscovererStreamInfo;
use pbutils::DiscovererStreamInfoExt;

extern crate glib;

use std::env;

#[path = "../examples-common.rs"]
mod examples_common;

fn print_tags(info : &DiscovererInfo) {
    println!("Tags:");

    let tags = info.get_tags();
    match tags {
        Some(taglist) => {
            println!("  {}", taglist.to_string()); // FIXME use an iterator
        },
        None => {
            println!("  no tags");
        }
    }
}

fn print_stream_info(stream : &DiscovererStreamInfo) {
    println!("Stream: ");
    match stream.get_stream_id() {
        Some(id) => println!("  Stream id: {}", id),
        None => {}
    }
    println!("  Format: {}", stream.get_caps().unwrap().to_string());
}

fn print_discoverer_info(info : &DiscovererInfo) {
    println!("URI: {}", info.get_uri().unwrap());
    println!("Duration: {}", info.get_duration());
    print_tags(info);
    print_stream_info(&info.get_stream_info().unwrap());

    let children = info.get_stream_list();
    println!("Children streams:");
    for child in children {
        print_stream_info(&child);
    }
}

fn example_main() {
    gst::init().unwrap();

    let args: Vec<_> = env::args().collect();
    let uri: &str = if args.len() == 2 {
        args[1].as_ref()
    } else {
        println!("Usage: discoverer uri");
        std::process::exit(-1)
    };


    let timeout : gst::ClockTime = gst::ClockTime::from_seconds(15);
    let discoverer = pbutils::Discoverer::new(timeout).unwrap();
    let info = discoverer.discover_uri(uri).unwrap();
    print_discoverer_info(&info);
}

fn main() {
    // tutorials_common::run is only required to set up the application environent on macOS
    // (but not necessary in normal Cocoa applications where this is set up autmatically)
    examples_common::run(example_main);
}
