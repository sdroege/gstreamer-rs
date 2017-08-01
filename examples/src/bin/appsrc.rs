extern crate gstreamer as gst;
use gst::*;
extern crate gstreamer_app as gst_app;
use gst_app::*;

extern crate glib;
use glib::*;

use std::u64;
use std::thread;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

fn main() {
    gst::init().unwrap();

    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("appsrc", None).unwrap();
    let videoconvert = gst::ElementFactory::make("videoconvert", None).unwrap();
    let sink = gst::ElementFactory::make("autovideosink", None).unwrap();

    pipeline.add_many(&[&src, &videoconvert, &sink]).unwrap();
    gst::Element::link_many(&[&src, &videoconvert, &sink]).unwrap();

    let appsrc = src.clone().dynamic_cast::<AppSrc>().unwrap();
    appsrc.set_caps(&Caps::new_simple(
        "video/x-raw",
        &[
            (&"format", &"BGRx"),
            (&"width", &(WIDTH as i32)),
            (&"height", &(HEIGHT as i32)),
            (&"framerate", &Fraction::new(2, 1)),
        ],
    ));
    appsrc.set_property_format(Format::Time);
    appsrc.set_max_bytes(1);
    appsrc.set_property_block(true);

    thread::spawn(move || {
        for i in 0..100 {
            println!("Producing frame {}", i);

            // TODO: This is not very efficient
            let mut vec = Vec::with_capacity(WIDTH * HEIGHT * 4);
            let r = if i % 2 == 0 { 0 } else { 255 };
            let g = if i % 3 == 0 { 0 } else { 255 };
            let b = if i % 5 == 0 { 0 } else { 255 };

            for _ in 0..(320 * 240) {
                vec.push(b);
                vec.push(g);
                vec.push(r);
                vec.push(0);
            }

            let mut buffer = Buffer::from_vec(vec).unwrap();
            buffer.get_mut().unwrap().set_pts(i * 500_000_000);

            if appsrc.push_buffer(buffer) != FlowReturn::Ok {
                break;
            }
        }

        appsrc.end_of_stream();
    });

    assert_ne!(
        pipeline.set_state(gst::State::Playing),
        gst::StateChangeReturn::Failure
    );

    let bus = pipeline.get_bus().unwrap();

    loop {
        let msg = match bus.timed_pop(u64::MAX) {
            None => break,
            Some(msg) => msg,
        };

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {}: {} ({:?})",
                    msg.get_src().get_path_string(),
                    err.get_error(),
                    err.get_debug()
                );
                break;
            }
            _ => (),
        }
    }

    assert_ne!(
        pipeline.set_state(gst::State::Null),
        gst::StateChangeReturn::Failure
    );
}
