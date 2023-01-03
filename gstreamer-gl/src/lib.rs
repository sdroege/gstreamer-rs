// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::non_send_fields_in_send_ty)]
#![doc = include_str!("../README.md")]

pub use ffi;
pub use glib;
pub use gst;
pub use gst_base;
pub use gst_video;

macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            #[allow(unused_unsafe)]
            if unsafe { gst::ffi::gst_is_initialized() } != glib::ffi::GTRUE {
                panic!("GStreamer has not been initialized. Call `gst::init` first.");
            } else {
                gst::INITIALIZED.store(true, std::sync::atomic::Ordering::SeqCst);
            }
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::unreadable_literal)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::match_same_arms)]
#[allow(clippy::use_self)]
#[allow(unused_imports)]
mod auto;
pub use crate::auto::{functions::*, *};

#[cfg(feature = "serde")]
mod flag_serde;

mod caps_features;
pub use crate::caps_features::CAPS_FEATURES_MEMORY_GL_MEMORY;
mod context;
pub mod functions;
pub use crate::functions::*;
mod gl_context;
mod gl_sync_meta;
mod gl_video_frame;
pub use crate::gl_sync_meta::*;
mod gl_base_memory;
pub use self::gl_base_memory::*;
mod gl_memory;
pub use crate::gl_memory::*;
mod gl_memory_pbo;
pub use crate::gl_memory_pbo::*;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_gl::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst_video::prelude::*;

    pub use crate::{
        auto::traits::*, context::ContextGLExt, gl_context::GLContextExtManual,
        gl_video_frame::VideoFrameGLExt,
    };
}

pub mod subclass;
