// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::cast_ptr_alignment)]

mod audio_base_sink;
mod audio_base_src;
mod audio_decoder;
mod audio_encoder;
mod audio_sink;
mod audio_src;

pub mod prelude {
    #[doc(hidden)]
    pub use gst_base::subclass::prelude::*;

    pub use super::audio_base_sink::AudioBaseSinkImpl;
    pub use super::audio_base_src::AudioBaseSrcImpl;
    pub use super::audio_decoder::{AudioDecoderImpl, AudioDecoderImplExt};
    pub use super::audio_encoder::{AudioEncoderImpl, AudioEncoderImplExt};
    pub use super::audio_sink::{AudioSinkImpl, AudioSinkImplExt};
    pub use super::audio_src::{AudioSrcImpl, AudioSrcImplExt};
}
