// Copyright (C) 2020 Mathieu Duponchelle <mathieu@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst::CapsFeatures;
use gst_video_sys;
use std::ffi::CStr;

lazy_static! {
    pub static ref CAPS_FEATURE_FORMAT_INTERLACED: &'static str = unsafe {
        CStr::from_ptr(gst_video_sys::GST_CAPS_FEATURE_FORMAT_INTERLACED)
            .to_str()
            .unwrap()
    };
    pub static ref CAPS_FEATURES_FORMAT_INTERLACED: CapsFeatures =
        CapsFeatures::new(&[*CAPS_FEATURE_FORMAT_INTERLACED]);
    pub static ref CAPS_FEATURE_META_GST_VIDEO_AFFINE_TRANSFORMATION_META: &'static str = unsafe {
        CStr::from_ptr(gst_video_sys::GST_CAPS_FEATURE_META_GST_VIDEO_AFFINE_TRANSFORMATION_META)
            .to_str()
            .unwrap()
    };
    pub static ref CAPS_FEATURES_META_GST_VIDEO_AFFINE_TRANSFORMATION_META: CapsFeatures =
        CapsFeatures::new(&[*CAPS_FEATURE_META_GST_VIDEO_AFFINE_TRANSFORMATION_META]);
    pub static ref CAPS_FEATURE_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META: &'static str = unsafe {
        CStr::from_ptr(gst_video_sys::GST_CAPS_FEATURE_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META)
            .to_str()
            .unwrap()
    };
    pub static ref CAPS_FEATURES_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META: CapsFeatures =
        CapsFeatures::new(&[*CAPS_FEATURE_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META]);
    pub static ref CAPS_FEATURE_META_GST_VIDEO_META: &'static str = unsafe {
        CStr::from_ptr(gst_video_sys::GST_CAPS_FEATURE_META_GST_VIDEO_META)
            .to_str()
            .unwrap()
    };
    pub static ref CAPS_FEATURES_META_GST_VIDEO_META: CapsFeatures =
        CapsFeatures::new(&[*CAPS_FEATURE_META_GST_VIDEO_META]);
    pub static ref CAPS_FEATURE_META_GST_VIDEO_OVERLAY_COMPOSITION: &'static str = unsafe {
        CStr::from_ptr(gst_video_sys::GST_CAPS_FEATURE_META_GST_VIDEO_OVERLAY_COMPOSITION)
            .to_str()
            .unwrap()
    };
    pub static ref CAPS_FEATURES_META_GST_VIDEO_OVERLAY_COMPOSITION: CapsFeatures =
        CapsFeatures::new(&[*CAPS_FEATURE_META_GST_VIDEO_OVERLAY_COMPOSITION]);
}
