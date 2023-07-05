// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};
use gst::prelude::*;

use crate::AggregatorPad;

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::AggregatorPad>> Sealed for T {}
}

pub trait AggregatorPadExtManual: sealed::Sealed + IsA<AggregatorPad> + 'static {
    #[doc(alias = "get_segment")]
    fn segment(&self) -> gst::Segment {
        unsafe {
            let ptr: &ffi::GstAggregatorPad = &*(self.as_ptr() as *const _);
            let _guard = self.as_ref().object_lock();
            from_glib_none(&ptr.segment as *const gst::ffi::GstSegment)
        }
    }
}

impl<O: IsA<AggregatorPad>> AggregatorPadExtManual for O {}
