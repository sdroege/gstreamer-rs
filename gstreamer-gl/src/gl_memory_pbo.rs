use ffi::GstGLMemoryPBO;
use glib::translate::*;
use gst::{Memory, MemoryRef};

use crate::{GLBaseMemory, GLBaseMemoryRef, GLMemory, GLMemoryRef};

gst::memory_object_wrapper!(
    GLMemoryPBO,
    GLMemoryPBORef,
    GstGLMemoryPBO,
    |mem: &MemoryRef| { unsafe { from_glib(ffi::gst_is_gl_memory_pbo(mem.as_mut_ptr())) } },
    GLMemory,
    GLMemoryRef,
    GLBaseMemory,
    GLBaseMemoryRef,
    Memory,
    MemoryRef
);

impl std::fmt::Debug for GLMemoryPBO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        GLMemoryPBORef::fmt(self, f)
    }
}

impl std::fmt::Debug for GLMemoryPBORef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        GLMemoryRef::fmt(self, f)
    }
}
