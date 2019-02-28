// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
#[cfg_attr(feature = "subclassing", macro_use)]
extern crate gstreamer as gst;
extern crate gstreamer_base_sys as ffi;
extern crate gstreamer_sys as gst_ffi;

extern crate libc;

#[macro_use]
extern crate glib;

macro_rules! assert_initialized_main_thread {
    () => {
        if unsafe { ::gst_ffi::gst_is_initialized() } != ::glib_ffi::GTRUE {
            panic!("GStreamer has not been initialized. Call `gst::init` first.");
        }
    };
}

pub use glib::{Cast, Continue, Error, IsA, StaticType, ToValue, Type, TypedValue, Value};

#[allow(clippy::unreadable_literal)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::match_same_arms)]
#[allow(clippy::type_complexity)]
#[rustfmt::skip]
mod auto;
pub use auto::functions::*;
pub use auto::*;

pub mod functions;
pub use functions::*;

mod adapter;
pub use adapter::*;
mod flow_combiner;
pub use flow_combiner::*;
#[cfg(any(feature = "v1_14", feature = "dox"))]
mod aggregator;
#[cfg(any(feature = "v1_14", feature = "dox"))]
mod aggregator_pad;
mod base_sink;
mod base_src;
mod base_transform;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;
    pub use gst::prelude::*;

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub use aggregator::AggregatorExtManual;
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub use aggregator_pad::AggregatorPadExtManual;
    pub use auto::traits::*;
    pub use base_sink::BaseSinkExtManual;
    pub use base_src::BaseSrcExtManual;
    pub use base_transform::BaseTransformExtManual;
}

mod utils;

#[cfg(feature = "subclassing")]
pub mod subclass;
