// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;
pub use glib;
pub use gst;
pub use gst_base;

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
#[allow(clippy::use_self)]
#[allow(clippy::needless_borrow)]
mod auto;
pub use crate::auto::*;

mod audio_format;
pub use crate::audio_format::*;
mod audio_format_info;
pub use crate::audio_format_info::*;
mod audio_ring_buffer_spec;
pub use crate::audio_ring_buffer_spec::*;
mod audio_info;
pub use crate::audio_info::*;
mod audio_meta;
pub use crate::audio_meta::*;
mod audio_channel_position;
pub use crate::audio_channel_position::*;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
mod audio_stream_align;
mod functions;
pub use crate::functions::*;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
pub mod audio_buffer;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
pub use audio_buffer::{AudioBuffer, AudioBufferRef};

mod audio_decoder;
mod audio_encoder;

mod audio_converter;
pub use crate::audio_converter::AudioConverterConfig;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_audio::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst_base::prelude::*;

    pub use super::audio_decoder::AudioDecoderExtManual;
    pub use super::audio_encoder::AudioEncoderExtManual;
    pub use crate::audio_format::AudioFormatIteratorExt;
    pub use crate::auto::traits::*;
}

pub mod subclass;
