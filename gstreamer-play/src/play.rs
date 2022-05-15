// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Play;
use glib::translate::*;

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
}
