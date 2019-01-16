// Copyright (C) 2018 Thibault Saunier <tsaunier@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate libc;

use std::sync::{Once, ONCE_INIT};

extern crate gio_sys as gio_ffi;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gstreamer as gst;
extern crate gstreamer_base as gst_base;
extern crate gstreamer_base_sys as gst_base_ffi;
extern crate gstreamer_editing_services_sys as ffi;
extern crate gstreamer_pbutils as gst_pbutils;
extern crate gstreamer_pbutils_sys as gst_pbutils_ffi;
extern crate gstreamer_sys as gst_ffi;

use glib::translate::from_glib;

#[macro_use]
extern crate glib;
extern crate gio;

static GES_INIT: Once = ONCE_INIT;

pub use glib::{
    BoolError, Cast, Continue, Error, IsA, StaticType, ToValue, Type, TypedValue, Value,
};

pub fn init() -> Result<(), BoolError> {
    if gst::init().is_err() {
        return Err(glib_bool_error!("Could not initialize GStreamer."));
    }

    unsafe {
        if from_glib(ffi::ges_init()) {
            Ok(())
        } else {
            Err(glib_bool_error!("Could not initialize GES."))
        }
    }
}

pub unsafe fn deinit() {
    ffi::ges_deinit();
}

macro_rules! assert_initialized_main_thread {
    () => {
        if unsafe { ::gst_ffi::gst_is_initialized() } != ::glib_ffi::GTRUE {
            panic!("GStreamer has not been initialized. Call `gst::init` first.");
        }
        ::GES_INIT.call_once(|| {
            unsafe { ::ffi::ges_init() };
        });
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
#[rustfmt::skip]
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
