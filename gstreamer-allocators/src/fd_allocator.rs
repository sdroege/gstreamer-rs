use std::{fmt, os::unix::prelude::RawFd};

use glib::{translate::*, Cast};
use gst::{Memory, MemoryRef};

use crate::{FdAllocator, FdMemoryFlags};

gst::memory_object_wrapper!(
    FdMemory,
    FdMemoryRef,
    gst::ffi::GstMemory,
    |mem: &gst::MemoryRef| { unsafe { from_glib(ffi::gst_is_fd_memory(mem.as_mut_ptr())) } },
    Memory,
    MemoryRef,
);

impl fmt::Debug for FdMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        FdMemoryRef::fmt(self, f)
    }
}

impl fmt::Debug for FdMemoryRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FdMemory")
            .field("ptr", &self.as_ptr())
            .field("allocator", &self.allocator())
            .field("parent", &self.parent())
            .field("maxsize", &self.maxsize())
            .field("align", &self.align())
            .field("offset", &self.offset())
            .field("size", &self.size())
            .field("flags", &self.flags())
            .field("fd", &self.fd())
            .finish()
    }
}

impl FdMemoryRef {
    #[doc(alias = "gst_fd_memory_get_fd")]
    pub fn fd(&self) -> RawFd {
        assert_initialized_main_thread!();
        unsafe { ffi::gst_fd_memory_get_fd(self.as_mut_ptr()) }
    }
}

impl FdAllocator {
    #[doc(alias = "gst_fd_allocator_alloc")]
    pub unsafe fn alloc(
        &self,
        fd: RawFd,
        size: usize,
        flags: FdMemoryFlags,
    ) -> Result<gst::Memory, glib::BoolError> {
        assert_initialized_main_thread!();
        Option::<_>::from_glib_full(ffi::gst_fd_allocator_alloc(
            self.unsafe_cast_ref::<gst::Allocator>().to_glib_none().0,
            fd,
            size,
            flags.into_glib(),
        ))
        .ok_or_else(|| glib::bool_error!("Failed to allocate memory"))
    }
}
