// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
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
            gst::assert_initialized();
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::type_complexity)]
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

pub use crate::{rtsp_context::*, rtsp_thread::*, rtsp_token::*};

pub static RTSP_ADDRESS_POOL_ANY_IPV4: &glib::GStr =
    unsafe { glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_RTSP_ADDRESS_POOL_ANY_IPV4) };
pub static RTSP_ADDRESS_POOL_ANY_IPV6: &glib::GStr =
    unsafe { glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_RTSP_ADDRESS_POOL_ANY_IPV6) };
pub static RTSP_AUTH_CHECK_CONNECT: &glib::GStr =
    unsafe { glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_RTSP_AUTH_CHECK_CONNECT) };
pub static RTSP_AUTH_CHECK_MEDIA_FACTORY_ACCESS: &glib::GStr = unsafe {
    glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_RTSP_AUTH_CHECK_MEDIA_FACTORY_ACCESS)
};
pub static RTSP_AUTH_CHECK_MEDIA_FACTORY_CONSTRUCT: &glib::GStr = unsafe {
    glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_RTSP_AUTH_CHECK_MEDIA_FACTORY_CONSTRUCT)
};
pub static RTSP_AUTH_CHECK_TRANSPORT_CLIENT_SETTINGS: &glib::GStr = unsafe {
    glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_RTSP_AUTH_CHECK_TRANSPORT_CLIENT_SETTINGS)
};
pub static RTSP_AUTH_CHECK_URL: &glib::GStr =
    unsafe { glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_RTSP_AUTH_CHECK_URL) };
pub static RTSP_PERM_MEDIA_FACTORY_ACCESS: &glib::GStr =
    unsafe { glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_RTSP_PERM_MEDIA_FACTORY_ACCESS) };
pub static RTSP_PERM_MEDIA_FACTORY_CONSTRUCT: &glib::GStr =
    unsafe { glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_RTSP_PERM_MEDIA_FACTORY_CONSTRUCT) };
pub static RTSP_TOKEN_MEDIA_FACTORY_ROLE: &glib::GStr =
    unsafe { glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_RTSP_TOKEN_MEDIA_FACTORY_ROLE) };
pub static RTSP_TOKEN_TRANSPORT_CLIENT_SETTINGS: &glib::GStr = unsafe {
    glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_RTSP_TOKEN_TRANSPORT_CLIENT_SETTINGS)
};

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_rtsp_server::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gio::prelude::*;
    #[doc(hidden)]
    pub use gst_net::prelude::*;
    #[doc(hidden)]
    pub use gst_rtsp::prelude::*;

    pub use crate::{
        auto::traits::*, rtsp_address_pool::RTSPAddressPoolExtManual, rtsp_auth::RTSPAuthExtManual,
        rtsp_client::RTSPClientExtManual, rtsp_media::RTSPMediaExtManual,
        rtsp_media_factory::RTSPMediaFactoryExtManual,
        rtsp_onvif_media_factory::RTSPOnvifMediaFactoryExtManual, rtsp_server::RTSPServerExtManual,
        rtsp_session::RTSPSessionExtManual, rtsp_session_pool::RTSPSessionPoolExtManual,
    };
}
