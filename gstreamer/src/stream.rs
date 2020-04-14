// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use gst_sys;
use std::fmt;
use Caps;
use Stream;
use StreamFlags;
use StreamType;

impl Stream {
    pub fn new(
        stream_id: Option<&str>,
        caps: Option<&Caps>,
        type_: StreamType,
        flags: StreamFlags,
    ) -> Stream {
        assert_initialized_main_thread!();
        let stream_id = stream_id.to_glib_none();
        let caps = caps.to_glib_none();

        let (major, minor, _, _) = ::version();
        if (major, minor) > (1, 12) {
            unsafe {
                from_glib_full(gst_sys::gst_stream_new(
                    stream_id.0,
                    caps.0,
                    type_.to_glib(),
                    flags.to_glib(),
                ))
            }
        } else {
            // Work-around for 1.14 switching from transfer-floating to transfer-full
            unsafe {
                from_glib_none(gst_sys::gst_stream_new(
                    stream_id.0,
                    caps.0,
                    type_.to_glib(),
                    flags.to_glib(),
                ))
            }
        }
    }

    pub fn debug(&self) -> Debug {
        Debug(self)
    }
}

pub struct Debug<'a>(&'a Stream);

impl<'a> fmt::Debug for Debug<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Stream")
            .field("stream_id", &self.0.get_stream_id())
            .field("stream_type", &self.0.get_stream_type())
            .field("stream_flags", &self.0.get_stream_flags())
            .field("caps", &self.0.get_caps())
            .field("tags", &self.0.get_tags())
            .finish()
    }
}
