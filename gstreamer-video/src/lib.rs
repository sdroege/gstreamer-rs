// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
extern crate libc;

#[macro_use]
extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;
#[macro_use]
extern crate gstreamer as gst;
extern crate gstreamer_base as gst_base;
extern crate gstreamer_base_sys as gst_base_sys;
extern crate gstreamer_sys as gst_sys;
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

pub use glib::{Cast, Continue, Error, IsA, StaticType, ToValue, Type, TypedValue, Value};

#[allow(clippy::unreadable_literal)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::match_same_arms)]
#[rustfmt::skip]
mod auto;
pub use auto::*;

mod video_format;
pub use video_format::*;
mod video_format_info;
pub use video_format_info::*;
mod video_info;
pub use video_info::*;
pub mod video_frame;
pub use video_frame::{VideoFrame, VideoFrameRef};
mod video_overlay;
pub use video_overlay::*;
mod video_event;
pub use video_event::*;
mod functions;
pub use functions::*;
mod video_rectangle;
pub use video_rectangle::*;
mod video_overlay_composition;
pub use video_overlay_composition::*;
mod video_meta;
pub use video_meta::*;
#[cfg(any(feature = "v1_10", feature = "dox"))]
mod video_time_code;
#[cfg(any(feature = "v1_10", feature = "dox"))]
pub use video_time_code::{ValidVideoTimeCode, VideoTimeCode, VideoTimeCodeMeta};
#[cfg(any(feature = "v1_12", feature = "dox"))]
mod video_time_code_interval;
#[cfg(any(feature = "v1_12", feature = "dox"))]
pub use video_time_code_interval::VideoTimeCodeInterval;
mod video_buffer_pool;
pub use video_buffer_pool::{
    BUFFER_POOL_OPTION_VIDEO_AFFINE_TRANSFORMATION_META, BUFFER_POOL_OPTION_VIDEO_ALIGNMENT,
    BUFFER_POOL_OPTION_VIDEO_GL_TEXTURE_UPLOAD_META, BUFFER_POOL_OPTION_VIDEO_META,
};

mod video_codec_frame;
mod video_decoder;
pub use video_codec_frame::VideoCodecFrame;
pub mod video_codec_state;
pub use video_codec_state::VideoCodecState;
mod utils;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;
    pub use gst::prelude::*;

    pub use auto::traits::*;
    pub use video_decoder::VideoDecoderExtManual;
    pub use video_overlay::VideoOverlayExtManual;
}

#[cfg(feature = "subclassing")]
pub mod subclass;
