// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};
use gst::prelude::*;
use gst_base::prelude::*;

use crate::VideoFilter;

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::VideoFilter>> Sealed for T {}
}

pub trait VideoFilterExtManual: sealed::Sealed + IsA<VideoFilter> + 'static {
    fn input_video_info(&self) -> Option<crate::VideoInfo> {
        unsafe {
            let ptr = self.as_ptr() as *mut ffi::GstVideoFilter;
            let sinkpad = self.as_ref().sink_pad();
            let _guard = sinkpad.stream_lock();

            let info = &(*ptr).in_info;

            if info.finfo.is_null() || info.width <= 0 || info.height <= 0 {
                return None;
            }

            Some(from_glib_none(mut_override(
                info as *const ffi::GstVideoInfo,
            )))
        }
    }

    fn output_video_info(&self) -> Option<crate::VideoInfo> {
        unsafe {
            let ptr = self.as_ptr() as *mut ffi::GstVideoFilter;
            let sinkpad = self.as_ref().sink_pad();
            let _guard = sinkpad.stream_lock();

            let info = &(*ptr).out_info;

            if info.finfo.is_null() || info.width <= 0 || info.height <= 0 {
                return None;
            }

            Some(from_glib_none(mut_override(
                info as *const ffi::GstVideoInfo,
            )))
        }
    }
}

impl<O: IsA<VideoFilter>> VideoFilterExtManual for O {}
