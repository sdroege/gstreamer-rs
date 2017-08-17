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

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gstreamer_sys as gst_ffi;
extern crate gstreamer_video_sys as ffi;
extern crate gstreamer as gst;
#[macro_use]
extern crate glib;

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

mod video_format;
pub use video_format::*;
mod video_format_info;
pub use video_format_info::*;
mod video_info;
pub use video_info::*;
mod video_frame;
pub use video_frame::VideoFrame;
mod video_overlay;
pub use video_overlay::VideoOverlayExtManual;
