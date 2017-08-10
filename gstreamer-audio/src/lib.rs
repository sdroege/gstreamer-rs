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
extern crate array_init;

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gstreamer_sys as gst_ffi;
extern crate gstreamer_audio_sys as ffi;
extern crate gstreamer as gst;
#[macro_use]
extern crate glib;

macro_rules! assert_initialized_main_thread {
    () => (
        use gst_ffi;
        assert_eq!(unsafe {gst_ffi::gst_is_initialized()}, ::glib_ffi::GTRUE)
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
pub use auto::traits::*;

mod audio_format;
pub use audio_format::*;
mod audio_format_info;
pub use audio_format_info::*;
mod audio_info;
pub use audio_info::*;
mod audio_channel_position;
pub use audio_channel_position::*;
