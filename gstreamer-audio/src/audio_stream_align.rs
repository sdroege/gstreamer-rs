// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::AudioStreamAlign;

use glib::translate::*;
use std::mem;

impl AudioStreamAlign {
    #[cfg(any(feature = "v1_14", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_14")))]
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
                discont.to_glib(),
                timestamp.to_glib(),
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
