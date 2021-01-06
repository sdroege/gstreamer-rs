use glib::translate::*;

use crate::GLBaseMemoryRef;
use crate::GLFormat;
use crate::GLTextureTarget;

use ffi::{GstGLBaseMemory, GstGLMemory};
use gst::{result_from_gboolean, LoggableError, CAT_RUST};

gst::mini_object_wrapper!(GLMemory, GLMemoryRef, GstGLMemory);

impl std::ops::Deref for GLMemoryRef {
    type Target = GLBaseMemoryRef;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(&self.0.mem as *const GstGLBaseMemory).cast::<Self::Target>() }
    }
}

impl std::ops::DerefMut for GLMemoryRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(&mut self.0.mem as *mut GstGLBaseMemory).cast::<Self::Target>() }
    }
}

impl GLMemoryRef {
    // rustdoc-stripper-ignore-next
    /// # Safety
    /// `tex_id` is not validated to be a valid GL texture, which may lead to undefined behaviour.
    #[doc(alias = "gst_gl_memory_copy_into")]
    pub unsafe fn copy_into(
        &self,
        tex_id: u32,
        target: GLTextureTarget,
        tex_format: GLFormat,
        width: i32,
        height: i32,
    ) -> Result<(), LoggableError> {
        Self::init_once();
        result_from_gboolean!(
            ffi::gst_gl_memory_copy_into(
                mut_override(&self.0),
                tex_id,
                target.to_glib(),
                tex_format.to_glib(),
                width,
                height,
            ),
            CAT_RUST,
            "Failed to copy memory into GL texture"
        )
    }

    // rustdoc-stripper-ignore-next
    /// # Safety
    /// `tex_id` is not validated to be a valid GL texture, which may lead to undefined behaviour.
    #[doc(alias = "gst_gl_memory_copy_teximage")]
    pub unsafe fn copy_teximage(
        &self,
        tex_id: u32,
        out_target: GLTextureTarget,
        out_tex_format: GLFormat,
        out_width: i32,
        out_height: i32,
    ) -> Result<(), LoggableError> {
        Self::init_once();
        result_from_gboolean!(
            ffi::gst_gl_memory_copy_teximage(
                mut_override(&self.0),
                tex_id,
                out_target.to_glib(),
                out_tex_format.to_glib(),
                out_width,
                out_height,
            ),
            CAT_RUST,
            "Failed to copy memory into GL texture"
        )
    }

    #[doc(alias = "gst_gl_memory_get_texture_format")]
    pub fn texture_format(&self) -> GLFormat {
        unsafe { from_glib(ffi::gst_gl_memory_get_texture_format(mut_override(&self.0))) }
    }

    #[doc(alias = "gst_gl_memory_get_texture_height")]
    pub fn texture_height(&self) -> i32 {
        unsafe { ffi::gst_gl_memory_get_texture_height(mut_override(&self.0)) }
    }

    #[doc(alias = "gst_gl_memory_get_texture_id")]
    pub fn texture_id(&self) -> u32 {
        unsafe { ffi::gst_gl_memory_get_texture_id(mut_override(&self.0)) }
    }

    #[doc(alias = "gst_gl_memory_get_texture_target")]
    pub fn texture_target(&self) -> GLTextureTarget {
        unsafe { from_glib(ffi::gst_gl_memory_get_texture_target(mut_override(&self.0))) }
    }

    #[doc(alias = "gst_gl_memory_get_texture_width")]
    pub fn texture_width(&self) -> i32 {
        unsafe { ffi::gst_gl_memory_get_texture_width(mut_override(&self.0)) }
    }

    //#[doc(alias = "gst_gl_memory_init")]
    //pub fn init<P: IsA<gst::Allocator>, Q: IsA<GLContext>>(&self, allocator: &P, parent: Option<&mut gst::Memory>, context: &Q, target: GLTextureTarget, tex_format: GLFormat, params: Option<&mut gst::AllocationParams>, info: &mut gst_video::VideoInfo, plane: u32, valign: Option<&mut gst_video::VideoAlignment>, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:gst_gl_memory_init() }
    //}

    //#[doc(alias = "gst_gl_memory_read_pixels")]
    //pub fn read_pixels(&self, write_pointer: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
    //    unsafe { TODO: call ffi:gst_gl_memory_read_pixels() }
    //}

    //#[doc(alias = "gst_gl_memory_texsubimage")]
    //pub fn texsubimage(&mut self, read_pointer: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:gst_gl_memory_texsubimage() }
    //}

    #[doc(alias = "gst_gl_memory_init_once")]
    fn init_once() {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gst_gl_memory_init_once();
        }
    }

    //#[doc(alias = "gst_gl_memory_setup_buffer")]
    //pub fn setup_buffer<P: IsA<GLMemoryAllocator>>(allocator: &P, buffer: &gst::Buffer, params: &mut GLVideoAllocationParams, tex_formats: /*Unimplemented*/Option<&CArray TypeId { ns_id: 1, id: 55 }>, wrapped_data: /*Unimplemented*/&[&Fundamental: Pointer]) -> bool {
    //    unsafe { TODO: call ffi:gst_gl_memory_setup_buffer() }
    //}
}
