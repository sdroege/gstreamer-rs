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
#[cfg_attr(feature = "cargo-clippy", allow(non_snake_case))]
mod auto;
pub use auto::*;

mod m_i_k_e_y_decrypt_info;
pub use m_i_k_e_y_decrypt_info::*;
mod m_i_k_e_y_encrypt_info;
pub use m_i_k_e_y_encrypt_info::*;
mod m_i_k_e_y_map_s_r_t_p;
pub use m_i_k_e_y_map_s_r_t_p::*;
mod m_i_k_e_y_payload_s_p_param;
pub use m_i_k_e_y_payload_s_p_param::*;
mod s_d_p_attribute;
pub use s_d_p_attribute::*;
mod s_d_p_bandwidth;
pub use s_d_p_bandwidth::*;
mod s_d_p_connection;
pub use s_d_p_connection::*;
mod s_d_p_key;
pub use s_d_p_key::*;
mod s_d_p_media;
pub use s_d_p_media::*;
mod s_d_p_message;
pub use s_d_p_message::*;
mod s_d_p_origin;
pub use s_d_p_origin::*;
mod s_d_p_time;
pub use s_d_p_time::*;
mod s_d_p_zone;
pub use s_d_p_zone::*;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;
    pub use gst::prelude::*;

    pub use auto::traits::*;
}
