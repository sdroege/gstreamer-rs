// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]
#![doc = include_str!("../README.md")]

pub use glib;
pub use gst;
pub use gst_base;
pub use gstreamer_app_sys as ffi;

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

pub mod app_sink;
pub use crate::app_sink::AppSinkCallbacks;

pub mod app_src;
pub use crate::app_src::AppSrcCallbacks;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_app::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst_base::prelude::*;
}
