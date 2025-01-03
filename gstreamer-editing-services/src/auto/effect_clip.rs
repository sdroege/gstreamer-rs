// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{
    ffi, BaseEffectClip, Clip, Container, Extractable, MetaContainer, OperationClip,
    TimelineElement,
};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GESEffectClip")]
    pub struct EffectClip(Object<ffi::GESEffectClip, ffi::GESEffectClipClass>) @extends BaseEffectClip, OperationClip, Clip, Container, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_effect_clip_get_type(),
    }
}

impl EffectClip {
    pub const NONE: Option<&'static EffectClip> = None;

    #[doc(alias = "ges_effect_clip_new")]
    pub fn new(
        video_bin_description: Option<&str>,
        audio_bin_description: Option<&str>,
    ) -> Option<EffectClip> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::ges_effect_clip_new(
                video_bin_description.to_glib_none().0,
                audio_bin_description.to_glib_none().0,
            ))
        }
    }
}

pub trait EffectClipExt: IsA<EffectClip> + 'static {
    #[doc(alias = "audio-bin-description")]
    fn audio_bin_description(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "audio-bin-description")
    }

    #[doc(alias = "video-bin-description")]
    fn video_bin_description(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "video-bin-description")
    }
}

impl<O: IsA<EffectClip>> EffectClipExt for O {}
