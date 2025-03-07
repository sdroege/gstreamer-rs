// This example demonstrates how to use the gstreamer editing services.
// This is gstreamer's framework to implement non-linear editing.
// It provides a timeline API that internally manages a dynamically changing
// pipeline. (e.g.: alternating video streams in second 1, 2, and 3)
// Timeline:
//         _________________________________________________
//         |     00:01    |     00:02    |     00:03    |
//         =================================================
// Layer0: ||  ###CLIP####||             ||   ###CLIP###||
//         ||  ####00#####||             ||   ####01####||
//         =================================================
// Layer1: ||             ###CLIP####    ||             ||
//         ||             ####00#####    ||             ||
//         =================================================

// - Assets are the base of most components in GES. One asset essentially represents
//    one resource (e.g. a file). Different files and filetypes can contain different
//    types of things. Thus - you can extract different high-level types from an
//    asset. If you created an asset from a video file, you could for example "extract"
//    a GESClip from it. Same goes for audio files.
// - There even is the GESProject subclass of GESAsset, which can be used to load a whole
//    previously saved project. And since GESProject essentially is a GESAsset itself, you
//    can then extract the stored components (like the timeline e.g.) from it.
// - Clips are the high-level types (above assets), managing multimedia elements (such as
//    videos or audio clips). Within the timeline, they are arranged in layers.
//    Those layers essentially behave like in common photo editing software: They specify
//    the order in which they are composited, and can therefore overlay each other.
//    Clips are essentially wrappers around the underlying GStreamer elements needed
//    to work with them. They also provide high-level APIs to add effects into the
//    clip's internal pipeline.
//    Multiple clips can also be grouped together (even across layers!) to one, making it
//    possible to work with all of them as if they were one.
// - Like noted above, Layers specify the order in which the different layers are composited.
//    This is specified by their priority. Layers with higher priority (lower number) trump
//    those with lowers (higher number). Thus, Layers with higher priority are "in the front".
// - The timeline is the enclosing element, grouping all layers and providing a timeframe.

use std::env;

use ges::prelude::*;

#[allow(unused_imports)]
#[path = "../examples-common.rs"]
mod examples_common;

fn configure_pipeline(pipeline: &ges::Pipeline, output_name: &str) {
    // Every audiostream piped into the encodebin should be encoded using opus.
    let audio_profile =
        gst_pbutils::EncodingAudioProfile::builder(&gst::Caps::builder("audio/x-opus").build())
            .build();

    // Every videostream piped into the encodebin should be encoded using vp8.
    let video_profile =
        gst_pbutils::EncodingVideoProfile::builder(&gst::Caps::builder("video/x-vp8").build())
            .build();

    // All streams are then finally combined into a webm container.
    let container_profile =
        gst_pbutils::EncodingContainerProfile::builder(&gst::Caps::builder("video/webm").build())
            .name("container")
            .add_profile(video_profile)
            .add_profile(audio_profile)
            .build();

    // Apply the EncodingProfile to the pipeline, and set it to render mode
    let output_uri = format!("{output_name}.webm");
    pipeline
        .set_render_settings(&output_uri, &container_profile)
        .expect("Failed to set render settings");
    pipeline
        .set_mode(ges::PipelineFlags::RENDER)
        .expect("Failed to set pipeline to render mode");
}

fn main_loop(uri: &str, output: Option<&String>) -> Result<(), glib::BoolError> {
    ges::init()?;

    // Begin by creating a timeline with audio and video tracks
    let timeline = ges::Timeline::new_audio_video();
    // Create a new layer that will contain our timed clips.
    let layer = timeline.append_layer();
    let pipeline = ges::Pipeline::new();
    pipeline.set_timeline(&timeline)?;

    // If requested, configure the pipeline so it renders to a file.
    if let Some(output_name) = output {
        configure_pipeline(&pipeline, output_name);
    }

    // Load a clip from the given uri and add it to the layer.
    let clip = ges::UriClip::new(uri).expect("Failed to create clip");
    layer.add_clip(&clip)?;

    // Add an effect to the clip's video stream.
    let effect = ges::Effect::new("agingtv").expect("Failed to create effect");
    clip.add(&effect).unwrap();

    println!(
        "Agingtv scratch-lines: {}",
        clip.child_property("scratch-lines")
            .unwrap()
            .serialize()
            .unwrap()
    );

    // Retrieve the asset that was automatically used behind the scenes, to
    // extract the clip from.
    let asset = clip.asset().unwrap();
    let duration = asset
        .downcast::<ges::UriClipAsset>()
        .unwrap()
        .duration()
        .expect("unknown duration");
    println!(
        "Clip duration: {} - playing file from {} for {}",
        duration,
        duration / 2,
        duration / 4,
    );

    // The inpoint specifies where in the clip we start, the duration specifies
    // how much we play from that point onwards. Setting the inpoint to something else
    // than 0, or the duration something smaller than the clip's actual duration will
    // cut the clip.
    clip.set_inpoint(duration / 2);
    clip.set_duration(duration / 4);

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
    }

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");

    Ok(())
}

#[allow(unused_variables)]
fn example_main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        println!("Usage: ges input [output]");
        std::process::exit(-1)
    }

    let input_uri: &str = args[1].as_ref();
    let output = args.get(2);

    match main_loop(input_uri, output) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {e}"),
    }
}

fn main() {
    // examples_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    examples_common::run(example_main);
}
