// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]
#![doc = include_str!("../README.md")]

pub use gio;
pub use gst;
pub use gstreamer_net_sys as ffi;

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
pub use crate::{auto::*, net_address_meta::*};
mod net_address_meta;

mod ptp_clock;
pub use ptp_clock::PtpStatisticsCallback;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_net::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gio::prelude::*;
    #[doc(hidden)]
    pub use gst::prelude::*;
}
