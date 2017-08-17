extern crate gstreamer as gst;
#[cfg(feature = "tokio")]
use gst::*;

#[cfg(feature = "tokio")]
extern crate futures;
#[cfg(feature = "tokio")]
use futures::stream::Stream;
#[cfg(feature = "tokio")]
extern crate tokio_core;
#[cfg(feature = "tokio")]
use tokio_core::reactor::Core;

#[cfg(feature = "tokio")]
use std::env;

#[cfg(feature = "tokio")]
fn main() {
    let pipeline_str = env::args().collect::<Vec<String>>()[1..].join(" ");

    gst::init().unwrap();

    let mut core = Core::new().unwrap();

    let pipeline = gst::parse_launch(&pipeline_str).unwrap();
    let bus = pipeline.get_bus().unwrap();

    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    let messages = BusStream::new(&bus).for_each(|msg| {
        let quit = match msg.view() {
            MessageView::Eos(..) => true,
            MessageView::Error(err) => {
                println!(
                    "Error from {}: {} ({:?})",
                    msg.get_src().get_path_string(),
                    err.get_error(),
                    err.get_debug()
                );
                true
            }
            _ => false,
        };

        if quit {
            Err(())
        } else {
            Ok(())
        }
    });

    let _ = core.run(messages);

    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}

#[cfg(not(feature = "tokio"))]
fn main() {
    println!("Please compile with --features tokio");
}
