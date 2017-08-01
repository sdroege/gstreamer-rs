extern crate gstreamer as gst;
use gst::*;
extern crate gstreamer_app as gst_app;
use gst_app::*;

extern crate glib;
use glib::*;

use std::u64;
use std::i16;
use std::i32;

fn main() {
    gst::init().unwrap();

    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("audiotestsrc", None).unwrap();
    let sink = gst::ElementFactory::make("appsink", None).unwrap();

    pipeline.add_many(&[&src, &sink]).unwrap();
    gst::Element::link_many(&[&src, &sink]).unwrap();

    let appsink = sink.clone().dynamic_cast::<AppSink>().unwrap();
    appsink.set_caps(&Caps::new_simple(
        "audio/x-raw",
        &[
            (&"format", &"S16BE"),
            (&"layout", &"interleaved"),
            (&"channels", &(1i32)),
            (&"rate", &IntRange::<i32>::new(1, i32::MAX)),
        ],
    ));

    appsink.set_callbacks(AppSinkCallbacks::new(
        /* eos */
        |_| {},
        /* new_preroll */
        |_| FlowReturn::Ok,
        /* new_samples */
        |appsink| {
            let sample = match appsink.pull_sample() {
                None => return FlowReturn::Eos,
                Some(sample) => sample,
            };

            let buffer = sample.get_buffer().unwrap();
            assert_eq!(buffer.get_size() % 2, 0);
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

            FlowReturn::Ok
        },
    ));

    assert_ne!(
        pipeline.set_state(gst::State::Playing),
        gst::StateChangeReturn::Failure
    );

    let bus = pipeline.get_bus().unwrap();

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

    assert_ne!(
        pipeline.set_state(gst::State::Null),
        gst::StateChangeReturn::Failure
    );
}
