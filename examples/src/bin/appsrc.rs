extern crate gstreamer as gst;
use gst::*;
extern crate gstreamer_app as gst_app;
use gst_app::*;

extern crate glib;

use std::fmt;
use std::u64;
use std::thread;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

#[derive(Debug)]
enum AppSrcExError {
    InitFailed(glib::Error),
    ElementNotFound(&'static str),
    ElementLinkFailed(&'static str, &'static str),
    SetStateError(&'static str),
    ElementError(std::string::String, glib::Error, std::string::String),
}

impl fmt::Display for AppSrcExError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppSrcExError::InitFailed(ref e) => {
                write!(f, "GStreamer initialization failed: {:?}", e)
            }
            AppSrcExError::ElementNotFound(ref e) => write!(f, "Element {} not found", e),
            AppSrcExError::ElementLinkFailed(ref e1, ref e2) => {
                write!(f, "Link failed between {} and {}", e1, e2)
            }
            AppSrcExError::SetStateError(ref state) => {
                write!(f, "Pipeline failed to switch to state {}", state)
            }
            AppSrcExError::ElementError(ref element, ref err, ref debug) => {
                write!(f, "Error from {}: {} ({:?})", element, err, debug)
            }
        }
    }
}

fn create_pipeline() -> Result<(Pipeline, AppSrc), AppSrcExError> {
    gst::init().map_err(|e| AppSrcExError::InitFailed(e))?;

    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("appsrc", None)
        .ok_or(AppSrcExError::ElementNotFound("appsrc"))?;

    let videoconvert = gst::ElementFactory::make("videoconvert", None)
        .ok_or(AppSrcExError::ElementNotFound("videoconvert"))?;
    let sink = gst::ElementFactory::make("autovideosink", None)
        .ok_or(AppSrcExError::ElementNotFound("autovideosink"))?;

    pipeline
        .add_many(&[&src, &videoconvert, &sink])
        .expect("Unable to add elements in the pipeline");
    gst::Element::link(&src, &videoconvert)
        .map_err(|_| AppSrcExError::ElementLinkFailed("src", "videoconvert"))?;
    gst::Element::link(&videoconvert, &sink)
        .map_err(|_| AppSrcExError::ElementLinkFailed("videoconvert", "sink"))?;

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

fn main_loop() -> Result<(), AppSrcExError> {
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

    if let gst::StateChangeReturn::Failure = pipeline.set_state(gst::State::Playing) {
        return Err(AppSrcExError::SetStateError("playing"));
    }

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
                pipeline.set_state(gst::State::Null);
                return Err(AppSrcExError::ElementError(
                    msg.get_src().get_path_string(),
                    err.get_error(),
                    err.get_debug().unwrap(),
                ));
            }
            _ => (),
        }
    }

    if let gst::StateChangeReturn::Failure = pipeline.set_state(gst::State::Null) {
        return Err(AppSrcExError::SetStateError("null"));
    }

    Ok(())
}

fn main() {
    match main_loop() {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}
