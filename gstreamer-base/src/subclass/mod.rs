// Copyright (C) 2016-2018 Sebastian Dröge <sebastian@centricular.com>
//               2016 Luis de Bethencourt <luisbg@osg.samsung.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![allow(clippy::cast_ptr_alignment)]

pub mod base_sink;
pub mod base_src;
pub mod base_transform;

pub use self::base_transform::BaseTransformMode;

#[cfg(any(feature = "v1_14", feature = "dox"))]
pub mod aggregator;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub mod aggregator_pad;

pub mod prelude {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub use super::aggregator::{AggregatorImpl, AggregatorImplExt};
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub use super::aggregator_pad::{AggregatorPadImpl, AggregatorPadImplExt};
    pub use super::base_sink::{BaseSinkImpl, BaseSinkImplExt};
    pub use super::base_src::{BaseSrcImpl, BaseSrcImplExt};
    pub use super::base_transform::{
        BaseTransformClassSubclassExt, BaseTransformImpl, BaseTransformImplExt,
    };
}
