// Copyright (C) 2020 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![allow(clippy::cast_ptr_alignment)]

mod rtsp_client;
mod rtsp_media;
mod rtsp_media_factory;
mod rtsp_server;

pub use self::rtsp_media::SDPInfo;

pub mod prelude {
    pub use super::rtsp_client::{RTSPClientImpl, RTSPClientImplExt};
    pub use super::rtsp_media::{RTSPMediaImpl, RTSPMediaImplExt};
    pub use super::rtsp_media_factory::{RTSPMediaFactoryImpl, RTSPMediaFactoryImplExt};
    pub use super::rtsp_server::{RTSPServerImpl, RTSPServerImplExt};
}
