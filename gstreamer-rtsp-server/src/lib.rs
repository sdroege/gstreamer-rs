// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;

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
#[allow(clippy::type_complexity)]
#[allow(clippy::let_and_return)]
mod auto;
pub use crate::auto::*;

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

pub use crate::rtsp_address_pool::RTSPAddressPoolExtManual;
pub use crate::rtsp_auth::RTSPAuthExtManual;
pub use crate::rtsp_client::RTSPClientExtManual;
pub use crate::rtsp_media::RTSPMediaExtManual;
pub use crate::rtsp_media_factory::RTSPMediaFactoryExtManual;
pub use crate::rtsp_server::RTSPServerExtManual;
pub use crate::rtsp_session_pool::RTSPSessionPoolExtManual;
pub use crate::rtsp_stream::RTSPStreamExtManual;
pub use crate::rtsp_stream_transport::RTSPStreamTransportExtManual;
pub use crate::rtsp_thread::*;

pub use crate::rtsp_context::*;
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
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;
    pub use gst::prelude::*;

    pub use crate::auto::traits::*;

    pub use crate::rtsp_address_pool::RTSPAddressPoolExtManual;
    pub use crate::rtsp_auth::RTSPAuthExtManual;
    pub use crate::rtsp_client::RTSPClientExtManual;
    pub use crate::rtsp_media::RTSPMediaExtManual;
    pub use crate::rtsp_media_factory::RTSPMediaFactoryExtManual;
    pub use crate::rtsp_server::RTSPServerExtManual;
    pub use crate::rtsp_session_pool::RTSPSessionPoolExtManual;
    pub use crate::rtsp_stream::RTSPStreamExtManual;
    pub use crate::rtsp_stream_transport::RTSPStreamTransportExtManual;
}
