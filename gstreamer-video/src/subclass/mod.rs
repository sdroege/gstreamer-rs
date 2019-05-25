// Copyright (C) 2019 Philippe Normand <philn@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]

pub mod video_decoder;
pub mod video_encoder;

pub mod prelude {
    pub use super::video_decoder::{VideoDecoderImpl, VideoDecoderImplExt};
    pub use super::video_encoder::{VideoEncoderImpl, VideoEncoderImplExt};
}
