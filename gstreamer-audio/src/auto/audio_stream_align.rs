// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AudioStreamAlign(Boxed<ffi::GstAudioStreamAlign>);

    match fn {
        copy => |ptr| ffi::gst_audio_stream_align_copy(ptr),
        free => |ptr| ffi::gst_audio_stream_align_free(ptr),
        type_ => || ffi::gst_audio_stream_align_get_type(),
    }
}

impl AudioStreamAlign {
    #[doc(alias = "gst_audio_stream_align_new")]
    pub fn new(
        rate: i32,
        alignment_threshold: gst::ClockTime,
        discont_wait: gst::ClockTime,
    ) -> AudioStreamAlign {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_audio_stream_align_new(
                rate,
                alignment_threshold.into_glib(),
                discont_wait.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_audio_stream_align_get_alignment_threshold")]
    pub fn alignment_threshold(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_audio_stream_align_get_alignment_threshold(
                mut_override(self.to_glib_none().0),
            ))
        }
    }

    #[doc(alias = "gst_audio_stream_align_get_discont_wait")]
    pub fn discont_wait(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_audio_stream_align_get_discont_wait(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[doc(alias = "gst_audio_stream_align_get_rate")]
    pub fn rate(&self) -> i32 {
        unsafe { ffi::gst_audio_stream_align_get_rate(mut_override(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_audio_stream_align_get_samples_since_discont")]
    pub fn samples_since_discont(&self) -> u64 {
        unsafe {
            ffi::gst_audio_stream_align_get_samples_since_discont(mut_override(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_audio_stream_align_get_timestamp_at_discont")]
    pub fn timestamp_at_discont(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_audio_stream_align_get_timestamp_at_discont(
                mut_override(self.to_glib_none().0),
            ))
        }
    }

    #[doc(alias = "gst_audio_stream_align_mark_discont")]
    pub fn mark_discont(&mut self) {
        unsafe {
            ffi::gst_audio_stream_align_mark_discont(self.to_glib_none_mut().0);
        }
    }

    #[doc(alias = "gst_audio_stream_align_set_alignment_threshold")]
    pub fn set_alignment_threshold(&mut self, alignment_threshold: gst::ClockTime) {
        unsafe {
            ffi::gst_audio_stream_align_set_alignment_threshold(
                self.to_glib_none_mut().0,
                alignment_threshold.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_audio_stream_align_set_discont_wait")]
    pub fn set_discont_wait(&mut self, discont_wait: gst::ClockTime) {
        unsafe {
            ffi::gst_audio_stream_align_set_discont_wait(
                self.to_glib_none_mut().0,
                discont_wait.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_audio_stream_align_set_rate")]
    pub fn set_rate(&mut self, rate: i32) {
        unsafe {
            ffi::gst_audio_stream_align_set_rate(self.to_glib_none_mut().0, rate);
        }
    }
}

unsafe impl Send for AudioStreamAlign {}
unsafe impl Sync for AudioStreamAlign {}
