// Take a look at the license at the top of the repository in the LICENSE file.

use crate::PlayerVideoInfo;
use glib::translate::*;
use std::mem;

impl PlayerVideoInfo {
    #[doc(alias = "get_framerate")]
    #[doc(alias = "gst_player_video_info_get_framerate")]
    pub fn framerate(&self) -> gst::Fraction {
        unsafe {
            let mut fps_n = mem::MaybeUninit::uninit();
            let mut fps_d = mem::MaybeUninit::uninit();
            ffi::gst_player_video_info_get_framerate(
                self.to_glib_none().0,
                fps_n.as_mut_ptr(),
                fps_d.as_mut_ptr(),
            );
            (fps_n.assume_init() as i32, fps_d.as_mut_ptr() as i32).into()
        }
    }

    #[doc(alias = "get_pixel_aspect_ratio")]
    #[doc(alias = "gst_player_video_info_get_pixel_aspect_ratio")]
    pub fn pixel_aspect_ratio(&self) -> gst::Fraction {
        unsafe {
            let mut par_n = mem::MaybeUninit::uninit();
            let mut par_d = mem::MaybeUninit::uninit();
            ffi::gst_player_video_info_get_pixel_aspect_ratio(
                self.to_glib_none().0,
                par_n.as_mut_ptr(),
                par_d.as_mut_ptr(),
            );
            (par_n.assume_init() as i32, par_d.assume_init() as i32).into()
        }
    }
}
