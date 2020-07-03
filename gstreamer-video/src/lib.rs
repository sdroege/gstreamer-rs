// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate once_cell;

#[macro_use]
extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;
#[macro_use]
extern crate gstreamer as gst;
extern crate futures_channel;
extern crate futures_util;
extern crate gstreamer_base as gst_base;
extern crate gstreamer_base_sys as gst_base_sys;
extern crate gstreamer_sys as gst_sys;
extern crate gstreamer_video_sys as gst_video_sys;

#[cfg(test)]
extern crate itertools;

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

mod caps_features;
#[cfg(any(feature = "v1_16", feature = "dox"))]
pub use caps_features::{CAPS_FEATURES_FORMAT_INTERLACED, CAPS_FEATURE_FORMAT_INTERLACED};
pub use caps_features::{
    CAPS_FEATURES_META_GST_VIDEO_AFFINE_TRANSFORMATION_META,
    CAPS_FEATURES_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META, CAPS_FEATURES_META_GST_VIDEO_META,
    CAPS_FEATURES_META_GST_VIDEO_OVERLAY_COMPOSITION,
    CAPS_FEATURE_META_GST_VIDEO_AFFINE_TRANSFORMATION_META,
    CAPS_FEATURE_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META, CAPS_FEATURE_META_GST_VIDEO_META,
    CAPS_FEATURE_META_GST_VIDEO_OVERLAY_COMPOSITION,
};
mod video_format;
pub use video_format::*;
mod video_format_info;
pub use video_format_info::*;
mod video_info;
pub use video_info::*;
pub mod video_frame;
pub use video_frame::{VideoBufferExt, VideoFrame, VideoFrameRef};
mod video_overlay;
pub use video_overlay::{is_video_overlay_prepare_window_handle_message, VideoOverlayExtManual};
pub mod video_event;
pub use video_event::{
    DownstreamForceKeyUnitEvent, ForceKeyUnitEvent, StillFrameEvent, UpstreamForceKeyUnitEvent,
};
mod functions;
pub use functions::*;
mod video_rectangle;
pub use video_rectangle::*;
mod video_overlay_composition;
pub use video_overlay_composition::*;
pub mod video_meta;
#[cfg(any(feature = "v1_16", feature = "dox"))]
pub use video_meta::VideoCaptionMeta;
#[cfg(any(feature = "v1_18", feature = "dox"))]
pub use video_meta::{VideoAFDMeta, VideoBarMeta};
pub use video_meta::{
    VideoAffineTransformationMeta, VideoCropMeta, VideoMeta, VideoOverlayCompositionMeta,
    VideoRegionOfInterestMeta,
};
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
    VideoAlignment, VideoBufferPoolConfig, BUFFER_POOL_OPTION_VIDEO_AFFINE_TRANSFORMATION_META,
    BUFFER_POOL_OPTION_VIDEO_ALIGNMENT, BUFFER_POOL_OPTION_VIDEO_GL_TEXTURE_UPLOAD_META,
    BUFFER_POOL_OPTION_VIDEO_META,
};
pub mod video_converter;
pub use video_converter::{VideoConverter, VideoConverterConfig};

mod video_codec_frame;
mod video_decoder;
pub use video_decoder::VideoDecoderExtManual;
mod video_encoder;
pub use video_codec_frame::VideoCodecFrame;
pub use video_encoder::VideoEncoderExtManual;
pub mod video_codec_state;
pub use video_codec_state::{VideoCodecState, VideoCodecStateContext};
mod utils;

pub const VIDEO_ENCODER_FLOW_NEED_DATA: gst::FlowSuccess = gst::FlowSuccess::CustomSuccess;
pub const VIDEO_DECODER_FLOW_NEED_DATA: gst::FlowSuccess = gst::FlowSuccess::CustomSuccess;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;
    pub use gst::prelude::*;

    pub use auto::traits::*;
    pub use video_buffer_pool::VideoBufferPoolConfig;
    pub use video_decoder::VideoDecoderExtManual;
    pub use video_encoder::VideoEncoderExtManual;
    pub use video_format::VideoFormatIteratorExt;
    pub use video_frame::VideoBufferExt;
    pub use video_overlay::VideoOverlayExtManual;
}

pub mod subclass;
