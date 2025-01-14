// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use crate::ffi;

glib::wrapper! {
    #[doc(alias = "GstSourceBufferInterval")]
    pub struct SourceBufferInterval(BoxedInline<ffi::GstSourceBufferInterval>);

    match fn {}
}

impl SourceBufferInterval {
    pub fn new(start: gst::ClockTime, end: gst::ClockTime) -> Self {
        skip_assert_initialized!();

        let inner = ffi::GstSourceBufferInterval {
            start: start.nseconds(),
            end: end.nseconds(),
        };

        Self { inner }
    }

    pub fn start(&self) -> gst::ClockTime {
        gst::ClockTime::from_nseconds(self.inner.start)
    }

    pub fn set_start(&mut self, start: gst::ClockTime) {
        self.inner.start = start.nseconds();
    }

    pub fn end(&self) -> gst::ClockTime {
        gst::ClockTime::from_nseconds(self.inner.end)
    }

    pub fn set_end(&mut self, end: gst::ClockTime) {
        self.inner.end = end.nseconds();
    }
}

unsafe impl Send for SourceBufferInterval {}
unsafe impl Sync for SourceBufferInterval {}

impl PartialEq for SourceBufferInterval {
    fn eq(&self, other: &Self) -> bool {
        self.inner.start == other.inner.start && self.inner.end == other.inner.end
    }
}

impl Eq for SourceBufferInterval {}

impl fmt::Debug for SourceBufferInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SourceBufferInterval")
            .field("start", &self.start())
            .field("end", &self.end())
            .finish()
    }
}
