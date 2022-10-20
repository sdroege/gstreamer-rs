// Take a look at the license at the top of the repository in the LICENSE file.

use crate::GLDisplay;
use glib::prelude::*;
use glib::translate::*;
use gst::ContextRef;
use std::ptr;

pub trait ContextGLExt {
    #[doc(alias = "get_gl_display")]
    #[doc(alias = "gst_context_get_gl_display")]
    fn gl_display(&self) -> Option<GLDisplay>;
    #[doc(alias = "gst_context_set_gl_display")]
    fn set_gl_display<'a, T: IsA<GLDisplay>>(&self, display: impl Into<Option<&'a T>>);
}

impl ContextGLExt for ContextRef {
    fn gl_display(&self) -> Option<GLDisplay> {
        unsafe {
            let mut display = ptr::null_mut();
            if from_glib(ffi::gst_context_get_gl_display(
                self.as_mut_ptr(),
                &mut display,
            )) {
                Some(from_glib_full(display))
            } else {
                None
            }
        }
    }

    fn set_gl_display<'a, T: IsA<GLDisplay>>(&self, display: impl Into<Option<&'a T>>) {
        unsafe {
            ffi::gst_context_set_gl_display(
                self.as_mut_ptr(),
                display.into().map(|d| d.as_ref()).to_glib_none().0,
            );
        }
    }
}
