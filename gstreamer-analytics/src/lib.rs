// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]
#![doc = include_str!("../README.md")]

pub use glib;
pub use gst;
pub use gstreamer_analytics_sys as ffi;

#[cfg(feature = "v1_28")]
macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            gst::assert_initialized();
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(unused_imports)]
mod auto;
pub use crate::auto::*;

#[cfg(feature = "v1_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
mod tensor;
#[cfg(feature = "v1_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
pub use crate::tensor::*;

#[cfg(feature = "v1_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
mod tensor_meta;
#[cfg(feature = "v1_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
pub use crate::tensor_meta::*;

mod relation_meta;
pub use crate::relation_meta::*;

mod object_detection;
pub use crate::object_detection::*;

mod tracking;
pub use crate::tracking::*;

mod classification;
pub use crate::classification::*;

#[cfg(feature = "v1_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
mod batchmeta;
#[cfg(feature = "v1_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
pub use crate::batchmeta::*;

#[cfg(feature = "v1_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
mod model_info;

mod enums;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_app::prelude::*" without getting conflicts
pub mod prelude {
    pub use crate::classification::AnalyticsRelationMetaClassificationExt;
    pub use crate::object_detection::AnalyticsRelationMetaODExt;
    pub use crate::tracking::AnalyticsRelationMetaTrackingExt;
}
