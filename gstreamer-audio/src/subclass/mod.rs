// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::cast_ptr_alignment)]

mod audio_aggregator;
mod audio_aggregator_convert_pad;
mod audio_aggregator_pad;
mod audio_base_sink;
mod audio_base_src;
mod audio_decoder;
mod audio_encoder;
mod audio_filter;
mod audio_sink;
mod audio_src;

pub mod prelude {
    #[doc(hidden)]
    pub use gst_base::subclass::prelude::*;

    pub use super::{
        audio_aggregator::{AudioAggregatorImpl, AudioAggregatorImplExt},
        audio_aggregator_convert_pad::AudioAggregatorConvertPadImpl,
        audio_aggregator_pad::{AudioAggregatorPadImpl, AudioAggregatorPadImplExt},
        audio_base_sink::AudioBaseSinkImpl,
        audio_base_src::AudioBaseSrcImpl,
        audio_decoder::{AudioDecoderImpl, AudioDecoderImplExt},
        audio_encoder::{AudioEncoderImpl, AudioEncoderImplExt},
        audio_filter::{AudioFilterImpl, AudioFilterImplExt},
        audio_sink::{AudioSinkImpl, AudioSinkImplExt},
        audio_src::{AudioSrcImpl, AudioSrcImplExt},
    };
}
