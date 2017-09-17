extern crate gstreamer as gst;
use gst::prelude::*;

fn main() {
    gst::init().unwrap();

    let identity = gst::ElementFactory::make("identity", None).unwrap();
    let mut iter = identity.iterate_pads().unwrap();
    while let Some(res) = iter.next() {
        match res {
            Ok(pad) => println!("Pad: {}", pad.get::<gst::Pad>().unwrap().get_name()),
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
