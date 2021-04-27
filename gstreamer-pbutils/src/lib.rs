// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;
pub use glib;
pub use gst;

use std::sync::Once;

static PBUTILS_INIT: Once = Once::new();

macro_rules! assert_initialized_main_thread {
    () => {
        if unsafe { gst::ffi::gst_is_initialized() } != glib::ffi::GTRUE {
            panic!("GStreamer has not been initialized. Call `gst::init` first.");
        }
        crate::PBUTILS_INIT.call_once(|| {
            unsafe { ffi::gst_pb_utils_init() };
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
pub use crate::auto::functions::*;
pub use crate::auto::*;

mod discoverer;
pub use crate::discoverer::*;

pub mod discoverer_stream_info;

mod discoverer_video_info;
pub use crate::discoverer_video_info::*;

mod encoding_profile;
pub use crate::encoding_profile::*;

pub mod functions;
pub use crate::functions::*;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_pbutils::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst::prelude::*;

    pub use crate::auto::traits::*;
    pub use crate::encoding_profile::{
        EncodingProfileBuilder, EncodingProfileHasRestrictionGetter,
    };

    pub use crate::functions::CodecTag;
}
