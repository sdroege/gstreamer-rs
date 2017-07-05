extern crate gstreamer as gst;
use gst::*;

extern crate glib;
use glib::translate::{from_glib_none, ToGlibPtr};

extern crate gstreamer_sys as gst_ffi;
extern crate gobject_sys as gobject_ffi;

use std::env;
use std::u64;

fn main() {
    gst::init().unwrap();

    let args: Vec<_> = env::args().collect();
    let uri: &str = if args.len() == 2 {
        args[1].as_ref()
    } else {
        panic!("Usage: decodebin file_path");
    };

    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("filesrc", None).unwrap();
    let decodebin = gst::ElementFactory::make("decodebin", None).unwrap();

    // FIXME: https://github.com/gtk-rs/glib/pull/189
    unsafe {
        let uri = Value::from(uri);
        gobject_ffi::g_object_set_property(src.to_glib_none().0, "location".to_glib_none().0, uri.to_glib_none().0);
    }

    pipeline.add_many(&[&src, &decodebin]).unwrap();
    gst::Element::link_many(&[&src, &decodebin]).unwrap();

    // Need to move a new reference into the closure
    let pipeline_clone = pipeline.clone();
    decodebin.connect_pad_added(move |_, src_pad| {
        let ref pipeline = pipeline_clone;

        // FIXME: Needs caps/structure bindings
        let (is_audio, is_video) = unsafe {
            let caps = gst_ffi::gst_pad_get_current_caps(src_pad.to_glib_none().0);
            let structure = gst_ffi::gst_caps_get_structure(caps, 0);

            let name: String = from_glib_none(gst_ffi::gst_structure_get_name(structure));

            (name.starts_with("audio/"), name.starts_with("video/"))
        };

        if is_audio {
            let queue = gst::ElementFactory::make("queue", None).unwrap();
            let convert = gst::ElementFactory::make("audioconvert", None).unwrap();
            let resample = gst::ElementFactory::make("audioresample", None).unwrap();
            let sink = gst::ElementFactory::make("autoaudiosink", None).unwrap();

            let elements = &[&queue, &convert, &resample, &sink];
            pipeline.add_many(elements).unwrap();
            gst::Element::link_many(elements).unwrap();

            for e in elements {
                e.sync_state_with_parent().unwrap();
            }

            let sink_pad = queue.get_static_pad("sink").unwrap();
            assert_eq!(src_pad.link(&sink_pad), gst::PadLinkReturn::Ok);
        } else if is_video {
            let queue = gst::ElementFactory::make("queue", None).unwrap();
            let convert = gst::ElementFactory::make("videoconvert", None).unwrap();
            let scale = gst::ElementFactory::make("videoscale", None).unwrap();
            let sink = gst::ElementFactory::make("autovideosink", None).unwrap();

            let elements = &[&queue, &convert, &scale, &sink];
            pipeline.add_many(elements).unwrap();
            gst::Element::link_many(elements).unwrap();

            for e in elements {
                e.sync_state_with_parent().unwrap();
            }

            let sink_pad = queue.get_static_pad("sink").unwrap();
            assert_eq!(src_pad.link(&sink_pad), gst::PadLinkReturn::Ok);
        }
    });

    assert_ne!(pipeline.set_state(gst::State::Playing), gst::StateChangeReturn::Failure);

    let bus = pipeline.get_bus().unwrap();

    loop {
        let msg = match bus.timed_pop(u64::MAX) {
            None => break,
            Some(msg) => msg,
        };

        match msg.view() {
            MessageView::Eos => break,
            MessageView::Error(err) => {
                println!("Error from {}: {} ({:?})", msg.get_src().get_path_string(),
                    err.get_error(), err.get_debug());
                break;
            },
            MessageView::StateChanged(s) => {
                println!("State changed from {}: {:?} -> {:?} ({:?})", msg.get_src().get_path_string(),
                    s.get_old(), s.get_current(), s.get_pending());
            },
            _ => (),
        }
    }

    assert_ne!(pipeline.set_state(gst::State::Null), gst::StateChangeReturn::Failure);
}
