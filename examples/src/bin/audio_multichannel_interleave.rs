// This example demonstrates how to mix multiple audio
// streams into a single output using the audiomixer element.
// In this case, we're mixing 4 stereo streams into a single 8 channel output.

use gst::prelude::*;
use std::env;

#[path = "../examples-common.rs"]
mod examples_common;

const TRACKS: i32 = 4;

fn create_source_and_link(pipeline: &gst::Pipeline, mixer: &gst::Element, track_number: i32) {
    let freq = ((track_number + 1) * 1000) as f64;
    let audiosrc = gst::ElementFactory::make("audiotestsrc")
        .property("freq", freq)
        .property("num-buffers", 2000)
        .build()
        .unwrap();
    let caps = gst_audio::AudioCapsBuilder::new().channels(2).build();
    let capsfilter = gst::ElementFactory::make("capsfilter")
        .property("caps", &caps)
        .build()
        .unwrap();

    pipeline.add_many([&audiosrc, &capsfilter]).unwrap();
    gst::Element::link_many([&audiosrc, &capsfilter]).unwrap();

    let src_pad = capsfilter.static_pad("src").unwrap();
    let mixer_pad = mixer.request_pad_simple("sink_%u").unwrap();

    // audiomixer expects a mix-matrix set on each input pad,
    // indicating which output channels our input should appear in.
    // Rows => input channels, columns => output channels.
    // Here each input channel will appear in exactly one output channel.
    let mut mix_matrix: Vec<Vec<f32>> = vec![];
    for i in 0..TRACKS {
        if i == track_number {
            mix_matrix.push(vec![1.0, 0.0]);
            mix_matrix.push(vec![0.0, 1.0]);
        } else {
            mix_matrix.push(vec![0.0, 0.0]);
            mix_matrix.push(vec![0.0, 0.0]);
        }
    }
    let mut audiomixer_config = gst_audio::AudioConverterConfig::new();
    audiomixer_config.set_mix_matrix(&mix_matrix);
    mixer_pad.set_property("converter-config", audiomixer_config);

    src_pad.link(&mixer_pad).unwrap();
}

fn example_main() {
    gst::init().unwrap();

    let args: Vec<_> = env::args().collect();
    let output_file = if args.len() == 2 {
        &args[1]
    } else {
        println!("Usage: audiomixer <output file>");
        std::process::exit(-1);
    };

    let pipeline = gst::Pipeline::new();
    let audiomixer = gst::ElementFactory::make("audiomixer").build().unwrap();

    // Using an arbitrary layout of 4 stereo pairs.
    let positions = [
        gst_audio::AudioChannelPosition::FrontLeft,
        gst_audio::AudioChannelPosition::FrontRight,
        gst_audio::AudioChannelPosition::RearLeft,
        gst_audio::AudioChannelPosition::RearRight,
        gst_audio::AudioChannelPosition::SideLeft,
        gst_audio::AudioChannelPosition::SideRight,
        gst_audio::AudioChannelPosition::TopFrontLeft,
        gst_audio::AudioChannelPosition::TopFrontRight,
    ];

    let mask = gst_audio::AudioChannelPosition::positions_to_mask(&positions, true).unwrap();
    let caps = gst_audio::AudioCapsBuilder::new()
        .channels(positions.len() as i32)
        .channel_mask(mask)
        .build();
    let capsfilter = gst::ElementFactory::make("capsfilter")
        .property("caps", &caps)
        .build()
        .unwrap();

    let audioconvert = gst::ElementFactory::make("audioconvert").build().unwrap();
    let audioresample = gst::ElementFactory::make("audioresample").build().unwrap();
    let wavenc = gst::ElementFactory::make("wavenc").build().unwrap();
    let sink = gst::ElementFactory::make("filesink")
        .property("location", output_file)
        .build()
        .unwrap();

    pipeline
        .add_many([
            &audiomixer,
            &capsfilter,
            &audioconvert,
            &audioresample,
            &wavenc,
            &sink,
        ])
        .unwrap();
    gst::Element::link_many([
        &audiomixer,
        &capsfilter,
        &audioconvert,
        &audioresample,
        &wavenc,
        &sink,
    ])
    .unwrap();

    for i in 0..TRACKS {
        create_source_and_link(&pipeline, &audiomixer, i);
    }

    let bus = pipeline.bus().expect("Pipeline without bus");

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to start pipeline");

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;
        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                eprintln!(
                    "Error from {:?}: {} ({:?})",
                    msg.src().map(|s| s.path_string()),
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
        .expect("Unable to change pipeline state to NULL");
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    examples_common::run(example_main);
}
