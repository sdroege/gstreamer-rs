extern crate gstreamer as gst;
use gst::*;
extern crate gstreamer_player as gst_player;
use gst_player::*;

extern crate glib;
use glib::ObjectExt;

use std::env;

pub mod utils;

fn main_loop(uri: &str) -> Result<(), utils::ExampleError> {
    gst::init().map_err(utils::ExampleError::InitFailed)?;

    let main_loop = glib::MainLoop::new(None, false);

    let dispatcher = PlayerGMainContextSignalDispatcher::new(None).unwrap();
    let player = Player::new(None, Some(&dispatcher));
    player.set_property("uri", &Value::from(uri)).unwrap();

    let player_clone = player.clone();
    let main_loop_clone = main_loop.clone();
    player.connect_end_of_stream(move |_| {
        let main_loop = &main_loop_clone;
        let player = &player_clone;
        player.stop();
        main_loop.quit();
    });

    player.play();
    main_loop.run();
    Ok(())
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let uri: &str = if args.len() == 2 {
        args[1].as_ref()
    } else {
        panic!("Usage: player uri");
    };
    match main_loop(uri) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }

}
