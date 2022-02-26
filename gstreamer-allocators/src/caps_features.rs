// Take a look at the license at the top of the repository in the LICENSE file.

use gst::CapsFeatures;

use once_cell::sync::Lazy;

pub static CAPS_FEATURES_MEMORY_DMABUF: Lazy<CapsFeatures> =
    Lazy::new(|| CapsFeatures::new(&[*crate::CAPS_FEATURE_MEMORY_DMABUF]));
