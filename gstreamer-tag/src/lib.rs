// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]
#![doc = include_str!("../README.md")]

pub use glib;
pub use gst;
pub use gstreamer_tag_sys as ffi;

macro_rules! skip_assert_initialized {
    () => {};
}

macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            gst::assert_initialized();
        }
    };
}

mod auto;
pub use crate::auto::*;

mod tags;
pub use crate::tags::*;
pub mod language_codes;
mod sample_ext;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_tag::prelude::*" without getting conflicts
pub mod prelude {
    pub use crate::sample_ext::ImageSampleExt;
    #[doc(hidden)]
    pub use gst::prelude::*;
}
