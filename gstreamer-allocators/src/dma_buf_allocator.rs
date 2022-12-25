use std::{
    fmt,
    os::unix::prelude::{IntoRawFd, RawFd},
};

use glib::{translate::*, Cast};
use gst::{Memory, MemoryRef};

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use crate::FdMemoryFlags;
use crate::{DmaBufAllocator, FdMemory, FdMemoryRef};

gst::memory_object_wrapper!(
    DmaBufMemory,
    DmaBufMemoryRef,
    gst::ffi::GstMemory,
    |mem: &gst::MemoryRef| { unsafe { from_glib(ffi::gst_is_dmabuf_memory(mem.as_mut_ptr())) } },
    FdMemory,
    FdMemoryRef,
    Memory,
    MemoryRef
);

impl fmt::Debug for DmaBufMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        DmaBufMemoryRef::fmt(self, f)
    }
}

impl fmt::Debug for DmaBufMemoryRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MemoryRef::fmt(self, f)
    }
}

impl DmaBufMemoryRef {
    #[doc(alias = "gst_dmabuf_memory_get_fd")]
    pub fn fd(&self) -> RawFd {
        skip_assert_initialized!();
        unsafe { ffi::gst_dmabuf_memory_get_fd(self.as_mut_ptr()) }
    }
}

impl DmaBufAllocator {
    #[doc(alias = "gst_dmabuf_allocator_alloc")]
    pub unsafe fn alloc<A: IntoRawFd>(
        &self,
        fd: A,
        size: usize,
    ) -> Result<gst::Memory, glib::BoolError> {
        skip_assert_initialized!();
        Option::<_>::from_glib_full(ffi::gst_dmabuf_allocator_alloc(
            self.unsafe_cast_ref::<gst::Allocator>().to_glib_none().0,
            fd.into_raw_fd(),
            size,
        ))
        .ok_or_else(|| glib::bool_error!("Failed to allocate memory"))
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_dmabuf_allocator_alloc_with_flags")]
    pub unsafe fn alloc_with_flags(
        &self,
        fd: RawFd,
        size: usize,
        flags: FdMemoryFlags,
    ) -> Result<gst::Memory, glib::BoolError> {
        skip_assert_initialized!();
        Option::<_>::from_glib_full(ffi::gst_dmabuf_allocator_alloc_with_flags(
            self.unsafe_cast_ref::<gst::Allocator>().to_glib_none().0,
            fd,
            size,
            flags.into_glib(),
        ))
        .ok_or_else(|| glib::bool_error!("Failed to allocate memory"))
    }
}
