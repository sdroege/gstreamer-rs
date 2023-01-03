// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::Play;

impl Play {
    #[doc(alias = "get_config")]
    #[doc(alias = "gst_play_get_config")]
    pub fn config(&self) -> crate::PlayConfig {
        unsafe { from_glib_full(ffi::gst_play_get_config(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_play_set_config")]
    pub fn set_config(&self, config: crate::PlayConfig) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_play_set_config(self.to_glib_none().0, config.into_glib_ptr()),
                "Failed to set config",
            )
        }
    }

    #[doc(alias = "gst_play_get_video_snapshot")]
    #[doc(alias = "get_video_snapshot")]
    pub fn video_snapshot(
        &self,
        format: crate::PlaySnapshotFormat,
        config: Option<&gst::StructureRef>,
    ) -> Option<gst::Sample> {
        unsafe {
            from_glib_full(ffi::gst_play_get_video_snapshot(
                self.to_glib_none().0,
                format.into_glib(),
                mut_override(config.map(|c| c.as_ptr()).unwrap_or(std::ptr::null())),
            ))
        }
    }
}
