// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};
use libc::uintptr_t;

use crate::VideoOverlay;

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::VideoOverlay>> Sealed for T {}
}

pub trait VideoOverlayExtManual: sealed::Sealed + IsA<VideoOverlay> + 'static {
    unsafe fn set_window_handle(&self, handle: uintptr_t) {
        ffi::gst_video_overlay_set_window_handle(self.as_ref().to_glib_none().0, handle)
    }

    unsafe fn got_window_handle(&self, handle: uintptr_t) {
        ffi::gst_video_overlay_got_window_handle(self.as_ref().to_glib_none().0, handle)
    }
}

impl<O: IsA<VideoOverlay>> VideoOverlayExtManual for O {}

#[doc(alias = "gst_is_video_overlay_prepare_window_handle_message")]
pub fn is_video_overlay_prepare_window_handle_message(msg: &gst::MessageRef) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gst_is_video_overlay_prepare_window_handle_message(
            msg.as_mut_ptr(),
        ))
    }
}
