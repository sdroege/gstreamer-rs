use std::fmt;

use glib::translate::*;
use gst::{Memory, MemoryRef};

gst::memory_object_wrapper!(
    PhysMemory,
    PhysMemoryRef,
    gst::ffi::GstMemory,
    |mem: &gst::MemoryRef| { unsafe { from_glib(ffi::gst_is_phys_memory(mem.as_mut_ptr())) } },
    Memory,
    MemoryRef,
);

impl fmt::Debug for PhysMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PhysMemoryRef::fmt(self, f)
    }
}

impl fmt::Debug for PhysMemoryRef {
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
            .field("phys_addr", &format!("{:x}", self.phys_addr()))
            .finish()
    }
}

impl PhysMemoryRef {
    #[doc(alias = "gst_phys_memory_get_phys_addr")]
    pub fn phys_addr(&self) -> libc::uintptr_t {
        skip_assert_initialized!();
        unsafe { ffi::gst_phys_memory_get_phys_addr(self.as_mut_ptr()) }
    }
}
