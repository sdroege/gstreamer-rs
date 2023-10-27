// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![doc = include_str!("../README.md")]

pub use ffi;
pub use glib;
pub use gst;

macro_rules! skip_assert_initialized {
    () => {};
}

mod tags;
pub use crate::tags::*;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_tag::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst::prelude::*;
}
