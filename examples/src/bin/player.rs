extern crate gstreamer as gst;

#[cfg(feature = "gst-player")]
extern crate gstreamer_player as gst_player;

extern crate glib;

#[allow(unused_imports)]
use glib::ObjectExt;

extern crate send_cell;

use std::env;

pub mod utils;


#[cfg(feature = "gst-player")]
fn main_loop(uri: &str) -> Result<(), utils::ExampleError> {
    gst::init().map_err(utils::ExampleError::InitFailed)?;

    let main_loop = glib::MainLoop::new(None, false);

    let dispatcher = gst_player::PlayerGMainContextSignalDispatcher::new(None).unwrap();
    let player = gst_player::Player::new(None, Some(&dispatcher));
    player
        .set_property("uri", &glib::Value::from(uri))
        .expect("Can't set uri property");

    let result = std::cell::RefCell::new(Some(Ok(())));
    let error = std::sync::Arc::new(std::sync::Mutex::new(send_cell::SendCell::new(result)));
    let error_clone = error.clone();

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
    player.connect_error(move |_, err| {
        let main_loop = &main_loop_clone;
        let player = &player_clone;

        let error = std::sync::Arc::clone(&error_clone);
        let guard = error.lock().unwrap();
        let cell = &*guard;
        let refcell = cell.get();
        *refcell.borrow_mut() = Some(Err(utils::ExampleError::ElementError(
            "player".to_owned(),
            err.clone(),
            "".to_owned(),
        )));

        player.stop();
        main_loop.quit();
    });

    player.play();
    main_loop.run();

    let guard = error.as_ref().lock().unwrap();
    let cell = &*guard;
    let refcell = cell.get();
    let result = refcell.borrow();

    if let Some(ref e) = *result {
        return e.clone();
    } else {
        Ok(())
    }
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
        panic!("Usage: player uri");
    };

    match main_loop(uri) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}
