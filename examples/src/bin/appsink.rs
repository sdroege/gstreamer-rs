extern crate gstreamer as gst;
use gst::*;
extern crate gstreamer_app as gst_app;
use gst_app::*;

extern crate glib;

use std::fmt;
use std::u64;
use std::i16;
use std::i32;

#[derive(Debug)]
enum AppSinkExError {
    InitFailed(glib::Error),
    ElementNotFound(&'static str),
    ElementLinkFailed(&'static str, &'static str),
    SetStateError(&'static str),
    ElementError(std::string::String, glib::Error, std::string::String),
}

impl fmt::Display for AppSinkExError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppSinkExError::InitFailed(ref e) => {
                write!(f, "GStreamer initialization failed: {:?}", e)
            }
            AppSinkExError::ElementNotFound(ref e) => write!(f, "Element {} not found", e),
            AppSinkExError::ElementLinkFailed(ref e1, ref e2) => {
                write!(f, "Link failed between {} and {}", e1, e2)
            }
            AppSinkExError::SetStateError(ref state) => {
                write!(f, "Pipeline failed to switch to state {}", state)
            }
            AppSinkExError::ElementError(ref element, ref err, ref debug) => {
                write!(f, "Error from {}: {} ({:?})", element, err, debug)
            }
        }
    }
}

fn create_pipeline() -> Result<Pipeline, AppSinkExError> {
    gst::init().map_err(|e| AppSinkExError::InitFailed(e))?;
    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("audiotestsrc", None)
        .ok_or(AppSinkExError::ElementNotFound("audiotestsrc"))?;
    let sink = gst::ElementFactory::make("appsink", None)
        .ok_or(AppSinkExError::ElementNotFound("appsink"))?;

    pipeline
        .add_many(&[&src, &sink])
        .expect("Unable to add elements in the pipeline");

    gst::Element::link(&src, &sink)
        .map_err(|_| {
            AppSinkExError::ElementLinkFailed("audiotestsrc", "appsink")
        })?;

    let appsink = sink.clone()
        .dynamic_cast::<AppSink>()
        .expect("Sink element is expected to be an appsink!");

    appsink.set_caps(&Caps::new_simple(
        "audio/x-raw",
        &[
            ("format", &"S16BE"),
            ("layout", &"interleaved"),
            ("channels", &(1i32)),
            ("rate", &IntRange::<i32>::new(1, i32::MAX)),
        ],
    ));

    appsink.set_callbacks(AppSinkCallbacks::new(
        /* eos */
        |_| {},
        /* new_preroll */
        |_| FlowReturn::Ok,
        /* new_samples */
        |appsink| {
            let sample = match appsink.pull_sample() {
                None => return FlowReturn::Eos,
                Some(sample) => sample,
            };

            let buffer = sample
                .get_buffer()
                .expect("Unable to extract buffer from the sample");
            assert_eq!(buffer.get_size() % 2, 0);
            let map = buffer.map_read().expect("Unable to map buffer for reading");
            let data = map.as_slice();
            let sum: f64 = data.chunks(2)
                .map(|sample| {
                    let u: u16 = ((sample[0] as u16) << 8) | (sample[1] as u16);
                    let f = (u as i16 as f64) / (i16::MAX as f64);
                    f * f
                })
                .sum();
            let rms = (sum / ((data.len() / 2) as f64)).sqrt();
            println!("rms: {}", rms);

            FlowReturn::Ok
        },
    ));

    Ok(pipeline)
}

fn main_loop() -> Result<(), AppSinkExError> {
    let pipeline = create_pipeline()?;

    if let gst::StateChangeReturn::Failure = pipeline.set_state(gst::State::Playing) {
        return Err(AppSinkExError::SetStateError("playing"));
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
                if let gst::StateChangeReturn::Failure = pipeline.set_state(gst::State::Null) {
                    return Err(AppSinkExError::SetStateError("null"));
                }
                return Err(AppSinkExError::ElementError(
                    msg.get_src().get_path_string(),
                    err.get_error(),
                    err.get_debug().unwrap(),
                ));
            }
            _ => (),
        }
    }

    if let gst::StateChangeReturn::Failure = pipeline.set_state(gst::State::Null) {
        return Err(AppSinkExError::SetStateError("null"));
    }

    Ok(())
}

fn main() {
    match main_loop() {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}
