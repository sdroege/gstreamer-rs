extern crate gstreamer as gst;
use gst::prelude::*;

extern crate glib;

use std::env;

#[path = "../examples-common.rs"]
mod examples_common;

fn example_main() {
    gst::init().unwrap();

    let args: Vec<_> = env::args().collect();
    let uri: &str = if args.len() == 2 {
        args[1].as_ref()
    } else {
        println!("Usage: playbin uri");
        std::process::exit(-1)
    };

    let playbin = gst::ElementFactory::make("playbin", None).unwrap();
    playbin
        .set_property("uri", &glib::Value::from(uri))
        .unwrap();

    // For flags handling
    // let flags = playbin.get_property("flags").unwrap();
    // let flags_class = FlagsClass::new(flags.type_()).unwrap();
    // let flags = flags_class.builder_with_value(flags).unwrap()
    //     .unset_by_nick("text")
    //     .unset_by_nick("video")
    //     .build()
    //     .unwrap();
    // playbin.set_property("flags", &flags).unwrap();

    playbin
        .connect("audio-tags-changed", false, |values| {
            let playbin = values[0].get::<glib::Object>().unwrap();
            let idx = values[1].get::<i32>().unwrap();

            println!("audio tags of audio stream {} changed:", idx);

            let tags = playbin
                .emit("get-audio-tags", &[&idx.to_value()])
                .unwrap()
                .unwrap();
            let tags = tags.get::<gst::TagList>().unwrap();

            if let Some(artist) = tags.get::<gst::tags::Artist>() {
                println!("  Artist: {}", artist.get().unwrap());
            }

            if let Some(title) = tags.get::<gst::tags::Title>() {
                println!("  Title: {}", title.get().unwrap());
            }

            if let Some(album) = tags.get::<gst::tags::Album>() {
                println!("  Album: {}", album.get().unwrap());
            }

            None
        })
        .unwrap();

    let bus = playbin.get_bus().unwrap();

    let ret = playbin.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    while let Some(msg) = bus.timed_pop(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    msg.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                break;
            }
            _ => (),
        }
    }

    let ret = playbin.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}

fn main() {
    // tutorials_common::run is only required to set up the application environent on macOS
    // (but not necessary in normal Cocoa applications where this is set up autmatically)
    examples_common::run(example_main);
}
