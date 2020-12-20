// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::cast_ptr_alignment)]

mod audio_decoder;
mod audio_encoder;
mod audio_sink;
mod audio_src;

pub mod prelude {
    #[doc(hidden)]
    pub use glib::subclass::prelude::*;
    #[doc(hidden)]
    pub use gst::subclass::prelude::*;
    #[doc(hidden)]
    pub use gst_base::subclass::prelude::*;

    pub use super::audio_decoder::{AudioDecoderImpl, AudioDecoderImplExt};
    pub use super::audio_encoder::{AudioEncoderImpl, AudioEncoderImplExt};
    pub use super::audio_sink::{AudioSinkImpl, AudioSinkImplExt};
    pub use super::audio_src::{AudioSrcImpl, AudioSrcImplExt};
}
