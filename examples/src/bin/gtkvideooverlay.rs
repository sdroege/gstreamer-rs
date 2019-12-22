// This example demonstrates another type of combination of gtk and gstreamer,
// in comparision to the gtksink example.
// This example uses regions that are managed by the window system, and uses
// the window system's api to insert a videostream into these regions.
// So essentially, the window system of the system overlays our gui with
// the video frames - within the region that we tell it to use.
// Disadvantage of this method is, that it's highly platform specific, since
// the big platforms all have their own window system. Thus, this example
// has special code to handle differences between platforms.
// Windows could theoretically be supported by this example, but is not yet implemented.
// One of the very few (if not the single one) platform, that can not provide the API
// needed for this are Linux desktops using Wayland.
// TODO: Add Windows support
// In this case, a testvideo is displayed within our gui, using the
// following pipeline:

// {videotestsrc} - {xvimagesink(on linux)}
// {videotestsrc} - {glimagesink(on mac)}

extern crate gstreamer as gst;
use gst::prelude::*;

extern crate gstreamer_video as gst_video;
use gst_video::prelude::*;

extern crate glib;
use glib::object::ObjectType;

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

#[cfg(all(target_os = "linux", feature = "gtkvideooverlay-x11"))]
fn create_video_sink() -> gst::Element {
    // When we are on linux with the Xorg display server, we use the
    // X11 protocol's XV extension, which allows to overlay regions
    // with video streams. For this, we use the xvimagesink element.
    gst::ElementFactory::make("xvimagesink", None).unwrap()
}
#[cfg(all(target_os = "linux", feature = "gtkvideooverlay-x11"))]
fn set_window_handle(video_overlay: &gst_video::VideoOverlay, gdk_window: &gdk::Window) {
    let display_type_name = gdk_window.get_display().get_type().name();

    // Check if we're using X11 or ...
    if display_type_name == "GdkX11Display" {
        extern "C" {
            pub fn gdk_x11_window_get_xid(window: *mut glib::object::GObject) -> *mut c_void;
        }

        // This is unsafe because the "window handle" we pass here is basically like a raw pointer.
        // If a wrong value were to be passed here (and you can pass any integer), then the window
        // system will most likely cause the application to crash.
        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            // Here we ask gdk what native window handle we got assigned for
            // our video region from the window system, and then we will
            // pass this unique identifier to the overlay provided by our
            // sink - so the sink can then arrange the overlay.
            let xid = gdk_x11_window_get_xid(gdk_window.as_ptr() as *mut _);
            video_overlay.set_window_handle(xid as usize);
        }
    } else {
        println!("Add support for display type '{}'", display_type_name);
        process::exit(-1);
    }
}

#[cfg(all(target_os = "macos", feature = "gtkvideooverlay-quartz"))]
fn create_video_sink() -> gst::Element {
    // On Mac, this is done by overlaying a window region with an
    // OpenGL-texture, using the glimagesink element.
    gst::ElementFactory::make("glimagesink", None).unwrap()
}

#[cfg(all(target_os = "macos", feature = "gtkvideooverlay-quartz"))]
fn set_window_handle(video_overlay: &gst_video::VideoOverlay, gdk_window: &gdk::Window) {
    let display_type_name = gdk_window.get_display().get_type().name();

    if display_type_name == "GdkQuartzDisplay" {
        extern "C" {
            pub fn gdk_quartz_window_get_nsview(window: *mut glib::object::GObject) -> *mut c_void;
        }

        // This is unsafe because the "window handle" we pass here is basically like a raw pointer.
        // If a wrong value were to be passed here (and you can pass any integer), then the window
        // system will most likely cause the application to crash.
        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            // Here we ask gdk what native window handle we got assigned for
            // our video region from the windowing system, and then we will
            // pass this unique identifier to the overlay provided by our
            // sink - so the sink can then arrange the overlay.
            let window = gdk_quartz_window_get_nsview(gdk_window.as_ptr() as *mut _);
            video_overlay.set_window_handle(window as usize);
        }
    } else {
        println!("Unsupported display type '{}", display_type_name);
        process::exit(-1);
    }
}

fn create_ui(app: &gtk::Application) {
    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("videotestsrc", None).unwrap();

    // Since using the window system to overlay our gui window is making
    // direct contact with the windowing system, this is highly platform-
    // specific. This example supports Linux and Mac (using X11 and Quartz).
    let sink = create_video_sink();

    pipeline.add_many(&[&src, &sink]).unwrap();
    src.link(&sink).unwrap();

    // First, we create our gtk window - which will contain a region where
    // our overlayed video will be displayed in.
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_default_size(320, 240);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    // This creates the widget we will display our overlay in.
    // Later, we will try to tell our window system about this region, so
    // it can overlay it with our video stream.
    let video_window = gtk::DrawingArea::new();
    video_window.set_size_request(320, 240);

    // Use the platform-specific sink to create our overlay.
    // Since we only use the video_overlay in the closure below, we need a weak reference.
    // !!ATTENTION!!:
    // It might seem appealing to use .clone() here, because that greatly
    // simplifies the code within the callback. What this actually does, however, is creating
    // a memory leak.
    let video_overlay = sink
        .dynamic_cast::<gst_video::VideoOverlay>()
        .unwrap()
        .downgrade();
    // Connect to this widget's realize signal, which will be emitted
    // after its display has been initialized. This is neccessary, because
    // the window system doesn't know about our region until it was initialized.
    video_window.connect_realize(move |video_window| {
        // Here we temporarily retrieve a strong reference on the video-overlay from the
        // weak reference that we moved into the closure.
        let video_overlay = match video_overlay.upgrade() {
            Some(video_overlay) => video_overlay,
            None => return,
        };

        // Gtk uses gdk under the hood, to handle its drawing. Drawing regions are
        // called gdk windows. We request this underlying drawing region from the
        // widget we will overlay with our video.
        let gdk_window = video_window.get_window().unwrap();

        // This is where we tell our window system about the drawing-region we
        // want it to overlay. Most often, the window system would only know
        // about our most outer region (or: our window).
        if !gdk_window.ensure_native() {
            println!("Can't create native window for widget");
            process::exit(-1);
        }

        set_window_handle(&video_overlay, &gdk_window);
    });

    vbox.pack_start(&video_window, true, true, 0);

    let label = gtk::Label::new(Some("Position: 00:00:00"));
    vbox.pack_start(&label, true, true, 5);
    window.add(&vbox);

    window.show_all();

    app.add_window(&window);

    // Need to move a new reference into the closure.
    // !!ATTENTION!!:
    // It might seem appealing to use pipeline.clone() here, because that greatly
    // simplifies the code within the callback. What this actually does, however, is creating
    // a memory leak. The clone of a pipeline is a new strong reference on the pipeline.
    // Storing this strong reference of the pipeline within the callback (we are moving it in!),
    // which is in turn stored in another strong reference on the pipeline is creating a
    // reference cycle.
    // DO NOT USE pipeline.clone() TO USE THE PIPELINE WITHIN A CALLBACK
    let pipeline_weak = pipeline.downgrade();
    // Add a timeout to the main loop that will periodically (every 500ms) be
    // executed. This will query the current position within the stream from
    // the underlying pipeline, and display it in our gui.
    // Since this closure is called by the mainloop thread, we are allowed
    // to modify the gui widgets here.
    let timeout_id = gtk::timeout_add(500, move || {
        // Here we temporarily retrieve a strong reference on the pipeline from the weak one
        // we moved into this callback.
        let pipeline = match pipeline_weak.upgrade() {
            Some(pipeline) => pipeline,
            None => return glib::Continue(false),
        };

        // Query the current playing position from the underlying pipeline.
        let position = pipeline
            .query_position::<gst::ClockTime>()
            .unwrap_or_else(|| 0.into());
        // Display the playing position in the gui.
        label.set_text(&format!("Position: {:.0}", position));
        // Tell the timeout to continue calling this callback.
        glib::Continue(true)
    });

    let bus = pipeline.get_bus().unwrap();

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    let app_weak = app.downgrade();
    bus.add_watch_local(move |_, msg| {
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
    })
    .expect("Failed to add bus watch");

    // Pipeline reference is owned by the closure below, so will be
    // destroyed once the app is destroyed
    let timeout_id = RefCell::new(Some(timeout_id));
    app.connect_shutdown(move |_| {
        pipeline
            .set_state(gst::State::Null)
            .expect("Unable to set the pipeline to the `Null` state");

        bus.remove_watch().unwrap();
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

    // Initialize gstreamer and the gtk widget toolkit libraries.
    gst::init().unwrap();
    gtk::init().unwrap();

    let app = gtk::Application::new(None, gio::ApplicationFlags::FLAGS_NONE).unwrap();

    app.connect_activate(create_ui);
    let args = env::args().collect::<Vec<_>>();
    app.run(&args);
}
