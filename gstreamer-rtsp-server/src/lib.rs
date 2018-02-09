// Copyright (C) 2018 Mathieu Duponchelle <mathieu@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate bitflags;
extern crate libc;

#[macro_use]
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gio;
extern crate gio_sys as gio_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gstreamer as gst;
extern crate gstreamer_sys as gst_ffi;
extern crate gstreamer_rtsp as gst_rtsp;
extern crate gstreamer_rtsp_sys as gst_rtsp_ffi;
extern crate gstreamer_net as gst_net;
extern crate gstreamer_net_sys as gst_net_ffi;
extern crate gstreamer_rtsp_server_sys as ffi;

macro_rules! callback_guard {
    () => (
        let _guard = ::glib::CallbackGuard::new();
    )
}

macro_rules! assert_initialized_main_thread {
    () => (
        if unsafe {::gst_ffi::gst_is_initialized()} != ::glib_ffi::GTRUE {
            panic!("GStreamer has not been initialized. Call `gst::init` first.");
        }
    )
}

macro_rules! skip_assert_initialized {
    () => (
    )
}

pub use glib::{Cast, Continue, Error, IsA, StaticType, ToValue, Type, TypedValue, Value};

#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
mod auto;
pub use auto::*;

mod r_t_s_p_server;
mod r_t_s_p_address_pool;
mod r_t_s_p_client;
mod r_t_s_p_session_pool;

pub use r_t_s_p_server::RTSPServerExtManual;
pub use r_t_s_p_address_pool::RTSPAddressPoolExtManual;
pub use r_t_s_p_client::RTSPClientExtManual;
pub use r_t_s_p_session_pool::RTSPSessionPoolExtManual;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;
    pub use gst::prelude::*;

    pub use auto::traits::*;

    pub use r_t_s_p_server::RTSPServerExtManual;
    pub use r_t_s_p_address_pool::RTSPAddressPoolExtManual;
    pub use r_t_s_p_client::RTSPClientExtManual;
    pub use r_t_s_p_session_pool::RTSPSessionPoolExtManual;
}
