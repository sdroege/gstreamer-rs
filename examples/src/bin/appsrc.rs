extern crate gstreamer as gst;
use gst::prelude::*;
extern crate gstreamer_app as gst_app;
extern crate gstreamer_video as gst_video;

extern crate glib;

use std::error::Error as StdError;

extern crate failure;
use failure::Error;

#[macro_use]
extern crate failure_derive;

#[path = "../examples-common.rs"]
mod examples_common;

#[derive(Debug, Fail)]
#[fail(display = "Missing element {}", _0)]
struct MissingElement(&'static str);

#[derive(Debug, Fail)]
#[fail(display = "Received error from {}: {} (debug: {:?})", src, error, debug)]
struct ErrorMessage {
    src: String,
    error: String,
    debug: Option<String>,
    #[cause]
    cause: glib::Error,
}

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    gst::init()?;

    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("appsrc", None).ok_or(MissingElement("appsrc"))?;
    let videoconvert =
        gst::ElementFactory::make("videoconvert", None).ok_or(MissingElement("videoconvert"))?;
    let sink =
        gst::ElementFactory::make("autovideosink", None).ok_or(MissingElement("autovideosink"))?;

    pipeline.add_many(&[&src, &videoconvert, &sink])?;
    gst::Element::link_many(&[&src, &videoconvert, &sink])?;

    let appsrc = src.clone()
        .dynamic_cast::<gst_app::AppSrc>()
        .expect("Source element is expected to be an appsrc!");

    let info = gst_video::VideoInfo::new(gst_video::VideoFormat::Bgrx, WIDTH as u32, HEIGHT as u32)
        .fps(gst::Fraction::new(2, 1))
        .build()
        .expect("Failed to create video info");

    appsrc.set_caps(&info.to_caps().unwrap());
    appsrc.set_property_format(gst::Format::Time);

    // Our frame counter, that is stored in the mutable environment
    // of the closure of the need-data callback
    //
    // Alternatively we could also simply start a new thread that
    // pushes a buffer to the appsrc whenever it wants to, but this
    // is not really needed here. It is *not required* to use the
    // need-data callback.
    let mut i = 0;
    appsrc.set_callbacks(
        gst_app::AppSrcCallbacks::new()
            .need_data(move |appsrc, _| {
                if i == 100 {
                    let _ = appsrc.end_of_stream();
                    return;
                }

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

                i += 1;

                // appsrc already handles the error here
                let _ = appsrc.push_buffer(buffer);
            })
            .build(),
    );

    Ok(pipeline)
}

fn main_loop(pipeline: gst::Pipeline) -> Result<(), Error> {
    pipeline.set_state(gst::State::Playing).into_result()?;

    let bus = pipeline
        .get_bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    while let Some(msg) = bus.timed_pop(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null).into_result()?;
                Err(ErrorMessage {
                    src: err.get_src()
                        .map(|s| s.get_path_string())
                        .unwrap_or_else(|| String::from("None")),
                    error: err.get_error().description().into(),
                    debug: err.get_debug(),
                    cause: err.get_error(),
                })?;
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null).into_result()?;

    Ok(())
}

fn example_main() {
    match create_pipeline().and_then(main_loop) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}

fn main() {
    // tutorials_common::run is only required to set up the application environent on macOS
    // (but not necessary in normal Cocoa applications where this is set up autmatically)
    examples_common::run(example_main);
}
