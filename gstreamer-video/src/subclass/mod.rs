// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::cast_ptr_alignment)]

mod navigation;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
mod video_aggregator;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
mod video_aggregator_convert_pad;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
mod video_aggregator_pad;
mod video_decoder;
mod video_encoder;
mod video_filter;
mod video_sink;

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
pub use video_aggregator::AggregateFramesToken;

pub mod prelude {
    #[doc(hidden)]
    pub use gst_base::subclass::prelude::*;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub use super::video_aggregator::{VideoAggregatorImpl, VideoAggregatorImplExt};
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub use super::video_aggregator_convert_pad::VideoAggregatorConvertPadImpl;
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub use super::video_aggregator_pad::{VideoAggregatorPadImpl, VideoAggregatorPadImplExt};
    pub use super::{
        navigation::NavigationImpl,
        video_decoder::{VideoDecoderImpl, VideoDecoderImplExt},
        video_encoder::{VideoEncoderImpl, VideoEncoderImplExt},
        video_filter::{VideoFilterImpl, VideoFilterImplExt},
        video_sink::{VideoSinkImpl, VideoSinkImplExt},
    };
}
