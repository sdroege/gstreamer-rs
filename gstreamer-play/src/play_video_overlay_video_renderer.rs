// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use glib::{prelude::*, translate::*};
use libc::uintptr_t;

use crate::PlayVideoOverlayVideoRenderer;

impl PlayVideoOverlayVideoRenderer {
    pub unsafe fn new(window_handle: uintptr_t) -> PlayVideoOverlayVideoRenderer {
        assert_initialized_main_thread!();

        from_glib_full(
            ffi::gst_play_video_overlay_video_renderer_new(window_handle as *mut _) as *mut _,
        )
    }

    pub unsafe fn with_handle_and_sink<P: IsA<gst::Element>>(
        window_handle: uintptr_t,
        video_sink: &P,
    ) -> PlayVideoOverlayVideoRenderer {
        skip_assert_initialized!();

        from_glib_full(ffi::gst_play_video_overlay_video_renderer_new_with_sink(
            window_handle as *mut _,
            video_sink.as_ref().to_glib_none().0,
        ) as *mut _)
    }

    #[doc(alias = "gst_play_video_overlay_video_renderer_new_with_sink")]
    pub fn with_sink<P: IsA<gst::Element>>(video_sink: &P) -> PlayVideoOverlayVideoRenderer {
        skip_assert_initialized!();

        unsafe {
            from_glib_full(ffi::gst_play_video_overlay_video_renderer_new_with_sink(
                ptr::null_mut(),
                video_sink.as_ref().to_glib_none().0,
            ) as *mut _)
        }
    }

    #[doc(alias = "get_window_handle")]
    pub unsafe fn window_handle(&self) -> uintptr_t {
        ffi::gst_play_video_overlay_video_renderer_get_window_handle(self.to_glib_none().0)
            as uintptr_t
    }

    pub unsafe fn set_window_handle(&self, window_handle: uintptr_t) {
        ffi::gst_play_video_overlay_video_renderer_set_window_handle(
            self.to_glib_none().0,
            window_handle as *mut _,
        )
    }
}
