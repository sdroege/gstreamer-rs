// Copyright (C) 2018 Thibault Saunier <tsaunier@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(all(not(doctest), doc), feature(doc_cfg))]

pub use ffi;

use std::sync::Once;

use glib::translate::from_glib;

static GES_INIT: Once = Once::new();

pub fn init() -> Result<(), glib::BoolError> {
    if gst::init().is_err() {
        return Err(glib::glib_bool_error!("Could not initialize GStreamer."));
    }

    unsafe {
        if from_glib(ffi::ges_init()) {
            Ok(())
        } else {
            Err(glib::glib_bool_error!("Could not initialize GES."))
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
#[allow(unused_imports)]
mod auto;
pub use crate::auto::*;

mod timeline_element;
pub use crate::timeline_element::TimelineElementExtManual;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use crate::timeline_element::TimelineElementExtManual;
    pub use glib::prelude::*;
    pub use gst::prelude::*;

    pub use crate::auto::traits::*;
}
