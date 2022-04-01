// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::cast_ptr_alignment)]

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
mod audio_aggregator;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
mod audio_aggregator_convert_pad;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
mod audio_aggregator_pad;
mod audio_base_sink;
mod audio_base_src;
mod audio_decoder;
mod audio_encoder;
mod audio_sink;
mod audio_src;

pub mod prelude {
    #[doc(hidden)]
    pub use gst_base::subclass::prelude::*;

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    pub use super::audio_aggregator::{AudioAggregatorImpl, AudioAggregatorImplExt};
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    pub use super::audio_aggregator_convert_pad::AudioAggregatorConvertPadImpl;
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    pub use super::audio_aggregator_pad::{AudioAggregatorPadImpl, AudioAggregatorPadImplExt};
    pub use super::audio_base_sink::AudioBaseSinkImpl;
    pub use super::audio_base_src::AudioBaseSrcImpl;
    pub use super::audio_decoder::{AudioDecoderImpl, AudioDecoderImplExt};
    pub use super::audio_encoder::{AudioEncoderImpl, AudioEncoderImplExt};
    pub use super::audio_sink::{AudioSinkImpl, AudioSinkImplExt};
    pub use super::audio_src::{AudioSrcImpl, AudioSrcImplExt};
}
