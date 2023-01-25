use std::{io, io::Write};

use gst::prelude::*;

#[path = "../tutorials-common.rs"]
mod tutorials_common;

struct CustomData {
    /// Our one and only element
    playbin: gst::Element,
    /// Are we in the PLAYING state?
    playing: bool,
    /// Should we terminate execution?
    terminate: bool,
    /// Is seeking enabled for this media?
    seek_enabled: bool,
    /// Have we performed the seek already?
    seek_done: bool,
    /// How long does this media last, in nanoseconds
    duration: Option<gst::ClockTime>,
}

fn tutorial_main() {
    // Initialize GStreamer
    gst::init().unwrap();

    let uri =
        "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm";

    // Creat the playbin element
    let playbin = gst::ElementFactory::make("playbin")
        .name("playbin")
        // Set the URI to play
        .property("uri", uri)
        .build()
        .expect("Failed to create playbin element");

    // Start playing
    playbin
        .set_state(gst::State::Playing)
        .expect("Unable to set the playbin to the `Playing` state");

    // Listen to the bus
    let bus = playbin.bus().unwrap();
    let mut custom_data = CustomData {
        playbin,
        playing: false,
        terminate: false,
        seek_enabled: false,
        seek_done: false,
        duration: gst::ClockTime::NONE,
    };

    while !custom_data.terminate {
        let msg = bus.timed_pop(100 * gst::ClockTime::MSECOND);

        match msg {
            Some(msg) => {
                handle_message(&mut custom_data, &msg);
            }
            None => {
                if custom_data.playing {
                    let position = custom_data
                        .playbin
                        .query_position::<gst::ClockTime>()
                        .expect("Could not query current position.");

                    // If we didn't know it yet, query the stream duration
                    if custom_data.duration == gst::ClockTime::NONE {
                        custom_data.duration = custom_data.playbin.query_duration();
                    }

                    // Print current position and total duration
                    print!(
                        "\rPosition {} / {}",
                        position,
                        custom_data.duration.display()
                    );
                    io::stdout().flush().unwrap();

                    if custom_data.seek_enabled
                        && !custom_data.seek_done
                        && position > 10 * gst::ClockTime::SECOND
                    {
                        println!("\nReached 10s, performing seek...");
                        custom_data
                            .playbin
                            .seek_simple(
                                gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
                                30 * gst::ClockTime::SECOND,
                            )
                            .expect("Failed to seek.");
                        custom_data.seek_done = true;
                    }
                }
            }
        }
    }

    // Shutdown pipeline
    custom_data
        .playbin
        .set_state(gst::State::Null)
        .expect("Unable to set the playbin to the `Null` state");
}

fn handle_message(custom_data: &mut CustomData, msg: &gst::Message) {
    use gst::MessageView;

    match msg.view() {
        MessageView::Error(err) => {
            println!(
                "Error received from element {:?}: {} ({:?})",
                err.src().map(|s| s.path_string()),
                err.error(),
                err.debug()
            );
            custom_data.terminate = true;
        }
        MessageView::Eos(..) => {
            println!("End-Of-Stream reached.");
            custom_data.terminate = true;
        }
        MessageView::DurationChanged(_) => {
            // The duration has changed, mark the current one as invalid
            custom_data.duration = gst::ClockTime::NONE;
        }
        MessageView::StateChanged(state_changed) => {
            if state_changed
                .src()
                .map(|s| s == &custom_data.playbin)
                .unwrap_or(false)
            {
                let new_state = state_changed.current();
                let old_state = state_changed.old();

                println!("Pipeline state changed from {old_state:?} to {new_state:?}");

                custom_data.playing = new_state == gst::State::Playing;
                if custom_data.playing {
                    let mut seeking = gst::query::Seeking::new(gst::Format::Time);
                    if custom_data.playbin.query(&mut seeking) {
                        let (seekable, start, end) = seeking.result();
                        custom_data.seek_enabled = seekable;
                        if seekable {
                            println!("Seeking is ENABLED from {start} to {end}")
                        } else {
                            println!("Seeking is DISABLED for this stream.")
                        }
                    } else {
                        eprintln!("Seeking query failed.")
                    }
                }
            }
        }
        _ => (),
    }
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    tutorials_common::run(tutorial_main);
}
