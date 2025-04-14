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

mod tags;
pub use crate::tags::*;

pub mod language_codes;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_tag::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst::prelude::*;
}
