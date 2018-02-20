extern crate gstreamer as gst;

extern crate gstreamer_pbutils as pbutils;
use pbutils::prelude::*;

use pbutils::DiscovererStreamInfo;
use pbutils::DiscovererInfo;

extern crate glib;

extern crate failure;
use failure::Error;

#[macro_use]
extern crate failure_derive;

use std::env;

#[path = "../examples-common.rs"]
mod examples_common;

#[derive(Debug, Fail)]
#[fail(display = "Discoverer error {}", _0)]
struct DiscovererError(&'static str);

fn print_tags(info: &DiscovererInfo) {
    println!("Tags:");

    let tags = info.get_tags();
    match tags {
        Some(taglist) => {
            println!("  {}", taglist.to_string()); // FIXME use an iterator
        }
        None => {
            println!("  no tags");
        }
    }
}

fn print_stream_info(stream: &DiscovererStreamInfo) {
    println!("Stream: ");
    match stream.get_stream_id() {
        Some(id) => println!("  Stream id: {}", id),
        None => {}
    }
    let caps_str = match stream.get_caps() {
        Some(caps) => caps.to_string(),
        None => String::from("--")
    };
    println!("  Format: {}", caps_str);
}

fn print_discoverer_info(info: &DiscovererInfo) -> Result<(), Error> {
    let uri = info.get_uri().ok_or(DiscovererError("URI should not be null"))?;
    println!("URI: {}", uri);
    println!("Duration: {}", info.get_duration());
    print_tags(info);
    print_stream_info(&info.get_stream_info().ok_or(DiscovererError("Error while obtaining stream info"))?);

    let children = info.get_stream_list();
    println!("Children streams:");
    for child in children {
        print_stream_info(&child);
    }

    Ok(())
}

fn run_discoverer() -> Result<(), Error> {
    gst::init()?;

    let args: Vec<_> = env::args().collect();
    let uri: &str = if args.len() == 2 {
        args[1].as_ref()
    } else {
        println!("Usage: discoverer uri");
        std::process::exit(-1)
    };

    let timeout: gst::ClockTime = gst::ClockTime::from_seconds(15);
    let discoverer = pbutils::Discoverer::new(timeout)?;
    let info = discoverer.discover_uri(uri)?;
    print_discoverer_info(&info)?;
    Ok(())
}

fn example_main() {
    match run_discoverer() {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e)
    }
}

fn main() {
    // tutorials_common::run is only required to set up the application environent on macOS
    // (but not necessary in normal Cocoa applications where this is set up autmatically)
    examples_common::run(example_main);
}
