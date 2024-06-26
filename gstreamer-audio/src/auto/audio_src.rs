// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, AudioBaseSrc};

glib::wrapper! {
    #[doc(alias = "GstAudioSrc")]
    pub struct AudioSrc(Object<ffi::GstAudioSrc, ffi::GstAudioSrcClass>) @extends AudioBaseSrc, gst_base::BaseSrc, gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_audio_src_get_type(),
    }
}

impl AudioSrc {
    pub const NONE: Option<&'static AudioSrc> = None;
}

unsafe impl Send for AudioSrc {}
unsafe impl Sync for AudioSrc {}
