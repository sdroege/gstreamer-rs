// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::non_send_fields_in_send_ty)]
#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub static INITIALIZED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            #[allow(unused_unsafe)]
            if unsafe { gst::ffi::gst_is_initialized() } != glib::ffi::GTRUE {
                panic!("GStreamer has not been initialized. Call `gst::init` first.");
            } else {
                gst::INITIALIZED.store(true, std::sync::atomic::Ordering::SeqCst);
            }
        }

        if !crate::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            if unsafe { ffi::gst_validate_is_initialized() } != glib::ffi::GTRUE {
                panic!(
                    "GStreamer Validate has not been initialized. Call `gst_validate::init` first."
                );
            } else {
                crate::INITIALIZED.store(true, std::sync::atomic::Ordering::SeqCst);
            }
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::unreadable_literal)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::match_same_arms)]
#[allow(non_snake_case)]
#[allow(clippy::use_self)]
#[allow(unused_imports)]
mod auto;
pub use crate::auto::*;

#[allow(dead_code)]
mod functions;
pub use functions::*;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_validate::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst::prelude::*;

    pub use crate::auto::traits::*;
}
