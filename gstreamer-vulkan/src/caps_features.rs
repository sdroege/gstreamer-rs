// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ffi;
use gst::CapsFeatures;
use once_cell::sync::Lazy;

pub static CAPS_FEATURE_MEMORY_VULKAN_IMAGE: &glib::GStr =
    unsafe { glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_CAPS_FEATURE_MEMORY_VULKAN_IMAGE) };
pub static CAPS_FEATURES_MEMORY_VULKAN_IMAGE: Lazy<CapsFeatures> =
    Lazy::new(|| CapsFeatures::new([CAPS_FEATURE_MEMORY_VULKAN_IMAGE]));
