// Copyright (C) 2019 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![allow(clippy::cast_ptr_alignment)]

pub mod audio_decoder;
pub mod audio_encoder;
pub mod audio_sink;

pub mod prelude {
    pub use super::audio_decoder::{AudioDecoderImpl, AudioDecoderImplExt};
    pub use super::audio_encoder::{AudioEncoderImpl, AudioEncoderImplExt};
    pub use super::audio_sink::{AudioSinkImpl, AudioSinkImplExt};
}
