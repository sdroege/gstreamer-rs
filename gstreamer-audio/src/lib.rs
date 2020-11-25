// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
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
#[allow(unused_imports)]
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
#[cfg(any(feature = "v1_14", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_14")))]
mod audio_stream_align;
mod functions;
pub use crate::functions::*;
#[cfg(any(feature = "v1_16", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_16")))]
pub mod audio_buffer;
#[cfg(any(feature = "v1_16", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_16")))]
pub use audio_buffer::{AudioBuffer, AudioBufferRef};

mod audio_decoder;
pub use crate::audio_decoder::AudioDecoderExtManual;
mod audio_encoder;
pub use crate::audio_encoder::AudioEncoderExtManual;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;
    pub use gst::prelude::*;

    pub use super::audio_decoder::AudioDecoderExtManual;
    pub use super::audio_encoder::AudioEncoderExtManual;
    pub use crate::audio_format::AudioFormatIteratorExt;
    pub use crate::auto::traits::*;
}

pub mod subclass;
