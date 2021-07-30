use gst::element_error;
use gst::prelude::*;

#[path = "../examples-common.rs"]
mod examples_common;

use std::env;

use anyhow::Error;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "Missing element {}", _0)]
struct MissingElement(#[error(not(source))] &'static str);

#[derive(Debug, Display, Error)]
#[display(fmt = "No such pad {} in {}", _0, _1)]
struct NoSuchPad(&'static str, String);

#[derive(Debug, Display, Error)]
#[display(fmt = "Usage: {} URI FEC_PERCENTAGE", _0)]
struct UsageError(#[error(not(source))] String);

#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {}: {} (debug: {:?})", src, error, debug)]
struct ErrorMessage {
    src: String,
    error: String,
    debug: Option<String>,
    source: glib::Error,
}

fn make_element(
    factory_name: &'static str,
    element_name: Option<&str>,
) -> Result<gst::Element, Error> {
    match gst::ElementFactory::make(factory_name, element_name) {
        Ok(elem) => Ok(elem),
        Err(_) => Err(Error::from(MissingElement(factory_name))),
    }
}

#[doc(alias = "get_static_pad")]
fn static_pad(element: &gst::Element, pad_name: &'static str) -> Result<gst::Pad, Error> {
    match element.static_pad(pad_name) {
        Some(pad) => Ok(pad),
        None => {
            let element_name = element.name();
            Err(Error::from(NoSuchPad(pad_name, element_name.to_string())))
        }
    }
}

#[doc(alias = "get_request_pad")]
fn request_pad(element: &gst::Element, pad_name: &'static str) -> Result<gst::Pad, Error> {
    match element.request_pad_simple(pad_name) {
        Some(pad) => Ok(pad),
        None => {
            let element_name = element.name();
            Err(Error::from(NoSuchPad(pad_name, element_name.to_string())))
        }
    }
}

fn connect_decodebin_pad(src_pad: &gst::Pad, sink: &gst::Element) -> Result<(), Error> {
    let sinkpad = static_pad(sink, "sink")?;
    src_pad.link(&sinkpad)?;

    Ok(())
}

fn make_fec_encoder(fec_percentage: u32) -> Result<gst::Element, Error> {
    let fecenc = make_element("rtpulpfecenc", None)?;

    fecenc.set_property("pt", &100u32)?;
    fecenc.set_property("multipacket", &true)?;
    fecenc.set_property("percentage", &fec_percentage)?;

    Ok(fecenc)
}

fn example_main() -> Result<(), Error> {
    gst::init()?;

    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        return Err(Error::from(UsageError(args[0].clone())));
    }

    let uri = &args[1];
    let fec_percentage = args[2].parse::<u32>()?;

    let pipeline = gst::Pipeline::new(None);
    let src = make_element("uridecodebin", None)?;
    let conv = make_element("videoconvert", None)?;
    let q1 = make_element("queue", None)?;
    let enc = make_element("vp8enc", None)?;
    let q2 = make_element("queue", None)?;
    let pay = make_element("rtpvp8pay", None)?;
    let rtpbin = make_element("rtpbin", None)?;
    let sink = make_element("udpsink", None)?;

    pipeline.add_many(&[&src, &conv, &q1, &enc, &q2, &pay, &rtpbin, &sink])?;

    conv.link(&q1)?;
    q1.link(&enc)?;
    enc.link(&pay)?;
    pay.link(&q2)?;

    rtpbin.connect("request-fec-encoder", false, move |values| {
        let rtpbin = values[0]
            .get::<gst::Element>()
            .expect("rtpbin \"request-fec-encoder\" signal values[0]");

        match make_fec_encoder(fec_percentage) {
            Ok(elem) => Some(elem.to_value()),
            Err(err) => {
                element_error!(
                    rtpbin,
                    gst::LibraryError::Failed,
                    ("Failed to make FEC encoder"),
                    ["{}", err]
                );
                None
            }
        }
    })?;

    let srcpad = static_pad(&q2, "src")?;
    let sinkpad = request_pad(&rtpbin, "send_rtp_sink_0")?;
    srcpad.link(&sinkpad)?;

    let srcpad = static_pad(&rtpbin, "send_rtp_src_0")?;
    let sinkpad = static_pad(&sink, "sink")?;
    srcpad.link(&sinkpad)?;

    src.connect_pad_added(
        move |decodebin, src_pad| match connect_decodebin_pad(src_pad, &conv) {
            Ok(_) => (),
            Err(err) => {
                element_error!(
                    decodebin,
                    gst::LibraryError::Failed,
                    ("Failed to link decodebin srcpad"),
                    ["{}", err]
                );
            }
        },
    );

    let video_caps = gst::Caps::new_simple("video/x-raw", &[]);

    src.set_property_from_str("pattern", "ball");
    sink.set_property("host", &"127.0.0.1")?;
    sink.set_property("sync", &true)?;
    enc.set_property("keyframe-max-dist", &30i32)?;
    enc.set_property("threads", &12i32)?;
    enc.set_property("cpu-used", &(-16i32))?;
    enc.set_property("deadline", &1i64)?;
    enc.set_property_from_str("error-resilient", "default");
    src.set_property("expose-all-streams", &false)?;
    src.set_property("caps", &video_caps)?;
    src.set_property("uri", &uri)?;

    let bus = pipeline
        .bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline
                    .set_state(gst::State::Null)
                    .expect("Unable to set the pipeline to the `Null` state");

                return Err(ErrorMessage {
                    src: msg
                        .src()
                        .map(|s| String::from(s.path_string()))
                        .unwrap_or_else(|| String::from("None")),
                    error: err.error().to_string(),
                    debug: err.debug(),
                    source: err.error(),
                }
                .into());
            }
            MessageView::StateChanged(s) => {
                if let Some(element) = msg.src() {
                    if element == pipeline && s.current() == gst::State::Playing {
                        eprintln!("PLAYING");
                        gst::debug_bin_to_dot_file(
                            &pipeline,
                            gst::DebugGraphDetails::all(),
                            "server-playing",
                        );
                    }
                }
            }
            _ => (),
        }
    }

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");

    Ok(())
}

fn main() {
    match examples_common::run(example_main) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}
