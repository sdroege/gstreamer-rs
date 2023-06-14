// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use glib::{translate::*, Cast};

use crate::{DiscovererStreamInfo, DiscovererVideoInfo};

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

    pub fn debug(&self) -> Debug {
        Debug(self)
    }
}

pub struct Debug<'a>(&'a DiscovererVideoInfo);

impl<'a> fmt::Debug for Debug<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let info = self.0.upcast_ref::<DiscovererStreamInfo>();

        f.debug_struct("DiscovererVideoInfo")
            .field("width", &self.0.width())
            .field("height", &self.0.height())
            .field("depth", &self.0.depth())
            .field("bitrate", &self.0.bitrate())
            .field("max-bitrate", &self.0.max_bitrate())
            .field("is-image", &self.0.is_image())
            .field("is-interlaced", &self.0.is_interlaced())
            .field("stream", &info.debug())
            .finish()
    }
}
