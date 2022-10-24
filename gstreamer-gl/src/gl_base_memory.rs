use glib::prelude::*;
use glib::translate::*;
use gst::Memory;

use crate::GLAllocationParams;
use crate::GLBaseMemoryAllocator;

use ffi::GstGLBaseMemory;
use gst::MemoryRef;

gst::memory_object_wrapper!(
    GLBaseMemory,
    GLBaseMemoryRef,
    GstGLBaseMemory,
    |mem: &MemoryRef| { unsafe { from_glib(ffi::gst_is_gl_base_memory(mem.as_mut_ptr())) } },
    Memory,
    MemoryRef
);

impl std::fmt::Debug for GLBaseMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        GLBaseMemoryRef::fmt(self, f)
    }
}

impl std::fmt::Debug for GLBaseMemoryRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        gst::MemoryRef::fmt(self, f)
    }
}

impl GLBaseMemoryRef {
    // Note: only intended for subclass usage to allocate the system memory buffer
    // on demand.  If there is already a non-NULL data pointer in @gl_mem->data,
    // then this function imply returns TRUE.
    // #[doc(alias = "gst_gl_base_memory_alloc_data")]
    // pub fn alloc_data(&mut self) -> bool {
    //     Self::init_once();
    //     unsafe { from_glib(ffi::gst_gl_base_memory_alloc_data(&mut self.0)) }
    // }

    //#[doc(alias = "gst_gl_base_memory_init")]
    //pub fn init<P: IsA<gst::Allocator>, Q: IsA<GLContext>>(&mut self, allocator: &P, parent: Option<&mut gst::Memory>, context: &Q, params: Option<&mut gst::AllocationParams>, size: usize, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:gst_gl_base_memory_init() }
    //}

    #[doc(alias = "gst_gl_base_memory_memcpy")]
    pub unsafe fn memcpy(
        &self,
        dest: &mut GLBaseMemory,
        offset: isize,
        size: isize,
    ) -> Result<(), glib::BoolError> {
        Self::init_once();
        glib::result_from_gboolean!(
            ffi::gst_gl_base_memory_memcpy(
                mut_override(&self.0),
                dest.to_glib_none_mut().0,
                offset,
                size,
            ),
            "Failed to copy memory"
        )
    }

    #[doc(alias = "gst_gl_base_memory_alloc")]
    pub fn alloc<P: IsA<GLBaseMemoryAllocator>>(
        allocator: &P,
        params: &GLAllocationParams,
    ) -> Result<GLBaseMemory, glib::BoolError> {
        skip_assert_initialized!();
        Self::init_once();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_gl_base_memory_alloc(
                allocator.as_ref().to_glib_none().0,
                mut_override(params.to_glib_none().0),
            ))
            .ok_or_else(|| glib::bool_error!("Failed to allocate memory"))
        }
    }

    #[doc(alias = "gst_gl_base_memory_init_once")]
    fn init_once() {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gst_gl_base_memory_init_once();
        }
    }

    pub fn context(&self) -> &crate::GLContext {
        unsafe {
            &*(&(*self.as_ptr()).context as *const *mut ffi::GstGLContext
                as *const crate::GLContext)
        }
    }
}
