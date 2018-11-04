extern crate gstreamer as gst;
use gst::prelude::*;

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;

use std::env;

#[allow(unused_imports)]
#[path = "../examples-common.rs"]
mod examples_common;

extern crate failure;
#[allow(unused_imports)]
use failure::Error;

extern crate glib;

fn main_loop(uri: &str) -> Result<(), glib::BoolError> {
    ges::init()?;

    let timeline = ges::Timeline::new_audio_video();
    let layer = timeline.append_layer();
    let pipeline = ges::Pipeline::new();
    pipeline.set_timeline(&timeline);

    let clip = ges::UriClip::new(uri);
    layer.add_clip(&clip);

    let effect = ges::Effect::new("agingtv");
    clip.add(&effect).unwrap();

    println!(
        "Agingtv scratch-lines: {}",
        clip.get_child_property("scratch-lines")
            .unwrap()
            .serialize()
            .unwrap()
    );

    let asset = clip.get_asset().unwrap();
    let duration = asset
        .downcast::<ges::UriClipAsset>()
        .unwrap()
        .get_duration();
    println!(
        "Clip duration: {} - playing file from {} for {}",
        duration,
        duration / 2,
        duration / 4
    );

    clip.set_inpoint(duration / 2);
    clip.set_duration(duration / 4);

    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    let bus = pipeline.get_bus().unwrap();
    while let Some(msg) = bus.timed_pop(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                break;
            }
            _ => (),
        }
    }

    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    Ok(())
}

#[allow(unused_variables)]
fn example_main() {
    let args: Vec<_> = env::args().collect();
    let uri: &str = if args.len() == 2 {
        args[1].as_ref()
    } else {
        println!("Usage: ges launch");
        std::process::exit(-1)
    };

    match main_loop(uri) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}

fn main() {
    // tutorials_common::run is only required to set up the application environent on macOS
    // (but not necessary in normal Cocoa applications where this is set up autmatically)
    examples_common::run(example_main);
}
