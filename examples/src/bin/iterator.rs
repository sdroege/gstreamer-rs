// This example demonstrates how to use GStreamer's iteration APIs.
// This is used at multiple occasions - for example to iterate an
// element's pads.

use gst::prelude::*;

#[path = "../examples-common.rs"]
mod examples_common;

fn example_main() {
    gst::init().unwrap();

    // Create and use an identity element here.
    // This element does nothing, really. We also never add it to a pipeline.
    // We just want to iterate the identity element's pads.
    let identity = gst::ElementFactory::make("identity").build().unwrap();
    // Get an iterator over all pads of the identity-element.
    let mut iter = identity.iterate_pads();
    loop {
        // In an endless-loop, we use the iterator until we either reach the end
        // or we hit an error.
        match iter.next() {
            Ok(Some(pad)) => println!("Pad: {}", pad.name()),
            Ok(None) => {
                // We reached the end of the iterator, there are no more pads
                println!("Done");
                break;
            }
            // It is very important to handle this resync error by calling resync
            // on the iterator. This error happens, when the container that is iterated
            // changed during iteration. (e.g. a pad was added while we used the
            // iterator to iterate over all of an element's pads).
            // After calling resync on the iterator, iteration will start from the beginning
            // again. So the application should be able to handle that.
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
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    examples_common::run(example_main);
}
