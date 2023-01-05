#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::non_send_fields_in_send_ty)]

use std::sync::Once;

pub use ffi::*;
pub use glib;
pub use gst;

static MPEGTS_INIT: Once = Once::new();

macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            gst::assert_initialized();
        }
        crate::MPEGTS_INIT.call_once(|| unsafe { ffi::gst_mpegts_initialize() });
    };
}

pub fn init() {
    assert_initialized_main_thread!();
}

mod auto;
pub use crate::auto::*;
