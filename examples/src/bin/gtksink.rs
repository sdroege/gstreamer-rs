extern crate gstreamer as gst;
use gst::*;

extern crate glib;
use glib::*;

extern crate gtk;
use gtk::prelude::*;
use gtk::{Window, WindowType};

fn main() {
    gst::init().unwrap();
    gtk::init().unwrap();

    let pipeline = Pipeline::new(None);
    let src = ElementFactory::make("videotestsrc", None).unwrap();
    let (sink, widget) = if let Some(gtkglsink) = ElementFactory::make("gtkglsink", None) {
        let glsinkbin = ElementFactory::make("glsinkbin", None).unwrap();
        glsinkbin
            .set_property("sink", &gtkglsink.to_value())
            .unwrap();

        let widget = gtkglsink.get_property("widget").unwrap();
        (glsinkbin, widget.get::<gtk::Widget>().unwrap())
    } else {
        let sink = ElementFactory::make("gtksink", None).unwrap();
        let widget = sink.get_property("widget").unwrap();
        (sink, widget.get::<gtk::Widget>().unwrap())
    };

    pipeline.add_many(&[&src, &sink]).unwrap();
    src.link(&sink).unwrap();

    let window = Window::new(WindowType::Toplevel);
    window.set_default_size(320, 240);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.pack_start(&widget, true, true, 0);
    let label = gtk::Label::new("Position: 00:00:00");
    vbox.pack_start(&label, true, true, 5);
    window.add(&vbox);
    window.show_all();

    let pipeline_clone = pipeline.clone();
    gtk::timeout_add(500, move || {
        let pipeline = &pipeline_clone;
        let position = pipeline.query_position(Format::Time);

        if let Some(position) = position {
            let mut seconds = position / 1_000_000_000;
            let mut minutes = seconds / 60;
            let hours = minutes / 60;

            seconds -= hours * 60 * 60 + minutes * 60;
            minutes -= hours * 60;

            label.set_text(&format!(
                "Position: {:02}:{:02}:{:02}",
                hours,
                minutes,
                seconds
            ));
        } else {
            label.set_text("Position: 00:00:00");
        }

        glib::Continue(true)
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let bus = pipeline.get_bus().unwrap();

    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    bus.add_watch(move |_, msg| {
        match msg.view() {
            MessageView::Eos(..) => gtk::main_quit(),
            MessageView::Error(err) => {
                println!(
                    "Error from {}: {} ({:?})",
                    msg.get_src().get_path_string(),
                    err.get_error(),
                    err.get_debug()
                );
                gtk::main_quit();
            }
            _ => (),
        };

        glib::Continue(true)
    });

    gtk::main();

    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}
