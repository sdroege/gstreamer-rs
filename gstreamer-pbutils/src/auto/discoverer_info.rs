// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::DiscovererResult;
use crate::DiscovererSerializeFlags;
use crate::DiscovererStreamInfo;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GstDiscovererInfo")]
    pub struct DiscovererInfo(Object<ffi::GstDiscovererInfo>);

    match fn {
        type_ => || ffi::gst_discoverer_info_get_type(),
    }
}

impl DiscovererInfo {
    #[doc(alias = "gst_discoverer_info_copy")]
    #[must_use]
    pub fn copy(&self) -> DiscovererInfo {
        unsafe { from_glib_full(ffi::gst_discoverer_info_copy(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_discoverer_info_get_audio_streams")]
    #[doc(alias = "get_audio_streams")]
    pub fn audio_streams(&self) -> Vec<DiscovererStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_discoverer_info_get_audio_streams(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_discoverer_info_get_container_streams")]
    #[doc(alias = "get_container_streams")]
    pub fn container_streams(&self) -> Vec<DiscovererStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_discoverer_info_get_container_streams(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_discoverer_info_get_duration")]
    #[doc(alias = "get_duration")]
    pub fn duration(&self) -> Option<gst::ClockTime> {
        unsafe { from_glib(ffi::gst_discoverer_info_get_duration(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[doc(alias = "gst_discoverer_info_get_live")]
    #[doc(alias = "get_live")]
    pub fn is_live(&self) -> bool {
        unsafe { from_glib(ffi::gst_discoverer_info_get_live(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_discoverer_info_get_misc")]
    #[doc(alias = "get_misc")]
    pub fn misc(&self) -> Option<gst::Structure> {
        unsafe { from_glib_none(ffi::gst_discoverer_info_get_misc(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_discoverer_info_get_missing_elements_installer_details")]
    #[doc(alias = "get_missing_elements_installer_details")]
    pub fn missing_elements_installer_details(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(
                ffi::gst_discoverer_info_get_missing_elements_installer_details(
                    self.to_glib_none().0,
                ),
            )
        }
    }

    #[doc(alias = "gst_discoverer_info_get_result")]
    #[doc(alias = "get_result")]
    pub fn result(&self) -> DiscovererResult {
        unsafe { from_glib(ffi::gst_discoverer_info_get_result(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_discoverer_info_get_seekable")]
    #[doc(alias = "get_seekable")]
    pub fn is_seekable(&self) -> bool {
        unsafe { from_glib(ffi::gst_discoverer_info_get_seekable(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_discoverer_info_get_stream_info")]
    #[doc(alias = "get_stream_info")]
    pub fn stream_info(&self) -> Option<DiscovererStreamInfo> {
        unsafe {
            from_glib_full(ffi::gst_discoverer_info_get_stream_info(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_discoverer_info_get_stream_list")]
    #[doc(alias = "get_stream_list")]
    pub fn stream_list(&self) -> Vec<DiscovererStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_discoverer_info_get_stream_list(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_discoverer_info_get_streams")]
    #[doc(alias = "get_streams")]
    pub fn streams(&self, streamtype: glib::types::Type) -> Vec<DiscovererStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_discoverer_info_get_streams(
                self.to_glib_none().0,
                streamtype.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_discoverer_info_get_subtitle_streams")]
    #[doc(alias = "get_subtitle_streams")]
    pub fn subtitle_streams(&self) -> Vec<DiscovererStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_discoverer_info_get_subtitle_streams(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v1_20", deprecated = "Since 1.20")]
    #[doc(alias = "gst_discoverer_info_get_tags")]
    #[doc(alias = "get_tags")]
    pub fn tags(&self) -> Option<gst::TagList> {
        unsafe { from_glib_none(ffi::gst_discoverer_info_get_tags(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_discoverer_info_get_toc")]
    #[doc(alias = "get_toc")]
    pub fn toc(&self) -> Option<gst::Toc> {
        unsafe { from_glib_none(ffi::gst_discoverer_info_get_toc(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_discoverer_info_get_uri")]
    #[doc(alias = "get_uri")]
    pub fn uri(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gst_discoverer_info_get_uri(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_discoverer_info_get_video_streams")]
    #[doc(alias = "get_video_streams")]
    pub fn video_streams(&self) -> Vec<DiscovererStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_discoverer_info_get_video_streams(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_discoverer_info_to_variant")]
    pub fn to_variant(
        &self,
        flags: DiscovererSerializeFlags,
    ) -> Result<glib::Variant, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_discoverer_info_to_variant(
                self.to_glib_none().0,
                flags.into_glib(),
            ))
            .ok_or_else(|| glib::bool_error!("Failed to serialize DiscovererInfo to Variant"))
        }
    }

    #[doc(alias = "gst_discoverer_info_from_variant")]
    pub fn from_variant(variant: &glib::Variant) -> Result<DiscovererInfo, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_discoverer_info_from_variant(
                variant.to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to deserialize DiscovererInfo from Variant"))
        }
    }
}

unsafe impl Send for DiscovererInfo {}
unsafe impl Sync for DiscovererInfo {}
