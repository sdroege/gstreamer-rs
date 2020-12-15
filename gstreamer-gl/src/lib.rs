// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]

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
