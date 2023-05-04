// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![doc = include_str!("../README.md")]

pub use ffi;
pub use glib;
pub use gst;
pub use gst_sdp;

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::needless_borrow)]
#[allow(unused_imports)]
mod auto;
pub use crate::auto::*;

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
mod web_rtc_ice_candidate_stats;
mod web_rtc_session_description;
#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
mod web_rtcice;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_webrtc::prelude::*" without getting conflicts
pub mod prelude {
    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    pub use crate::web_rtcice::WebRTCICEExtManual;
    #[doc(hidden)]
    pub use gst_sdp::prelude::*;
}
