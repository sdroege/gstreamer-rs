// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![doc = include_str!("../README.md")]

pub use ffi;
pub use glib;
pub use gst;

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
