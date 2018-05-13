// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate bitflags;
extern crate libc;

use std::sync::{Once, ONCE_INIT};

#[macro_use]
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gstreamer as gst;
extern crate gstreamer_pbutils_sys as ffi;
extern crate gstreamer_sys as gst_ffi;

static PBUTILS_INIT: Once = ONCE_INIT;

macro_rules! assert_initialized_main_thread {
    () => {
        if unsafe { ::gst_ffi::gst_is_initialized() } != ::glib_ffi::GTRUE {
            panic!("GStreamer has not been initialized. Call `gst::init` first.");
        }
        ::PBUTILS_INIT.call_once(|| {
            unsafe { ::ffi::gst_pb_utils_init() };
        });
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

pub use glib::{Cast, Continue, Error, IsA, StaticType, ToValue, Type, TypedValue, Value};

#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
#[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
mod auto;
pub use auto::*;

mod discoverer;
pub use discoverer::*;

pub mod discoverer_stream_info;

mod discoverer_video_info;
pub use discoverer_video_info::*;

mod encoding_profile;
pub use encoding_profile::*;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;
    pub use gst::prelude::*;

    pub use discoverer::*;
    pub use discoverer_stream_info::*;
    pub use discoverer_video_info::*;

    pub use encoding_profile::*;

    pub use auto::traits::*;
}
