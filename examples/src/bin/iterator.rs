extern crate gstreamer as gst;
use gst::prelude::*;

#[path = "../examples-common.rs"]
mod examples_common;

fn example_main() {
    gst::init().unwrap();

    let identity = gst::ElementFactory::make("identity", None).unwrap();
    let mut iter = identity.iterate_pads();
    while let Some(res) = iter.next() {
        match res {
            Ok(pad) => println!("Pad: {}", pad.get_name()),
            Err(gst::IteratorError::Resync) => {
                println!("Iterator resync");
                iter.resync();
            }
            Err(gst::IteratorError::Error) => {
                println!("Error");
                break;
            }
        }
    }
}

fn main() {
    // tutorials_common::run is only required to set up the application environent on macOS
    // (but not necessary in normal Cocoa applications where this is set up autmatically)
    examples_common::run(example_main);
}
