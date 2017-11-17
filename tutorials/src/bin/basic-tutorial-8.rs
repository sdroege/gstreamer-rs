use std::sync::{Arc, Mutex};

extern crate byte_slice_cast;
use byte_slice_cast::*;

extern crate gstreamer as gst;
use gst::prelude::*;
extern crate gstreamer_audio as gst_audio;
use gst_audio::AudioInfo;
extern crate gstreamer_app as gst_app;
use gst_app::{AppSrc, AppSink};
extern crate glib;
use glib::source::SourceId;

const CHUNK_SIZE: usize = 1024; // Amount of bytes we are sending in each buffer
const SAMPLE_RATE: u32 = 44100; // Samples per second we are sending

#[derive(Debug)]
struct CustomData {
    source_id: Option<SourceId>,

    num_samples: u64, // Number of samples generated so far (for timestamp generation)
    // For waveform generation
    a: f64,
    b: f64,
    c: f64,
    d: f64,

    appsrc: AppSrc,
    appsink: AppSink,
}

impl CustomData {
    fn new(appsrc: &AppSrc, appsink: &AppSink) -> CustomData {
        CustomData {
            source_id: None,
            num_samples: 0,
            a: 0.0,
            b: 1.0,
            c: 0.0,
            d: 1.0,
            appsrc: appsrc.clone(),
            appsink: appsink.clone(),
        }
    }
}

fn main() {
    // Initialize GStreamer
    if let Err(err) = gst::init() {
        eprintln!("Failed to initialize Gst: {}", err);
        return;
    }

    let appsrc = gst::ElementFactory::make("appsrc", "audio_source").unwrap();
    let tee = gst::ElementFactory::make("tee", "tee").unwrap();
    let audio_queue = gst::ElementFactory::make("queue", "audio_queue").unwrap();
    let audio_convert1 = gst::ElementFactory::make("audioconvert", "audio_convert1").unwrap();
    let audio_resample = gst::ElementFactory::make("audioresample", "audio_resample").unwrap();
    let audio_sink = gst::ElementFactory::make("autoaudiosink", "audio_sink").unwrap();
    let video_queue = gst::ElementFactory::make("queue", "video_queue").unwrap();
    let audio_convert2 = gst::ElementFactory::make("audioconvert", "audio_convert2").unwrap();
    let visual = gst::ElementFactory::make("wavescope", "visual").unwrap();
    let video_convert = gst::ElementFactory::make("videoconvert", "video_convert").unwrap();
    let video_sink = gst::ElementFactory::make("autovideosink", "video_sink").unwrap();
    let app_queue = gst::ElementFactory::make("queue", "app_queue").unwrap();
    let appsink = gst::ElementFactory::make("appsink", "app_sink").unwrap();

    let pipeline = gst::Pipeline::new("test-pipeline");

    visual.set_property_from_str("shader", "none");
    visual.set_property_from_str("style", "lines");

    pipeline
        .add_many(
            &[
                &appsrc,
                &tee,
                &audio_queue,
                &audio_convert1,
                &audio_resample,
                &audio_sink,
                &video_queue,
                &audio_convert2,
                &visual,
                &video_convert,
                &video_sink,
                &app_queue,
                &appsink,
            ],
        )
        .unwrap();

    gst::Element::link_many(&[&appsrc, &tee]).unwrap();
    gst::Element::link_many(
        &[&audio_queue, &audio_convert1, &audio_resample, &audio_sink],
    ).unwrap();
    gst::Element::link_many(
        &[
            &video_queue,
            &audio_convert2,
            &visual,
            &video_convert,
            &video_sink,
        ],
    ).unwrap();
    gst::Element::link_many(&[&app_queue, &appsink]).unwrap();

    let tee_audio_pad = tee.get_request_pad("src_%u").unwrap();
    println!(
        "Obtained request pad {} for audio branch",
        tee_audio_pad.get_name()
    );
    let queue_audio_pad = audio_queue.get_static_pad("sink").unwrap();
    tee_audio_pad.link(&queue_audio_pad).into_result().unwrap();

    let tee_video_pad = tee.get_request_pad("src_%u").unwrap();
    println!(
        "Obtained request pad {} for video branch",
        tee_video_pad.get_name()
    );
    let queue_video_pad = video_queue.get_static_pad("sink").unwrap();
    tee_video_pad.link(&queue_video_pad).into_result().unwrap();
    let tee_app_pad = tee.get_request_pad("src_%u").unwrap();
    let queue_app_pad = app_queue.get_static_pad("sink").unwrap();
    tee_app_pad.link(&queue_app_pad).into_result().unwrap();

    // configure appsrc
    let info = AudioInfo::new(gst_audio::AudioFormat::S16le, SAMPLE_RATE, 1)
        .build()
        .unwrap();
    let audio_caps = info.to_caps().unwrap();

    let appsrc = appsrc.clone().dynamic_cast::<AppSrc>().expect(
        "Source element is expected to be an appsrc!",
    );
    appsrc.set_caps(&audio_caps);
    appsrc.set_property_format(gst::Format::Time);

    let appsink = appsink.clone().dynamic_cast::<AppSink>().expect(
        "Sink element is expected to be an appsink!",
    );

    let data: Arc<Mutex<CustomData>> = Arc::new(Mutex::new(CustomData::new(&appsrc, &appsink)));

    let data_clone = data.clone();
    appsrc.connect_need_data(move |_, _size| {
        let data = &data_clone;
        let mut d = data.lock().unwrap();

        if d.source_id.is_none() {
            println!("start feeding");

            let data_clone = data.clone();
            d.source_id = Some(glib::source::idle_add(move || {
                let data = &data_clone;

                let (appsrc, buffer) = {
                    let mut data = data.lock().unwrap();
                    let mut buffer = gst::Buffer::with_size(CHUNK_SIZE).unwrap();
                    let num_samples = CHUNK_SIZE / 2; /* Each sample is 16 bits */
                    let pts = gst::SECOND
                        .mul_div_floor(data.num_samples, SAMPLE_RATE as u64)
                        .expect("u64 overflow");
                    let duration = gst::SECOND
                        .mul_div_floor(CHUNK_SIZE as u64, SAMPLE_RATE as u64)
                        .expect("u64 overflow");

                    {
                        let buffer = buffer.get_mut().unwrap();
                        {
                            let mut samples = buffer.map_writable().unwrap();
                            let samples = samples.as_mut_slice().as_mut_slice_of::<i16>().unwrap();

                            // Generate some psychodelic waveforms
                            data.c += data.d;
                            data.d -= data.c / 1000.0;
                            let freq = 1100.0 + 1000.0 * data.d;

                            for sample in samples.iter_mut() {
                                data.a += data.b;
                                data.b -= data.a / freq;
                                *sample = 500 * (data.a as i16);
                            }

                            data.num_samples += num_samples as u64;

                        }

                        buffer.set_pts(pts);
                        buffer.set_duration(duration);
                    }

                    (data.appsrc.clone(), buffer)
                };

                match appsrc.push_buffer(buffer) {
                    gst::FlowReturn::Ok => glib::Continue(true),
                    _ => glib::Continue(false),
                }
            }));
        }
    });

    let data_clone = data.clone();
    appsrc.connect_enough_data(move |_| {
        let data = &data_clone;

        let mut data = data.lock().unwrap();
        if let Some(source) = data.source_id.take() {
            println!("stop feeding");
            glib::source::source_remove(source);
        }
    });

    // configure appsink
    appsink.set_emit_signals(true);
    appsink.set_caps(&audio_caps);

    let data_clone = data.clone();
    appsink.connect_new_sample(move |_| {
        let appsink = {
            let data = &data_clone.lock().unwrap();
            data.appsink.clone()
        };

        if let Some(_sample) = appsink.pull_sample() {
            // The only thing we do in this example is print a * to indicate a received buffer
            print!("*");
        }

        gst::FlowReturn::Ok
    });



    pipeline
        .set_state(gst::State::Playing)
        .into_result()
        .expect("Unable to set the pipeline to the Playing state.");
    let main_loop = glib::MainLoop::new(None, false);
    let main_loop_clone = main_loop.clone();
    let bus = pipeline.get_bus().unwrap();
    bus.connect_message(move |_, msg| match msg.view() {
        gst::MessageView::Error(err) => {
            let main_loop = &main_loop_clone;
            eprintln!(
                "Error received from element {:?}: {}",
                msg.get_src().map(|s| s.get_path_string()),
                err.get_error()
            );
            eprintln!("Debugging information: {:?}", err.get_debug());
            main_loop.quit();
        }
        _ => (),
    });
    bus.add_signal_watch();

    main_loop.run();

    pipeline.set_state(gst::State::Null).into_result().expect(
        "Unable to set the pipeline to the Null state.",
    );
}
