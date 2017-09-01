// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate bitflags;
extern crate array_init;

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gstreamer_sys as gst_ffi;
extern crate gstreamer_audio_sys as ffi;
extern crate gstreamer as gst;
#[macro_use]
extern crate glib;

macro_rules! callback_guard {
    () => (
        let _guard = ::glib::CallbackGuard::new();
    )
}

macro_rules! assert_initialized_main_thread {
    () => (
        if unsafe {::gst_ffi::gst_is_initialized()} != ::glib_ffi::GTRUE {
            panic!("GStreamer has not been initialized. Call `gst::init` first.");
        }
    )
}

macro_rules! skip_assert_initialized {
    () => (
    )
}

pub use glib::{Cast, Continue, Error, IsA, StaticType, ToValue, Type, TypedValue, Value};

#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
mod auto;
pub use auto::*;

mod audio_format;
pub use audio_format::*;
mod audio_format_info;
pub use audio_format_info::*;
mod audio_info;
pub use audio_info::*;
mod audio_channel_position;
pub use audio_channel_position::*;

use glib::translate::{from_glib_full, ToGlibPtr};
pub fn audio_buffer_clip(
    buffer: gst::Buffer,
    segment: &gst::Segment,
    rate: u32,
    channels: u32,
) -> gst::Buffer {
    skip_assert_initialized!();

    unsafe {
        from_glib_full(ffi::gst_audio_buffer_clip(
            buffer.into_ptr(),
            segment.to_glib_none().0,
            rate as i32,
            channels as i32,
        ))
    }
}

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;
    pub use gst::prelude::*;

    pub use auto::traits::*;
}
