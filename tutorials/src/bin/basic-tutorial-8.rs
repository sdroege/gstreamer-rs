use std::sync::{Arc, Mutex};

use byte_slice_cast::*;
use glib::source::SourceId;
use gst::prelude::*;
use gst_app::{AppSink, AppSrc};
use gst_audio::AudioInfo;

const CHUNK_SIZE: usize = 1024; // Amount of bytes we are sending in each buffer
const SAMPLE_RATE: u32 = 44_100; // Samples per second we are sending

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
        eprintln!("Failed to initialize Gst: {err}");
        return;
    }

    let info = AudioInfo::builder(gst_audio::AudioFormat::S16le, SAMPLE_RATE, 1)
        .build()
        .unwrap();
    let audio_caps = info.to_caps().unwrap();

    let appsrc = gst_app::AppSrc::builder()
        .name("audio_source")
        .caps(&audio_caps)
        .format(gst::Format::Time)
        .build();
    let tee = gst::ElementFactory::make("tee")
        .name("tee")
        .build()
        .unwrap();
    let audio_queue = gst::ElementFactory::make("queue")
        .name("audio_queue")
        .build()
        .unwrap();
    let audio_convert1 = gst::ElementFactory::make("audioconvert")
        .name("audio_convert1")
        .build()
        .unwrap();
    let audio_resample = gst::ElementFactory::make("audioresample")
        .name("audio_resample")
        .build()
        .unwrap();
    let audio_sink = gst::ElementFactory::make("autoaudiosink")
        .name("audio_sink")
        .build()
        .unwrap();
    let video_queue = gst::ElementFactory::make("queue")
        .name("video_queue")
        .build()
        .unwrap();
    let audio_convert2 = gst::ElementFactory::make("audioconvert")
        .name("audio_convert2")
        .build()
        .unwrap();
    let visual = gst::ElementFactory::make("wavescope")
        .name("visual")
        .property_from_str("shader", "none")
        .property_from_str("style", "lines")
        .build()
        .unwrap();
    let video_convert = gst::ElementFactory::make("videoconvert")
        .name("video_convert")
        .build()
        .unwrap();
    let video_sink = gst::ElementFactory::make("autovideosink")
        .name("video_sink")
        .build()
        .unwrap();
    let app_queue = gst::ElementFactory::make("queue")
        .name("app_queue")
        .build()
        .unwrap();
    let appsink = gst_app::AppSink::builder()
        .caps(&audio_caps)
        .name("app_sink")
        .build();

    let pipeline = gst::Pipeline::with_name("test-pipeline");

    pipeline
        .add_many([
            appsrc.upcast_ref(),
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
            appsink.upcast_ref(),
        ])
        .unwrap();

    gst::Element::link_many([appsrc.upcast_ref(), &tee]).unwrap();
    gst::Element::link_many([&audio_queue, &audio_convert1, &audio_resample, &audio_sink]).unwrap();
    gst::Element::link_many([
        &video_queue,
        &audio_convert2,
        &visual,
        &video_convert,
        &video_sink,
    ])
    .unwrap();
    gst::Element::link_many([&app_queue, appsink.upcast_ref()]).unwrap();

    let tee_audio_pad = tee.request_pad_simple("src_%u").unwrap();
    println!(
        "Obtained request pad {} for audio branch",
        tee_audio_pad.name()
    );
    let queue_audio_pad = audio_queue.static_pad("sink").unwrap();
    tee_audio_pad.link(&queue_audio_pad).unwrap();

    let tee_video_pad = tee.request_pad_simple("src_%u").unwrap();
    println!(
        "Obtained request pad {} for video branch",
        tee_video_pad.name()
    );
    let queue_video_pad = video_queue.static_pad("sink").unwrap();
    tee_video_pad.link(&queue_video_pad).unwrap();
    let tee_app_pad = tee.request_pad_simple("src_%u").unwrap();
    let queue_app_pad = app_queue.static_pad("sink").unwrap();
    tee_app_pad.link(&queue_app_pad).unwrap();

    let data: Arc<Mutex<CustomData>> = Arc::new(Mutex::new(CustomData::new(&appsrc, &appsink)));

    let data_weak = Arc::downgrade(&data);
    let data_weak2 = Arc::downgrade(&data);
    appsrc.set_callbacks(
        gst_app::AppSrcCallbacks::builder()
            .need_data(move |_, _size| {
                let data = match data_weak.upgrade() {
                    Some(data) => data,
                    None => return,
                };
                let mut d = data.lock().unwrap();

                if d.source_id.is_none() {
                    println!("start feeding");

                    let data_weak = Arc::downgrade(&data);
                    d.source_id = Some(glib::source::idle_add(move || {
                        let data = match data_weak.upgrade() {
                            Some(data) => data,
                            None => return glib::Continue(false),
                        };

                        let (appsrc, buffer) = {
                            let mut data = data.lock().unwrap();
                            let mut buffer = gst::Buffer::with_size(CHUNK_SIZE).unwrap();
                            let num_samples = CHUNK_SIZE / 2; /* Each sample is 16 bits */
                            let pts = gst::ClockTime::SECOND
                                .mul_div_floor(data.num_samples, u64::from(SAMPLE_RATE))
                                .expect("u64 overflow");
                            let duration = gst::ClockTime::SECOND
                                .mul_div_floor(num_samples as u64, u64::from(SAMPLE_RATE))
                                .expect("u64 overflow");

                            {
                                let buffer = buffer.get_mut().unwrap();
                                {
                                    let mut samples = buffer.map_writable().unwrap();
                                    let samples = samples.as_mut_slice_of::<i16>().unwrap();

                                    // Generate some psychedelic waveforms
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

                        glib::Continue(appsrc.push_buffer(buffer).is_ok())
                    }));
                }
            })
            .enough_data(move |_| {
                let data = match data_weak2.upgrade() {
                    Some(data) => data,
                    None => return,
                };

                let mut data = data.lock().unwrap();
                if let Some(source) = data.source_id.take() {
                    println!("stop feeding");
                    source.remove();
                }
            })
            .build(),
    );

    let data_weak = Arc::downgrade(&data);
    appsink.set_callbacks(
        gst_app::AppSinkCallbacks::builder()
            .new_sample(move |_| {
                let data = match data_weak.upgrade() {
                    Some(data) => data,
                    None => return Ok(gst::FlowSuccess::Ok),
                };

                let appsink = {
                    let data = data.lock().unwrap();
                    data.appsink.clone()
                };

                if let Ok(_sample) = appsink.pull_sample() {
                    use std::io::{self, Write};
                    // The only thing we do in this example is print a * to indicate a received buffer
                    print!("*");
                    let _ = io::stdout().flush();
                }

                Ok(gst::FlowSuccess::Ok)
            })
            .build(),
    );

    let main_loop = glib::MainLoop::new(None, false);
    let main_loop_clone = main_loop.clone();
    let bus = pipeline.bus().unwrap();
    #[allow(clippy::single_match)]
    bus.connect_message(Some("error"), move |_, msg| match msg.view() {
        gst::MessageView::Error(err) => {
            let main_loop = &main_loop_clone;
            eprintln!(
                "Error received from element {:?}: {}",
                err.src().map(|s| s.path_string()),
                err.error()
            );
            eprintln!("Debugging information: {:?}", err.debug());
            main_loop.quit();
        }
        _ => unreachable!(),
    });
    bus.add_signal_watch();

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state.");

    main_loop.run();

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state.");

    bus.remove_signal_watch();
}
