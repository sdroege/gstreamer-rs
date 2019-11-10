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

use std::sync::Once;

#[macro_use]
extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;
extern crate gstreamer as gst;
extern crate gstreamer_pbutils_sys as gst_pbutils_sys;
extern crate gstreamer_sys as gst_sys;

static PBUTILS_INIT: Once = Once::new();

macro_rules! assert_initialized_main_thread {
    () => {
        if unsafe { ::gst_sys::gst_is_initialized() } != ::glib_sys::GTRUE {
            panic!("GStreamer has not been initialized. Call `gst::init` first.");
        }
        ::PBUTILS_INIT.call_once(|| {
            unsafe { ::gst_pbutils_sys::gst_pb_utils_init() };
        });
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::unreadable_literal)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::match_same_arms)]
#[allow(clippy::type_complexity)]
mod auto;
pub use auto::functions::*;
pub use auto::*;

mod discoverer;
pub use discoverer::*;

pub mod discoverer_stream_info;

mod discoverer_video_info;
pub use discoverer_video_info::*;

mod encoding_profile;
pub use encoding_profile::*;

pub mod functions;
pub use functions::*;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;
    pub use gst::prelude::*;

    pub use auto::traits::*;
    pub use encoding_profile::EncodingProfileBuilder;

    pub use functions::CodecTag;
}
