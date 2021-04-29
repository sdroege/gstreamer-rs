// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;
pub use glib;
pub use gst;
pub use gst_base;

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::unreadable_literal)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::match_same_arms)]
#[allow(clippy::use_self)]
mod auto;
pub use crate::auto::*;

pub mod app_sink;
pub use crate::app_sink::AppSinkCallbacks;

pub mod app_src;
pub use crate::app_src::AppSrcCallbacks;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_app::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst_base::prelude::*;

    pub use crate::auto::traits::*;
}
