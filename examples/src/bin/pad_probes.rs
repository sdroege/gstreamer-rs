extern crate gstreamer as gst;
use gst::prelude::*;
extern crate gstreamer_audio as gst_audio;

extern crate byte_slice_cast;
use byte_slice_cast::*;

use std::u64;
use std::i16;

fn main() {
    gst::init().unwrap();

    let pipeline = gst::parse_launch(&format!(
        "audiotestsrc name=src ! audio/x-raw,format={},channels=1 ! fakesink",
        gst_audio::AUDIO_FORMAT_S16.to_string()
    )).unwrap();
    let bus = pipeline.get_bus().unwrap();

    let src = pipeline
        .clone()
        .dynamic_cast::<gst::Bin>()
        .unwrap()
        .get_by_name("src")
        .unwrap();
    let src_pad = src.get_static_pad("src").unwrap();
    src_pad.add_probe(gst::PAD_PROBE_TYPE_BUFFER, |_, probe_info| {
        if let Some(gst::PadProbeData::Buffer(ref buffer)) = probe_info.data {
            let map = buffer.map_readable().unwrap();

            let samples = if let Ok(samples) = map.as_slice().as_slice_of::<i16>() {
                samples
            } else {
                return gst::PadProbeReturn::Ok;
            };

            let sum: f64 = samples
                .iter()
                .map(|sample| {
                    let f = f64::from(*sample) / f64::from(i16::MAX);
                    f * f
                })
                .sum();
            let rms = (sum / (samples.len() as f64)).sqrt();
            println!("rms: {}", rms);
        }

        gst::PadProbeReturn::Ok
    });

    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    loop {
        use gst::MessageView;

        let msg = match bus.timed_pop(u64::MAX) {
            None => break,
            Some(msg) => msg,
        };

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {}: {} ({:?})",
                    msg.get_src().get_path_string(),
                    err.get_error(),
                    err.get_debug()
                );
                break;
            }
            _ => (),
        }
    }

    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}
