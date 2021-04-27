// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::cast_ptr_alignment)]

mod base_parse;
mod base_sink;
// Public to namespace CreateSuccess
pub mod base_src;
// Public to namespace GenerateOutputSuccess and PrepareOutputBufferSuccess,
pub mod base_transform;
mod push_src;

pub use self::base_transform::BaseTransformMode;

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
mod aggregator;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
mod aggregator_pad;

pub mod prelude {
    #[doc(hidden)]
    pub use gst::subclass::prelude::*;

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    pub use super::aggregator::{AggregatorImpl, AggregatorImplExt};
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    pub use super::aggregator_pad::{AggregatorPadImpl, AggregatorPadImplExt};
    pub use super::base_parse::{BaseParseImpl, BaseParseImplExt};
    pub use super::base_sink::{BaseSinkImpl, BaseSinkImplExt};
    pub use super::base_src::{BaseSrcImpl, BaseSrcImplExt};
    pub use super::base_transform::{BaseTransformImpl, BaseTransformImplExt};
    pub use super::push_src::{PushSrcImpl, PushSrcImplExt};
}
