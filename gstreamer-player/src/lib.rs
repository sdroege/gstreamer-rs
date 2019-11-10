// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate libc;

extern crate glib_sys;
extern crate gobject_sys;
extern crate gstreamer as gst;
extern crate gstreamer_player_sys as gst_player_sys;
extern crate gstreamer_sys as gst_sys;
extern crate gstreamer_video as gst_video;

#[macro_use]
extern crate glib;

macro_rules! skip_assert_initialized {
    () => {};
}

macro_rules! assert_initialized_main_thread {
    () => {};
}

#[allow(clippy::unreadable_literal)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::match_same_arms)]
#[allow(clippy::type_complexity)]
#[allow(clippy::cast_ptr_alignment)]
mod auto;
pub use auto::*;

mod config;
mod player;
pub use config::*;

mod player_video_info;

mod player_g_main_context_signal_dispatcher;
mod player_video_overlay_video_renderer;
mod player_visualization;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;
    pub use gst::prelude::*;

    pub use auto::traits::*;
}
