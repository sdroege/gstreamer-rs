use std::sync::{Arc, Mutex};

use anyhow::Error;
use byte_slice_cast::*;
use glib::source::SourceId;
use gst::prelude::*;
use gst_app::AppSrc;
use gst_audio::AudioInfo;

#[path = "../tutorials-common.rs"]
mod tutorials_common;

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
}

impl CustomData {
    fn new(appsrc: &AppSrc) -> CustomData {
        CustomData {
            source_id: None,
            num_samples: 0,
            a: 0.0,
            b: 1.0,
            c: 0.0,
            d: 1.0,
            appsrc: appsrc.clone(),
        }
    }
}

fn tutorial_main() -> Result<(), Error> {
    // Initialize GStreamer
    gst::init().unwrap();

    // Create the playbin element
    let pipeline = gst::parse_launch("playbin uri=appsrc://").unwrap();

    // This part is called when playbin has created the appsrc element,
    // so we have a chance to configure it.
    pipeline.connect("source-setup", false, |args| {
        println!("Source has been created. Configuring.");

        let _pipeline = args[0].get::<gst::Element>().unwrap();
        let source = args[1]
            .get::<gst_app::AppSrc>()
            .expect("Source element is expected to be an appsrc!");

        let audio_info = AudioInfo::builder(gst_audio::AudioFormat::S16le, SAMPLE_RATE, 1)
            .build()
            .unwrap();
        let audio_caps = audio_info.to_caps().unwrap();

        source.set_caps(Some(&audio_caps));
        source.set_format(gst::Format::Time);

        let data: Arc<Mutex<CustomData>> = Arc::new(Mutex::new(CustomData::new(&source)));
        let data_clone = data.clone();

        source.set_callbacks(
            gst_app::AppSrcCallbacks::builder()
                // This signal callback is triggered when appsrc needs data.
                // Here, we add an idle handler to the mainloop to start pushing data into the appsrc.
                .need_data(move |_, _size| {
                    let data = &data_clone;
                    let mut d = data.lock().unwrap();

                    if d.source_id.is_none() {
                        println!("Start feeding");

                        let data_weak = Arc::downgrade(data);
                        d.source_id = Some(glib::source::idle_add(move || {
                            let data = match data_weak.upgrade() {
                                Some(data) => data,
                                None => return glib::Continue(false),
                            };

                            let (appsrc, buffer) = {
                                let mut data = data.lock().unwrap();
                                // Create a new empty buffer
                                let mut buffer = gst::Buffer::with_size(CHUNK_SIZE).unwrap();
                                // Each sample is 16 bits
                                let num_samples = CHUNK_SIZE / 2;
                                // Calculate timestamp and duration
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
                                    }

                                    data.num_samples += num_samples as u64;
                                    buffer.set_pts(pts);
                                    buffer.set_duration(duration);
                                }

                                (data.appsrc.clone(), buffer)
                            };

                            // Push the buffer into the appsrc
                            glib::Continue(appsrc.push_buffer(buffer).is_ok())
                        }));
                    }
                })
                // This callback is triggered when appsrc has enough data and we can stop sending.
                .enough_data(move |_| {
                    let mut d = data.lock().unwrap();
                    if let Some(source) = d.source_id.take() {
                        println!("Stop feeding");
                        source.remove();
                    }
                })
                .build(),
        );
        None
    });

    // Create a GLib main loop
    let main_loop = glib::MainLoop::new(None, false);
    let main_loop_clone = main_loop.clone();
    let bus = pipeline.bus().unwrap();

    // Instruct the bus to emit signals for each received message, and connect to the interesting signals
    #[allow(clippy::single_match)]
    bus.connect_message(Some("error"), move |_, msg| match msg.view() {
        gst::MessageView::Error(err) => {
            eprintln!(
                "Error received from element {:?}: {}",
                err.src().map(|s| s.path_string()),
                err.error()
            );
            eprintln!("Debugging information: {:?}", err.debug());
            main_loop_clone.quit();
        }
        _ => unreachable!(),
    });
    bus.add_signal_watch();

    // Start playing
    pipeline.set_state(gst::State::Playing)?;

    // Run the GLib main loop
    main_loop.run();

    // Cleanup
    pipeline.set_state(gst::State::Null)?;
    bus.remove_signal_watch();

    Ok(())
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    match tutorials_common::run(tutorial_main) {
        Ok(_) => {}
        Err(err) => eprintln!("Failed: {err}"),
    };
}
