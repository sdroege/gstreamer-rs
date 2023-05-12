use anyhow::Error;
use gst::prelude::*;

#[path = "../tutorials-common.rs"]
mod tutorials_common;

fn tutorial_main() -> Result<(), Error> {
    // Initialize GStreamer
    gst::init()?;

    // Build the pipeline
    let pipeline = gst::parse_launch(
        "playbin uri=https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm")?;

    // Create elements that go inside the sink bin
    let equalizer = gst::ElementFactory::make("equalizer-3bands")
        .name("equalizer")
        .build()
        .expect("Could not create equalizer element.");
    let convert = gst::ElementFactory::make("audioconvert")
        .name("convert")
        .build()
        .expect("Could not create audioconvert element.");
    let sink = gst::ElementFactory::make("autoaudiosink")
        .name("audio_sink")
        .build()
        .expect("Could not create autoaudiosink element.");

    // Create the sink bin, add the elements and link them
    let bin = gst::Bin::with_name("audio_sink_bin");
    bin.add_many([&equalizer, &convert, &sink]).unwrap();
    gst::Element::link_many([&equalizer, &convert, &sink]).expect("Failed to link elements.");

    let pad = equalizer
        .static_pad("sink")
        .expect("Failed to get a static pad from equalizer.");
    let ghost_pad = gst::GhostPad::builder_with_target(&pad).unwrap().build();
    ghost_pad.set_active(true)?;
    bin.add_pad(&ghost_pad)?;

    // Configure the equalizer
    equalizer.set_property("band1", -24.0);
    equalizer.set_property("band2", -24.0);

    pipeline.set_property("audio-sink", &bin);

    // Set to PLAYING
    pipeline.set_state(gst::State::Playing)?;

    // Wait until an EOS or error message appears
    let bus = pipeline.bus().unwrap();
    let _msg = bus.timed_pop_filtered(
        gst::ClockTime::NONE,
        &[gst::MessageType::Error, gst::MessageType::Eos],
    );

    // Clean up
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
