// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Player;
use PlayerSignalDispatcher;
use PlayerVideoRenderer;
use ffi;
use glib::translate::*;
use gst;

impl Player {
    pub fn new(
        video_renderer: Option<&PlayerVideoRenderer>,
        signal_dispatcher: Option<&PlayerSignalDispatcher>,
    ) -> Player {
        assert_initialized_main_thread!();
        let (major, minor, _, _) = gst::version();
        if (major, minor) > (1, 12) {
            let video_renderer = video_renderer.to_glib_full();
            let signal_dispatcher = signal_dispatcher.to_glib_full();
            unsafe { from_glib_full(ffi::gst_player_new(video_renderer, signal_dispatcher)) }
        } else {
            // Workaround for bad floating reference handling in 1.12. This issue was fixed for 1.13 in
            // https://cgit.freedesktop.org/gstreamer/gst-plugins-bad/commit/gst-libs/gst/player/gstplayer.c?id=634cd87c76f58b5e1383715bafd5614db825c7d1
            let video_renderer = video_renderer.to_glib_none();
            let signal_dispatcher = signal_dispatcher.to_glib_none();
            unsafe { from_glib_none(ffi::gst_player_new(video_renderer.0, signal_dispatcher.0)) }
        }

    }

    #[allow(dead_code)]
    pub fn set_config(&self, config: gst::Structure) -> bool {
        unsafe {
            from_glib(ffi::gst_player_set_config(
                self.to_glib_none().0,
                config.into_ptr(),
            ))
        }
    }
}
