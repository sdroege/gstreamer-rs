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

pub mod utils;

#[cfg(feature = "gst-player")]
fn main_loop(uri: &str) -> Result<(), utils::ExampleError> {
    gst::init().map_err(utils::ExampleError::InitFailed)?;

    let main_loop = glib::MainLoop::new(None, false);

    let dispatcher = gst_player::PlayerGMainContextSignalDispatcher::new(None);
    let player = gst_player::Player::new(
        None,
        Some(&dispatcher.upcast::<gst_player::PlayerSignalDispatcher>()),
    );
    player
        .set_property("uri", &glib::Value::from(uri))
        .expect("Can't set uri property");

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

        *error.lock().unwrap() = Err(utils::ExampleError::ElementError(
            "player".to_owned(),
            err.clone(),
            "".to_owned(),
        ));

        player.stop();
        main_loop.quit();
    });

    player.play();
    main_loop.run();

    let guard = error.as_ref().lock().unwrap();

    guard.clone()
}

#[cfg(not(feature = "gst-player"))]
#[allow(unused_variables)]
fn main_loop(uri: &str) -> Result<(), utils::ExampleError> {
    Err(utils::ExampleError::MissingFeature("gst-player"))
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let uri: &str = if args.len() == 2 {
        args[1].as_ref()
    } else {
        println!("Usage: player uri");
        std::process::exit(-1);
    };

    match main_loop(uri) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}
