#[cfg(feature = "tutorial5")]
mod tutorial5 {
    use std::{ops, os::raw::c_void, process};

    use gdk::prelude::*;
    use gst_video::prelude::*;
    use gtk::prelude::*;

    // Custom struct to keep our window reference alive
    // and to store the timeout id so that we can remove
    // it from the main context again later and drop the
    // references it keeps inside its closures
    struct AppWindow {
        main_window: gtk::Window,
        timeout_id: Option<glib::SourceId>,
    }

    impl ops::Deref for AppWindow {
        type Target = gtk::Window;

        fn deref(&self) -> &gtk::Window {
            &self.main_window
        }
    }

    impl Drop for AppWindow {
        fn drop(&mut self) {
            if let Some(source_id) = self.timeout_id.take() {
                source_id.remove();
            }
        }
    }

    // Extract tags from streams of @stype and add the info in the UI.
    fn add_streams_info(playbin: &gst::Element, textbuf: &gtk::TextBuffer, stype: &str) {
        let propname: &str = &format!("n-{stype}");
        let signame: &str = &format!("get-{stype}-tags");

        let x = playbin.property::<i32>(propname);
        for i in 0..x {
            let tags = playbin.emit_by_name::<Option<gst::TagList>>(signame, &[&i]);

            if let Some(tags) = tags {
                textbuf.insert_at_cursor(&format!("{stype} stream {i}:\n "));

                if let Some(codec) = tags.get::<gst::tags::VideoCodec>() {
                    textbuf.insert_at_cursor(&format!("    codec: {} \n", codec.get()));
                }

                if let Some(codec) = tags.get::<gst::tags::AudioCodec>() {
                    textbuf.insert_at_cursor(&format!("    codec: {} \n", codec.get()));
                }

                if let Some(lang) = tags.get::<gst::tags::LanguageCode>() {
                    textbuf.insert_at_cursor(&format!("    language: {} \n", lang.get()));
                }

                if let Some(bitrate) = tags.get::<gst::tags::Bitrate>() {
                    textbuf.insert_at_cursor(&format!("    bitrate: {} \n", bitrate.get()));
                }
            }
        }
    }

    // Extract metadata from all the streams and write it to the text widget in the GUI
    fn analyze_streams(playbin: &gst::Element, textbuf: &gtk::TextBuffer) {
        {
            textbuf.set_text("");
        }

        add_streams_info(playbin, textbuf, "video");
        add_streams_info(playbin, textbuf, "audio");
        add_streams_info(playbin, textbuf, "text");
    }

    // This creates all the GTK+ widgets that compose our application, and registers the callbacks
    fn create_ui(playbin: &gst::Element) -> AppWindow {
        let main_window = gtk::Window::new(gtk::WindowType::Toplevel);
        main_window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let play_button =
            gtk::Button::from_icon_name(Some("media-playback-start"), gtk::IconSize::SmallToolbar);
        let pipeline = playbin.clone();
        play_button.connect_clicked(move |_| {
            let pipeline = &pipeline;
            pipeline
                .set_state(gst::State::Playing)
                .expect("Unable to set the pipeline to the `Playing` state");
        });

        let pause_button =
            gtk::Button::from_icon_name(Some("media-playback-pause"), gtk::IconSize::SmallToolbar);
        let pipeline = playbin.clone();
        pause_button.connect_clicked(move |_| {
            let pipeline = &pipeline;
            pipeline
                .set_state(gst::State::Paused)
                .expect("Unable to set the pipeline to the `Paused` state");
        });

        let stop_button =
            gtk::Button::from_icon_name(Some("media-playback-stop"), gtk::IconSize::SmallToolbar);
        let pipeline = playbin.clone();
        stop_button.connect_clicked(move |_| {
            let pipeline = &pipeline;
            pipeline
                .set_state(gst::State::Ready)
                .expect("Unable to set the pipeline to the `Ready` state");
        });

        let slider = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 100.0, 1.0);
        let pipeline = playbin.clone();
        let slider_update_signal_id = slider.connect_value_changed(move |slider| {
            let pipeline = &pipeline;
            let value = slider.value() as u64;
            if pipeline
                .seek_simple(
                    gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
                    value * gst::ClockTime::SECOND,
                )
                .is_err()
            {
                eprintln!("Seeking to {value} failed");
            }
        });

        slider.set_draw_value(false);
        let pipeline = playbin.clone();
        let lslider = slider.clone();
        // Update the UI (seekbar) every second
        let timeout_id = glib::timeout_add_seconds_local(1, move || {
            let pipeline = &pipeline;
            let lslider = &lslider;

            if let Some(dur) = pipeline.query_duration::<gst::ClockTime>() {
                lslider.set_range(0.0, dur.seconds() as f64);

                if let Some(pos) = pipeline.query_position::<gst::ClockTime>() {
                    lslider.block_signal(&slider_update_signal_id);
                    lslider.set_value(pos.seconds() as f64);
                    lslider.unblock_signal(&slider_update_signal_id);
                }
            }

            Continue(true)
        });

        let controls = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        controls.pack_start(&play_button, false, false, 0);
        controls.pack_start(&pause_button, false, false, 0);
        controls.pack_start(&stop_button, false, false, 0);
        controls.pack_start(&slider, true, true, 2);

        let video_window = gtk::DrawingArea::new();

        let video_overlay = playbin
            .clone()
            .dynamic_cast::<gst_video::VideoOverlay>()
            .unwrap();

        video_window.connect_realize(move |video_window| {
            let video_overlay = &video_overlay;
            let gdk_window = video_window.window().unwrap();

            if !gdk_window.ensure_native() {
                println!("Can't create native window for widget");
                process::exit(-1);
            }

            let display_type_name = gdk_window.display().type_().name();
            #[cfg(all(target_os = "linux", feature = "tutorial5-x11"))]
            {
                // Check if we're using X11 or ...
                if display_type_name == "GdkX11Display" {
                    extern "C" {
                        pub fn gdk_x11_window_get_xid(
                            window: *mut glib::gobject_ffi::GObject,
                        ) -> *mut c_void;
                    }

                    #[allow(clippy::cast_ptr_alignment)]
                    unsafe {
                        let xid = gdk_x11_window_get_xid(gdk_window.as_ptr() as *mut _);
                        video_overlay.set_window_handle(xid as usize);
                    }
                } else {
                    println!("Add support for display type '{display_type_name}'");
                    process::exit(-1);
                }
            }
            #[cfg(all(target_os = "macos", feature = "tutorial5-quartz"))]
            {
                if display_type_name == "GdkQuartzDisplay" {
                    extern "C" {
                        pub fn gdk_quartz_window_get_nsview(
                            window: *mut glib::gobject_ffi::GObject,
                        ) -> *mut c_void;
                    }

                    #[allow(clippy::cast_ptr_alignment)]
                    unsafe {
                        let window = gdk_quartz_window_get_nsview(gdk_window.as_ptr() as *mut _);
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
        let streams_list_weak = glib::SendWeakRef::from(streams_list.downgrade());
        let bus = playbin.bus().unwrap();

        #[allow(clippy::single_match)]
        bus.connect_message(Some("application"), move |_, msg| match msg.view() {
            gst::MessageView::Application(application) => {
                let pipeline = match pipeline_weak.upgrade() {
                    Some(pipeline) => pipeline,
                    None => return,
                };

                let streams_list = match streams_list_weak.upgrade() {
                    Some(streams_list) => streams_list,
                    None => return,
                };

                if application.structure().map(|s| s.name().as_str()) == Some("tags-changed") {
                    let textbuf = streams_list
                        .buffer()
                        .expect("Couldn't get buffer from text_view");
                    analyze_streams(&pipeline, &textbuf);
                }
            }
            _ => unreachable!(),
        });

        let vbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        vbox.pack_start(&video_window, true, true, 0);
        vbox.pack_start(&streams_list, false, false, 2);

        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
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
        let _ = playbin.post_message(gst::message::Application::new(gst::Structure::new_empty(
            "tags-changed",
        )));
    }

    pub fn run() {
        // Make sure the right features were activated
        #[allow(clippy::eq_op)]
        {
            if !cfg!(feature = "tutorial5-x11") && !cfg!(feature = "tutorial5-quartz") {
                eprintln!(
                    "No Gdk backend selected, compile with --features tutorial5[-x11][-quartz]."
                );

                return;
            }
        }

        // Initialize GTK
        if let Err(err) = gtk::init() {
            eprintln!("Failed to initialize GTK: {err}");
            return;
        }

        // Initialize GStreamer
        if let Err(err) = gst::init() {
            eprintln!("Failed to initialize Gst: {err}");
            return;
        }

        let uri = "https://www.freedesktop.org/software/gstreamer-sdk/\
                   data/media/sintel_trailer-480p.webm";
        let playbin = gst::ElementFactory::make("playbin")
            .property("uri", uri)
            .build()
            .unwrap();

        playbin.connect("video-tags-changed", false, |args| {
            let pipeline = args[0]
                .get::<gst::Element>()
                .expect("playbin \"video-tags-changed\" args[0]");
            post_app_message(&pipeline);
            None
        });

        playbin.connect("audio-tags-changed", false, |args| {
            let pipeline = args[0]
                .get::<gst::Element>()
                .expect("playbin \"audio-tags-changed\" args[0]");
            post_app_message(&pipeline);
            None
        });

        playbin.connect("text-tags-changed", false, move |args| {
            let pipeline = args[0]
                .get::<gst::Element>()
                .expect("playbin \"text-tags-changed\" args[0]");
            post_app_message(&pipeline);
            None
        });

        let window = create_ui(&playbin);

        let bus = playbin.bus().unwrap();
        bus.add_signal_watch();

        let pipeline_weak = playbin.downgrade();
        bus.connect_message(None, move |_, msg| {
            let pipeline = match pipeline_weak.upgrade() {
                Some(pipeline) => pipeline,
                None => return,
            };

            match msg.view() {
                //  This is called when an End-Of-Stream message is posted on the bus.
                // We just set the pipeline to READY (which stops playback).
                gst::MessageView::Eos(..) => {
                    println!("End-Of-Stream reached.");
                    pipeline
                        .set_state(gst::State::Ready)
                        .expect("Unable to set the pipeline to the `Ready` state");
                }

                // This is called when an error message is posted on the bus
                gst::MessageView::Error(err) => {
                    println!(
                        "Error from {:?}: {} ({:?})",
                        err.src().map(|s| s.path_string()),
                        err.error(),
                        err.debug()
                    );
                }
                // This is called when the pipeline changes states. We use it to
                // keep track of the current state.
                gst::MessageView::StateChanged(state_changed) => {
                    if state_changed.src().map(|s| s == &pipeline).unwrap_or(false) {
                        println!("State set to {:?}", state_changed.current());
                    }
                }
                _ => (),
            }
        });

        playbin
            .set_state(gst::State::Playing)
            .expect("Unable to set the playbin to the `Playing` state");

        gtk::main();
        window.hide();
        playbin
            .set_state(gst::State::Null)
            .expect("Unable to set the playbin to the `Null` state");

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
