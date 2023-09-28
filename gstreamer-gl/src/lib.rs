// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![doc = include_str!("../README.md")]

pub use ffi;
pub use glib;
pub use gst;
pub use gst_base;
pub use gst_video;

macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            gst::assert_initialized();
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

mod auto;
pub use crate::auto::*;

#[cfg(feature = "serde")]
mod flag_serde;

mod caps_features;
pub use crate::caps_features::CAPS_FEATURES_MEMORY_GL_MEMORY;
mod context;
pub mod functions;
pub use crate::functions::*;
mod gl_context;
mod gl_display;
mod gl_sync_meta;
mod gl_video_frame;
pub use crate::gl_sync_meta::*;
mod gl_base_memory;
pub use self::gl_base_memory::*;
mod gl_memory;
pub use crate::gl_memory::*;
mod gl_framebuffer;
mod gl_memory_pbo;
pub use crate::gl_memory_pbo::*;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_gl::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst_video::prelude::*;

    pub use crate::{
        auto::traits::*, context::ContextGLExt, gl_context::GLContextExtManual,
        gl_display::GLDisplayExtManual, gl_framebuffer::GLFramebufferExtManual,
        gl_video_frame::VideoFrameGLExt,
    };
}

pub mod subclass;
