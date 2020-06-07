// Copyright (C) 2017-2020 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::{from_glib_full, ToGlibPtr};
use gst;


pub fn audio_buffer_clip(
    buffer: gst::Buffer,
    segment: &gst::Segment,
    rate: u32,
    bpf: u32,
) -> Option<gst::Buffer> {
    skip_assert_initialized!();

    unsafe {
        from_glib_full(gst_audio_sys::gst_audio_buffer_clip(
            buffer.into_ptr(),
            segment.to_glib_none().0,
            rate as i32,
            bpf as i32,
        ))
    }
}
