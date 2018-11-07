#[macro_use]
extern crate gstreamer as gst;
use gst::prelude::*;

extern crate glib;

use std::env;
use std::error::Error as StdError;

#[path = "../examples-common.rs"]
mod examples_common;

extern crate failure;
use failure::Error;

#[macro_use]
extern crate failure_derive;

#[derive(Debug, Fail)]
#[fail(display = "Missing element {}", _0)]
struct MissingElement(&'static str);

#[derive(Debug, Fail)]
#[fail(display = "No such pad {} in {}", _0, _1)]
struct NoSuchPad(&'static str, String);

#[derive(Debug, Fail)]
#[fail(display = "Unknown payload type {}", _0)]
struct UnknownPT(u32);

#[derive(Debug, Fail)]
#[fail(display = "Usage: {} (play | record) DROP_PROBABILITY", _0)]
struct UsageError(String);

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

fn make_element<'a, P: Into<Option<&'a str>>>(
    factory_name: &'static str,
    element_name: P,
) -> Result<gst::Element, Error> {
    match gst::ElementFactory::make(factory_name, element_name.into()) {
        Some(elem) => Ok(elem),
        None => Err(Error::from(MissingElement(factory_name))),
    }
}

fn get_static_pad(element: &gst::Element, pad_name: &'static str) -> Result<gst::Pad, Error> {
    match element.get_static_pad(pad_name) {
        Some(pad) => Ok(pad),
        None => {
            let element_name = element.get_name();
            Err(Error::from(NoSuchPad(pad_name, element_name)))
        }
    }
}

fn get_request_pad(element: &gst::Element, pad_name: &'static str) -> Result<gst::Pad, Error> {
    match element.get_request_pad(pad_name) {
        Some(pad) => Ok(pad),
        None => {
            let element_name = element.get_name();
            Err(Error::from(NoSuchPad(pad_name, element_name)))
        }
    }
}

fn connect_rtpbin_srcpad(src_pad: &gst::Pad, sink: &gst::Element) -> Result<(), Error> {
    let name = src_pad.get_name();
    let split_name = name.split('_');
    let split_name = split_name.collect::<Vec<&str>>();
    let pt = split_name[5].parse::<u32>()?;

    match pt {
        96 => {
            let sinkpad = get_static_pad(sink, "sink")?;
            src_pad.link(&sinkpad).into_result()?;
            Ok(())
        }
        _ => Err(Error::from(UnknownPT(pt))),
    }
}

fn make_fec_decoder(rtpbin: &gst::Element, sess_id: u32) -> Result<gst::Element, Error> {
    let fecdec = make_element("rtpulpfecdec", None)?;
    let internal_storage = rtpbin
        .emit("get-internal-storage", &[&sess_id.to_value()])
        .unwrap()
        .unwrap();

    fecdec.set_property("storage", &internal_storage.to_value())?;
    fecdec.set_property("pt", &100u32.to_value())?;

    Ok(fecdec)
}

fn example_main() -> Result<(), Error> {
    gst::init()?;

    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        return Err(Error::from(UsageError(args[0].clone())));
    }

    let drop_probability = args[2].parse::<f32>()?;

    let pipeline = gst::Pipeline::new(None);
    let src = make_element("udpsrc", None)?;
    let netsim = make_element("netsim", None)?;
    let rtpbin = make_element("rtpbin", None)?;
    let depay = make_element("rtpvp8depay", None)?;
    let dec = make_element("vp8dec", None)?;
    let conv = make_element("videoconvert", None)?;
    let scale = make_element("videoscale", None)?;
    let filter = make_element("capsfilter", None)?;

    pipeline.add_many(&[&src, &netsim, &rtpbin, &depay, &dec, &conv, &scale, &filter])?;
    gst::Element::link_many(&[&depay, &dec, &conv, &scale, &filter])?;

    match args[1].as_str() {
        "play" => {
            let sink = make_element("autovideosink", None)?;
            pipeline.add(&sink)?;
            filter.link(&sink)?;
        }
        "record" => {
            let enc = make_element("x264enc", None)?;
            let mux = make_element("matroskamux", None)?;
            let sink = make_element("filesink", None)?;

            pipeline.add_many(&[&enc, &mux, &sink])?;
            gst::Element::link_many(&[&filter, &enc, &mux, &sink])?;
            sink.set_property("location", &"out.mkv".to_value())?;
            enc.set_property_from_str("tune", "zerolatency");
            eprintln!("Recording to out.mkv");
        }
        _ => return Err(Error::from(UsageError(args[0].clone()))),
    }

    src.link(&netsim)?;

    rtpbin.connect("new-storage", false, |values| {
        let storage = values[1].get::<gst::Element>().expect("Invalid argument");
        storage
            .set_property("size-time", &250_000_000u64.to_value())
            .unwrap();

        None
    })?;

    rtpbin.connect("request-pt-map", false, |values| {
        let pt = values[2].get::<u32>().expect("Invalid argument");
        match pt {
            100 => Some(
                gst::Caps::new_simple(
                    "application/x-rtp",
                    &[
                        ("media", &"video"),
                        ("clock-rate", &90000i32),
                        ("is-fec", &true),
                    ],
                )
                .to_value(),
            ),
            96 => Some(
                gst::Caps::new_simple(
                    "application/x-rtp",
                    &[
                        ("media", &"video"),
                        ("clock-rate", &90000i32),
                        ("encoding-name", &"VP8"),
                    ],
                )
                .to_value(),
            ),
            _ => None,
        }
    })?;

    rtpbin.connect("request-fec-decoder", false, |values| {
        let rtpbin = values[0].get::<gst::Element>().expect("Invalid argument");
        let sess_id = values[1].get::<u32>().expect("Invalid argument");

        match make_fec_decoder(&rtpbin, sess_id) {
            Ok(elem) => Some(elem.to_value()),
            Err(err) => {
                gst_element_error!(
                    rtpbin,
                    gst::LibraryError::Failed,
                    ("Failed to make FEC decoder"),
                    ["{}", err]
                );
                None
            }
        }
    })?;

    let srcpad = get_static_pad(&netsim, "src")?;
    let sinkpad = get_request_pad(&rtpbin, "recv_rtp_sink_0")?;
    srcpad.link(&sinkpad).into_result()?;

    let depay_weak = depay.downgrade();
    rtpbin.connect_pad_added(move |rtpbin, src_pad| {
        let depay = match depay_weak.upgrade() {
            Some(depay) => depay,
            None => return,
        };

        match connect_rtpbin_srcpad(&src_pad, &depay) {
            Ok(_) => (),
            Err(err) => {
                gst_element_error!(
                    rtpbin,
                    gst::LibraryError::Failed,
                    ("Failed to link srcpad"),
                    ["{}", err]
                );
                ()
            }
        }
    });

    let rtp_caps = gst::Caps::new_simple("application/x-rtp", &[("clock-rate", &90000i32)]);

    let video_caps =
        gst::Caps::new_simple("video/x-raw", &[("width", &1920i32), ("height", &1080i32)]);

    src.set_property("address", &"127.0.0.1".to_value())?;
    src.set_property("caps", &rtp_caps.to_value())?;
    netsim.set_property("drop-probability", &drop_probability.to_value())?;
    rtpbin.set_property("do-lost", &true.to_value())?;
    filter.set_property("caps", &video_caps.to_value())?;

    let bus = pipeline
        .get_bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    while let Some(msg) = bus.timed_pop(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                let ret = pipeline.set_state(gst::State::Null);
                assert_ne!(ret, gst::StateChangeReturn::Failure);

                return Err(ErrorMessage {
                    src: msg
                        .get_src()
                        .map(|s| s.get_path_string())
                        .unwrap_or_else(|| String::from("None")),
                    error: err.get_error().description().into(),
                    debug: err.get_debug(),
                    cause: err.get_error(),
                }
                .into());
            }
            MessageView::StateChanged(s) => match msg.get_src() {
                Some(element) => {
                    if element == pipeline && s.get_current() == gst::State::Playing {
                        eprintln!("PLAYING");
                        gst::debug_bin_to_dot_file(
                            &pipeline,
                            gst::DebugGraphDetails::all(),
                            "client-playing",
                        );
                    }
                }
                None => (),
            },
            _ => (),
        }
    }

    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    Ok(())
}

fn main() {
    match examples_common::run(example_main) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}
