// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![doc = include_str!("../README.md")]

use std::sync::Once;

pub use ffi;
pub use gio;
pub use glib;
use glib::translate::from_glib;
pub use gst;
pub use gst_base;
pub use gst_pbutils;

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
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            gst::assert_initialized();
        }
        crate::GES_INIT.call_once(|| {
            unsafe { ffi::ges_init() };
        });
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::needless_borrow)]
#[allow(deprecated)]
#[allow(unused_imports)]
mod auto;
mod formatter;
pub use crate::auto::*;
#[cfg(feature = "v1_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
mod composition_meta;
pub mod subclass;

#[cfg(feature = "serde")]
mod flag_serde;

// Re-export all the traits in a prelude module, so that applications
// can always "use ges::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use glib::prelude::*;

    #[doc(hidden)]
    pub use gio::prelude::*;

    #[doc(hidden)]
    pub use gst_base::prelude::*;
    #[doc(hidden)]
    pub use gst_pbutils::prelude::*;

    pub use crate::auto::traits::*;
    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    pub use crate::composition_meta::FrameCompositionMeta;
    pub use crate::formatter::FormatterExtManual;
}
