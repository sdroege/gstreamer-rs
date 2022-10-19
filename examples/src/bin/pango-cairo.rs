// This example demonstrates how to overlay a video using the cairo
// library. For this, the cairooverlay element is used on a video stream.
// Additionally, this example uses functionality of the pango library, which handles
// text layouting. The pangocairo crate is a nice wrapper combining both libraries
// into a nice interface.
// The drawing surface which the cairooverlay element creates internally can then
// normally be drawn on using the cairo library.
// The operated pipeline looks like this:

// {videotestsrc} - {cairooverlay} - {capsfilter} - {videoconvert} - {autovideosink}
// The capsfilter element allows us to dictate the video resolution we want for the
// videotestsrc and the cairooverlay element.

use gst::prelude::*;

use pango::prelude::*;

use std::ops;
use std::sync::{Arc, Mutex};

use anyhow::Error;
use derive_more::{Display, Error};

#[path = "../examples-common.rs"]
mod examples_common;

#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {}: {} (debug: {:?})", src, error, debug)]
struct ErrorMessage {
    src: String,
    error: String,
    debug: Option<String>,
    source: glib::Error,
}

struct DrawingContext {
    layout: LayoutWrapper,
    info: Option<gst_video::VideoInfo>,
}

#[derive(Debug)]
struct LayoutWrapper(pango::Layout);

impl ops::Deref for LayoutWrapper {
    type Target = pango::Layout;

    fn deref(&self) -> &pango::Layout {
        assert_eq!(self.0.ref_count(), 1);
        &self.0
    }
}

// SAFETY: We ensure that there are never multiple references to the layout.
unsafe impl Send for LayoutWrapper {}

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    gst::init()?;

    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("videotestsrc")
        // The videotestsrc supports multiple test patterns. In this example, we will use the
        // pattern with a white ball moving around the video's center point.
        .property_from_str("pattern", "ball")
        .build()?;
    let overlay = gst::ElementFactory::make("cairooverlay").build()?;

    // Plug in a capsfilter element that will force the videotestsrc and the cairooverlay to work
    // with images of the size 800x800.
    let caps = gst_video::VideoCapsBuilder::new()
        .width(800)
        .height(800)
        .build();
    let capsfilter = gst::ElementFactory::make("capsfilter")
        .property("caps", &caps)
        .build()?;

    let videoconvert = gst::ElementFactory::make("videoconvert").build()?;
    let sink = gst::ElementFactory::make("autovideosink").build()?;

    pipeline.add_many(&[&src, &overlay, &capsfilter, &videoconvert, &sink])?;
    gst::Element::link_many(&[&src, &overlay, &capsfilter, &videoconvert, &sink])?;

    // The PangoFontMap represents the set of fonts available for a particular rendering system.
    let fontmap = pangocairo::FontMap::new();
    // Create a new pango layouting context for the fontmap.
    let context = fontmap.create_context();
    // Create a pango layout object. This object is a string of text we want to layout.
    // It is wrapped in a LayoutWrapper (defined above) to be able to send it across threads.
    let layout = LayoutWrapper(pango::Layout::new(&context));

    // Select the text content and the font we want to use for the piece of text.
    let font_desc = pango::FontDescription::from_string("Sans Bold 26");
    layout.set_font_description(Some(&font_desc));
    layout.set_text("GStreamer");

    // The following is a context struct (containing the pango layout and the configured video info).
    // We have to wrap it in an Arc (or Rc) to get reference counting, that is: to be able to have
    // shared ownership of it in multiple different places (the two signal handlers here).
    // We have to wrap it in a Mutex because Rust's type-system can't know that both signals are
    // only ever called from a single thread (the streaming thread). It would be enough to have
    // something that is Send in theory but that's not how signal handlers are generated unfortunately.
    // The Mutex (or otherwise if we didn't need the Sync bound we could use a RefCell) is to implement
    // interior mutability (see Rust docs). Via this we can get a mutable reference to the contained
    // data which is checked at runtime for uniqueness (blocking in case of mutex, panic in case
    // of refcell) instead of compile-time (like with normal references).
    let drawer = Arc::new(Mutex::new(DrawingContext { layout, info: None }));

    let drawer_clone = drawer.clone();
    // Connect to the cairooverlay element's "draw" signal, which is emitted for
    // each videoframe piped through the element. Here we have the possibility to
    // draw on top of the frame (overlay it), using the cairo render api.
    // Signals connected with the connect(<name>, ...) API get their arguments
    // passed as array of glib::Value. For a documentation about the actual arguments
    // it is always a good idea to either check the element's signals using either
    // gst-inspect, or the online documentation.
    overlay.connect("draw", false, move |args| {
        use std::f64::consts::PI;

        let drawer = &drawer_clone;
        let drawer = drawer.lock().unwrap();

        // Get the signal's arguments
        let _overlay = args[0].get::<gst::Element>().unwrap();
        // This is the cairo context. This is the root of all of cairo's
        // drawing functionality.
        let cr = args[1].get::<cairo::Context>().unwrap();
        let timestamp = args[2].get::<gst::ClockTime>().unwrap();
        let _duration = args[3].get::<gst::ClockTime>().unwrap();

        let info = drawer.info.as_ref().unwrap();
        let layout = &drawer.layout;

        let angle = 2.0 * PI * (timestamp % (10 * gst::ClockTime::SECOND)).nseconds() as f64
            / (10.0 * gst::ClockTime::SECOND.nseconds() as f64);

        // The image we draw (the text) will be static, but we will change the
        // transformation on the drawing context, which rotates and shifts everything
        // that we draw afterwards. Like this, we have no complicated calulations
        // in the actual drawing below.
        // Calling multiple transformation methods after each other will apply the
        // new transformation on top. If you repeat the cr.rotate(angle) line below
        // this a second time, everything in the canvas will rotate twice as fast.
        cr.translate(
            f64::from(info.width()) / 2.0,
            f64::from(info.height()) / 2.0,
        );
        cr.rotate(angle);

        // This loop will render 10 times the string "GStreamer" in a circle
        for i in 0..10 {
            // Cairo, like most rendering frameworks, is using a stack for transformations
            // with this, we push our current transformation onto this stack - allowing us
            // to make temporary changes / render something / and then returning to the
            // previous transformations.
            cr.save().expect("Failed to save state");

            let angle = (360. * f64::from(i)) / 10.0;
            let red = (1.0 + f64::cos((angle - 60.0) * PI / 180.0)) / 2.0;
            cr.set_source_rgb(red, 0.0, 1.0 - red);
            cr.rotate(angle * PI / 180.0);

            // Update the text layout. This function is only updating pango's internal state.
            // So e.g. that after a 90 degree rotation it knows that what was previously going
            // to end up as a 200x100 rectangle would now be 100x200.
            pangocairo::functions::update_layout(&cr, &**layout);
            let (width, _height) = layout.size();
            // Using width and height of the text, we can properly possition it within
            // our canvas.
            cr.move_to(
                -(f64::from(width) / f64::from(pango::SCALE)) / 2.0,
                -(f64::from(info.height())) / 2.0,
            );
            // After telling the layout object where to draw itself, we actually tell
            // it to draw itself into our cairo context.
            pangocairo::functions::show_layout(&cr, &**layout);

            // Here we go one step up in our stack of transformations, removing any
            // changes we did to them since the last call to cr.save();
            cr.restore().expect("Failed to restore state");
        }

        None
    });

    // Add a signal handler to the overlay's "caps-changed" signal. This could e.g.
    // be called when the sink that we render to does not support resizing the image
    // itself - but the user just changed the window-size. The element after the overlay
    // will then change its caps and we use the notification about this change to
    // resize our canvas's size.
    // Another possibility for when this might happen is, when our video is a network
    // stream that dynamically changes resolution when enough bandwith is available.
    overlay.connect("caps-changed", false, move |args| {
        let _overlay = args[0].get::<gst::Element>().unwrap();
        let caps = args[1].get::<gst::Caps>().unwrap();

        let mut drawer = drawer.lock().unwrap();
        drawer.info = Some(gst_video::VideoInfo::from_caps(&caps).unwrap());

        None
    });

    Ok(pipeline)
}

fn main_loop(pipeline: gst::Pipeline) -> Result<(), Error> {
    pipeline.set_state(gst::State::Playing)?;

    let bus = pipeline
        .bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null)?;
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
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    examples_common::run(example_main);
}
