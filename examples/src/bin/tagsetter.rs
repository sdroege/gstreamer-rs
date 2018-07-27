extern crate gstreamer as gst;
use gst::prelude::*;

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
struct MissingElement(String);

#[derive(Debug, Fail)]
#[fail(
    display = "Received error from {}: {} (debug: {:?})",
    src,
    error,
    debug
)]
struct ErrorMessage {
    src: String,
    error: String,
    debug: Option<String>,
    #[cause]
    cause: glib::Error,
}

fn example_main() -> Result<(), Error> {
    gst::init()?;

    let mut context = gst::ParseContext::new();
    let pipeline = match gst::parse_launch_full(
        "audiotestsrc wave=white-noise num-buffers=100 ! flacenc ! filesink location=test.flac",
        Some(&mut context),
        gst::ParseFlags::NONE,
    ) {
        Ok(pipeline) => pipeline,
        Err(err) => {
            if let Some(gst::ParseError::NoSuchElement) = err.kind::<gst::ParseError>() {
                return Err(MissingElement(context.get_missing_elements().join(",")).into());
            } else {
                return Err(err.into());
            }
        }
    };

    let pipeline = pipeline
        .downcast::<gst::Pipeline>()
        .map_err(|_| failure::err_msg("Generated pipeline is no pipeline"))?;

    let tagsetter = pipeline
        .get_by_interface(gst::TagSetter::static_type())
        .ok_or_else(|| failure::err_msg("No TagSetter found"))?;
    let tagsetter = tagsetter
        .dynamic_cast::<gst::TagSetter>()
        .map_err(|_| failure::err_msg("No TagSetter found"))?;

    tagsetter.set_tag_merge_mode(gst::TagMergeMode::KeepAll);
    tagsetter.add::<gst::tags::Title>(&"Special randomized white-noise", gst::TagMergeMode::Append);

    let bus = pipeline.get_bus().unwrap();

    pipeline.set_state(gst::State::Playing).into_result()?;

    while let Some(msg) = bus.timed_pop(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                Err(ErrorMessage {
                    src: err
                        .get_src()
                        .map(|s| s.get_path_string())
                        .unwrap_or_else(|| String::from("None")),
                    error: err.get_error().description().into(),
                    debug: err.get_debug(),
                    cause: err.get_error(),
                })?;
                break;
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null).into_result()?;

    Ok(())
}

fn main() {
    // tutorials_common::run is only required to set up the application environent on macOS
    // (but not necessary in normal Cocoa applications where this is set up autmatically)
    match examples_common::run(example_main) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}
