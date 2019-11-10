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
extern crate glib_sys;
extern crate gobject_sys;
extern crate gstreamer as gst;
extern crate gstreamer_gl_sys as gst_gl_sys;
extern crate gstreamer_sys as gst_sys;
extern crate gstreamer_video as gst_video;
extern crate gstreamer_video_sys as gst_video_sys;

macro_rules! assert_initialized_main_thread {
    () => {
        if unsafe { ::gst_sys::gst_is_initialized() } != ::glib_sys::GTRUE {
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
#[cfg(any(feature = "wayland", feature = "dox"))]
mod gl_display_wayland;
#[cfg(any(feature = "x11", feature = "dox"))]
mod gl_display_x11;
mod gl_video_frame;
pub use gl_video_frame::VideoFrameGLExt;
mod gl_sync_meta;
pub use gl_sync_meta::*;

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
