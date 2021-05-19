// Take a look at the license at the top of the repository in the LICENSE file.

use crate::AudioStreamAlign;

use glib::translate::*;
use std::mem;

impl AudioStreamAlign {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[doc(alias = "gst_audio_stream_align_process")]
    pub fn process(
        &mut self,
        discont: bool,
        timestamp: gst::ClockTime,
        n_samples: u32,
    ) -> (bool, gst::ClockTime, gst::ClockTime, u64) {
        unsafe {
            let mut out_timestamp = mem::MaybeUninit::uninit();
            let mut out_duration = mem::MaybeUninit::uninit();
            let mut out_sample_position = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_audio_stream_align_process(
                self.to_glib_none_mut().0,
                discont.into_glib(),
                timestamp.into_glib(),
                n_samples,
                out_timestamp.as_mut_ptr(),
                out_duration.as_mut_ptr(),
                out_sample_position.as_mut_ptr(),
            ));
            (
                ret,
                from_glib(out_timestamp.assume_init()),
                from_glib(out_duration.assume_init()),
                out_sample_position.assume_init(),
            )
        }
    }
}
