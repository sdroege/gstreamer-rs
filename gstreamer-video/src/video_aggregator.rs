use glib::{object::IsA, translate::*};
use gst::prelude::*;

use crate::auto::VideoAggregator;

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::VideoAggregator>> Sealed for T {}
}

pub trait VideoAggregatorExtManual: sealed::Sealed + IsA<VideoAggregator> + 'static {
    fn video_info(&self) -> Option<crate::VideoInfo> {
        unsafe {
            let ptr = self.as_ptr() as *mut ffi::GstVideoAggregator;
            let _guard = self.as_ref().object_lock();

            let info = &(*ptr).info;

            if info.finfo.is_null() || info.width <= 0 || info.height <= 0 {
                return None;
            }

            Some(from_glib_none(mut_override(
                info as *const ffi::GstVideoInfo,
            )))
        }
    }
}

impl<O: IsA<VideoAggregator>> VideoAggregatorExtManual for O {}
