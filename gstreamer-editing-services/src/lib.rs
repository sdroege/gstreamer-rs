// Copyright (C) 2018 Thibault Saunier <tsaunier@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate libc;

use std::sync::Once;

extern crate gio_sys;
extern crate glib_sys;
extern crate gobject_sys;
extern crate gstreamer as gst;
extern crate gstreamer_base as gst_base;
extern crate gstreamer_editing_services_sys as ges_sys;
extern crate gstreamer_pbutils as gst_pbutils;
extern crate gstreamer_sys as gst_sys;

use glib::translate::from_glib;

#[macro_use]
extern crate glib;
extern crate gio;

static GES_INIT: Once = Once::new();

pub fn init() -> Result<(), glib::BoolError> {
    if gst::init().is_err() {
        return Err(glib_bool_error!("Could not initialize GStreamer."));
    }

    unsafe {
        if from_glib(ges_sys::ges_init()) {
            Ok(())
        } else {
            Err(glib_bool_error!("Could not initialize GES."))
        }
    }
}

pub unsafe fn deinit() {
    ges_sys::ges_deinit();
}

macro_rules! assert_initialized_main_thread {
    () => {
        if unsafe { ::gst_sys::gst_is_initialized() } != ::glib_sys::GTRUE {
            panic!("GStreamer has not been initialized. Call `gst::init` first.");
        }
        ::GES_INIT.call_once(|| {
            unsafe { ::ges_sys::ges_init() };
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
pub use auto::*;

#[macro_use]
extern crate bitflags;

mod timeline_element;
pub use timeline_element::TimelineElementExtManual;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;
    pub use gst::prelude::*;
    pub use timeline_element::TimelineElementExtManual;

    pub use auto::traits::*;
}
