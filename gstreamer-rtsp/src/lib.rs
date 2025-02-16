// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]
#![doc = include_str!("../README.md")]

pub use glib;
pub use gst;
pub use gst_sdp;
pub use gstreamer_rtsp_sys as ffi;

macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            gst::assert_initialized();
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(unused_imports)]
mod auto;
pub use crate::auto::*;

#[cfg(feature = "serde")]
mod flag_serde;

pub mod rtsp_auth_credential;
pub mod rtsp_message;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_rtsp::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst_sdp::prelude::*;
}
