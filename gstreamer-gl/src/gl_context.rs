// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};
use libc::uintptr_t;

use crate::{GLContext, GLDisplay, GLPlatform, GLAPI};

impl GLContext {
    pub unsafe fn new_wrapped<T: IsA<GLDisplay>>(
        display: &T,
        handle: uintptr_t,
        context_type: GLPlatform,
        available_apis: GLAPI,
    ) -> Option<Self> {
        from_glib_full(ffi::gst_gl_context_new_wrapped(
            display.as_ref().to_glib_none().0,
            handle,
            context_type.into_glib(),
            available_apis.into_glib(),
        ))
    }

    #[doc(alias = "get_current_gl_context")]
    #[doc(alias = "gst_gl_context_get_current_gl_context")]
    pub fn current_gl_context(context_type: GLPlatform) -> uintptr_t {
        skip_assert_initialized!();
        unsafe { ffi::gst_gl_context_get_current_gl_context(context_type.into_glib()) as uintptr_t }
    }

    #[doc(alias = "get_proc_address_with_platform")]
    #[doc(alias = "gst_gl_context_get_proc_address_with_platform")]
    pub fn proc_address_with_platform(
        context_type: GLPlatform,
        gl_api: GLAPI,
        name: &str,
    ) -> uintptr_t {
        skip_assert_initialized!();
        unsafe {
            ffi::gst_gl_context_get_proc_address_with_platform(
                context_type.into_glib(),
                gl_api.into_glib(),
                name.to_glib_none().0,
            ) as uintptr_t
        }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::GLContext>> Sealed for T {}
}

pub trait GLContextExtManual: sealed::Sealed + IsA<GLContext> + 'static {
    #[doc(alias = "get_gl_context")]
    #[doc(alias = "gst_gl_context_get_gl_context")]
    fn gl_context(&self) -> uintptr_t {
        unsafe { ffi::gst_gl_context_get_gl_context(self.as_ref().to_glib_none().0) as uintptr_t }
    }

    #[doc(alias = "get_proc_address")]
    #[doc(alias = "gst_gl_context_get_proc_address")]
    fn proc_address(&self, name: &str) -> uintptr_t {
        unsafe {
            ffi::gst_gl_context_get_proc_address(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ) as uintptr_t
        }
    }
}

impl<O: IsA<GLContext>> GLContextExtManual for O {}
