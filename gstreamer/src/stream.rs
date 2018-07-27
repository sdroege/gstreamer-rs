// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib::translate::*;
use Caps;
use Stream;
use StreamFlags;
use StreamType;

impl Stream {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn new<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b Caps>>>(
        stream_id: P,
        caps: Q,
        type_: StreamType,
        flags: StreamFlags,
    ) -> Stream {
        assert_initialized_main_thread!();
        let stream_id = stream_id.into();
        let stream_id = stream_id.to_glib_none();
        let caps = caps.into();
        let caps = caps.to_glib_none();

        let (major, minor, _, _) = ::version();
        if (major, minor) > (1, 12) {
            unsafe {
                from_glib_full(ffi::gst_stream_new(
                    stream_id.0,
                    caps.0,
                    type_.to_glib(),
                    flags.to_glib(),
                ))
            }
        } else {
            // Work-around for 1.14 switching from transfer-floating to transfer-full
            unsafe {
                from_glib_none(ffi::gst_stream_new(
                    stream_id.0,
                    caps.0,
                    type_.to_glib(),
                    flags.to_glib(),
                ))
            }
        }
    }
}
