// This example shows how to use the appsrc element.
// It operates the following pipeline:

// {appsrc} - {videoconvert} - {autovideosink}

// The application itself provides the video-data for the pipeline, by providing
// it in the callback of the appsrc element. Videoconvert makes sure that the
// format the application provides can be displayed by the autovideosink
// at the end of the pipeline.
// The application provides data of the following format:
// Video / BGRx (4 bytes) / 2 fps

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
#[fail(
    display = "Received error from {}: {} (debug: {:?})",
    src, error, debug
)]
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

    let appsrc = src
        .dynamic_cast::<gst_app::AppSrc>()
        .expect("Source element is expected to be an appsrc!");

    // Specify the format we want to provide as application into the pipeline
    // by creating a video info with the given format and creating caps from it for the appsrc element.
    let video_info =
        gst_video::VideoInfo::new(gst_video::VideoFormat::Bgrx, WIDTH as u32, HEIGHT as u32)
            .fps(gst::Fraction::new(2, 1))
            .build()
            .expect("Failed to create video info");

    appsrc.set_caps(Some(&video_info.to_caps().unwrap()));
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
        // Since our appsrc element operates in pull mode (it asks us to provide data),
        // we add a handler for the need-data callback and provide new data from there.
        // In our case, we told gstreamer that we do 2 frames per second. While the
        // buffers of all elements of the pipeline are still empty, this will be called
        // a couple of times until all of them are filled. After this initial period,
        // this handler will be called (on average) twice per second.
        gst_app::AppSrcCallbacks::new()
            .need_data(move |appsrc, _| {
                // We only produce 100 frames
                if i == 100 {
                    let _ = appsrc.end_of_stream();
                    return;
                }

                println!("Producing frame {}", i);

                let r = if i % 2 == 0 { 0 } else { 255 };
                let g = if i % 3 == 0 { 0 } else { 255 };
                let b = if i % 5 == 0 { 0 } else { 255 };

                // Create the buffer that can hold exactly one BGRx frame.
                let mut buffer = gst::Buffer::with_size(video_info.size()).unwrap();
                {
                    let buffer = buffer.get_mut().unwrap();
                    // For each frame we produce, we set the timestamp when it should be displayed
                    // (pts = presentation time stamp)
                    // The autovideosink will use this information to display the frame at the right time.
                    buffer.set_pts(i * 500 * gst::MSECOND);

                    // At this point, buffer is only a reference to an existing memory region somewhere.
                    // When we want to access its content, we have to map it while requesting the required
                    // mode of access (read, read/write).
                    // See: https://gstreamer.freedesktop.org/documentation/plugin-development/advanced/allocation.html
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
    pipeline.set_state(gst::State::Playing)?;

    let bus = pipeline
        .get_bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null)?;
                return Err(ErrorMessage {
                    src: msg
                        .get_src()
                        .map(|s| String::from(s.get_path_string()))
                        .unwrap_or_else(|| String::from("None")),
                    error: err.get_error().description().into(),
                    debug: Some(err.get_debug().unwrap().to_string()),
                    cause: err.get_error(),
                }
                .into());
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null)?;

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
