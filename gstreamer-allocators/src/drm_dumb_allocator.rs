use std::{fmt, mem, os::unix::prelude::IntoRawFd};

use glib::{prelude::*, translate::*};
use gst::{Memory, MemoryRef};

use crate::{DRMDumbAllocator, DmaBufMemory};

gst::memory_object_wrapper!(
    DRMDumbMemory,
    DRMDumbMemoryRef,
    gst::ffi::GstMemory,
    |mem: &gst::MemoryRef| { unsafe { from_glib(ffi::gst_is_drm_dumb_memory(mem.as_mut_ptr())) } },
    Memory,
    MemoryRef
);

impl fmt::Debug for DRMDumbMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        DRMDumbMemoryRef::fmt(self, f)
    }
}

impl fmt::Debug for DRMDumbMemoryRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MemoryRef::fmt(self, f)
    }
}

impl DRMDumbMemoryRef {
    #[doc(alias = "gst_drm_dumb_memory_get_handle")]
    pub fn fd(&self) -> u32 {
        skip_assert_initialized!();
        unsafe { ffi::gst_drm_dumb_memory_get_handle(self.as_mut_ptr()) }
    }

    #[doc(alias = "gst_drm_dumb_memory_export_dmabuf")]
    pub fn export_dmabuf(&self) -> Result<DmaBufMemory, glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            Option::<DmaBufMemory>::from_glib_full(ffi::gst_drm_dumb_memory_export_dmabuf(
                self.as_mut_ptr(),
            ))
            .ok_or_else(|| glib::bool_error!("Failed to export as dmabuf"))
        }
    }
}

impl DRMDumbAllocator {
    #[doc(alias = "gst_drm_dumb_allocator_new_with_fd")]
    #[doc(alias = "new_with_fd")]
    pub fn with_fd<A: IntoRawFd>(drm_fd: A) -> Result<DRMDumbAllocator, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<gst::Allocator>::from_glib_full(ffi::gst_drm_dumb_allocator_new_with_fd(
                drm_fd.into_raw_fd(),
            ))
            .map(|o| o.unsafe_cast())
            .ok_or_else(|| glib::bool_error!("Failed to create allocator"))
        }
    }

    #[doc(alias = "gst_drm_dumb_allocator_alloc")]
    pub unsafe fn alloc(
        &self,
        drm_fourcc: u32,
        width: u32,
        height: u32,
    ) -> Result<(gst::Memory, u32), glib::BoolError> {
        skip_assert_initialized!();
        let mut out_pitch = mem::MaybeUninit::uninit();
        Option::<_>::from_glib_full(ffi::gst_drm_dumb_allocator_alloc(
            self.to_glib_none().0,
            drm_fourcc,
            width,
            height,
            out_pitch.as_mut_ptr(),
        ))
        .ok_or_else(|| glib::bool_error!("Failed to allocate memory"))
        .map(|mem| (mem, unsafe { out_pitch.assume_init() }))
    }
}
