// Take a look at the license at the top of the repository in the LICENSE file.

use crate::BaseTransform;
use glib::prelude::*;
use glib::translate::*;
use std::mem;
use std::ptr;

pub trait BaseTransformExtManual: 'static {
    fn allocator(&self) -> (Option<gst::Allocator>, gst::AllocationParams);

    fn segment(&self) -> gst::Segment;
}

impl<O: IsA<BaseTransform>> BaseTransformExtManual for O {
    fn allocator(&self) -> (Option<gst::Allocator>, gst::AllocationParams) {
        unsafe {
            let mut allocator = ptr::null_mut();
            let mut params = mem::zeroed();
            ffi::gst_base_transform_get_allocator(
                self.as_ref().to_glib_none().0,
                &mut allocator,
                &mut params,
            );
            (from_glib_full(allocator), params.into())
        }
    }

    fn segment(&self) -> gst::Segment {
        unsafe {
            let trans: &ffi::GstBaseTransform = &*(self.as_ptr() as *const _);
            let _guard = crate::utils::MutexGuard::lock(&trans.element.object.lock);
            from_glib_none(&trans.segment as *const _)
        }
    }
}
