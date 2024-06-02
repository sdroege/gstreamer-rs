// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![doc = include_str!("../README.md")]

pub use glib;
pub use gst;
pub use gstreamer_check_sys as ffi;

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

mod auto;
pub use crate::auto::*;

mod test_clock;

pub mod harness;
pub use crate::harness::Harness;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_check::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst::prelude::*;
}
