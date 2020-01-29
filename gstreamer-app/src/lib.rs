// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate libc;

extern crate futures_core;
extern crate futures_sink;
extern crate glib_sys;
extern crate gobject_sys;
extern crate gstreamer as gst;
extern crate gstreamer_app_sys as gst_app_sys;
extern crate gstreamer_base as gst_base;
extern crate gstreamer_sys as gst_sys;

#[macro_use]
extern crate glib;

#[cfg(test)]
extern crate futures_util;

#[cfg(test)]
extern crate futures_executor;

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::unreadable_literal)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::match_same_arms)]
mod auto;
pub use auto::*;

pub mod app_sink;
pub use app_sink::AppSinkCallbacks;

pub mod app_src;
pub use app_src::AppSrcCallbacks;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;
    pub use gst::prelude::*;

    pub use auto::traits::*;
}
