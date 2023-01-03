use std::ptr;

use glib::{object::IsA, translate::*};

use crate::{GLContext, GLDisplay};

#[doc(alias = "gst_gl_handle_context_query")]
pub fn gl_handle_context_query(
    element: &impl IsA<gst::Element>,
    query: &mut gst::query::Context,
    display: Option<&impl IsA<GLDisplay>>,
    context: Option<&impl IsA<GLContext>>,
    other_context: Option<&impl IsA<GLContext>>,
) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_gl_handle_context_query(
            element.as_ref().to_glib_none().0,
            query.as_mut_ptr(),
            display.map(|p| p.as_ref()).to_glib_none().0,
            context.map(|p| p.as_ref()).to_glib_none().0,
            other_context.map(|p| p.as_ref()).to_glib_none().0,
        ))
    }
}

#[doc(alias = "gst_gl_handle_set_context")]
pub fn gl_handle_set_context(
    element: &impl IsA<gst::Element>,
    context: &gst::Context,
) -> (Option<GLDisplay>, Option<GLContext>) {
    assert_initialized_main_thread!();
    unsafe {
        let mut display = ptr::null_mut();
        let mut other_context = ptr::null_mut();
        let ret = from_glib(ffi::gst_gl_handle_set_context(
            element.as_ref().to_glib_none().0,
            context.to_glib_none().0,
            &mut display,
            &mut other_context,
        ));
        if ret {
            (from_glib_full(display), from_glib_full(other_context))
        } else {
            (None, None)
        }
    }
}
