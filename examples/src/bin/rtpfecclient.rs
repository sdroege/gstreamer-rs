use gst::element_error;
use gst::prelude::*;

use std::env;

#[path = "../examples-common.rs"]
mod examples_common;

use anyhow::Error;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "No such pad {} in {}", _0, _1)]
struct NoSuchPad(#[error(not(source))] &'static str, String);

#[derive(Debug, Display, Error)]
#[display(fmt = "Unknown payload type {}", _0)]
struct UnknownPT(#[error(not(source))] u32);

#[derive(Debug, Display, Error)]
#[display(fmt = "Usage: {} (play | record) DROP_PROBABILITY", _0)]
struct UsageError(#[error(not(source))] String);

#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {}: {} (debug: {:?})", src, error, debug)]
struct ErrorMessage {
    src: String,
    error: String,
    debug: Option<String>,
    source: glib::Error,
}

fn static_pad(element: &gst::Element, pad_name: &'static str) -> Result<gst::Pad, Error> {
    match element.static_pad(pad_name) {
        Some(pad) => Ok(pad),
        None => {
            let element_name = element.name();
            Err(Error::from(NoSuchPad(pad_name, element_name.to_string())))
        }
    }
}

fn request_pad(element: &gst::Element, pad_name: &'static str) -> Result<gst::Pad, Error> {
    match element.request_pad_simple(pad_name) {
        Some(pad) => Ok(pad),
        None => {
            let element_name = element.name();
            Err(Error::from(NoSuchPad(pad_name, element_name.to_string())))
        }
    }
}

fn connect_rtpbin_srcpad(src_pad: &gst::Pad, sink: &gst::Element) -> Result<(), Error> {
    let name = src_pad.name();
    let split_name = name.split('_');
    let split_name = split_name.collect::<Vec<&str>>();
    let pt = split_name[5].parse::<u32>()?;

    match pt {
        96 => {
            let sinkpad = static_pad(sink, "sink")?;
            src_pad.link(&sinkpad)?;
            Ok(())
        }
        _ => Err(Error::from(UnknownPT(pt))),
    }
}

fn make_fec_decoder(rtpbin: &gst::Element, sess_id: u32) -> Result<gst::Element, Error> {
    let internal_storage = rtpbin.emit_by_name::<glib::Object>("get-internal-storage", &[&sess_id]);
    let fecdec = gst::ElementFactory::make("rtpulpfecdec")
        .property("storage", &internal_storage)
        .property("pt", 100u32)
        .build()?;

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

    let rtp_caps = gst::Caps::builder("application/x-rtp")
        .field("clock-rate", 90000i32)
        .build();

    let video_caps = gst_video::VideoCapsBuilder::new()
        .width(1920)
        .height(1080)
        .build();

    let src = gst::ElementFactory::make("udpsrc")
        .property("address", "127.0.0.1")
        .property("caps", &rtp_caps)
        .build()?;
    let netsim = gst::ElementFactory::make("netsim")
        .property("drop-probability", drop_probability)
        .build()?;
    let rtpbin = gst::ElementFactory::make("rtpbin")
        .property("do-lost", true)
        .build()?;
    let depay = gst::ElementFactory::make("rtpvp8depay").build()?;
    let dec = gst::ElementFactory::make("vp8dec").build()?;
    let conv = gst::ElementFactory::make("videoconvert").build()?;
    let scale = gst::ElementFactory::make("videoscale").build()?;
    let filter = gst::ElementFactory::make("capsfilter")
        .property("caps", &video_caps)
        .build()?;

    pipeline.add_many(&[&src, &netsim, &rtpbin, &depay, &dec, &conv, &scale, &filter])?;
    gst::Element::link_many(&[&depay, &dec, &conv, &scale, &filter])?;

    match args[1].as_str() {
        "play" => {
            let sink = gst::ElementFactory::make("autovideosink").build()?;
            pipeline.add(&sink)?;
            filter.link(&sink)?;
        }
        "record" => {
            let enc = gst::ElementFactory::make("x264enc")
                .property_from_str("tune", "zerolatency")
                .build()?;
            let mux = gst::ElementFactory::make("matroskamux").build()?;
            let sink = gst::ElementFactory::make("filesink")
                .property("location", "out.mkv")
                .build()?;

            pipeline.add_many(&[&enc, &mux, &sink])?;
            gst::Element::link_many(&[&filter, &enc, &mux, &sink])?;
            eprintln!("Recording to out.mkv");
        }
        _ => return Err(Error::from(UsageError(args[0].clone()))),
    }

    src.link(&netsim)?;

    rtpbin.connect("new-storage", false, |values| {
        let storage = values[1]
            .get::<gst::Element>()
            .expect("rtpbin \"new-storage\" signal values[1]");
        storage.set_property("size-time", 250_000_000u64);

        None
    });

    rtpbin.connect("request-pt-map", false, |values| {
        let pt = values[2]
            .get::<u32>()
            .expect("rtpbin \"new-storage\" signal values[2]");
        match pt {
            100 => Some(
                gst::Caps::builder("application/x-rtp")
                    .field("media", "video")
                    .field("clock-rate", 90000i32)
                    .field("is-fec", true)
                    .build()
                    .to_value(),
            ),
            96 => Some(
                gst::Caps::builder("application/x-rtp")
                    .field("media", "video")
                    .field("clock-rate", 90000i32)
                    .field("encoding-name", "VP8")
                    .build()
                    .to_value(),
            ),
            _ => None,
        }
    });

    rtpbin.connect("request-fec-decoder", false, |values| {
        let rtpbin = values[0]
            .get::<gst::Element>()
            .expect("rtpbin \"request-fec-decoder\" signal values[0]");
        let sess_id = values[1]
            .get::<u32>()
            .expect("rtpbin \"request-fec-decoder\" signal values[1]");

        match make_fec_decoder(&rtpbin, sess_id) {
            Ok(elem) => Some(elem.to_value()),
            Err(err) => {
                element_error!(
                    rtpbin,
                    gst::LibraryError::Failed,
                    ("Failed to make FEC decoder"),
                    ["{}", err]
                );
                None
            }
        }
    });

    let srcpad = static_pad(&netsim, "src")?;
    let sinkpad = request_pad(&rtpbin, "recv_rtp_sink_0")?;
    srcpad.link(&sinkpad)?;

    let depay_weak = depay.downgrade();
    rtpbin.connect_pad_added(move |rtpbin, src_pad| {
        let depay = match depay_weak.upgrade() {
            Some(depay) => depay,
            None => return,
        };

        match connect_rtpbin_srcpad(src_pad, &depay) {
            Ok(_) => (),
            Err(err) => {
                element_error!(
                    rtpbin,
                    gst::LibraryError::Failed,
                    ("Failed to link srcpad"),
                    ["{}", err]
                );
            }
        }
    });

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
                            "client-playing",
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
