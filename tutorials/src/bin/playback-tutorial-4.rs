use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use anyhow::Error;
use glib::FlagsClass;
use gst::prelude::*;

#[path = "../tutorials-common.rs"]
mod tutorials_common;

const GRAPH_LENGTH: usize = 78;

fn tutorial_main() -> Result<(), Error> {
    // Initialize GStreamer
    gst::init()?;

    // Build the pipeline
    let uri =
        "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm";
    let pipeline = gst::ElementFactory::make("playbin")
        .name("playbin")
        .property("uri", uri)
        .build()?;

    // Set the download flag
    let flags = pipeline.property_value("flags");
    let flags_class = FlagsClass::with_type(flags.type_()).unwrap();
    let flags = flags_class
        .builder_with_value(flags)
        .unwrap()
        .set_by_nick("download")
        .build()
        .unwrap();
    pipeline.set_property_from_value("flags", &flags);

    // Uncomment this line to limit the amount of downloaded data.
    // pipeline.set_property("ring-buffer-max-size", 4_000_000u64);

    // Start playing
    let mut is_live = false;
    let ret = pipeline.set_state(gst::State::Playing)?;
    if ret == gst::StateChangeSuccess::NoPreroll {
        is_live = true;
    }

    let buffering_level = Arc::new(Mutex::new(100));
    let buffering_level_clone = buffering_level.clone();

    let main_loop = glib::MainLoop::new(None, false);
    let main_loop_clone = main_loop.clone();
    let pipeline_weak = pipeline.downgrade();
    let bus = pipeline.bus().unwrap();
    let _bus_watch = bus
        .add_watch(move |_, msg| {
            use gst::MessageView;

            let buffering_level = &buffering_level_clone;
            let pipeline = match pipeline_weak.upgrade() {
                Some(pipeline) => pipeline,
                None => return glib::Continue(false),
            };
            let main_loop = &main_loop_clone;
            match msg.view() {
                MessageView::Error(err) => {
                    println!(
                        "Error from {:?}: {} ({:?})",
                        err.src().map(|s| s.path_string()),
                        err.error(),
                        err.debug()
                    );
                    main_loop.quit();
                }
                MessageView::Eos(..) => {
                    main_loop.quit();
                }
                MessageView::Buffering(buffering) => {
                    // If the stream is live, we do not care about buffering.
                    if is_live {
                        return glib::Continue(true);
                    }

                    // Wait until buffering is complete before start/resume playing.
                    let percent = buffering.percent();
                    if percent < 100 {
                        let _ = pipeline.set_state(gst::State::Paused);
                    } else {
                        let _ = pipeline.set_state(gst::State::Playing);
                    }
                    *buffering_level.lock().unwrap() = percent;
                }
                MessageView::ClockLost(_) => {
                    // Get a new clock.
                    let _ = pipeline.set_state(gst::State::Paused);
                    let _ = pipeline.set_state(gst::State::Playing);
                }
                _ => (),
            };

            glib::Continue(true)
        })
        .expect("Failed to add bus watch");

    pipeline.connect("deep-notify::temp-location", false, |args| {
        let download_buffer = args[1].get::<gst::Object>().unwrap();
        println!(
            "Temporary file: {:?}",
            download_buffer.property::<Option<String>>("temp-location")
        );
        // Uncomment this line to keep the temporary file after the program exists.
        // download_buffer.set_property("temp-remove", false).ok();
        None
    });

    let pipeline_weak_ = pipeline.downgrade();
    let timeout_id = glib::timeout_add_seconds(1, move || {
        use gst::{format::Percent, GenericFormattedValue as GFV};

        let pipeline = match pipeline_weak_.upgrade() {
            Some(pipeline) => pipeline,
            None => return glib::Continue(false),
        };
        let mut graph = vec![b' '; GRAPH_LENGTH];
        let mut buffering = gst::query::Buffering::new(gst::Format::Percent);
        if pipeline.query(&mut buffering) {
            let ranges = buffering.ranges();
            for range in &ranges {
                let start = range.0;
                let stop = range.1;
                let start = if let GFV::Percent(start) = start {
                    start.unwrap()
                } else {
                    Percent::ZERO
                } / Percent::MAX;
                let stop = if let GFV::Percent(stop) = stop {
                    stop.unwrap()
                } else {
                    Percent::ZERO
                } / Percent::MAX;
                if start == 0 && stop == 0 {
                    continue;
                }
                let start_ = (start * GRAPH_LENGTH as u32) / (stop - start);
                let stop_ = (stop * GRAPH_LENGTH as u32) / (stop - start);
                for j in start_..stop_ {
                    graph[j as usize] = b'-';
                }
            }
        }

        if let Some(position) = pipeline.query_position::<gst::ClockTime>() {
            if let Some(duration) = pipeline.query_duration::<gst::ClockTime>() {
                let current_progress =
                    GRAPH_LENGTH as u64 * position.seconds() / duration.seconds();
                let buffering_level = buffering_level.lock().unwrap();
                graph[current_progress as usize] = if *buffering_level < 100 { b'X' } else { b'>' };
            }
        }

        print!("[{}]", std::str::from_utf8(&graph).unwrap());

        let buffering_level = buffering_level.lock().unwrap();
        if *buffering_level < 100 {
            print!("Buffering: {}%", *buffering_level);
        } else {
            print!("                ");
        }
        print!("\r");

        std::io::stdout().flush().unwrap();

        glib::Continue(true)
    });

    main_loop.run();

    // Shutdown pipeline
    pipeline.set_state(gst::State::Null)?;

    timeout_id.remove();

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
