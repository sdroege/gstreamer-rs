// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;
pub use gio;
pub use glib;
pub use gst;
pub use gst_base;
pub use gst_pbutils;

use std::sync::Once;

use glib::translate::from_glib;

static GES_INIT: Once = Once::new();

#[doc(alias = "ges_init")]
pub fn init() -> Result<(), glib::BoolError> {
    if gst::init().is_err() {
        return Err(glib::bool_error!("Could not initialize GStreamer."));
    }

    unsafe {
        if from_glib(ffi::ges_init()) {
            Ok(())
        } else {
            Err(glib::bool_error!("Could not initialize GES."))
        }
    }
}

pub unsafe fn deinit() {
    ffi::ges_deinit();
}

macro_rules! assert_initialized_main_thread {
    () => {
        if unsafe { gst::ffi::gst_is_initialized() } != glib::ffi::GTRUE {
            panic!("GStreamer has not been initialized. Call `gst::init` first.");
        }
        crate::GES_INIT.call_once(|| {
            unsafe { ffi::ges_init() };
        });
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::unreadable_literal)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::match_same_arms)]
#[allow(clippy::use_self)]
mod auto;
pub use crate::auto::*;

mod timeline_element;

// Re-export all the traits in a prelude module, so that applications
// can always "use ges::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gio::prelude::*;
    #[doc(hidden)]
    pub use gst_base::prelude::*;
    #[doc(hidden)]
    pub use gst_pbutils::prelude::*;

    pub use crate::timeline_element::TimelineElementExtManual;

    pub use crate::auto::traits::*;
}
