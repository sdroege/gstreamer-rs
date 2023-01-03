// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CStr;

use gst::CapsFeatures;
use once_cell::sync::Lazy;

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
pub static CAPS_FEATURE_FORMAT_INTERLACED: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_CAPS_FEATURE_FORMAT_INTERLACED)
        .to_str()
        .unwrap()
});
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
pub static CAPS_FEATURES_FORMAT_INTERLACED: Lazy<CapsFeatures> =
    Lazy::new(|| CapsFeatures::new(&[*CAPS_FEATURE_FORMAT_INTERLACED]));

pub static CAPS_FEATURE_META_GST_VIDEO_AFFINE_TRANSFORMATION_META: Lazy<&'static str> =
    Lazy::new(|| unsafe {
        CStr::from_ptr(ffi::GST_CAPS_FEATURE_META_GST_VIDEO_AFFINE_TRANSFORMATION_META)
            .to_str()
            .unwrap()
    });
pub static CAPS_FEATURES_META_GST_VIDEO_AFFINE_TRANSFORMATION_META: Lazy<CapsFeatures> =
    Lazy::new(|| CapsFeatures::new(&[*CAPS_FEATURE_META_GST_VIDEO_AFFINE_TRANSFORMATION_META]));

pub static CAPS_FEATURE_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META: Lazy<&'static str> =
    Lazy::new(|| unsafe {
        CStr::from_ptr(ffi::GST_CAPS_FEATURE_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META)
            .to_str()
            .unwrap()
    });
pub static CAPS_FEATURES_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META: Lazy<CapsFeatures> =
    Lazy::new(|| CapsFeatures::new(&[*CAPS_FEATURE_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META]));

pub static CAPS_FEATURE_META_GST_VIDEO_META: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_CAPS_FEATURE_META_GST_VIDEO_META)
        .to_str()
        .unwrap()
});
pub static CAPS_FEATURES_META_GST_VIDEO_META: Lazy<CapsFeatures> =
    Lazy::new(|| CapsFeatures::new(&[*CAPS_FEATURE_META_GST_VIDEO_META]));

pub static CAPS_FEATURE_META_GST_VIDEO_OVERLAY_COMPOSITION: Lazy<&'static str> =
    Lazy::new(|| unsafe {
        CStr::from_ptr(ffi::GST_CAPS_FEATURE_META_GST_VIDEO_OVERLAY_COMPOSITION)
            .to_str()
            .unwrap()
    });
pub static CAPS_FEATURES_META_GST_VIDEO_OVERLAY_COMPOSITION: Lazy<CapsFeatures> =
    Lazy::new(|| CapsFeatures::new(&[*CAPS_FEATURE_META_GST_VIDEO_OVERLAY_COMPOSITION]));
