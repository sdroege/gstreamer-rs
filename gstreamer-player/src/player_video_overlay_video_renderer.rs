// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use glib::IsA;
use gst;
use gst_player_sys;
use PlayerVideoOverlayVideoRenderer;

use std::ptr;

use libc::uintptr_t;

impl PlayerVideoOverlayVideoRenderer {
    pub unsafe fn new(window_handle: uintptr_t) -> PlayerVideoOverlayVideoRenderer {
        assert_initialized_main_thread!();

        from_glib_full(gst_player_sys::gst_player_video_overlay_video_renderer_new(
            window_handle as *mut _,
        ) as *mut _)
    }

    pub unsafe fn with_handle_and_sink<P: IsA<gst::Element>>(
        window_handle: uintptr_t,
        video_sink: &P,
    ) -> PlayerVideoOverlayVideoRenderer {
        assert_initialized_main_thread!();

        from_glib_full(
            gst_player_sys::gst_player_video_overlay_video_renderer_new_with_sink(
                window_handle as *mut _,
                video_sink.as_ref().to_glib_none().0,
            ) as *mut _,
        )
    }

    pub fn with_sink<P: IsA<gst::Element>>(video_sink: &P) -> PlayerVideoOverlayVideoRenderer {
        assert_initialized_main_thread!();

        unsafe {
            from_glib_full(
                gst_player_sys::gst_player_video_overlay_video_renderer_new_with_sink(
                    ptr::null_mut(),
                    video_sink.as_ref().to_glib_none().0,
                ) as *mut _,
            )
        }
    }

    pub unsafe fn get_window_handle(&self) -> uintptr_t {
        gst_player_sys::gst_player_video_overlay_video_renderer_get_window_handle(
            self.to_glib_none().0,
        ) as uintptr_t
    }

    pub unsafe fn set_window_handle(&self, window_handle: uintptr_t) {
        gst_player_sys::gst_player_video_overlay_video_renderer_set_window_handle(
            self.to_glib_none().0,
            window_handle as *mut _,
        )
    }
}
