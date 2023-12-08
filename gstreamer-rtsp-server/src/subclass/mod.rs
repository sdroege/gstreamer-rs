// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::cast_ptr_alignment)]

mod rtsp_auth;
mod rtsp_client;
mod rtsp_media;
mod rtsp_media_factory;
mod rtsp_mount_points;
mod rtsp_server;

mod rtsp_onvif_client;
mod rtsp_onvif_media;
mod rtsp_onvif_media_factory;
mod rtsp_onvif_server;

pub use self::rtsp_media::SDPInfo;

pub mod prelude {
    #[doc(hidden)]
    pub use gst::subclass::prelude::*;

    pub use super::{
        rtsp_auth::{RTSPAuthImpl, RTSPAuthImplExt},
        rtsp_client::{RTSPClientImpl, RTSPClientImplExt},
        rtsp_media::{RTSPMediaImpl, RTSPMediaImplExt},
        rtsp_media_factory::{RTSPMediaFactoryImpl, RTSPMediaFactoryImplExt},
        rtsp_mount_points::{RTSPMountPointsImpl, RTSPMountPointsImplExt},
        rtsp_onvif_client::RTSPOnvifClientImpl,
        rtsp_onvif_media::RTSPOnvifMediaImpl,
        rtsp_onvif_media_factory::{RTSPOnvifMediaFactoryImpl, RTSPOnvifMediaFactoryImplExt},
        rtsp_onvif_server::RTSPOnvifServerImpl,
        rtsp_server::{RTSPServerImpl, RTSPServerImplExt},
    };
}
