// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use gst;
use gst_video_sys;
use libc::uintptr_t;
use VideoOverlay;

use glib::IsA;

pub trait VideoOverlayExtManual: 'static {
    unsafe fn set_window_handle(&self, handle: uintptr_t);
    unsafe fn got_window_handle(&self, handle: uintptr_t);
}

impl<O: IsA<VideoOverlay>> VideoOverlayExtManual for O {
    unsafe fn set_window_handle(&self, handle: uintptr_t) {
        gst_video_sys::gst_video_overlay_set_window_handle(self.as_ref().to_glib_none().0, handle)
    }

    unsafe fn got_window_handle(&self, handle: uintptr_t) {
        gst_video_sys::gst_video_overlay_got_window_handle(self.as_ref().to_glib_none().0, handle)
    }
}

pub fn is_video_overlay_prepare_window_handle_message(msg: &gst::MessageRef) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(
            gst_video_sys::gst_is_video_overlay_prepare_window_handle_message(msg.as_mut_ptr()),
        )
    }
}
