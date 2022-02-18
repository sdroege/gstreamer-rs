// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::cast_ptr_alignment)]

mod rtsp_client;
mod rtsp_media;
mod rtsp_media_factory;
mod rtsp_mount_points;
mod rtsp_server;

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
mod rtsp_onvif_client;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
mod rtsp_onvif_media;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
mod rtsp_onvif_media_factory;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
mod rtsp_onvif_server;

pub use self::rtsp_media::SDPInfo;

pub mod prelude {
    #[doc(hidden)]
    pub use gst::subclass::prelude::*;

    pub use super::rtsp_client::{RTSPClientImpl, RTSPClientImplExt};
    pub use super::rtsp_media::{RTSPMediaImpl, RTSPMediaImplExt};
    pub use super::rtsp_media_factory::{RTSPMediaFactoryImpl, RTSPMediaFactoryImplExt};
    pub use super::rtsp_mount_points::{RTSPMountPointsImpl, RTSPMountPointsImplExt};
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    pub use super::rtsp_onvif_client::RTSPOnvifClientImpl;
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    pub use super::rtsp_onvif_media::RTSPOnvifMediaImpl;
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    pub use super::rtsp_onvif_media_factory::{
        RTSPOnvifMediaFactoryImpl, RTSPOnvifMediaFactoryImplExt,
    };
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    pub use super::rtsp_onvif_server::RTSPOnvifServerImpl;
    pub use super::rtsp_server::{RTSPServerImpl, RTSPServerImplExt};
}
