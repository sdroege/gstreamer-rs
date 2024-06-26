// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, PlayStreamInfo};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GstPlayVideoInfo")]
    pub struct PlayVideoInfo(Object<ffi::GstPlayVideoInfo, ffi::GstPlayVideoInfoClass>) @extends PlayStreamInfo;

    match fn {
        type_ => || ffi::gst_play_video_info_get_type(),
    }
}

impl PlayVideoInfo {
    #[doc(alias = "gst_play_video_info_get_bitrate")]
    #[doc(alias = "get_bitrate")]
    pub fn bitrate(&self) -> i32 {
        unsafe { ffi::gst_play_video_info_get_bitrate(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_play_video_info_get_height")]
    #[doc(alias = "get_height")]
    pub fn height(&self) -> i32 {
        unsafe { ffi::gst_play_video_info_get_height(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_play_video_info_get_max_bitrate")]
    #[doc(alias = "get_max_bitrate")]
    pub fn max_bitrate(&self) -> i32 {
        unsafe { ffi::gst_play_video_info_get_max_bitrate(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_play_video_info_get_width")]
    #[doc(alias = "get_width")]
    pub fn width(&self) -> i32 {
        unsafe { ffi::gst_play_video_info_get_width(self.to_glib_none().0) }
    }
}

unsafe impl Send for PlayVideoInfo {}
unsafe impl Sync for PlayVideoInfo {}
