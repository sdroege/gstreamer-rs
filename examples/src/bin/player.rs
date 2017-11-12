extern crate gstreamer as gst;
#[cfg(feature = "gst-player")]
use gst::prelude::*;

#[cfg(feature = "gst-player")]
extern crate gstreamer_player as gst_player;

#[cfg(feature = "gst-player")]
extern crate glib;

use std::env;
#[cfg(feature = "gst-player")]
use std::sync::{Arc, Mutex};

extern crate failure;

#[allow(unused_imports)]
use failure::Error;

#[allow(unused_imports)]
#[path = "../examples-common.rs"]
mod examples_common;

#[cfg(feature = "gst-player")]
fn main_loop(uri: &str) -> Result<(), Error> {
    gst::init()?;

    let main_loop = glib::MainLoop::new(None, false);

    let dispatcher = gst_player::PlayerGMainContextSignalDispatcher::new(None);
    let player = gst_player::Player::new(
        None,
        Some(&dispatcher.upcast::<gst_player::PlayerSignalDispatcher>()),
    );

    player.set_property("uri", &glib::Value::from(uri))?;

    let error = Arc::new(Mutex::new(Ok(())));

    let player_clone = player.clone();
    let main_loop_clone = main_loop.clone();
    player.connect_end_of_stream(move |_| {
        let main_loop = &main_loop_clone;
        let player = &player_clone;
        player.stop();
        main_loop.quit();
    });

    let player_clone = player.clone();
    let main_loop_clone = main_loop.clone();
    let error_clone = Arc::clone(&error);
    player.connect_error(move |_, err| {
        let main_loop = &main_loop_clone;
        let player = &player_clone;
        let error = &error_clone;

        *error.lock().unwrap() = Err(err.clone());

        player.stop();
        main_loop.quit();
    });

    player.play();
    main_loop.run();

    let guard = error.as_ref().lock().unwrap();

    guard.clone().map_err(|e| e.into())
}

#[allow(unused_variables)]
fn example_main() {
    let args: Vec<_> = env::args().collect();
    let uri: &str = if args.len() == 2 {
        args[1].as_ref()
    } else {
        println!("Usage: player uri");
        std::process::exit(-1);
    };

    #[cfg(not(feature = "gst-player"))]
    {
        eprintln!("Feature gst-player is required. Please rebuild with --features gst-player");
        std::process::exit(-1);
    }

    #[cfg(feature = "gst-player")]
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
