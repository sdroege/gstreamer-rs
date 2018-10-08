extern crate glib;

extern crate gstreamer as gst;
use gst::prelude::*;

extern crate cairo;
extern crate gstreamer_video as gst_video;
extern crate pango;
use pango::prelude::*;
extern crate pangocairo;

use std::error::Error as StdError;
use std::ops;
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

struct DrawingContext {
    layout: glib::SendUniqueCell<LayoutWrapper>,
    info: Option<gst_video::VideoInfo>,
}

#[derive(Debug)]
struct LayoutWrapper(pango::Layout);

impl ops::Deref for LayoutWrapper {
    type Target = pango::Layout;

    fn deref(&self) -> &pango::Layout {
        &self.0
    }
}

unsafe impl glib::SendUnique for LayoutWrapper {
    fn is_unique(&self) -> bool {
        self.0.ref_count() == 1
    }
}

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    gst::init()?;

    let pipeline = gst::Pipeline::new(None);
    let src =
        gst::ElementFactory::make("videotestsrc", None).ok_or(MissingElement("videotestsrc"))?;
    let overlay =
        gst::ElementFactory::make("cairooverlay", None).ok_or(MissingElement("cairooverlay"))?;
    let capsfilter =
        gst::ElementFactory::make("capsfilter", None).ok_or(MissingElement("capsfilter"))?;
    let videoconvert =
        gst::ElementFactory::make("videoconvert", None).ok_or(MissingElement("videoconvert"))?;
    let sink =
        gst::ElementFactory::make("autovideosink", None).ok_or(MissingElement("autovideosink"))?;

    pipeline.add_many(&[&src, &overlay, &capsfilter, &videoconvert, &sink])?;
    gst::Element::link_many(&[&src, &overlay, &capsfilter, &videoconvert, &sink])?;

    let caps = gst::Caps::builder("video/x-raw")
        .field("width", &800i32)
        .field("height", &800i32)
        .build();
    capsfilter.set_property("caps", &caps).unwrap();

    src.set_property_from_str("pattern", "ball");

    let fontmap = pangocairo::FontMap::new().unwrap();
    let context = fontmap.create_context().unwrap();
    let layout = LayoutWrapper(pango::Layout::new(&context));

    let font_desc = pango::FontDescription::from_string("Sans Bold 26");
    layout.set_font_description(&font_desc);
    layout.set_text("GStreamer");

    let drawer = Arc::new(Mutex::new(DrawingContext {
        layout: glib::SendUniqueCell::new(layout).unwrap(),
        info: None,
    }));

    let drawer_clone = drawer.clone();
    overlay
        .connect("draw", false, move |args| {
            use std::f64::consts::PI;

            let drawer = &drawer_clone;
            let drawer = drawer.lock().unwrap();

            let _overlay = args[0].get::<gst::Element>().unwrap();
            let cr = args[1].get::<cairo::Context>().unwrap();
            let timestamp = args[2].get::<gst::ClockTime>().unwrap();
            let _duration = args[3].get::<gst::ClockTime>().unwrap();

            let info = drawer.info.as_ref().unwrap();
            let layout = drawer.layout.borrow();

            let angle = 2.0
                * PI
                * ((timestamp % (10 * gst::SECOND)).unwrap() as f64
                    / (10.0 * gst::SECOND_VAL as f64));

            cr.translate(info.width() as f64 / 2.0, info.height() as f64 / 2.0);
            cr.rotate(angle);

            for i in 0..10 {
                cr.save();

                let angle = (360. * i as f64) / 10.0;
                let red = (1.0 + f64::cos((angle - 60.0) * PI / 180.0)) / 2.0;
                cr.set_source_rgb(red, 0.0, 1.0 - red);
                cr.rotate(angle * PI / 180.0);

                pangocairo::functions::update_layout(&cr, &layout);
                let (width, _height) = layout.get_size();
                cr.move_to(
                    -(width as f64 / pango::SCALE as f64) / 2.0,
                    -(info.height() as f64) / 2.0,
                );
                pangocairo::functions::show_layout(&cr, &layout);

                cr.restore();
            }

            None
        })
        .unwrap();

    let drawer_clone = drawer.clone();
    overlay
        .connect("caps-changed", false, move |args| {
            let _overlay = args[0].get::<gst::Element>().unwrap();
            let caps = args[1].get::<gst::Caps>().unwrap();

            let drawer = &drawer_clone;
            let mut drawer = drawer.lock().unwrap();
            drawer.info = Some(gst_video::VideoInfo::from_caps(&caps).unwrap());

            None
        })
        .unwrap();

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
                    src: err
                        .get_src()
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
