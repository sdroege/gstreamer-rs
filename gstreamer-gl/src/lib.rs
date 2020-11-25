// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//               2018 Víctor M. Jáquez L. <vjaquez@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(all(not(doctest), doc), feature(doc_cfg))]

pub use ffi;

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
mod auto;
pub use crate::auto::*;

mod caps_features;
pub use crate::caps_features::{CAPS_FEATURES_MEMORY_GL_MEMORY, CAPS_FEATURE_MEMORY_GL_MEMORY};
mod context;
pub use crate::context::ContextGLExt;
mod gl_context;
pub use crate::gl_context::GLContextExtManual;
mod gl_display;
pub use crate::gl_display::GL_DISPLAY_CONTEXT_TYPE;
#[cfg(any(feature = "egl", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "egl")))]
mod gl_display_egl;
#[cfg(any(feature = "wayland", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "wayland")))]
mod gl_display_wayland;
#[cfg(any(feature = "x11", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "x11")))]
mod gl_display_x11;
mod gl_video_frame;
pub use crate::gl_video_frame::VideoFrameGLExt;
mod gl_sync_meta;
pub use crate::gl_sync_meta::*;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;
    pub use gst::prelude::*;

    pub use crate::auto::traits::*;

    pub use crate::context::ContextGLExt;
    pub use crate::gl_context::GLContextExtManual;
    pub use crate::gl_video_frame::VideoFrameGLExt;
}
