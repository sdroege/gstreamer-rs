use glib::{prelude::*, translate::*};

use crate::{GLFramebuffer, GLMemoryRef};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::GLFramebuffer>> Sealed for T {}
}

pub trait GLFramebufferExtManual: sealed::Sealed + IsA<GLFramebuffer> + 'static {
    #[doc(alias = "gst_gl_framebuffer_draw_to_texture")]
    fn draw_to_texture<F: FnOnce()>(&self, mem: &mut GLMemoryRef, func: F) {
        let mut func = std::mem::ManuallyDrop::new(func);
        let user_data: *mut F = &mut *func;

        unsafe extern "C" fn trampoline<F: FnOnce()>(
            data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let func = std::ptr::read(data as *mut F);
            func();
            glib::ffi::GTRUE
        }

        unsafe {
            ffi::gst_gl_framebuffer_draw_to_texture(
                self.as_ref().to_glib_none().0,
                mem.as_mut_ptr(),
                Some(trampoline::<F>),
                user_data as glib::ffi::gpointer,
            );
        }
    }
}

impl<O: IsA<GLFramebuffer>> GLFramebufferExtManual for O {}
