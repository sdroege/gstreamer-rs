use std::{cmp, thread, time};

use anyhow::Error;
use gst::prelude::*;
use gst_video::prelude::ColorBalanceExt;
use termion::{event::Key, input::TermRead};

#[path = "../tutorials-common.rs"]
mod tutorials_common;

// Commands that we get from the terminal and we send to the main thread.
#[derive(Clone, PartialEq)]
enum Command {
    UpdateChannel(String, bool),
    Quit,
}

fn handle_keyboard(ready_tx: glib::Sender<Command>) {
    let mut stdin = termion::async_stdin().keys();

    loop {
        if let Some(Ok(input)) = stdin.next() {
            let command = match input {
                Key::Char(key) => {
                    let increase = key.is_uppercase();
                    match key {
                        'c' | 'C' => Command::UpdateChannel(String::from("CONTRAST"), increase),
                        'b' | 'B' => Command::UpdateChannel(String::from("BRIGHTNESS"), increase),
                        'h' | 'H' => Command::UpdateChannel(String::from("HUE"), increase),
                        's' | 'S' => Command::UpdateChannel(String::from("SATURATION"), increase),
                        'q' | 'Q' => Command::Quit,
                        _ => continue,
                    }
                }
                Key::Ctrl('c' | 'C') => Command::Quit,
                _ => continue,
            };
            ready_tx
                .send(command.clone())
                .expect("Failed to send command to the main thread.");
            if command == Command::Quit {
                break;
            }
        }
        thread::sleep(time::Duration::from_millis(50));
    }
}

fn update_color_channel(
    channel_name: &str,
    increase: bool,
    color_balance: &gst_video::ColorBalance,
) {
    // Retrieve the list of all channels and locate the requested one
    let channels = color_balance.list_channels();
    if let Some(channel) = channels.iter().find(|c| c.label() == channel_name) {
        // Change the value in the requested direction
        let mut value = color_balance.value(channel);
        let step = (channel.max_value() - channel.min_value()) / 10;

        if increase {
            value = cmp::min(value + step, channel.max_value());
        } else {
            value = cmp::max(value - step, channel.min_value());
        }

        color_balance.set_value(channel, value);
    }
}

fn print_current_values(pipeline: &gst::Element) {
    let balance = pipeline
        .dynamic_cast_ref::<gst_video::ColorBalance>()
        .unwrap();
    let channels = balance.list_channels();

    for channel in channels.iter() {
        let value = balance.value(channel);
        let percentage =
            100 * (value - channel.min_value()) / (channel.max_value() - channel.min_value());

        print!("{}: {: >3}% ", channel.label(), percentage);
    }
    println!();
}

fn tutorial_main() -> Result<(), Error> {
    // Initialize GStreamer
    gst::init()?;

    println!(
        "USAGE: Choose one of the following options, then press enter:
'C' to increase contrast, 'c' to decrease contrast
'B' to increase brightness, 'b' to decrease brightness
'H' to increase hue, 'h' to decrease hue
'S' to increase saturation, 's' to decrease saturation
'Q' to quit"
    );

    // Get a main context...
    let main_context = glib::MainContext::default();
    // ... and make it the main context by default so that we can then have a channel to send the
    // commands we received from the terminal.
    let _guard = main_context.acquire().unwrap();

    // Build the channel to get the terminal inputs from a different thread.
    let (ready_tx, ready_rx) = glib::MainContext::channel(glib::Priority::DEFAULT);

    // Start the keyboard handling thread
    thread::spawn(move || handle_keyboard(ready_tx));

    // Build the pipeline
    let pipeline = gst::parse_launch(
        "playbin uri=https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm")?;

    let main_loop = glib::MainLoop::new(None, false);
    let main_loop_clone = main_loop.clone();
    let pipeline_weak = pipeline.downgrade();

    // Start playing
    pipeline.set_state(gst::State::Playing)?;

    ready_rx.attach(Some(&main_loop.context()), move |command: Command| {
        let pipeline = match pipeline_weak.upgrade() {
            Some(pipeline) => pipeline,
            None => return glib::Continue(true),
        };

        match command {
            Command::UpdateChannel(ref name, increase) => {
                let balance = pipeline
                    .dynamic_cast_ref::<gst_video::ColorBalance>()
                    .unwrap();
                update_color_channel(name, increase, balance);
                print_current_values(&pipeline);
            }
            Command::Quit => {
                main_loop_clone.quit();
            }
        }
        glib::Continue(true)
    });

    // Handle bus errors / EOS correctly
    let main_loop_clone = main_loop.clone();
    let bus = pipeline.bus().unwrap();
    let pipeline_weak = pipeline.downgrade();
    let _bus_watch = bus.add_watch(move |_bus, message| {
        use gst::MessageView;

        let pipeline = match pipeline_weak.upgrade() {
            Some(pipeline) => pipeline,
            None => return glib::Continue(true),
        };

        match message.view() {
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?} {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                eprintln!("Debugging information: {:?}", err.debug());
                main_loop_clone.quit();
                Continue(false)
            }
            MessageView::Eos(..) => {
                println!("Reached end of stream");
                pipeline
                    .set_state(gst::State::Ready)
                    .expect("Unable to set the pipeline to the `Ready` state");
                main_loop_clone.quit();
                Continue(false)
            }
            _ => Continue(true),
        }
    })?;

    // Print initial values for all channels
    print_current_values(&pipeline);

    // Run the GLib main loop
    main_loop.run();

    pipeline.set_state(gst::State::Null)?;

    Ok(())
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    match tutorials_common::run(tutorial_main) {
        Ok(_) => {}
        Err(err) => eprintln!("Failed: {err}"),
    };
}
