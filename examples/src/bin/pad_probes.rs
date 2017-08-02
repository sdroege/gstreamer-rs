extern crate gstreamer as gst;
use gst::*;

use std::u64;
use std::i16;

fn main() {
    gst::init().unwrap();

    let pipeline = gst::parse_launch(
        "audiotestsrc name=src ! audio/x-raw,format=S16BE,channels=1 ! fakesink",
    ).unwrap();
    let bus = pipeline.get_bus().unwrap();

    let src = pipeline
        .clone()
        .dynamic_cast::<Bin>()
        .unwrap()
        .get_by_name("src")
        .unwrap();
    let src_pad = src.get_static_pad("src").unwrap();
    src_pad.add_probe(PAD_PROBE_TYPE_BUFFER, |_, probe_info| {
        if let Some(PadProbeData::Buffer(ref buffer)) = probe_info.data {
            let map = buffer.map_read().unwrap();
            let data = map.as_slice();
            let sum: f64 = data.chunks(2)
                .map(|sample| {
                    let u: u16 = ((sample[0] as u16) << 8) | (sample[1] as u16);
                    let f = (u as i16 as f64) / (i16::MAX as f64);
                    f * f
                })
                .sum();
            let rms = (sum / ((data.len() / 2) as f64)).sqrt();
            println!("rms: {}", rms);
        }

        PadProbeReturn::Ok
    });

    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    loop {
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
