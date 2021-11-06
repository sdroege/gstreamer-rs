// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![doc = include_str!("../README.md")]

pub use ffi;
pub use glib;
pub use gst;

macro_rules! assert_initialized_main_thread {
    () => {
        if unsafe { gst::ffi::gst_is_initialized() } != glib::ffi::GTRUE {
            panic!("GStreamer has not been initialized. Call `gst::init` first.");
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::unreadable_literal)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::match_same_arms)]
#[allow(clippy::type_complexity)]
#[allow(clippy::use_self)]
#[allow(unused_imports)]
mod auto;
pub use crate::auto::functions::*;
pub use crate::auto::*;

pub mod functions;
pub use crate::functions::*;

mod adapter;
pub use crate::adapter::*;
mod flow_combiner;
pub use crate::flow_combiner::*;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
mod aggregator;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
mod aggregator_pad;
mod base_parse;
mod base_sink;
mod base_src;
mod base_transform;

pub mod base_parse_frame;
pub use crate::base_parse_frame::BaseParseFrame;

pub const BASE_TRANSFORM_FLOW_DROPPED: gst::FlowSuccess = gst::FlowSuccess::CustomSuccess;
pub const BASE_PARSE_FLOW_DROPPED: gst::FlowSuccess = gst::FlowSuccess::CustomSuccess;
pub const AGGREGATOR_FLOW_NEED_DATA: gst::FlowError = gst::FlowError::CustomError;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_base::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst::prelude::*;

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    pub use crate::aggregator::AggregatorExtManual;
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    pub use crate::aggregator_pad::AggregatorPadExtManual;
    pub use crate::auto::traits::*;
    pub use crate::base_parse::BaseParseExtManual;
    pub use crate::base_sink::BaseSinkExtManual;
    pub use crate::base_src::BaseSrcExtManual;
    pub use crate::base_transform::BaseTransformExtManual;
}

mod utils;

pub mod subclass;
