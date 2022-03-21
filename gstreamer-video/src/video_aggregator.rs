use crate::auto::VideoAggregator;
use glib::object::IsA;
use glib::translate::*;

pub trait VideoAggregatorExtManual: 'static {
    fn video_info(&self) -> Option<crate::VideoInfo>;
}

impl<O: IsA<VideoAggregator>> VideoAggregatorExtManual for O {
    fn video_info(&self) -> Option<crate::VideoInfo> {
        unsafe {
            let ptr = self.as_ptr() as *mut ffi::GstVideoAggregator;
            let _guard = crate::utils::MutexGuard::lock(&(*(ptr as *mut gst::ffi::GstObject)).lock);

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
