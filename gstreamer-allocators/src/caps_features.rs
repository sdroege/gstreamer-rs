// Take a look at the license at the top of the repository in the LICENSE file.

use gst::CapsFeatures;
use std::sync::LazyLock;

pub static CAPS_FEATURES_MEMORY_DMABUF: LazyLock<CapsFeatures> =
    LazyLock::new(|| CapsFeatures::new([crate::CAPS_FEATURE_MEMORY_DMABUF]));
