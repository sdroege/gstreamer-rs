// Copyright (C) 2019 Philippe Normand <philn@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![allow(clippy::cast_ptr_alignment)]

mod video_decoder;
mod video_encoder;
mod video_sink;

pub mod prelude {
    pub use super::video_decoder::{VideoDecoderImpl, VideoDecoderImplExt};
    pub use super::video_encoder::{VideoEncoderImpl, VideoEncoderImplExt};
    pub use super::video_sink::{VideoSinkImpl, VideoSinkImplExt};
}
