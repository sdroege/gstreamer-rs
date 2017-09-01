extern crate gstreamer as gst;
use gst::prelude::*;
extern crate gstreamer_app as gst_app;
extern crate gstreamer_audio as gst_audio;

extern crate byte_slice_cast;
use byte_slice_cast::*;

use std::u64;
use std::i16;
use std::i32;

pub mod utils;

fn create_pipeline() -> Result<gst::Pipeline, utils::ExampleError> {
    gst::init().map_err(utils::ExampleError::InitFailed)?;
    let pipeline = gst::Pipeline::new(None);
    let src = utils::create_element("audiotestsrc")?;
    let sink = utils::create_element("appsink")?;

    pipeline
        .add_many(&[&src, &sink])
        .expect("Unable to add elements in the pipeline");

    utils::link_elements(&src, &sink)?;

    let appsink = sink.clone()
        .dynamic_cast::<gst_app::AppSink>()
        .expect("Sink element is expected to be an appsink!");

    appsink.set_caps(&gst::Caps::new_simple(
        "audio/x-raw",
        &[
            ("format", &gst_audio::AUDIO_FORMAT_S16.to_string()),
            ("layout", &"interleaved"),
            ("channels", &(1i32)),
            ("rate", &gst::IntRange::<i32>::new(1, i32::MAX)),
        ],
    ));

    appsink.set_callbacks(gst_app::AppSinkCallbacks::new(
        /* eos */
        |_| {},
        /* new_preroll */
        |_| gst::FlowReturn::Ok,
        /* new_samples */
        |appsink| {
            let sample = match appsink.pull_sample() {
                None => return gst::FlowReturn::Eos,
                Some(sample) => sample,
            };

            let buffer = sample
                .get_buffer()
                .expect("Unable to extract buffer from the sample");

            let map = buffer
                .map_readable()
                .expect("Unable to map buffer for reading");

            let samples = if let Ok(samples) = map.as_slice().as_slice_of::<i16>() {
                samples
            } else {
                return gst::FlowReturn::Error;
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

            gst::FlowReturn::Ok
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
        use gst::MessageView;

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
