// This example demonstrates the use of the appsink element.
// It operates the following pipeline:

// {audiotestsrc} - {appsink}

// The application specifies what format it wants to handle. This format
// is applied by calling set_caps on the appsink. Now it's the audiotestsrc's
// task to provide this data format. If the element connected to the appsink's
// sink-pad were not able to provide what we ask them to, this would fail.
// This is the format we request:
// Audio / Signed 16bit / 1 channel / arbitrary sample rate

#[macro_use]
extern crate gstreamer as gst;
use gst::prelude::*;
extern crate gstreamer_app as gst_app;
extern crate gstreamer_audio as gst_audio;

extern crate glib;

extern crate byte_slice_cast;
use byte_slice_cast::*;

use std::error::Error as StdError;
use std::i16;
use std::i32;

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

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    gst::init()?;

    let pipeline = gst::Pipeline::new(None);
    let src =
        gst::ElementFactory::make("audiotestsrc", None).ok_or(MissingElement("audiotestsrc"))?;
    let sink = gst::ElementFactory::make("appsink", None).ok_or(MissingElement("appsink"))?;

    pipeline.add_many(&[&src, &sink])?;
    src.link(&sink)?;

    let appsink = sink
        .dynamic_cast::<gst_app::AppSink>()
        .expect("Sink element is expected to be an appsink!");

    // Tell the appsink what format we want. It will then be the audiotestsrc's job to
    // provide the format we request.
    // This can be set after linking the two objects, because format negotiation between
    // both elements will happen during pre-rolling of the pipeline.
    appsink.set_caps(Some(&gst::Caps::new_simple(
        "audio/x-raw",
        &[
            ("format", &gst_audio::AUDIO_FORMAT_S16.to_str()),
            ("layout", &"interleaved"),
            ("channels", &(1i32)),
            ("rate", &gst::IntRange::<i32>::new(1, i32::MAX)),
        ],
    )));

    // Getting data out of the appsink is done by setting callbacks on it.
    // The appsink will then call those handlers, as soon as data is available.
    appsink.set_callbacks(
        gst_app::AppSinkCallbacks::new()
            // Add a handler to the "new-sample" signal.
            .new_sample(|appsink| {
                // Pull the sample in question out of the appsink's buffer.
                let sample = appsink.pull_sample().ok_or(gst::FlowError::Eos)?;
                let buffer = sample.get_buffer().ok_or_else(|| {
                    gst_element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Failed to get buffer from appsink")
                    );

                    gst::FlowError::Error
                })?;

                // At this point, buffer is only a reference to an existing memory region somewhere.
                // When we want to access its content, we have to map it while requesting the required
                // mode of access (read, read/write).
                // This type of abstraction is necessary, because the buffer in question might not be
                // on the machine's main memory itself, but rather in the GPU's memory.
                // So mapping the buffer makes the underlying memory region accessible to us.
                // See: https://gstreamer.freedesktop.org/documentation/plugin-development/advanced/allocation.html
                let map = buffer.map_readable().ok_or_else(|| {
                    gst_element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Failed to map buffer readable")
                    );

                    gst::FlowError::Error
                })?;

                // We know what format the data in the memory region has, since we requested
                // it by setting the appsink's caps. So what we do here is interpret the
                // memory region we mapped as an array of signed 16 bit integers.
                let samples = map.as_slice_of::<i16>().map_err(|_| {
                    gst_element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Failed to interprete buffer as S16 PCM")
                    );

                    gst::FlowError::Error
                })?;

                // For buffer (= chunk of samples), we calculate the root mean square:
                // (https://en.wikipedia.org/wiki/Root_mean_square)
                let sum: f64 = samples
                    .iter()
                    .map(|sample| {
                        let f = f64::from(*sample) / f64::from(i16::MAX);
                        f * f
                    })
                    .sum();
                let rms = (sum / (samples.len() as f64)).sqrt();
                println!("rms: {}", rms);

                Ok(gst::FlowSuccess::Ok)
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
