// This example shows how to use the GstPlay API.
// The GstPlay API is a convenience API to allow implement playback applications
// without having to write too much code.
// Most of the tasks a play needs to support (such as seeking and switching
// audio / subtitle streams or changing the volume) are all supported by simple
// one-line function calls on the GstPlay.

use std::env;

use anyhow::Error;

#[path = "../examples-common.rs"]
mod examples_common;

use gst_play::{Play, PlayMessage, PlayVideoRenderer};

fn main_loop(uri: &str) -> Result<(), Error> {
    gst::init()?;

    let play = Play::new(PlayVideoRenderer::NONE);
    play.set_uri(Some(uri));
    play.play();

    let mut result = Ok(());
    for msg in play.message_bus().iter_timed(gst::ClockTime::NONE) {
        match PlayMessage::parse(&msg) {
            Ok(PlayMessage::EndOfStream) => {
                play.stop();
                break;
            }
            Ok(PlayMessage::Error { error, details: _ }) => {
                result = Err(error);
                play.stop();
                break;
            }
            Ok(_) => (),
            Err(_) => unreachable!(),
        }
    }
    result.map_err(|e| e.into())
}

fn example_main() {
    let args: Vec<_> = env::args().collect();
    let uri: &str = if args.len() == 2 {
        args[1].as_ref()
    } else {
        println!("Usage: play uri");
        std::process::exit(-1)
    };

    match main_loop(uri) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    examples_common::run(example_main);
}
