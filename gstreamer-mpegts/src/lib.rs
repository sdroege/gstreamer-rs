#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]

use std::sync::Once;

pub use glib;
pub use gst;
pub use gstreamer_mpegts_sys as ffi;

static MPEGTS_INIT: Once = Once::new();

macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            gst::assert_initialized();
        }
        crate::MPEGTS_INIT.call_once(|| unsafe { crate::ffi::gst_mpegts_initialize() });
    };
}

pub fn init() {
    assert_initialized_main_thread!();
}

#[allow(unused_imports)]
mod auto;
#[allow(unused_imports)]
pub use crate::auto::*;
