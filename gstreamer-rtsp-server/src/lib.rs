// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::non_send_fields_in_send_ty)]
#![doc = include_str!("../README.md")]

pub use ffi;
pub use gio;
pub use glib;
pub use gst;
pub use gst_net;
pub use gst_rtsp;
pub use gst_sdp;

macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            #[allow(unused_unsafe)]
            if unsafe { gst::ffi::gst_is_initialized() } != glib::ffi::GTRUE {
                panic!("GStreamer has not been initialized. Call `gst::init` first.");
            } else {
                gst::INITIALIZED.store(true, std::sync::atomic::Ordering::SeqCst);
            }
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
#[allow(clippy::use_self)]
#[allow(unused_imports)]
mod auto;
pub use crate::auto::*;

#[cfg(feature = "serde")]
mod flag_serde;

mod rtsp_address_pool;
mod rtsp_auth;
mod rtsp_client;
mod rtsp_context;
mod rtsp_media;
mod rtsp_media_factory;
mod rtsp_onvif_media_factory;
mod rtsp_server;
mod rtsp_session;
mod rtsp_session_pool;
mod rtsp_thread;
mod rtsp_token;

pub mod subclass;

pub use crate::rtsp_context::*;
pub use crate::rtsp_thread::*;
pub use crate::rtsp_token::*;

use once_cell::sync::Lazy;

use std::ffi::CStr;

pub static RTSP_ADDRESS_POOL_ANY_IPV4: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_RTSP_ADDRESS_POOL_ANY_IPV4)
        .to_str()
        .unwrap()
});
pub static RTSP_ADDRESS_POOL_ANY_IPV6: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_RTSP_ADDRESS_POOL_ANY_IPV6)
        .to_str()
        .unwrap()
});
pub static RTSP_AUTH_CHECK_CONNECT: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_RTSP_AUTH_CHECK_CONNECT)
        .to_str()
        .unwrap()
});
pub static RTSP_AUTH_CHECK_MEDIA_FACTORY_ACCESS: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_RTSP_AUTH_CHECK_MEDIA_FACTORY_ACCESS)
        .to_str()
        .unwrap()
});
pub static RTSP_AUTH_CHECK_MEDIA_FACTORY_CONSTRUCT: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_RTSP_AUTH_CHECK_MEDIA_FACTORY_CONSTRUCT)
        .to_str()
        .unwrap()
});
pub static RTSP_AUTH_CHECK_TRANSPORT_CLIENT_SETTINGS: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_RTSP_AUTH_CHECK_TRANSPORT_CLIENT_SETTINGS)
        .to_str()
        .unwrap()
});
pub static RTSP_AUTH_CHECK_URL: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_RTSP_AUTH_CHECK_URL)
        .to_str()
        .unwrap()
});
pub static RTSP_PERM_MEDIA_FACTORY_ACCESS: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_RTSP_PERM_MEDIA_FACTORY_ACCESS)
        .to_str()
        .unwrap()
});
pub static RTSP_PERM_MEDIA_FACTORY_CONSTRUCT: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_RTSP_PERM_MEDIA_FACTORY_CONSTRUCT)
        .to_str()
        .unwrap()
});
pub static RTSP_TOKEN_MEDIA_FACTORY_ROLE: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_RTSP_TOKEN_MEDIA_FACTORY_ROLE)
        .to_str()
        .unwrap()
});
pub static RTSP_TOKEN_TRANSPORT_CLIENT_SETTINGS: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_RTSP_TOKEN_TRANSPORT_CLIENT_SETTINGS)
        .to_str()
        .unwrap()
});

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_rtsp_server::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gio::prelude::*;
    #[doc(hidden)]
    pub use gst_net::prelude::*;
    #[doc(hidden)]
    pub use gst_rtsp::prelude::*;

    pub use crate::auto::traits::*;

    pub use crate::rtsp_address_pool::RTSPAddressPoolExtManual;
    pub use crate::rtsp_auth::RTSPAuthExtManual;
    pub use crate::rtsp_client::RTSPClientExtManual;
    pub use crate::rtsp_media::RTSPMediaExtManual;
    pub use crate::rtsp_media_factory::RTSPMediaFactoryExtManual;
    pub use crate::rtsp_onvif_media_factory::RTSPOnvifMediaFactoryExtManual;
    pub use crate::rtsp_server::RTSPServerExtManual;
    pub use crate::rtsp_session::RTSPSessionExtManual;
    pub use crate::rtsp_session_pool::RTSPSessionPoolExtManual;
}
