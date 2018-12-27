// This example demonstrates GStreamer's playbin element.
// This element takes an arbitrary URI as parameter, and if there is a source
// element within gstreamer, that supports this uri, the playbin will try
// to automatically create a pipeline that properly plays this media source.
// For this, the playbin internally relies on more bin elements, like the
// autovideosink and the decodebin.
// Essentially, this element is a single-element pipeline able to play
// any format from any uri-addressable source that gstreamer supports.
// Much of the playbin's behavior can be controlled by so-called flags, as well
// as the playbin's properties and signals.

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

    // Create a new playbin element, and tell it what uri to play back.
    let playbin = gst::ElementFactory::make("playbin", None).unwrap();
    playbin
        .set_property("uri", &glib::Value::from(uri))
        .unwrap();

    // For flags handling
    // With flags, one can configure playbin's behavior such as whether it
    // should play back contained video streams, or if it should render subtitles.
    // let flags = playbin.get_property("flags").unwrap();
    // let flags_class = FlagsClass::new(flags.type_()).unwrap();
    // let flags = flags_class.builder_with_value(flags).unwrap()
    //     .unset_by_nick("text")
    //     .unset_by_nick("video")
    //     .build()
    //     .unwrap();
    // playbin.set_property("flags", &flags).unwrap();

    // The playbin also provides any kind of metadata that it found in the played stream.
    // For this, the playbin provides signals notifying about changes in the metadata.
    // Doing this with a signal makes sense for multiple reasons.
    // - The metadata is only found after the pipeline has been started
    // - Live streams (such as internet radios) update this metadata during the stream
    // Note that this signal will be emitted from the streaming threads usually,
    // not the application's threads!
    playbin
        .connect("audio-tags-changed", false, |values| {
            // The metadata of any of the contained audio streams changed
            // In the case of a live-stream from an internet radio, this could for example
            // mark the beginning of a new track, or a new DJ.
            let playbin = values[0].get::<glib::Object>().unwrap();
            // This gets the index of the stream that changed. This is neccessary, since
            // there could e.g. be multiple audio streams (english, spanish, ...).
            let idx = values[1].get::<i32>().unwrap();

            println!("audio tags of audio stream {} changed:", idx);

            // HELP: is this correct?
            // We were only notified about the change of metadata. If we want to do
            // something with it, we first need to actually query the metadata from the playbin.
            // We do this by facilliating the get-audio-tags action-signal on playbin.
            // Sending an action-signal to an element essentially is a function call on the element.
            // It is done that way, because elements do not have their own function API, they are
            // relying on GStreamer and GLib's API. The only way an element can communicate with an
            // application is via properties, signals or action signals (or custom messages, events, queries).
            // So what the following code does, is essentially asking playbin to tell us its already
            // internally stored tag list for this stream index.
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

    // The playbin element itself is a playbin, so it can be used as one, despite being
    // created from an element factory.
    let bus = playbin.get_bus().unwrap();

    let ret = playbin.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
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

    let ret = playbin.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}

fn main() {
    // tutorials_common::run is only required to set up the application environent on macOS
    // (but not necessary in normal Cocoa applications where this is set up autmatically)
    examples_common::run(example_main);
}
