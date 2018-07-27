extern crate gstreamer as gst;
use gst::prelude::*;

extern crate gstreamer_video as gst_video;
use gst_video::prelude::*;

extern crate glib;
use glib::translate::ToGlibPtr;

extern crate gio;
use gio::prelude::*;

extern crate gtk;
use gtk::prelude::*;

extern crate gdk;
use gdk::prelude::*;

use std::env;

use std::os::raw::c_void;

use std::cell::RefCell;

use std::process;

fn create_ui(app: &gtk::Application) {
    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("videotestsrc", None).unwrap();

    let sink = if cfg!(feature = "gtkvideooverlay-x11") {
        gst::ElementFactory::make("xvimagesink", None).unwrap()
    } else if cfg!(feature = "gtkvideooverlay-quartz") {
        gst::ElementFactory::make("glimagesink", None).unwrap()
    } else {
        unreachable!()
    };

    pipeline.add_many(&[&src, &sink]).unwrap();
    src.link(&sink).unwrap();

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_default_size(320, 240);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let video_window = gtk::DrawingArea::new();
    video_window.set_size_request(320, 240);
    let video_overlay = sink.clone()
        .dynamic_cast::<gst_video::VideoOverlay>()
        .unwrap()
        .downgrade();
    video_window.connect_realize(move |video_window| {
        let video_overlay = match video_overlay.upgrade() {
            Some(video_overlay) => video_overlay,
            None => return,
        };

        let gdk_window = video_window.get_window().unwrap();

        if !gdk_window.ensure_native() {
            println!("Can't create native window for widget");
            process::exit(-1);
        }

        let display_type_name = gdk_window.get_display().get_type().name();

        if cfg!(feature = "gtkvideooverlay-x11") {
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
        } else if cfg!(feature = "gtkvideooverlay-quartz") {
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
                println!("Unsupported display type '{}", display_type_name);
                process::exit(-1);
            }
        }
    });

    vbox.pack_start(&video_window, true, true, 0);

    let label = gtk::Label::new("Position: 00:00:00");
    vbox.pack_start(&label, true, true, 5);
    window.add(&vbox);

    window.show_all();

    app.add_window(&window);

    let pipeline_weak = pipeline.downgrade();
    let timeout_id = gtk::timeout_add(500, move || {
        let pipeline = match pipeline_weak.upgrade() {
            Some(pipeline) => pipeline,
            None => return glib::Continue(true),
        };

        let position = pipeline
            .query_position::<gst::ClockTime>()
            .unwrap_or_else(|| 0.into());
        label.set_text(&format!("Position: {:.0}", position));

        glib::Continue(true)
    });

    let app_weak = app.downgrade();
    window.connect_delete_event(move |_, _| {
        let app = match app_weak.upgrade() {
            Some(app) => app,
            None => return Inhibit(false),
        };

        app.quit();
        Inhibit(false)
    });

    let bus = pipeline.get_bus().unwrap();

    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    let app_weak = glib::SendWeakRef::from(app.downgrade());
    bus.add_watch(move |_, msg| {
        use gst::MessageView;

        let app = match app_weak.upgrade() {
            Some(app) => app,
            None => return glib::Continue(false),
        };

        match msg.view() {
            MessageView::Eos(..) => gtk::main_quit(),
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                app.quit();
            }
            _ => (),
        };

        glib::Continue(true)
    });

    // Pipeline reference is owned by the closure below, so will be
    // destroyed once the app is destroyed
    let timeout_id = RefCell::new(Some(timeout_id));
    app.connect_shutdown(move |_| {
        let ret = pipeline.set_state(gst::State::Null);
        assert_ne!(ret, gst::StateChangeReturn::Failure);

        bus.remove_watch();
        if let Some(timeout_id) = timeout_id.borrow_mut().take() {
            glib::source_remove(timeout_id);
        }
    });
}

fn main() {
    #[cfg(not(unix))]
    {
        println!("Add support for target platform");
        process::exit(-1);
    }

    gst::init().unwrap();
    gtk::init().unwrap();

    let app = gtk::Application::new(None, gio::ApplicationFlags::FLAGS_NONE).unwrap();

    app.connect_activate(create_ui);
    let args = env::args().collect::<Vec<_>>();
    app.run(&args);
}
