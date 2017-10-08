extern crate gstreamer as gst;
use std::io;
use std::io::Write;
use gst::prelude::*;
use gst::MessageView;

fn main() {
    // Initialize GStreamer
    gst::init().unwrap();

    // Creat the playbin element
    let playbin = gst::ElementFactory::make("playbin", "playbin")
        .expect("Failed to create playbin element");

    // Set the URI to play
    let uri = "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm";
    playbin.set_property("uri", &uri)
        .expect("Can't set uri property on playbin");

    // Start playing
    let ret = playbin.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    // Listen to the bus
    let bus = playbin.get_bus().unwrap();
    let mut is_playing = false;
    let mut seek_enabled = false;
    let mut seek_done = false;

    loop {
        let msg = bus.timed_pop(100 * gst::MSECOND);

        match msg {
            Some(msg) => {
                match msg.view() {
                    MessageView::Eos(..) => {
                        println!("End-Of-Stream reached.");
                        break
                    },
                    MessageView::Error(err) => {
                        println!(
                            "Error from {}: {} ({:?})",
                            msg.get_src().get_path_string(),
                            err.get_error(),
                            err.get_debug()
                        );
                        break
                    }
                    MessageView::StateChanged(state) => {
                        if msg.get_src() == playbin {
                            let new_state = state.get_current();
                            let old_state = state.get_old();

                            println!("Pipeline state changed from {:?} to {:?}",
                                     old_state, new_state);

                            is_playing = new_state == gst::State::Playing;
                            if is_playing {
                                let mut query = gst::Query::new_seeking(gst::Format::Time);
                                if playbin.query(query.get_mut().unwrap()) {
                                    match query.view() {
                                        gst::QueryView::Seeking(seek) => {
                                            let (_fmt, seekable, start, end) = seek.get();
                                            seek_enabled = seekable;
                                            if seekable {
                                                println!("Seeking is ENABLED from {} to {}", start, end)
                                            } else {
                                                println!("Seeking is DISABLED for this stream.")
                                            }
                                        },
                                        _ => unreachable!(),
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
            None => {
                if is_playing {
                    let position = playbin.query_position(gst::Format::Time)
                        .expect("Could not query current position.");
                    let position = position as gst::ClockTime;

                    let duration = playbin.query_duration(gst::Format::Time)
                        .expect("Could not query current duration.");

                    print!("\rPosition {} / {}", position, duration);
                    io::stdout().flush().unwrap();

                    if seek_enabled && !seek_done && position > 10 * gst::SECOND {
                        println!("\nReached 10s, performing seek...");
                        playbin.seek_simple(gst::Format::Time,
                                            gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
                                            (30 * gst::SECOND) as i64);
                        seek_done = true;
                    }
                }
            }
        }
    }

    // Shutdown pipeline
    let ret = playbin.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}
