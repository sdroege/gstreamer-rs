// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, MetaContainer, Track};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GESVideoTrack")]
    pub struct VideoTrack(Object<ffi::GESVideoTrack, ffi::GESVideoTrackClass>) @extends Track, gst::Bin, gst::Element, gst::Object, @implements gst::ChildProxy, MetaContainer;

    match fn {
        type_ => || ffi::ges_video_track_get_type(),
    }
}

impl VideoTrack {
    pub const NONE: Option<&'static VideoTrack> = None;

    #[doc(alias = "ges_video_track_new")]
    pub fn new() -> VideoTrack {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_video_track_new()) }
    }
}

impl Default for VideoTrack {
    fn default() -> Self {
        Self::new()
    }
}
