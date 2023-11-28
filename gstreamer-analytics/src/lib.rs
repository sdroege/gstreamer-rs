// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![doc = include_str!("../README.md")]

pub use ffi;
pub use glib;
pub use gst;

macro_rules! skip_assert_initialized {
    () => {};
}

mod auto;
pub use crate::auto::*;

mod relation_meta;
pub use crate::relation_meta::*;

mod object_detection;
pub use crate::object_detection::*;

mod tracking;
pub use crate::tracking::*;

mod classification;
pub use crate::classification::*;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_app::prelude::*" without getting conflicts
pub mod prelude {
    pub use crate::classification::AnalyticsRelationMetaClassificationExt;
    pub use crate::object_detection::AnalyticsRelationMetaODExt;
    pub use crate::tracking::AnalyticsRelationMetaTrackingExt;
}
