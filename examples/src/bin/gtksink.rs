#[cfg(feature = "gtksink")]
extern crate gstreamer as gst;
#[cfg(feature = "gtksink")]
use gst::prelude::*;

#[cfg(feature = "gtksink")]
extern crate glib;

#[cfg(feature = "gtksink")]
extern crate gio;
#[cfg(feature = "gtksink")]
use gio::prelude::*;

#[cfg(feature = "gtksink")]
extern crate gtk;
#[cfg(feature = "gtksink")]
use gtk::prelude::*;

#[cfg(feature = "gtksink")]
use std::env;

#[cfg(feature = "gtksink")]
extern crate send_cell;
#[cfg(feature = "gtksink")]
use send_cell::SendCell;

#[cfg(feature = "gtksink")]
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

#[cfg(feature = "gtksink")]
fn main() {
    gst::init().unwrap();
    gtk::init().unwrap();

    let app = gtk::Application::new(None, gio::ApplicationFlags::FLAGS_NONE).unwrap();

    app.connect_activate(create_ui);
    let args = env::args().collect::<Vec<_>>();
    app.run(&args);
}

#[cfg(not(feature = "gtksink"))]
fn main() {
    println!("Please compile with --feature gtksink");
}
