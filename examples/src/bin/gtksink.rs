extern crate gstreamer as gst;
use gst::prelude::*;

extern crate glib;

extern crate gio;
use gio::prelude::*;

extern crate gtk;
use gtk::prelude::*;

use std::cell::RefCell;
use std::env;

fn create_ui(app: &gtk::Application) {
    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("videotestsrc", None).unwrap();
    let (sink, widget) = if let Some(gtkglsink) = gst::ElementFactory::make("gtkglsink", None) {
        let glsinkbin = gst::ElementFactory::make("glsinkbin", None).unwrap();
        glsinkbin
            .set_property("sink", &gtkglsink.to_value())
            .unwrap();

        let widget = gtkglsink.get_property("widget").unwrap();
        (glsinkbin, widget.get::<gtk::Widget>().unwrap())
    } else {
        let sink = gst::ElementFactory::make("gtksink", None).unwrap();
        let widget = sink.get_property("widget").unwrap();
        (sink, widget.get::<gtk::Widget>().unwrap())
    };

    pipeline.add_many(&[&src, &sink]).unwrap();
    src.link(&sink).unwrap();

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_default_size(320, 240);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.pack_start(&widget, true, true, 0);
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
    gst::init().unwrap();
    gtk::init().unwrap();

    let app = gtk::Application::new(None, gio::ApplicationFlags::FLAGS_NONE).unwrap();

    app.connect_activate(create_ui);
    let args = env::args().collect::<Vec<_>>();
    app.run(&args);
}
