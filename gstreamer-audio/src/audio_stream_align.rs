// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use AudioStreamAlign;

use gst;
use glib::translate::*;
use std::mem;

impl AudioStreamAlign {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn process(&mut self, discont: bool, timestamp: gst::ClockTime, n_samples: u32) -> (bool, gst::ClockTime, gst::ClockTime, u64) {
        unsafe {
            let mut out_timestamp = mem::uninitialized();
            let mut out_duration = mem::uninitialized();
            let mut out_sample_position = mem::uninitialized();
            let ret = from_glib(ffi::gst_audio_stream_align_process(self.to_glib_none_mut().0, discont.to_glib(), timestamp.to_glib(), n_samples, &mut out_timestamp, &mut out_duration, &mut out_sample_position));
            (ret, from_glib(out_timestamp), from_glib(out_duration), out_sample_position)
        }
    }
}
