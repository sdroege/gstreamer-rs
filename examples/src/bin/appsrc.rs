extern crate gstreamer as gst;
use gst::*;
extern crate gstreamer_app as gst_app;
use gst_app::*;

extern crate glib;

use std::u64;
use std::thread;

pub mod utils;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

fn create_pipeline() -> Result<(Pipeline, AppSrc), utils::ExampleError> {
    gst::init().map_err(|e| utils::ExampleError::InitFailed(e))?;

    let pipeline = gst::Pipeline::new(None);
    let src = utils::create_element("appsrc")?;
    let videoconvert = utils::create_element("videoconvert")?;
    let sink = utils::create_element("autovideosink")?;

    pipeline
        .add_many(&[&src, &videoconvert, &sink])
        .expect("Unable to add elements in the pipeline");
    utils::link_elements(&src, &videoconvert)?;
    utils::link_elements(&videoconvert, &sink)?;

    let appsrc = src.clone()
        .dynamic_cast::<AppSrc>()
        .expect("Source element is expected to be an appsrc!");
    appsrc.set_caps(&Caps::new_simple(
        "video/x-raw",
        &[
            ("format", &"BGRx"),
            ("width", &(WIDTH as i32)),
            ("height", &(HEIGHT as i32)),
            ("framerate", &Fraction::new(2, 1)),
        ],
    ));
    appsrc.set_property_format(Format::Time);
    appsrc.set_max_bytes(1);
    appsrc.set_property_block(true);

    Ok((pipeline, appsrc))
}

fn main_loop() -> Result<(), utils::ExampleError> {
    let (pipeline, appsrc) = create_pipeline()?;

    thread::spawn(move || {
        for i in 0..100 {
            println!("Producing frame {}", i);

            // TODO: This is not very efficient
            let mut vec = Vec::with_capacity(WIDTH * HEIGHT * 4);
            let r = if i % 2 == 0 { 0 } else { 255 };
            let g = if i % 3 == 0 { 0 } else { 255 };
            let b = if i % 5 == 0 { 0 } else { 255 };

            for _ in 0..(WIDTH * HEIGHT) {
                vec.push(b);
                vec.push(g);
                vec.push(r);
                vec.push(0);
            }

            let mut buffer = Buffer::from_vec(vec).expect("Unable to create a Buffer");
            buffer.get_mut().unwrap().set_pts(i * 500_000_000);

            if appsrc.push_buffer(buffer) != FlowReturn::Ok {
                break;
            }
        }

        appsrc.end_of_stream();
    });

    utils::set_state(&pipeline, gst::State::Playing)?;

    let bus = pipeline
        .get_bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    loop {
        let msg = match bus.timed_pop(u64::MAX) {
            None => break,
            Some(msg) => msg,
        };

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                utils::set_state(&pipeline, gst::State::Null)?;
                return Err(utils::ExampleError::ElementError(
                    msg.get_src().get_path_string(),
                    err.get_error(),
                    err.get_debug().unwrap(),
                ));
            }
            _ => (),
        }
    }

    utils::set_state(&pipeline, gst::State::Null)?;

    Ok(())
}

fn main() {
    match main_loop() {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}
