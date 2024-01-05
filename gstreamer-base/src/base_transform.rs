// Take a look at the license at the top of the repository in the LICENSE file.

use std::{mem, ptr};

use glib::{prelude::*, translate::*};
use gst::prelude::*;

use crate::BaseTransform;

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::BaseTransform>> Sealed for T {}
}

pub trait BaseTransformExtManual: sealed::Sealed + IsA<BaseTransform> + 'static {
    #[doc(alias = "get_allocator")]
    #[doc(alias = "gst_base_transform_get_allocator")]
    fn allocator(&self) -> (Option<gst::Allocator>, gst::AllocationParams) {
        unsafe {
            let mut allocator = ptr::null_mut();
            let mut params = mem::MaybeUninit::uninit();
            ffi::gst_base_transform_get_allocator(
                self.as_ref().to_glib_none().0,
                &mut allocator,
                params.as_mut_ptr(),
            );
            (from_glib_full(allocator), params.assume_init().into())
        }
    }

    #[doc(alias = "get_segment")]
    fn segment(&self) -> gst::Segment {
        unsafe {
            let trans: &ffi::GstBaseTransform = &*(self.as_ptr() as *const _);
            let sinkpad = self.sink_pad();
            let _guard = sinkpad.stream_lock();
            from_glib_none(&trans.segment as *const _)
        }
    }

    fn sink_pad(&self) -> &gst::Pad {
        unsafe {
            let elt = &*(self.as_ptr() as *const ffi::GstBaseTransform);
            &*(&elt.sinkpad as *const *mut gst::ffi::GstPad as *const gst::Pad)
        }
    }

    fn src_pad(&self) -> &gst::Pad {
        unsafe {
            let elt = &*(self.as_ptr() as *const ffi::GstBaseTransform);
            &*(&elt.srcpad as *const *mut gst::ffi::GstPad as *const gst::Pad)
        }
    }
}

impl<O: IsA<BaseTransform>> BaseTransformExtManual for O {}
