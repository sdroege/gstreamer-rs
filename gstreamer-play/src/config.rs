// Take a look at the license at the top of the repository in the LICENSE file.

use std::{mem, ops};

use glib::translate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayConfig(gst::Structure);

impl ops::Deref for PlayConfig {
    type Target = gst::StructureRef;

    #[inline]
    fn deref(&self) -> &gst::StructureRef {
        self.0.deref()
    }
}

impl ops::DerefMut for PlayConfig {
    #[inline]
    fn deref_mut(&mut self) -> &mut gst::StructureRef {
        self.0.deref_mut()
    }
}

impl AsRef<gst::StructureRef> for PlayConfig {
    #[inline]
    fn as_ref(&self) -> &gst::StructureRef {
        self.0.as_ref()
    }
}

impl AsMut<gst::StructureRef> for PlayConfig {
    #[inline]
    fn as_mut(&mut self) -> &mut gst::StructureRef {
        self.0.as_mut()
    }
}

impl PlayConfig {
    #[doc(alias = "get_position_update_interval")]
    #[doc(alias = "gst_play_config_get_position_update_interval")]
    pub fn position_update_interval(&self) -> u32 {
        skip_assert_initialized!();
        unsafe { ffi::gst_play_config_get_position_update_interval(self.0.to_glib_none().0) }
    }

    #[doc(alias = "gst_play_config_set_position_update_interval")]
    pub fn set_position_update_interval(&mut self, interval: u32) {
        skip_assert_initialized!();
        unsafe {
            ffi::gst_play_config_set_position_update_interval(
                self.0.to_glib_none_mut().0,
                interval,
            );
        }
    }

    #[doc(alias = "get_seek_accurate")]
    pub fn is_seek_accurate(&self) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gst_play_config_get_seek_accurate(
                self.0.to_glib_none().0,
            ))
        }
    }

    pub fn set_seek_accurate(&mut self, accurate: bool) {
        skip_assert_initialized!();
        // FIXME: Work-around for
        // http://cgit.freedesktop.org/gstreamer/gst-plugins-bad/commit/?id=cc58bd6ae071dec4ea7b4be626034accd0372755
        self.set("accurate-seek", accurate);
    }

    #[doc(alias = "get_user_agent")]
    #[doc(alias = "gst_play_config_get_user_agent")]
    pub fn user_agent(&self) -> Option<String> {
        skip_assert_initialized!();
        unsafe { from_glib_full(ffi::gst_play_config_get_user_agent(self.0.to_glib_none().0)) }
    }

    #[doc(alias = "gst_play_config_set_user_agent")]
    pub fn set_user_agent(&mut self, agent: &str) {
        skip_assert_initialized!();
        unsafe {
            ffi::gst_play_config_set_user_agent(
                self.0.to_glib_none_mut().0,
                agent.to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "gst_play_config_get_pipeline_dump_in_error_details")]
    pub fn pipeline_dump_in_error_details(&self) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gst_play_config_get_pipeline_dump_in_error_details(
                self.0.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "gst_play_config_set_pipeline_dump_in_error_details")]
    pub fn set_pipeline_dump_in_error_details(&mut self, pipeline_dump_in_error_details: bool) {
        skip_assert_initialized!();
        unsafe {
            ffi::gst_play_config_set_pipeline_dump_in_error_details(
                self.0.to_glib_none_mut().0,
                pipeline_dump_in_error_details.into_glib(),
            );
        }
    }
}

impl IntoGlibPtr<*mut gst::ffi::GstStructure> for PlayConfig {
    #[inline]
    unsafe fn into_glib_ptr(self) -> *mut gst::ffi::GstStructure {
        let mut s = mem::ManuallyDrop::new(self);
        s.0.to_glib_none_mut().0
    }
}

impl FromGlibPtrFull<*mut gst::ffi::GstStructure> for PlayConfig {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut gst::ffi::GstStructure) -> Self {
        PlayConfig(from_glib_full(ptr))
    }
}
