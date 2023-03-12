use std::{io, thread, time};

use anyhow::Error;
use gst::{
    event::{Seek, Step},
    prelude::*,
    Element, SeekFlags, SeekType, State,
};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

#[path = "../tutorials-common.rs"]
mod tutorials_common;

// Commands that we get from the terminal and we send to the main thread.
#[derive(Clone, Copy, PartialEq)]
enum Command {
    PlayPause,
    DataRateUp,
    DataRateDown,
    ReverseRate,
    NextFrame,
    Quit,
}

fn send_seek_event(pipeline: &Element, rate: f64) -> bool {
    // Obtain the current position, needed for the seek event
    let position = match pipeline.query_position::<gst::ClockTime>() {
        Some(pos) => pos,
        None => {
            eprintln!("Unable to retrieve current position...\r");
            return false;
        }
    };

    // Create the seek event
    let seek_event = if rate > 0. {
        Seek::new(
            rate,
            SeekFlags::FLUSH | SeekFlags::ACCURATE,
            SeekType::Set,
            position,
            SeekType::End,
            gst::ClockTime::ZERO,
        )
    } else {
        Seek::new(
            rate,
            SeekFlags::FLUSH | SeekFlags::ACCURATE,
            SeekType::Set,
            position,
            SeekType::Set,
            position,
        )
    };

    // If we have not done so, obtain the sink through which we will send the seek events
    if let Some(video_sink) = pipeline.property::<Option<Element>>("video-sink") {
        println!("Current rate: {rate}\r");
        // Send the event
        video_sink.send_event(seek_event)
    } else {
        eprintln!("Failed to update rate...\r");
        false
    }
}

// This is where we get the user input from the terminal.
fn handle_keyboard(ready_tx: glib::Sender<Command>) {
    // We set the terminal in "raw mode" so that we can get the keys without waiting for the user
    // to press return.
    let _stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();

    loop {
        if let Some(Ok(input)) = stdin.next() {
            let command = match input {
                Key::Char('p' | 'P') => Command::PlayPause,
                Key::Char('s') => Command::DataRateDown,
                Key::Char('S') => Command::DataRateUp,
                Key::Char('d' | 'D') => Command::ReverseRate,
                Key::Char('n' | 'N') => Command::NextFrame,
                Key::Char('q' | 'Q') => Command::Quit,
                Key::Ctrl('c' | 'C') => Command::Quit,
                _ => continue,
            };
            ready_tx
                .send(command)
                .expect("failed to send data through channel");
            if command == Command::Quit {
                break;
            }
        }
        thread::sleep(time::Duration::from_millis(50));
    }
}

fn tutorial_main() -> Result<(), Error> {
    // Initialize GStreamer.
    gst::init()?;

    // Print usage map.
    println!(
        "\
USAGE: Choose one of the following options, then press enter:
 'P' to toggle between PAUSE and PLAY
 'S' to increase playback speed, 's' to decrease playback speed
 'D' to toggle playback direction
 'N' to move to next frame (in the current direction, better in PAUSE)
 'Q' to quit"
    );

    // Get a main context...
    let main_context = glib::MainContext::default();
    // ... and make it the main context by default so that we can then have a channel to send the
    // commands we received from the terminal.
    let _guard = main_context.acquire().unwrap();

    // Build the channel to get the terminal inputs from a different thread.
    let (ready_tx, ready_rx) = glib::MainContext::channel(glib::Priority::DEFAULT);

    thread::spawn(move || handle_keyboard(ready_tx));

    // Build the pipeline.
    let uri =
        "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm";
    let pipeline = gst::parse_launch(&format!("playbin uri={uri}"))?;

    // Start playing.
    let _ = pipeline.set_state(State::Playing)?;

    let main_loop = glib::MainLoop::new(Some(&main_context), false);
    let main_loop_clone = main_loop.clone();
    let pipeline_weak = pipeline.downgrade();

    // Setting up "play" information.
    let mut playing = true;
    let mut rate = 1.;

    ready_rx.attach(Some(&main_loop.context()), move |command: Command| {
        let pipeline = match pipeline_weak.upgrade() {
            Some(pipeline) => pipeline,
            None => return glib::Continue(true),
        };
        match command {
            Command::PlayPause => {
                let status = if playing {
                    let _ = pipeline.set_state(State::Paused);
                    "PAUSE"
                } else {
                    let _ = pipeline.set_state(State::Playing);
                    "PLAYING"
                };
                playing = !playing;
                println!("Setting state to {status}\r");
            }
            Command::DataRateUp => {
                if send_seek_event(&pipeline, rate * 2.) {
                    rate *= 2.;
                }
            }
            Command::DataRateDown => {
                if send_seek_event(&pipeline, rate / 2.) {
                    rate /= 2.;
                }
            }
            Command::ReverseRate => {
                if send_seek_event(&pipeline, rate * -1.) {
                    rate *= -1.;
                }
            }
            Command::NextFrame => {
                if let Some(video_sink) = pipeline.property::<Option<Element>>("video-sink") {
                    // Send the event
                    let step = Step::new(gst::format::Buffers::ONE, rate.abs(), true, false);
                    video_sink.send_event(step);
                    println!("Stepping one frame\r");
                }
            }
            Command::Quit => {
                main_loop_clone.quit();
            }
        }
        glib::Continue(true)
    });

    main_loop.run();

    pipeline.set_state(State::Null)?;

    Ok(())
}

fn main() {
    match tutorials_common::run(tutorial_main) {
        Ok(_) => {}
        Err(err) => eprintln!("Failed: {err}"),
    }
}
