// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub use ffi;
pub use glib;
pub use gst;

macro_rules! assert_initialized_main_thread {
    () => {
        if unsafe { gst::ffi::gst_is_initialized() } != glib::ffi::GTRUE {
            panic!("GStreamer has not been initialized. Call `gst::init` first.");
        }
    };
}

#[allow(clippy::unreadable_literal)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::match_same_arms)]
#[allow(clippy::type_complexity)]
#[allow(clippy::use_self)]
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

    pub use crate::auto::traits::*;
}
