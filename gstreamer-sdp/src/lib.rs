// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature = "dox", feature(doc_cfg))]

#[macro_use]
extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;
extern crate gstreamer as gst;
extern crate gstreamer_sdp_sys as gst_sdp_sys;
extern crate gstreamer_sys as gst_sys;

macro_rules! assert_initialized_main_thread {
    () => {
        if unsafe { ::gst_sys::gst_is_initialized() } != ::glib_sys::GTRUE {
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
mod auto;
pub use auto::*;

mod sdp_attribute;
pub use sdp_attribute::*;
mod sdp_bandwidth;
pub use sdp_bandwidth::*;
mod sdp_connection;
pub use sdp_connection::*;
mod sdp_key;
pub use sdp_key::*;
pub mod sdp_media;
pub use sdp_media::{SDPMedia, SDPMediaRef};
pub mod sdp_message;
pub use sdp_message::{SDPMessage, SDPMessageRef};
mod sdp_origin;
pub use sdp_origin::*;
mod sdp_time;
pub use sdp_time::*;
mod sdp_zone;
pub use sdp_zone::*;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;
    pub use gst::prelude::*;

    pub use auto::traits::*;
}
