// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![doc = include_str!("../README.md")]

pub use ffi;
pub use gio;
pub use gst;

macro_rules! assert_initialized_main_thread {
    () => {
        if unsafe { gst::ffi::gst_is_initialized() } != glib::ffi::GTRUE {
            panic!("GStreamer has not been initialized. Call `gst::init` first.");
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::unreadable_literal)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::match_same_arms)]
#[allow(clippy::use_self)]
#[allow(unused_imports)]
mod auto;
pub use crate::auto::*;
mod net_client_clock;
mod net_time_provider;
mod ntp_clock;
mod ptp_clock;

pub use crate::net_address_meta::*;
mod net_address_meta;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_net::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gio::prelude::*;
    #[doc(hidden)]
    pub use gst::prelude::*;

    pub use crate::auto::traits::*;
}
