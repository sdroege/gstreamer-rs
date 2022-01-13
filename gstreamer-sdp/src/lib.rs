// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::non_send_fields_in_send_ty)]
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

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::unreadable_literal)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::match_same_arms)]
#[allow(non_snake_case)]
#[allow(clippy::use_self)]
mod auto;
pub use crate::auto::*;

mod sdp_attribute;
pub use crate::sdp_attribute::*;
mod sdp_bandwidth;
pub use crate::sdp_bandwidth::*;
mod sdp_connection;
pub use crate::sdp_connection::*;
mod sdp_key;
pub use crate::sdp_key::*;
pub mod sdp_media;
pub use crate::sdp_media::{SDPMedia, SDPMediaRef};
pub mod sdp_message;
pub use crate::sdp_message::{SDPMessage, SDPMessageRef};
mod sdp_origin;
pub use crate::sdp_origin::*;
mod sdp_time;
pub use crate::sdp_time::*;
mod sdp_zone;
pub use crate::sdp_zone::*;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_sdp::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst::prelude::*;
}
