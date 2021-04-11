// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Caps;
use crate::Stream;
use crate::StreamFlags;
use crate::StreamType;
use glib::translate::*;
use std::fmt;

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

        let (major, minor, _, _) = crate::version();
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

    pub fn debug(&self) -> Debug {
        Debug(self)
    }
}

pub struct Debug<'a>(&'a Stream);

impl<'a> fmt::Debug for Debug<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Stream")
            .field("stream_id", &self.0.stream_id())
            .field("stream_type", &self.0.stream_type())
            .field("stream_flags", &self.0.stream_flags())
            .field("caps", &self.0.caps())
            .field("tags", &self.0.tags())
            .finish()
    }
}
