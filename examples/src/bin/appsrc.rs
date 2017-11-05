extern crate gstreamer as gst;
use gst::prelude::*;
extern crate gstreamer_app as gst_app;
extern crate gstreamer_video as gst_video;

use std::thread;

pub mod utils;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

fn create_pipeline() -> Result<(gst::Pipeline, gst_app::AppSrc), utils::ExampleError> {
    gst::init().map_err(utils::ExampleError::InitFailed)?;

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
        .dynamic_cast::<gst_app::AppSrc>()
        .expect("Source element is expected to be an appsrc!");

    let info = gst_video::VideoInfo::new(gst_video::VideoFormat::Bgrx, WIDTH as u32, HEIGHT as u32)
        .fps(gst::Fraction::new(2, 1))
        .build()
        .unwrap();

    appsrc.set_caps(&info.to_caps().unwrap());
    appsrc.set_property_format(gst::Format::Time);
    appsrc.set_max_bytes(1);
    appsrc.set_property_block(true);

    Ok((pipeline, appsrc))
}

fn main_loop() -> Result<(), utils::ExampleError> {
    let (pipeline, appsrc) = create_pipeline()?;

    thread::spawn(move || {
        for i in 0..100 {
            println!("Producing frame {}", i);

            let r = if i % 2 == 0 { 0 } else { 255 };
            let g = if i % 3 == 0 { 0 } else { 255 };
            let b = if i % 5 == 0 { 0 } else { 255 };

            let mut buffer = gst::Buffer::with_size(WIDTH * HEIGHT * 4).unwrap();
            {
                let buffer = buffer.get_mut().unwrap();
                buffer.set_pts(i * 500 * gst::MSECOND);

                let mut data = buffer.map_writable().unwrap();

                for p in data.as_mut_slice().chunks_mut(4) {
                    assert_eq!(p.len(), 4);
                    p[0] = b;
                    p[1] = g;
                    p[2] = r;
                    p[3] = 0;
                }
            }

            if appsrc.push_buffer(buffer) != gst::FlowReturn::Ok {
                break;
            }
        }

        let _ = appsrc.end_of_stream();
    });

    utils::set_state(&pipeline, gst::State::Playing)?;

    let bus = pipeline
        .get_bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    while let Some(msg) = bus.timed_pop(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;

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
