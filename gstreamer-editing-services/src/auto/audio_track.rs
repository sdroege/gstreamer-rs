// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, MetaContainer, Track};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GESAudioTrack")]
    pub struct AudioTrack(Object<ffi::GESAudioTrack, ffi::GESAudioTrackClass>) @extends Track, gst::Bin, gst::Element, gst::Object, @implements gst::ChildProxy, MetaContainer;

    match fn {
        type_ => || ffi::ges_audio_track_get_type(),
    }
}

impl AudioTrack {
    pub const NONE: Option<&'static AudioTrack> = None;

    #[doc(alias = "ges_audio_track_new")]
    pub fn new() -> AudioTrack {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_audio_track_new()) }
    }
}

impl Default for AudioTrack {
    fn default() -> Self {
        Self::new()
    }
}
