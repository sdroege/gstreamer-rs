// This example demonstrates how to get a raw video frame at a given position
// and then rescale and store it with the image crate:

// {uridecodebin} - {videoconvert} - {appsink}

// The appsink enforces RGBA so that the image crate can use it. The image crate also requires
// tightly packed pixels, which is the case for RGBA by default in GStreamer.

use gst::element_error;
use gst::prelude::*;

use anyhow::Error;
use derive_more::{Display, Error};

#[path = "../examples-common.rs"]
mod examples_common;

#[derive(Debug, Display, Error)]
#[display(fmt = "Missing element {}", _0)]
struct MissingElement(#[error(not(source))] &'static str);

#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {}: {} (debug: {:?})", src, error, debug)]
struct ErrorMessage {
    src: String,
    error: String,
    debug: Option<String>,
    source: glib::Error,
}

fn create_pipeline(uri: String, out_path: std::path::PathBuf) -> Result<gst::Pipeline, Error> {
    gst::init()?;

    // Create our pipeline from a pipeline description string.
    let pipeline = gst::parse_launch(&format!(
        "uridecodebin uri={} ! videoconvert ! appsink name=sink",
        uri
    ))?
    .downcast::<gst::Pipeline>()
    .expect("Expected a gst::Pipeline");

    // Get access to the appsink element.
    let appsink = pipeline
        .by_name("sink")
        .expect("Sink element not found")
        .downcast::<gst_app::AppSink>()
        .expect("Sink element is expected to be an appsink!");

    // Don't synchronize on the clock, we only want a snapshot asap.
    appsink.set_property("sync", false).unwrap();

    // Tell the appsink what format we want.
    // This can be set after linking the two objects, because format negotiation between
    // both elements will happen during pre-rolling of the pipeline.
    appsink.set_caps(Some(
        &gst::Caps::builder("video/x-raw")
            .field("format", gst_video::VideoFormat::Rgba.to_str())
            .build(),
    ));

    let mut got_snapshot = false;

    // Getting data out of the appsink is done by setting callbacks on it.
    // The appsink will then call those handlers, as soon as data is available.
    appsink.set_callbacks(
        gst_app::AppSinkCallbacks::builder()
            // Add a handler to the "new-sample" signal.
            .new_sample(move |appsink| {
                // Pull the sample in question out of the appsink's buffer.
                let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
                let buffer = sample.buffer().ok_or_else(|| {
                    element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Failed to get buffer from appsink")
                    );

                    gst::FlowError::Error
                })?;

                let caps = sample.caps().expect("Sample without caps");
                let info = gst_video::VideoInfo::from_caps(caps).expect("Failed to parse caps");

                // Make sure that we only get a single buffer
                if got_snapshot {
                    return Err(gst::FlowError::Eos);
                }
                got_snapshot = true;

                // At this point, buffer is only a reference to an existing memory region somewhere.
                // When we want to access its content, we have to map it while requesting the required
                // mode of access (read, read/write).
                // This type of abstraction is necessary, because the buffer in question might not be
                // on the machine's main memory itself, but rather in the GPU's memory.
                // So mapping the buffer makes the underlying memory region accessible to us.
                // See: https://gstreamer.freedesktop.org/documentation/plugin-development/advanced/allocation.html
                let map = buffer.map_readable().map_err(|_| {
                    element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Failed to map buffer readable")
                    );

                    gst::FlowError::Error
                })?;

                // We only want to have a single buffer and then have the pipeline terminate
                println!("Have video frame");

                // Calculate a target width/height that keeps the display aspect ratio while having
                // a height of 240 pixels
                let display_aspect_ratio = (info.width() as f64 * info.par().numer() as f64)
                    / (info.height() as f64 * info.par().denom() as f64);
                let target_height = 240;
                let target_width = target_height as f64 * display_aspect_ratio;

                // Create an ImageBuffer around the borrowed video frame data from GStreamer.
                let img = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(
                    info.width(),
                    info.height(),
                    map,
                )
                .expect("Failed to create ImageBuffer, probably a stride mismatch");

                // Scale image to our target dimensions
                let scaled_img =
                    image::imageops::thumbnail(&img, target_width as u32, target_height as u32);

                // Save it at the specific location. This automatically detects the file type
                // based on the filename.
                scaled_img.save(&out_path).map_err(|err| {
                    element_error!(
                        appsink,
                        gst::ResourceError::Write,
                        (
                            "Failed to write thumbnail file {}: {}",
                            out_path.display(),
                            err
                        )
                    );

                    gst::FlowError::Error
                })?;

                println!("Wrote thumbnail to {}", out_path.display());

                Err(gst::FlowError::Eos)
            })
            .build(),
    );

    Ok(pipeline)
}

fn main_loop(pipeline: gst::Pipeline, position: u64) -> Result<(), Error> {
    pipeline.set_state(gst::State::Paused)?;

    let bus = pipeline
        .bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    let mut seeked = false;

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::AsyncDone(..) => {
                if !seeked {
                    // AsyncDone means that the pipeline has started now and that we can seek
                    println!("Got AsyncDone message, seeking to {}s", position);

                    if pipeline
                        .seek_simple(gst::SeekFlags::FLUSH, position * gst::ClockTime::SECOND)
                        .is_err()
                    {
                        println!("Failed to seek, taking first frame");
                    }

                    pipeline.set_state(gst::State::Playing)?;
                    seeked = true;
                } else {
                    println!("Got second AsyncDone message, seek finished");
                }
            }
            MessageView::Eos(..) => {
                // The End-of-stream message is posted when the stream is done, which in our case
                // happens immediately after creating the thumbnail because we return
                // gst::FlowError::Eos then.
                println!("Got Eos message, done");
                break;
            }
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null)?;
                return Err(ErrorMessage {
                    src: msg
                        .src()
                        .map(|s| String::from(s.path_string()))
                        .unwrap_or_else(|| String::from("None")),
                    error: err.error().to_string(),
                    debug: err.debug(),
                    source: err.error(),
                }
                .into());
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null)?;

    Ok(())
}

fn example_main() {
    use std::env;

    let mut args = env::args();

    // Parse commandline arguments: input URI, position in seconds, output path
    let _arg0 = args.next().unwrap();
    let uri = args
        .next()
        .expect("No input URI provided on the commandline");
    let position = args
        .next()
        .expect("No position in second on the commandline");
    let position = position
        .parse::<u64>()
        .expect("Failed to parse position as integer");
    let out_path = args
        .next()
        .expect("No output path provided on the commandline");
    let out_path = std::path::PathBuf::from(out_path);

    match create_pipeline(uri, out_path).and_then(|pipeline| main_loop(pipeline, position)) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    examples_common::run(example_main);
}
