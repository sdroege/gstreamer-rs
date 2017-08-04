extern crate gstreamer as gst;
use gst::*;
extern crate gstreamer_app as gst_app;
use gst_app::*;

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
            ("format", &"S16BE"),
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
            assert_eq!(buffer.get_size() % 2, 0);
            let map = buffer.map_read().expect("Unable to map buffer for reading");
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
