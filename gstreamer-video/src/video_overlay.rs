// Take a look at the license at the top of the repository in the LICENSE file.

use crate::VideoOverlay;
use libc::uintptr_t;

use glib::prelude::*;
use glib::translate::*;

pub trait VideoOverlayExtManual: 'static {
    unsafe fn set_window_handle(&self, handle: uintptr_t);
    unsafe fn got_window_handle(&self, handle: uintptr_t);
}

impl<O: IsA<VideoOverlay>> VideoOverlayExtManual for O {
    unsafe fn set_window_handle(&self, handle: uintptr_t) {
        ffi::gst_video_overlay_set_window_handle(self.as_ref().to_glib_none().0, handle)
    }

    unsafe fn got_window_handle(&self, handle: uintptr_t) {
        ffi::gst_video_overlay_got_window_handle(self.as_ref().to_glib_none().0, handle)
    }
}

pub fn is_video_overlay_prepare_window_handle_message(msg: &gst::MessageRef) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gst_is_video_overlay_prepare_window_handle_message(
            msg.as_mut_ptr(),
        ))
    }
}
