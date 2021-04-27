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
mod auto;
pub use crate::auto::*;

mod caps_features;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
pub use crate::caps_features::{CAPS_FEATURES_FORMAT_INTERLACED, CAPS_FEATURE_FORMAT_INTERLACED};
pub use crate::caps_features::{
    CAPS_FEATURES_META_GST_VIDEO_AFFINE_TRANSFORMATION_META,
    CAPS_FEATURES_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META, CAPS_FEATURES_META_GST_VIDEO_META,
    CAPS_FEATURES_META_GST_VIDEO_OVERLAY_COMPOSITION,
    CAPS_FEATURE_META_GST_VIDEO_AFFINE_TRANSFORMATION_META,
    CAPS_FEATURE_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META, CAPS_FEATURE_META_GST_VIDEO_META,
    CAPS_FEATURE_META_GST_VIDEO_OVERLAY_COMPOSITION,
};
mod video_color_matrix;
pub use video_color_matrix::*;
mod video_format;
pub use crate::video_format::*;
mod video_format_info;
pub use crate::video_format_info::*;
mod video_info;
pub use crate::video_info::*;
pub mod video_frame;
pub use crate::video_frame::{VideoFrame, VideoFrameRef};
mod video_overlay;
pub use crate::video_overlay::is_video_overlay_prepare_window_handle_message;
pub mod video_event;
pub use crate::video_event::{
    DownstreamForceKeyUnitEvent, ForceKeyUnitEvent, StillFrameEvent, UpstreamForceKeyUnitEvent,
};
mod functions;
pub use crate::functions::*;
mod video_rectangle;
pub use crate::video_rectangle::*;
mod video_overlay_composition;
pub use crate::video_overlay_composition::*;
pub mod video_meta;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
pub use crate::video_meta::VideoCaptionMeta;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
pub use crate::video_meta::{VideoAFDMeta, VideoBarMeta};
pub use crate::video_meta::{
    VideoAffineTransformationMeta, VideoCropMeta, VideoMeta, VideoOverlayCompositionMeta,
    VideoRegionOfInterestMeta,
};
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
mod video_time_code;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
pub use crate::video_time_code::{ValidVideoTimeCode, VideoTimeCode, VideoTimeCodeMeta};
#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
mod video_time_code_interval;
#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
pub use crate::video_time_code_interval::VideoTimeCodeInterval;
mod video_buffer_pool;
pub use crate::video_buffer_pool::{
    VideoAlignment, VideoBufferPoolConfig, BUFFER_POOL_OPTION_VIDEO_AFFINE_TRANSFORMATION_META,
    BUFFER_POOL_OPTION_VIDEO_ALIGNMENT, BUFFER_POOL_OPTION_VIDEO_GL_TEXTURE_UPLOAD_META,
    BUFFER_POOL_OPTION_VIDEO_META,
};
pub mod video_converter;
pub use crate::video_converter::{VideoConverter, VideoConverterConfig};

mod video_codec_frame;
mod video_decoder;
mod video_encoder;
pub use crate::video_codec_frame::VideoCodecFrame;
pub mod video_codec_state;
pub use crate::video_codec_state::{VideoCodecState, VideoCodecStateContext};
mod utils;

pub const VIDEO_ENCODER_FLOW_NEED_DATA: gst::FlowSuccess = gst::FlowSuccess::CustomSuccess;
pub const VIDEO_DECODER_FLOW_NEED_DATA: gst::FlowSuccess = gst::FlowSuccess::CustomSuccess;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_video::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst_base::prelude::*;

    pub use crate::auto::traits::*;
    pub use crate::video_buffer_pool::VideoBufferPoolConfig;
    pub use crate::video_decoder::VideoDecoderExtManual;
    pub use crate::video_encoder::VideoEncoderExtManual;
    pub use crate::video_format::VideoFormatIteratorExt;
    pub use crate::video_frame::VideoBufferExt;
    pub use crate::video_overlay::VideoOverlayExtManual;
}

pub mod subclass;
