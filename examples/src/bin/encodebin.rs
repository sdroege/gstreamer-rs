#[macro_use]
extern crate gstreamer as gst;
use gst::prelude::*;

extern crate gstreamer_pbutils as pbutils;
use pbutils::EncodingProfileBuilder;

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
#[fail(display = "Received error from {}: {} (debug: {:?})", src, error, debug)]
struct ErrorMessage {
    src: String,
    error: String,
    debug: Option<String>,
    #[cause]
    cause: glib::Error,
}

fn configure_encodebin(encodebin: &gst::Element) -> Result<(), Error> {
    let audio_profile = pbutils::EncodingAudioProfileBuilder::new()
        .format(&gst::Caps::new_simple(
            "audio/x-vorbis",
            &[],
        ))
        .presence(0)
        .build()?;

    let video_profile = pbutils::EncodingVideoProfileBuilder::new()
        .format(&gst::Caps::new_simple(
            "video/x-theora",
            &[],
        ))
        .presence(0)
        .build()?;

    let container_profile = pbutils::EncodingContainerProfileBuilder::new()
        .name("container")
        .format(&gst::Caps::new_simple(
            "video/x-matroska",
            &[],
        ))
        .add_profile(&(video_profile.upcast()))
        .add_profile(&(audio_profile.upcast()))
        .build()?;

    encodebin.set_property("profile", &container_profile)?;

    Ok(())
}

fn example_main() -> Result<(), Error> {
    gst::init()?;

    let args: Vec<_> = env::args().collect();
    let uri: &str;
    let output_file: &str;

    if args.len() == 3 {
        uri = args[1].as_ref();
        output_file = args[2].as_ref();
    } else {
        println!("Usage: encodebin URI output_file");
        std::process::exit(-1)
    };

    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("uridecodebin", None).ok_or(MissingElement("uridecodebin"))?;
    let encodebin = gst::ElementFactory::make("encodebin", None).ok_or(MissingElement("encodebin"))?;
    let sink = gst::ElementFactory::make("filesink", None).ok_or(MissingElement("filesink"))?;

    src.set_property("uri", &uri)?;
    sink.set_property("location", &output_file)?;

    configure_encodebin(&encodebin)?;

    pipeline.add_many(&[&src, &encodebin, &sink])?;
    gst::Element::link_many(&[&encodebin, &sink])?;

    // Need to move a new reference into the closure
    let pipeline_clone = pipeline.clone();
    src.connect_pad_added(move |dbin, dbin_src_pad| {
        let pipeline = &pipeline_clone;

        let (is_audio, is_video) = {
            let media_type = dbin_src_pad.get_current_caps().and_then(|caps| {
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
                        ("Failed to get media type from pad {}", dbin_src_pad.get_name())
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

                let elements = &[&queue, &convert, &resample];
                pipeline.add_many(elements)?;
                gst::Element::link_many(elements)?;

                let enc_sink_pad = encodebin.get_request_pad("audio_%u").expect("Could not get audio pad from encodebin");
                let src_pad = resample.get_static_pad("src").expect("resample has no srcpad");
                src_pad.link(&enc_sink_pad).into_result()?;

                for e in elements {
                    e.sync_state_with_parent()?;
                }

                let sink_pad = queue.get_static_pad("sink").expect("queue has no sinkpad");
                dbin_src_pad.link(&sink_pad).into_result()?;
            } else if is_video {
                let queue =
                    gst::ElementFactory::make("queue", None).ok_or(MissingElement("queue"))?;
                let convert = gst::ElementFactory::make("videoconvert", None)
                    .ok_or(MissingElement("videoconvert"))?;
                let scale = gst::ElementFactory::make("videoscale", None)
                    .ok_or(MissingElement("videoscale"))?;

                let elements = &[&queue, &convert, &scale];
                pipeline.add_many(elements)?;
                gst::Element::link_many(elements)?;

                let enc_sink_pad = encodebin.get_request_pad("video_%u").expect("Could not get video pad from encodebin");
                let src_pad = scale.get_static_pad("src").expect("videoscale has no srcpad");
                src_pad.link(&enc_sink_pad).into_result()?;

                for e in elements {
                    e.sync_state_with_parent()?
                }

                let sink_pad = queue.get_static_pad("sink").expect("queue has no sinkpad");
                dbin_src_pad.link(&sink_pad).into_result()?;
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
                                   &glib::AnySendValue::new(Arc::new(Mutex::new(Some(err)))))
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
                            .get::<&glib::AnySendValue>("error")
                            .cloned()
                            .and_then(|v| {
                                v.downcast_ref::<Arc<Mutex<Option<Error>>>>()
                                    .and_then(|v| v.lock().unwrap().take())
                            })
                            .map(Result::Err)
                            .expect("error-details message without actual error"),
                        _ => Err(ErrorMessage {
                            src: err.get_src()
                                .map(|s| s.get_path_string())
                                .unwrap_or_else(|| String::from("None")),
                            error: err.get_error().description().into(),
                            debug: err.get_debug(),
                            cause: err.get_error(),
                        }.into()),
                    }?;
                }
                #[cfg(not(feature = "v1_10"))]
                {
                    Err(ErrorMessage {
                        src: err.get_src()
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
