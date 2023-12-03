// This example implements a baseclass IirFilter, and a subclass Lowpass of that.
//
// The example shows how to provide and implement virtual methods, and how to provide non-virtual
// methods on the base class.

use gst::prelude::*;

mod iirfilter;
mod lowpass;

#[path = "../../examples-common.rs"]
mod examples_common;

fn example_main() {
    gst::init().unwrap();

    let pipeline = gst::Pipeline::new();
    let src = gst::ElementFactory::make("audiotestsrc")
        .property_from_str("wave", "white-noise")
        .build()
        .unwrap();
    let filter = glib::Object::builder::<lowpass::Lowpass>()
        .property("cutoff", 4000.0f32)
        .build();
    let conv = gst::ElementFactory::make("audioconvert").build().unwrap();
    let sink = gst::ElementFactory::make("autoaudiosink").build().unwrap();

    pipeline
        .add_many([&src, filter.as_ref(), &conv, &sink])
        .unwrap();
    gst::Element::link_many([&src, filter.as_ref(), &conv, &sink]).unwrap();

    let bus = pipeline.bus().unwrap();

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
    }

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    examples_common::run(example_main);
}
