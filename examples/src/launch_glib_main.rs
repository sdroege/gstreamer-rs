extern crate gstreamer as gst;
use gst::*;

extern crate gtk;

use std::u64;

fn main() {
    gst::init().unwrap();

    // FIXME: Use glib crate once it has mainloop/etc bindings
    // https://github.com/gtk-rs/glib/issues/168
    gtk::init().unwrap();

    let pipeline = gst::parse_launch("audiotestsrc ! autoaudiosink").unwrap();
    let bus = pipeline.get_bus().unwrap();

    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    bus.add_signal_watch();
    bus.connect_message(|_, msg| match msg.view() {
        MessageView::Eos => gtk::main_quit(),
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
    });

    gtk::main();

    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}
