#[cfg(feature = "tutorial5")]
mod tutorial5 {
    use std::os::raw::c_void;
    use std::process;

    extern crate glib;
    use self::glib::translate::*;
    use self::glib::*;

    extern crate gdk;
    use self::gdk::prelude::*;

    extern crate gtk;
    use self::gtk::*;

    extern crate send_cell;
    use self::send_cell::SendCell;

    extern crate gstreamer as gst;
    extern crate gstreamer_video as gst_video;
    use self::gst_video::prelude::*;

    use std::ops;

    // Custom struct to keep our window reference alive
    // and to store the timeout id so that we can remove
    // it from the main context again later and drop the
    // references it keeps inside its closures
    struct AppWindow {
        main_window: Window,
        timeout_id: Option<glib::SourceId>,
    }

    impl ops::Deref for AppWindow {
        type Target = Window;

        fn deref(&self) -> &Window {
            &self.main_window
        }
    }

    impl Drop for AppWindow {
        fn drop(&mut self) {
            if let Some(source_id) = self.timeout_id.take() {
                glib::source_remove(source_id);
            }
        }
    }

    // Extract tags from streams of @stype and add the info in the UI.
    fn add_streams_info(
        playbin: &gst::Element,
        textbufcell: &SendCell<gtk::TextBuffer>,
        stype: &str,
    ) {
        let textbuf = textbufcell.borrow();
        let propname: &str = &format!("n-{}", stype);
        let signame: &str = &format!("get-{}-tags", stype);

        match playbin.get_property(propname).unwrap().get() {
            Some(x) => for i in 0..x {
                let tags = playbin.emit(signame, &[&i]).unwrap().unwrap();

                if let Some(tags) = tags.get::<gst::TagList>() {
                    textbuf.insert_at_cursor(&format!("{} stream {}:\n ", stype, i));

                    if let Some(codec) = tags.get::<gst::tags::VideoCodec>() {
                        textbuf
                            .insert_at_cursor(&format!("    codec: {} \n", codec.get().unwrap()));
                    }

                    if let Some(codec) = tags.get::<gst::tags::AudioCodec>() {
                        textbuf
                            .insert_at_cursor(&format!("    codec: {} \n", codec.get().unwrap()));
                    }

                    if let Some(lang) = tags.get::<gst::tags::LanguageCode>() {
                        textbuf
                            .insert_at_cursor(&format!("    language: {} \n", lang.get().unwrap()));
                    }

                    if let Some(bitrate) = tags.get::<gst::tags::Bitrate>() {
                        textbuf.insert_at_cursor(&format!(
                            "    bitrate: {} \n",
                            bitrate.get().unwrap()
                        ));
                    }
                }
            },
            None => {
                eprintln!("Could not get {}!", propname);
            }
        }
    }

    // Extract metadata from all the streams and write it to the text widget in the GUI
    fn analyze_streams(playbin: &gst::Element, textbufcell: &SendCell<gtk::TextBuffer>) {
        {
            let textbuf = textbufcell.borrow();
            textbuf.set_text("");
        }

        add_streams_info(playbin, textbufcell, "video");
        add_streams_info(playbin, textbufcell, "audio");
        add_streams_info(playbin, textbufcell, "text");
    }

    // This creates all the GTK+ widgets that compose our application, and registers the callbacks
    fn create_ui(playbin: &gst::Element) -> AppWindow {
        let main_window = Window::new(WindowType::Toplevel);
        main_window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let play_button = gtk::Button::new_from_icon_name(
            "media-playback-start",
            gtk::IconSize::SmallToolbar.into(),
        );
        let pipeline = playbin.clone();
        play_button.connect_clicked(move |_| {
            let pipeline = &pipeline;
            pipeline
                .set_state(gst::State::Playing)
                .into_result()
                .unwrap();
        });

        let pause_button = gtk::Button::new_from_icon_name(
            "media-playback-pause",
            gtk::IconSize::SmallToolbar.into(),
        );
        let pipeline = playbin.clone();
        pause_button.connect_clicked(move |_| {
            let pipeline = &pipeline;
            pipeline
                .set_state(gst::State::Paused)
                .into_result()
                .unwrap();
        });

        let stop_button = gtk::Button::new_from_icon_name(
            "media-playback-stop",
            gtk::IconSize::SmallToolbar.into(),
        );
        let pipeline = playbin.clone();
        stop_button.connect_clicked(move |_| {
            let pipeline = &pipeline;
            pipeline.set_state(gst::State::Ready).into_result().unwrap();
        });

        let slider = gtk::Scale::new_with_range(
            gtk::Orientation::Horizontal,
            0.0 as f64,
            100.0 as f64,
            1.0 as f64,
        );
        let pipeline = playbin.clone();
        let slider_update_signal_id = slider.connect_value_changed(move |slider| {
            let pipeline = &pipeline;
            let value = slider.get_value() as u64;
            if let Err(_) = pipeline.seek_simple(
                gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
                value * gst::SECOND,
            ) {
                eprintln!("Seeking to {} failed", value);
            }
        });

        slider.set_draw_value(false);
        let pipeline = playbin.clone();
        let lslider = slider.clone();
        // Update the UI (seekbar) every second
        let timeout_id = gtk::timeout_add_seconds(1, move || {
            let pipeline = &pipeline;
            let lslider = &lslider;

            if let Some(dur) = pipeline.query_duration::<gst::ClockTime>() {
                let seconds = dur / gst::SECOND;
                lslider.set_range(0.0, seconds.map(|v| v as f64).unwrap_or(0.0));
            }

            if let Some(pos) = pipeline.query_position::<gst::ClockTime>() {
                let seconds = pos / gst::SECOND;
                lslider.block_signal(&slider_update_signal_id);
                lslider.set_value(seconds.map(|v| v as f64).unwrap_or(0.0));
                lslider.unblock_signal(&slider_update_signal_id);
            }

            Continue(true)
        });

        let controls = Box::new(Orientation::Horizontal, 0);
        controls.pack_start(&play_button, false, false, 0);
        controls.pack_start(&pause_button, false, false, 0);
        controls.pack_start(&stop_button, false, false, 0);
        controls.pack_start(&slider, true, true, 2);

        let video_window = DrawingArea::new();
        video_window.set_double_buffered(false);

        let video_overlay = playbin
            .clone()
            .dynamic_cast::<gst_video::VideoOverlay>()
            .unwrap();

        video_window.connect_realize(move |video_window| {
            let video_overlay = &video_overlay;
            let gdk_window = video_window.get_window().unwrap();

            if !gdk_window.ensure_native() {
                println!("Can't create native window for widget");
                process::exit(-1);
            }

            let display_type_name = gdk_window.get_display().get_type().name();
            if cfg!(feature = "tutorial5-x11") {
                // Check if we're using X11 or ...
                if display_type_name == "GdkX11Display" {
                    extern "C" {
                        pub fn gdk_x11_window_get_xid(
                            window: *mut glib::object::GObject,
                        ) -> *mut c_void;
                    }

                    unsafe {
                        let xid = gdk_x11_window_get_xid(gdk_window.to_glib_none().0);
                        video_overlay.set_window_handle(xid as usize);
                    }
                } else {
                    println!("Add support for display type '{}'", display_type_name);
                    process::exit(-1);
                }
            } else if cfg!(feature = "tutorial5-quartz") {
                if display_type_name == "GdkQuartzDisplay" {
                    extern "C" {
                        pub fn gdk_quartz_window_get_nsview(
                            window: *mut glib::object::GObject,
                        ) -> *mut c_void;
                    }

                    unsafe {
                        let window = gdk_quartz_window_get_nsview(gdk_window.to_glib_none().0);
                        video_overlay.set_window_handle(window as usize);
                    }
                } else {
                    println!(
                        "Unsupported display type '{}', compile with `--feature `",
                        display_type_name
                    );
                    process::exit(-1);
                }
            }
        });

        let streams_list = gtk::TextView::new();
        streams_list.set_editable(false);
        let pipeline_weak = playbin.downgrade();
        let textbuf = SendCell::new(
            streams_list
                .get_buffer()
                .expect("Couldn't get buffer from text_view"),
        );
        playbin
            .get_bus()
            .unwrap()
            .connect_message(move |_, msg| match msg.view() {
                gst::MessageView::Application(application) => {
                    let pipeline = match pipeline_weak.upgrade() {
                        Some(pipeline) => pipeline,
                        None => return,
                    };

                    if application.get_structure().map(|s| s.get_name()) == Some("tags-changed") {
                        analyze_streams(&pipeline, &textbuf);
                    }
                }
                _ => (),
            });

        let vbox = Box::new(Orientation::Horizontal, 0);
        vbox.pack_start(&video_window, true, true, 0);
        vbox.pack_start(&streams_list, false, false, 2);

        let main_box = Box::new(Orientation::Vertical, 0);
        main_box.pack_start(&vbox, true, true, 0);
        main_box.pack_start(&controls, false, false, 0);
        main_window.add(&main_box);
        main_window.set_default_size(640, 480);

        main_window.show_all();

        AppWindow {
            main_window,
            timeout_id: Some(timeout_id),
        }
    }

    // We are possibly in a GStreamer working thread, so we notify the main
    // thread of this event through a message in the bus
    fn post_app_message(playbin: &gst::Element) {
        let mbuilder = gst::Message::new_application(gst::Structure::new_empty("tags-changed"));
        let _ = playbin.post_message(&mbuilder.build());
    }

    pub fn run() {
        // Make sure the right features were activated
        if !cfg!(feature = "tutorial5-x11") && !cfg!(feature = "tutorial5-quartz") {
            eprintln!("No Gdk backend selected, compile with --features tutorial5[-x11][-quartz].");

            return;
        }

        // Initialize GTK
        if let Err(err) = gtk::init() {
            eprintln!("Failed to initialize GTK: {}", err);
            return;
        }

        // Initialize GStreamer
        if let Err(err) = gst::init() {
            eprintln!("Failed to initialize Gst: {}", err);
            return;
        }

        let uri = "https://www.freedesktop.org/software/gstreamer-sdk/\
                   data/media/sintel_trailer-480p.webm";
        let playbin = gst::ElementFactory::make("playbin", None).unwrap();
        playbin
            .set_property("uri", &uri)
            .unwrap();

        playbin
            .connect("video-tags-changed", false, |args| {
                let pipeline = args[0].get::<gst::Element>().unwrap();
                post_app_message(&pipeline);
                None
            })
            .unwrap();

        playbin
            .connect("audio-tags-changed", false, |args| {
                let pipeline = args[0].get::<gst::Element>().unwrap();
                post_app_message(&pipeline);
                None
            })
            .unwrap();

        playbin
            .connect("text-tags-changed", false, move |args| {
                let pipeline = args[0].get::<gst::Element>().unwrap();
                post_app_message(&pipeline);
                None
            })
            .unwrap();

        let window = create_ui(&playbin);

        let bus = playbin.get_bus().unwrap();
        bus.add_signal_watch();

        let pipeline_weak = playbin.downgrade();
        bus.connect_message(move |_, msg| {
            let pipeline = match pipeline_weak.upgrade() {
                Some(pipeline) => pipeline,
                None => return,
            };

            match msg.view() {
                //  This is called when an End-Of-Stream message is posted on the bus.
                // We just set the pipeline to READY (which stops playback).
                gst::MessageView::Eos(..) => {
                    println!("End-Of-Stream reached.");
                    pipeline.set_state(gst::State::Ready).into_result().unwrap();
                }

                // This is called when an error message is posted on the bus
                gst::MessageView::Error(err) => {
                    println!(
                        "Error from {:?}: {} ({:?})",
                        err.get_src().map(|s| s.get_path_string()),
                        err.get_error(),
                        err.get_debug()
                    );
                }
                // This is called when the pipeline changes states. We use it to
                // keep track of the current state.
                gst::MessageView::StateChanged(state_changed) => {
                    if state_changed
                        .get_src()
                        .map(|s| s == pipeline)
                        .unwrap_or(false)
                    {
                        println!("State set to {:?}", state_changed.get_current());
                    }
                }
                _ => (),
            }
        });

        playbin
            .set_state(gst::State::Playing)
            .into_result()
            .unwrap();

        gtk::main();
        window.hide();
        playbin.set_state(gst::State::Null).into_result().unwrap();

        bus.remove_signal_watch();
    }
}

#[cfg(feature = "tutorial5")]
fn main() {
    tutorial5::run();
}

#[cfg(not(feature = "tutorial5"))]
fn main() {
    println!("Please compile with --features tutorial5[-x11][-quartz]");
}
