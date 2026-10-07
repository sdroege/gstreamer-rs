// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{HipPoolAllocator, ffi};
use glib::{prelude::*, translate::*};

pub trait HipPoolAllocatorExtManual: IsA<HipPoolAllocator> + 'static {
    #[doc(alias = "gst_hip_pool_allocator_acquire_memory")]
    fn acquire_memory(&self) -> Result<gst::Memory, gst::FlowError> {
        unsafe {
            let mut memory = std::ptr::null_mut();
            gst::FlowSuccess::try_from_glib(ffi::gst_hip_pool_allocator_acquire_memory(
                self.as_ref().to_glib_none().0,
                &mut memory,
            ))
            .map(|_| from_glib_full(memory))
        }
    }
}

impl<O: IsA<HipPoolAllocator>> HipPoolAllocatorExtManual for O {}
