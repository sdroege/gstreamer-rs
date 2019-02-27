// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate bitflags;
extern crate libc;

#[macro_use]
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gstreamer as gst;
extern crate gstreamer_sdp_sys as ffi;
extern crate gstreamer_sys as gst_ffi;

macro_rules! assert_initialized_main_thread {
    () => {
        if unsafe { ::gst_ffi::gst_is_initialized() } != ::glib_ffi::GTRUE {
            panic!("GStreamer has not been initialized. Call `gst::init` first.");
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

pub use glib::{Cast, Continue, Error, IsA, StaticType, ToValue, Type, TypedValue, Value};

#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
#[allow(non_snake_case)]
#[rustfmt::skip]
mod auto;
pub use auto::*;

mod mikey_decrypt_info;
pub use mikey_decrypt_info::*;
mod mikey_encrypt_info;
pub use mikey_encrypt_info::*;
mod mikey_map_s_r_t_p;
pub use mikey_map_s_r_t_p::*;
mod mikey_payload_s_p_param;
pub use mikey_payload_s_p_param::*;
mod sdp_attribute;
pub use sdp_attribute::*;
mod sdp_bandwidth;
pub use sdp_bandwidth::*;
mod sdp_connection;
pub use sdp_connection::*;
mod sdp_key;
pub use sdp_key::*;
mod sdp_media;
pub use sdp_media::{SDPMedia, SDPMediaRef};
mod sdp_message;
pub use sdp_message::*;
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
