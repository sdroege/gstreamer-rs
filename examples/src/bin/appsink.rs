extern crate gstreamer as gst;
use gst::*;
extern crate gstreamer_app as gst_app;
use gst_app::*;
extern crate gstreamer_audio as gst_audio;

extern crate glib;

use std::u64;
use std::i16;
use std::i32;

pub mod utils;

fn create_pipeline() -> Result<Pipeline, utils::ExampleError> {
    gst::init().map_err(utils::ExampleError::InitFailed)?;
    let pipeline = gst::Pipeline::new(None);
    let src = utils::create_element("audiotestsrc")?;
    let sink = utils::create_element("appsink")?;

    pipeline
        .add_many(&[&src, &sink])
        .expect("Unable to add elements in the pipeline");

    utils::link_elements(&src, &sink)?;

    let appsink = sink.clone()
        .dynamic_cast::<AppSink>()
        .expect("Sink element is expected to be an appsink!");

    appsink.set_caps(&Caps::new_simple(
        "audio/x-raw",
        &[
            ("format", &gst_audio::AUDIO_FORMAT_S16.to_string()),
            ("layout", &"interleaved"),
            ("channels", &(1i32)),
            ("rate", &IntRange::<i32>::new(1, i32::MAX)),
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

            let buffer = sample
                .get_buffer()
                .expect("Unable to extract buffer from the sample");

            let map = buffer
                .map_readable()
                .expect("Unable to map buffer for reading");

            let data =
                gst_audio::AudioData::new(map.as_slice(), gst_audio::AUDIO_FORMAT_S16).unwrap();
            let samples = if let gst_audio::AudioData::S16(samples) = data {
                samples
            } else {
                return FlowReturn::NotNegotiated;
            };

            let sum: f64 = samples
                .iter()
                .map(|sample| {
                    let f = (*sample as f64) / (i16::MAX as f64);
                    f * f
                })
                .sum();
            let rms = (sum / (samples.len() as f64)).sqrt();
            println!("rms: {}", rms);

            FlowReturn::Ok
        },
    ));

    Ok(pipeline)
}

fn main_loop() -> Result<(), utils::ExampleError> {
    let pipeline = create_pipeline()?;

    utils::set_state(&pipeline, gst::State::Playing)?;

    let bus = pipeline
        .get_bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    loop {
        let msg = match bus.timed_pop(u64::MAX) {
            None => break,
            Some(msg) => msg,
        };

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                utils::set_state(&pipeline, gst::State::Null)?;
                return Err(utils::ExampleError::ElementError(
                    msg.get_src().get_path_string(),
                    err.get_error(),
                    err.get_debug().unwrap(),
                ));
            }
            _ => (),
        }
    }

    utils::set_state(&pipeline, gst::State::Null)?;

    Ok(())
}

fn main() {
    match main_loop() {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}
