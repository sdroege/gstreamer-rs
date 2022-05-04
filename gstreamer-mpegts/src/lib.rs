#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::non_send_fields_in_send_ty)]

pub use ffi::*;
pub use glib;
pub use gst;

use std::sync::Once;

static MPEGTS_INIT: Once = Once::new();

macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            #[allow(unused_unsafe)]
            if unsafe { gst::ffi::gst_is_initialized() } != glib::ffi::GTRUE {
                panic!("GStreamer has not been initialized. Call `gst::init` first.");
            } else {
                gst::INITIALIZED.store(true, std::sync::atomic::Ordering::SeqCst);
            }
        }
        crate::MPEGTS_INIT.call_once(|| unsafe { ffi::gst_mpegts_initialize() });
    };
}

pub fn init() {
    assert_initialized_main_thread!();
}

mod auto;
pub use crate::auto::*;
