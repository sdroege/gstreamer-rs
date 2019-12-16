// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate array_init;
#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;
#[macro_use]
extern crate gstreamer as gst;
extern crate gstreamer_audio_sys as gst_audio_sys;
extern crate gstreamer_base as gst_base;
extern crate gstreamer_base_sys as gst_base_sys;
extern crate gstreamer_sys as gst_sys;

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

mod audio_format;
pub use audio_format::*;
mod audio_format_info;
pub use audio_format_info::*;
mod audio_ring_buffer_spec;
pub use audio_ring_buffer_spec::*;
mod audio_info;
pub use audio_info::*;
mod audio_channel_position;
pub use audio_channel_position::*;
#[cfg(any(feature = "v1_14", feature = "dox"))]
mod audio_stream_align;

mod audio_decoder;
pub use audio_decoder::AudioDecoderExtManual;
mod audio_encoder;
pub use audio_encoder::AudioEncoderExtManual;

use glib::translate::{from_glib_full, ToGlibPtr};
pub fn audio_buffer_clip(
    buffer: gst::Buffer,
    segment: &gst::Segment,
    rate: u32,
    bpf: u32,
) -> Option<gst::Buffer> {
    skip_assert_initialized!();

    unsafe {
        from_glib_full(gst_audio_sys::gst_audio_buffer_clip(
            buffer.into_ptr(),
            segment.to_glib_none().0,
            rate as i32,
            bpf as i32,
        ))
    }
}

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;
    pub use gst::prelude::*;

    pub use super::audio_decoder::AudioDecoderExtManual;
    pub use super::audio_encoder::AudioEncoderExtManual;
    pub use auto::traits::*;
}

pub mod subclass;
