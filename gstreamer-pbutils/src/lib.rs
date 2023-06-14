// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![doc = include_str!("../README.md")]

use std::sync::Once;

pub use ffi;
pub use glib;
pub use gst;

static PBUTILS_INIT: Once = Once::new();

macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            gst::assert_initialized();
        }
        crate::PBUTILS_INIT.call_once(|| {
            unsafe { ffi::gst_pb_utils_init() };
        });
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

mod auto;
pub use crate::auto::{functions::*, *};

#[cfg(any(feature = "v1_20", feature = "dox"))]
mod element_properties;
#[cfg(any(feature = "v1_20", feature = "dox"))]
pub use crate::element_properties::{ElementProperties, ElementPropertiesMapItem};

#[cfg(feature = "serde")]
mod flag_serde;

mod discoverer;
pub use crate::discoverer::*;

mod discoverer_audio_info;
mod discoverer_container_info;
pub mod discoverer_stream_info;
mod discoverer_subtitle_info;
mod discoverer_video_info;

pub mod encoding_profile;

pub mod functions;
pub use crate::functions::*;

pub mod subclass;

pub mod audio_visualizer;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_pbutils::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst::prelude::*;

    pub use crate::{
        audio_visualizer::*,
        auto::traits::*,
        discoverer_stream_info::DiscovererStreamInfoExtManual,
        encoding_profile::{
            EncodingProfileBuilder, EncodingProfileExtManual, EncodingProfileHasRestrictionGetter,
        },
        functions::CodecTag,
    };
}
