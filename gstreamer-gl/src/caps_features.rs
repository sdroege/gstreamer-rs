// Take a look at the license at the top of the repository in the LICENSE file.

use glib::once_cell::sync::Lazy;
use gst::CapsFeatures;

pub static CAPS_FEATURES_MEMORY_GL_MEMORY: Lazy<CapsFeatures> =
    Lazy::new(|| CapsFeatures::new([crate::CAPS_FEATURE_MEMORY_GL_MEMORY]));
