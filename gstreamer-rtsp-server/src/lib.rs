// Copyright (C) 2018 Mathieu Duponchelle <mathieu@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
extern crate libc;

extern crate gio;
extern crate gio_sys as gio_sys;
use std::ffi::CStr;
#[macro_use]
extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;
#[macro_use]
extern crate gstreamer as gst;
extern crate gstreamer_net as gst_net;
extern crate gstreamer_net_sys as gst_net_sys;
extern crate gstreamer_rtsp as gst_rtsp;
extern crate gstreamer_rtsp_server_sys as gst_rtsp_server_sys;
extern crate gstreamer_rtsp_sys as gst_rtsp_sys;
extern crate gstreamer_sdp as gst_sdp;
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
#[allow(clippy::type_complexity)]
#[allow(clippy::let_and_return)]
mod auto;
pub use auto::*;

mod rtsp_address_pool;
mod rtsp_auth;
mod rtsp_client;
mod rtsp_context;
mod rtsp_media;
mod rtsp_media_factory;
mod rtsp_server;
mod rtsp_session_pool;
mod rtsp_stream;
mod rtsp_stream_transport;
mod rtsp_thread;
mod rtsp_token;

pub mod subclass;

pub use rtsp_address_pool::RTSPAddressPoolExtManual;
pub use rtsp_auth::RTSPAuthExtManual;
pub use rtsp_client::RTSPClientExtManual;
pub use rtsp_media::RTSPMediaExtManual;
pub use rtsp_media_factory::RTSPMediaFactoryExtManual;
pub use rtsp_server::RTSPServerExtManual;
pub use rtsp_session_pool::RTSPSessionPoolExtManual;
pub use rtsp_stream::RTSPStreamExtManual;
pub use rtsp_stream_transport::RTSPStreamTransportExtManual;
pub use rtsp_thread::*;

pub use rtsp_context::*;
pub use rtsp_token::*;

lazy_static! {
    pub static ref RTSP_ADDRESS_POOL_ANY_IPV4: &'static str = unsafe {
        CStr::from_ptr(gst_rtsp_server_sys::GST_RTSP_ADDRESS_POOL_ANY_IPV4)
            .to_str()
            .unwrap()
    };
    pub static ref RTSP_ADDRESS_POOL_ANY_IPV6: &'static str = unsafe {
        CStr::from_ptr(gst_rtsp_server_sys::GST_RTSP_ADDRESS_POOL_ANY_IPV6)
            .to_str()
            .unwrap()
    };
    pub static ref RTSP_AUTH_CHECK_CONNECT: &'static str = unsafe {
        CStr::from_ptr(gst_rtsp_server_sys::GST_RTSP_AUTH_CHECK_CONNECT)
            .to_str()
            .unwrap()
    };
    pub static ref RTSP_AUTH_CHECK_MEDIA_FACTORY_ACCESS: &'static str = unsafe {
        CStr::from_ptr(gst_rtsp_server_sys::GST_RTSP_AUTH_CHECK_MEDIA_FACTORY_ACCESS)
            .to_str()
            .unwrap()
    };
    pub static ref RTSP_AUTH_CHECK_MEDIA_FACTORY_CONSTRUCT: &'static str = unsafe {
        CStr::from_ptr(gst_rtsp_server_sys::GST_RTSP_AUTH_CHECK_MEDIA_FACTORY_CONSTRUCT)
            .to_str()
            .unwrap()
    };
    pub static ref RTSP_AUTH_CHECK_TRANSPORT_CLIENT_SETTINGS: &'static str = unsafe {
        CStr::from_ptr(gst_rtsp_server_sys::GST_RTSP_AUTH_CHECK_TRANSPORT_CLIENT_SETTINGS)
            .to_str()
            .unwrap()
    };
    pub static ref RTSP_AUTH_CHECK_URL: &'static str = unsafe {
        CStr::from_ptr(gst_rtsp_server_sys::GST_RTSP_AUTH_CHECK_URL)
            .to_str()
            .unwrap()
    };
    pub static ref RTSP_PERM_MEDIA_FACTORY_ACCESS: &'static str = unsafe {
        CStr::from_ptr(gst_rtsp_server_sys::GST_RTSP_PERM_MEDIA_FACTORY_ACCESS)
            .to_str()
            .unwrap()
    };
    pub static ref RTSP_PERM_MEDIA_FACTORY_CONSTRUCT: &'static str = unsafe {
        CStr::from_ptr(gst_rtsp_server_sys::GST_RTSP_PERM_MEDIA_FACTORY_CONSTRUCT)
            .to_str()
            .unwrap()
    };
    pub static ref RTSP_TOKEN_MEDIA_FACTORY_ROLE: &'static str = unsafe {
        CStr::from_ptr(gst_rtsp_server_sys::GST_RTSP_TOKEN_MEDIA_FACTORY_ROLE)
            .to_str()
            .unwrap()
    };
    pub static ref RTSP_TOKEN_TRANSPORT_CLIENT_SETTINGS: &'static str = unsafe {
        CStr::from_ptr(gst_rtsp_server_sys::GST_RTSP_TOKEN_TRANSPORT_CLIENT_SETTINGS)
            .to_str()
            .unwrap()
    };
}

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;
    pub use gst::prelude::*;

    pub use auto::traits::*;

    pub use rtsp_address_pool::RTSPAddressPoolExtManual;
    pub use rtsp_auth::RTSPAuthExtManual;
    pub use rtsp_client::RTSPClientExtManual;
    pub use rtsp_media::RTSPMediaExtManual;
    pub use rtsp_media_factory::RTSPMediaFactoryExtManual;
    pub use rtsp_server::RTSPServerExtManual;
    pub use rtsp_session_pool::RTSPSessionPoolExtManual;
    pub use rtsp_stream::RTSPStreamExtManual;
    pub use rtsp_stream_transport::RTSPStreamTransportExtManual;
}
