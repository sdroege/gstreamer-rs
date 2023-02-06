use glib::{object::IsA, translate::*};
use gst::prelude::*;

use crate::{auto::VideoAggregatorPad, subclass::AggregateFramesToken};

pub trait VideoAggregatorPadExtManual: 'static {
    #[doc(alias = "gst_video_aggregator_pad_has_current_buffer")]
    fn has_current_buffer(&self, token: &AggregateFramesToken) -> bool;

    #[doc(alias = "gst_video_aggregator_pad_get_current_buffer")]
    fn current_buffer(&self, token: &AggregateFramesToken) -> Option<gst::Buffer>;

    #[doc(alias = "gst_video_aggregator_pad_get_prepared_frame")]
    fn prepared_frame<'a>(
        &self,
        token: &'a AggregateFramesToken,
    ) -> Option<crate::VideoFrameRef<&'a gst::BufferRef>>;

    fn video_info(&self) -> Option<crate::VideoInfo>;
}

impl<O: IsA<VideoAggregatorPad>> VideoAggregatorPadExtManual for O {
    fn has_current_buffer(&self, _token: &AggregateFramesToken) -> bool {
        unsafe {
            from_glib(ffi::gst_video_aggregator_pad_has_current_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn current_buffer(&self, _token: &AggregateFramesToken) -> Option<gst::Buffer> {
        unsafe {
            from_glib_none(ffi::gst_video_aggregator_pad_get_current_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn prepared_frame<'a>(
        &self,
        _token: &'a AggregateFramesToken,
    ) -> Option<crate::VideoFrameRef<&'a gst::BufferRef>> {
        unsafe {
            let ptr =
                ffi::gst_video_aggregator_pad_get_prepared_frame(self.as_ref().to_glib_none().0);

            if ptr.is_null() {
                None
            } else {
                Some(crate::VideoFrameRef::from_glib_borrow(ptr).into_inner())
            }
        }
    }

    fn video_info(&self) -> Option<crate::VideoInfo> {
        unsafe {
            let ptr = self.as_ptr() as *mut ffi::GstVideoAggregatorPad;
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
