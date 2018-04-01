// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gstreamer as gst;
extern crate gstreamer_base_sys as ffi;
extern crate gstreamer_sys as gst_ffi;

#[macro_use]
extern crate glib;

macro_rules! callback_guard {
    () => {
        let _guard = ::glib::CallbackGuard::new();
    };
}

macro_rules! assert_initialized_main_thread {
    () => {
        if unsafe { ::gst_ffi::gst_is_initialized() } != ::glib_ffi::GTRUE {
            panic!("GStreamer has not been initialized. Call `gst::init` first.");
        }
    };
}

pub use glib::{Cast, Continue, Error, IsA, StaticType, ToValue, Type, TypedValue, Value};

#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
mod auto;
pub use auto::functions::*;
pub use auto::*;

pub mod functions;
pub use functions::*;

mod adapter;
mod flow_combiner;
pub use flow_combiner::*;
mod base_sink;
mod base_src;
mod base_transform;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;
    pub use gst::prelude::*;

    pub use auto::traits::*;
    pub use base_sink::BaseSinkExtManual;
    pub use base_src::BaseSrcExtManual;
    pub use base_transform::BaseTransformExtManual;
}

mod utils;
