// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//               2018 Víctor M. Jáquez L. <vjaquez@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate bitflags;
extern crate byteorder;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gstreamer as gst;
extern crate gstreamer_base as gst_base;
extern crate gstreamer_gl_sys as ffi;
extern crate gstreamer_sys as gst_ffi;
extern crate gstreamer_video as gst_video;
extern crate gstreamer_video_sys as gst_video_ffi;

macro_rules! assert_initialized_main_thread {
    () => {
        if unsafe { ::gst_ffi::gst_is_initialized() } != ::glib_ffi::GTRUE {
            panic!("GStreamer has not been initialized. Call `gst::init` first.");
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

pub use glib::{Cast, Continue, Error, IsA, StaticType, ToValue, Type, TypedValue, Value};

#[allow(clippy::unreadable_literal)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::match_same_arms)]
#[rustfmt::skip]
mod auto;
pub use auto::*;

mod caps_features;
pub use caps_features::{CAPS_FEATURES_MEMORY_GL_MEMORY, CAPS_FEATURE_MEMORY_GL_MEMORY};
mod context;
pub use context::ContextGLExt;
mod gl_context;
pub use gl_context::GLContextExtManual;
mod gl_display;
pub use gl_display::GL_DISPLAY_CONTEXT_TYPE;
#[cfg(any(feature = "egl", feature = "dox"))]
mod gl_display_egl;
mod gl_video_frame;
pub use gl_video_frame::VideoFrameGLExt;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;
    pub use gst::prelude::*;

    pub use auto::traits::*;

    pub use context::ContextGLExt;
    pub use gl_context::GLContextExtManual;
    pub use gl_video_frame::VideoFrameGLExt;
}
