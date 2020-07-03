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

use once_cell::sync::Lazy;

#[cfg(any(feature = "v1_16", feature = "dox"))]
pub static CAPS_FEATURE_FORMAT_INTERLACED: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(gst_video_sys::GST_CAPS_FEATURE_FORMAT_INTERLACED)
        .to_str()
        .unwrap()
});
#[cfg(any(feature = "v1_16", feature = "dox"))]
pub static CAPS_FEATURES_FORMAT_INTERLACED: Lazy<CapsFeatures> =
    Lazy::new(|| CapsFeatures::new(&[*CAPS_FEATURE_FORMAT_INTERLACED]));

pub static CAPS_FEATURE_META_GST_VIDEO_AFFINE_TRANSFORMATION_META: Lazy<&'static str> =
    Lazy::new(|| unsafe {
        CStr::from_ptr(gst_video_sys::GST_CAPS_FEATURE_META_GST_VIDEO_AFFINE_TRANSFORMATION_META)
            .to_str()
            .unwrap()
    });
pub static CAPS_FEATURES_META_GST_VIDEO_AFFINE_TRANSFORMATION_META: Lazy<CapsFeatures> =
    Lazy::new(|| CapsFeatures::new(&[*CAPS_FEATURE_META_GST_VIDEO_AFFINE_TRANSFORMATION_META]));

pub static CAPS_FEATURE_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META: Lazy<&'static str> =
    Lazy::new(|| unsafe {
        CStr::from_ptr(gst_video_sys::GST_CAPS_FEATURE_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META)
            .to_str()
            .unwrap()
    });
pub static CAPS_FEATURES_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META: Lazy<CapsFeatures> =
    Lazy::new(|| CapsFeatures::new(&[*CAPS_FEATURE_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META]));

pub static CAPS_FEATURE_META_GST_VIDEO_META: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(gst_video_sys::GST_CAPS_FEATURE_META_GST_VIDEO_META)
        .to_str()
        .unwrap()
});
pub static CAPS_FEATURES_META_GST_VIDEO_META: Lazy<CapsFeatures> =
    Lazy::new(|| CapsFeatures::new(&[*CAPS_FEATURE_META_GST_VIDEO_META]));

pub static CAPS_FEATURE_META_GST_VIDEO_OVERLAY_COMPOSITION: Lazy<&'static str> =
    Lazy::new(|| unsafe {
        CStr::from_ptr(gst_video_sys::GST_CAPS_FEATURE_META_GST_VIDEO_OVERLAY_COMPOSITION)
            .to_str()
            .unwrap()
    });
pub static CAPS_FEATURES_META_GST_VIDEO_OVERLAY_COMPOSITION: Lazy<CapsFeatures> =
    Lazy::new(|| CapsFeatures::new(&[*CAPS_FEATURE_META_GST_VIDEO_OVERLAY_COMPOSITION]));
