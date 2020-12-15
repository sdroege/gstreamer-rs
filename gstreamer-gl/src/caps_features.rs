// Take a look at the license at the top of the repository in the LICENSE file.

use gst::CapsFeatures;
use std::ffi::CStr;

use once_cell::sync::Lazy;

pub static CAPS_FEATURE_MEMORY_GL_MEMORY: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_CAPS_FEATURE_MEMORY_GL_MEMORY)
        .to_str()
        .unwrap()
});
pub static CAPS_FEATURES_MEMORY_GL_MEMORY: Lazy<CapsFeatures> =
    Lazy::new(|| CapsFeatures::new(&[*CAPS_FEATURE_MEMORY_GL_MEMORY]));
