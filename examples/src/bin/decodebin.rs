#[macro_use]
extern crate gstreamer as gst;
use gst::prelude::*;

#[macro_use]
extern crate glib;

use std::env;
use std::error::Error as StdError;
#[cfg(feature = "v1_10")]
use std::sync::{Arc, Mutex};

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

#[cfg(feature = "v1_10")]
#[derive(Clone, Debug)]
struct ErrorValue(Arc<Mutex<Option<Error>>>);

#[cfg(feature = "v1_10")]
impl glib::subclass::boxed::BoxedType for ErrorValue {
    const NAME: &'static str = "ErrorValue";

    glib_boxed_type!();
}

#[cfg(feature = "v1_10")]
glib_boxed_derive_traits!(ErrorValue);

fn example_main() -> Result<(), Error> {
    gst::init()?;

    let args: Vec<_> = env::args().collect();
    let uri: &str = if args.len() == 2 {
        args[1].as_ref()
    } else {
        println!("Usage: decodebin file_path");
        std::process::exit(-1)
    };

    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("filesrc", None).ok_or(MissingElement("filesrc"))?;
    let decodebin =
        gst::ElementFactory::make("decodebin", None).ok_or(MissingElement("decodebin"))?;

    src.set_property("location", &uri)?;

    pipeline.add_many(&[&src, &decodebin])?;
    gst::Element::link_many(&[&src, &decodebin])?;

    let pipeline_weak = pipeline.downgrade();
    decodebin.connect_pad_added(move |dbin, src_pad| {
        let pipeline = match pipeline_weak.upgrade() {
            Some(pipeline) => pipeline,
            None => return,
        };

        let (is_audio, is_video) = {
            let media_type = src_pad.get_current_caps().and_then(|caps| {
                caps.get_structure(0).map(|s| {
                    let name = s.get_name();
                    (name.starts_with("audio/"), name.starts_with("video/"))
                })
            });

            match media_type {
                None => {
                    gst_element_warning!(
                        dbin,
                        gst::CoreError::Negotiation,
                        ("Failed to get media type from pad {}", src_pad.get_name())
                    );

                    return;
                }
                Some(media_type) => media_type,
            }
        };

        let insert_sink = |is_audio, is_video| -> Result<(), Error> {
            if is_audio {
                let queue =
                    gst::ElementFactory::make("queue", None).ok_or(MissingElement("queue"))?;
                let convert = gst::ElementFactory::make("audioconvert", None)
                    .ok_or(MissingElement("audioconvert"))?;
                let resample = gst::ElementFactory::make("audioresample", None)
                    .ok_or(MissingElement("audioresample"))?;
                let sink = gst::ElementFactory::make("autoaudiosink", None)
                    .ok_or(MissingElement("autoaudiosink"))?;

                let elements = &[&queue, &convert, &resample, &sink];
                pipeline.add_many(elements)?;
                gst::Element::link_many(elements)?;

                for e in elements {
                    e.sync_state_with_parent()?;
                }

                let sink_pad = queue.get_static_pad("sink").expect("queue has no sinkpad");
                src_pad.link(&sink_pad).into_result()?;
            } else if is_video {
                let queue =
                    gst::ElementFactory::make("queue", None).ok_or(MissingElement("queue"))?;
                let convert = gst::ElementFactory::make("videoconvert", None)
                    .ok_or(MissingElement("videoconvert"))?;
                let scale = gst::ElementFactory::make("videoscale", None)
                    .ok_or(MissingElement("videoscale"))?;
                let sink = gst::ElementFactory::make("autovideosink", None)
                    .ok_or(MissingElement("autovideosink"))?;

                let elements = &[&queue, &convert, &scale, &sink];
                pipeline.add_many(elements)?;
                gst::Element::link_many(elements)?;

                for e in elements {
                    e.sync_state_with_parent()?
                }

                let sink_pad = queue.get_static_pad("sink").expect("queue has no sinkpad");
                src_pad.link(&sink_pad).into_result()?;
            }

            Ok(())
        };

        if let Err(err) = insert_sink(is_audio, is_video) {
            #[cfg(feature = "v1_10")]
            gst_element_error!(
                dbin,
                gst::LibraryError::Failed,
                ("Failed to insert sink"),
                details: gst::Structure::builder("error-details")
                            .field("error",
                                   &ErrorValue(Arc::new(Mutex::new(Some(err)))))
                            .build()
            );

            #[cfg(not(feature = "v1_10"))]
            gst_element_error!(
                dbin,
                gst::LibraryError::Failed,
                ("Failed to insert sink"),
                ["{}", err]
            );
        }
    });

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

                #[cfg(feature = "v1_10")]
                {
                    match err.get_details() {
                        Some(details) if details.get_name() == "error-details" => details
                            .get::<&ErrorValue>("error")
                            .and_then(|v| v.0.lock().unwrap().take())
                            .map(Result::Err)
                            .expect("error-details message without actual error"),
                        _ => Err(ErrorMessage {
                            src: err
                                .get_src()
                                .map(|s| s.get_path_string())
                                .unwrap_or_else(|| String::from("None")),
                            error: err.get_error().description().into(),
                            debug: err.get_debug(),
                            cause: err.get_error(),
                        }
                        .into()),
                    }?;
                }
                #[cfg(not(feature = "v1_10"))]
                {
                    Err(ErrorMessage {
                        src: err
                            .get_src()
                            .map(|s| s.get_path_string())
                            .unwrap_or_else(|| String::from("None")),
                        error: err.get_error().description().into(),
                        debug: err.get_debug(),
                        cause: err.get_error(),
                    })?;
                }
                break;
            }
            MessageView::StateChanged(s) => {
                println!(
                    "State changed from {:?}: {:?} -> {:?} ({:?})",
                    s.get_src().map(|s| s.get_path_string()),
                    s.get_old(),
                    s.get_current(),
                    s.get_pending()
                );
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
