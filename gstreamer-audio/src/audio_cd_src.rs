// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{AudioCdSrc, ffi};
use glib::translate::*;
use gst::prelude::*;
use std::mem;

pub trait AudioCdSrcExtManual: IsA<AudioCdSrc> + 'static {
    #[doc(alias = "gst_audio_cd_src_add_track")]
    fn add_track(
        &self,
        is_audio: bool,
        num: u32,
        start: u32,
        end: u32,
        tags: Option<gst::TagList>,
    ) -> Result<(), glib::BoolError> {
        let mut track = unsafe {
            ffi::GstAudioCdSrcTrack {
                is_audio: is_audio.into_glib(),
                tags: tags.into_glib_ptr(),
                num,
                start,
                end,
                ..mem::zeroed()
            }
        };
        unsafe {
            let is_ok = ffi::gst_audio_cd_src_add_track(self.as_ref().to_glib_none().0, &mut track);
            let result = glib::result_from_gboolean!(is_ok, "Failed to add track");
            if result.is_err() {
                let _ = Option::<gst::TagList>::from_glib_full(track.tags);
            }
            result
        }
    }
}

impl<O: IsA<AudioCdSrc>> AudioCdSrcExtManual for O {}
