// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::DiscovererVideoInfo;

impl DiscovererVideoInfo {
    #[doc(alias = "get_framerate")]
    #[doc(alias = "gst_discoverer_video_info_get_framerate_num")]
    #[doc(alias = "gst_discoverer_video_info_get_framerate_denom")]
    pub fn framerate(&self) -> gst::Fraction {
        unsafe {
            gst::Fraction::new(
                ffi::gst_discoverer_video_info_get_framerate_num(self.to_glib_none().0) as i32,
                ffi::gst_discoverer_video_info_get_framerate_denom(self.to_glib_none().0) as i32,
            )
        }
    }

    #[doc(alias = "get_par")]
    #[doc(alias = "gst_discoverer_video_info_get_par_num")]
    #[doc(alias = "gst_discoverer_video_info_get_par_denom")]
    pub fn par(&self) -> gst::Fraction {
        unsafe {
            gst::Fraction::new(
                ffi::gst_discoverer_video_info_get_par_num(self.to_glib_none().0) as i32,
                ffi::gst_discoverer_video_info_get_par_denom(self.to_glib_none().0) as i32,
            )
        }
    }
}
