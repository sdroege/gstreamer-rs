#[cfg(feature = "gtkvideooverlay")]
extern crate gstreamer as gst;
#[cfg(feature = "gtkvideooverlay")]
use gst::prelude::*;

#[cfg(feature = "gtkvideooverlay")]
extern crate gstreamer_video as gst_video;
#[cfg(feature = "gtkvideooverlay")]
use gst_video::prelude::*;

#[cfg(feature = "gtkvideooverlay")]
extern crate glib;
#[cfg(feature = "gtkvideooverlay")]
use glib::translate::ToGlibPtr;

#[cfg(feature = "gtkvideooverlay")]
extern crate gio;
#[cfg(feature = "gtkvideooverlay")]
use gio::prelude::*;

#[cfg(feature = "gtkvideooverlay")]
extern crate gtk;
#[cfg(feature = "gtkvideooverlay")]
use gtk::prelude::*;

#[cfg(feature = "gtkvideooverlay")]
extern crate gdk;
#[cfg(feature = "gtkvideooverlay")]
use gdk::prelude::*;

#[cfg(feature = "gtkvideooverlay")]
use std::env;

#[cfg(feature = "gtkvideooverlay")]
use std::os::raw::c_void;

#[cfg(feature = "gtkvideooverlay")]
extern crate send_cell;
#[cfg(feature = "gtkvideooverlay")]
use send_cell::SendCell;

#[cfg(feature = "gtkvideooverlay")]
use std::process;

#[cfg(feature = "gtkvideooverlay")]
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
        .unwrap();
    video_window.connect_realize(move |video_window| {
        let video_overlay = &video_overlay;

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

    let pipeline_clone = pipeline.clone();
    gtk::timeout_add(500, move || {
        let pipeline = &pipeline_clone;
        let position = if let Some(gst::FormatValue::Time(position)) =
            pipeline.query_position(gst::Format::Time)
        {
            position
        } else {
            0.into()
        };
        label.set_text(&format!("Position: {:.0}", position));

        glib::Continue(true)
    });

    let app_clone = app.clone();
    window.connect_delete_event(move |_, _| {
        let app = &app_clone;
        app.quit();
        Inhibit(false)
    });

    let bus = pipeline.get_bus().unwrap();

    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    let app_clone = SendCell::new(app.clone());
    bus.add_watch(move |_, msg| {
        use gst::MessageView;

        let app = app_clone.borrow();
        match msg.view() {
            MessageView::Eos(..) => gtk::main_quit(),
            MessageView::Error(err) => {
                println!(
                    "Error from {}: {} ({:?})",
                    msg.get_src().get_path_string(),
                    err.get_error(),
                    err.get_debug()
                );
                app.quit();
            }
            _ => (),
        };

        glib::Continue(true)
    });

    let pipeline_clone = pipeline.clone();
    app.connect_shutdown(move |_| {
        let pipeline = &pipeline_clone;
        let ret = pipeline.set_state(gst::State::Null);
        assert_ne!(ret, gst::StateChangeReturn::Failure);
    });
}

#[cfg(feature = "gtkvideooverlay")]
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

#[cfg(not(feature = "gtkvideooverlay"))]
fn main() {
    println!("Please compile with --feature gtkvideooverlay");
}
