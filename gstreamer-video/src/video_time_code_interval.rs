// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::cmp;
use std::fmt;
use std::mem;
use std::str;

glib::wrapper! {
    #[doc(alias = "GstVideoTimeCodeInterval")]
    pub struct VideoTimeCodeInterval(BoxedInline<ffi::GstVideoTimeCodeInterval>);

    match fn {
        type_ => || ffi::gst_video_time_code_interval_get_type(),
    }
}

impl VideoTimeCodeInterval {
    pub fn new(hours: u32, minutes: u32, seconds: u32, frames: u32) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut v = mem::MaybeUninit::zeroed();
            ffi::gst_video_time_code_interval_init(v.as_mut_ptr(), hours, minutes, seconds, frames);
            Self {
                inner: v.assume_init(),
            }
        }
    }

    #[doc(alias = "get_hours")]
    pub fn hours(&self) -> u32 {
        self.inner.hours
    }

    pub fn set_hours(&mut self, hours: u32) {
        self.inner.hours = hours
    }

    #[doc(alias = "get_minutes")]
    pub fn minutes(&self) -> u32 {
        self.inner.minutes
    }

    pub fn set_minutes(&mut self, minutes: u32) {
        assert!(minutes < 60);
        self.inner.minutes = minutes
    }

    #[doc(alias = "get_seconds")]
    pub fn seconds(&self) -> u32 {
        self.inner.seconds
    }

    pub fn set_seconds(&mut self, seconds: u32) {
        assert!(seconds < 60);
        self.inner.seconds = seconds
    }

    #[doc(alias = "get_frames")]
    pub fn frames(&self) -> u32 {
        self.inner.frames
    }

    pub fn set_frames(&mut self, frames: u32) {
        self.inner.frames = frames
    }
}

unsafe impl Send for VideoTimeCodeInterval {}
unsafe impl Sync for VideoTimeCodeInterval {}

impl PartialEq for VideoTimeCodeInterval {
    fn eq(&self, other: &Self) -> bool {
        self.inner.hours == other.inner.hours
            && self.inner.minutes == other.inner.minutes
            && self.inner.seconds == other.inner.seconds
            && self.inner.frames == other.inner.frames
    }
}

impl Eq for VideoTimeCodeInterval {}

impl PartialOrd for VideoTimeCodeInterval {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VideoTimeCodeInterval {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.inner
            .hours
            .cmp(&other.inner.hours)
            .then_with(|| self.inner.minutes.cmp(&other.inner.minutes))
            .then_with(|| self.inner.seconds.cmp(&other.inner.seconds))
            .then_with(|| self.inner.frames.cmp(&other.inner.frames))
    }
}

impl fmt::Debug for VideoTimeCodeInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoTimeCodeInterval")
            .field("hours", &self.inner.hours)
            .field("minutes", &self.inner.minutes)
            .field("seconds", &self.inner.seconds)
            .field("frames", &self.inner.frames)
            .finish()
    }
}

impl fmt::Display for VideoTimeCodeInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}:{:02}",
            self.inner.hours, self.inner.minutes, self.inner.seconds, self.inner.frames
        )
    }
}

impl str::FromStr for VideoTimeCodeInterval {
    type Err = glib::error::BoolError;

    #[doc(alias = "gst_video_time_code_interval_new_from_string")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<Self>::from_glib_full(ffi::gst_video_time_code_interval_new_from_string(
                s.to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to create VideoTimeCodeInterval from string"))
        }
    }
}
