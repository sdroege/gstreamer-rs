use std::ptr;

use glib::{object::IsA, translate::*};

pub use crate::auto::functions::*;
use crate::{GLContext, GLDisplay};

#[doc(alias = "gst_gl_handle_context_query")]
pub fn gl_handle_context_query(
    element: &impl IsA<gst::Element>,
    query: &mut gst::query::Context,
    display: Option<&impl IsA<GLDisplay>>,
    context: Option<&impl IsA<GLContext>>,
    other_context: Option<&impl IsA<GLContext>>,
) -> bool {
    skip_assert_initialized!();
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
    skip_assert_initialized!();
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

#[cfg(feature = "v1_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
#[doc(alias = "gst_gl_swizzle_invert")]
pub fn gl_swizzle_invert(swizzle: [i32; 4]) -> [i32; 4] {
    unsafe {
        use std::mem;

        let mut inversion = mem::MaybeUninit::uninit();
        ffi::gst_gl_swizzle_invert(
            mut_override(swizzle.as_ptr() as *const _),
            inversion.as_mut_ptr(),
        );
        inversion.assume_init()
    }
}

#[cfg(feature = "v1_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
#[doc(alias = "gst_gl_video_format_swizzle")]
pub fn gl_video_format_swizzle(video_format: gst_video::VideoFormat) -> Option<[i32; 4]> {
    unsafe {
        use std::mem;

        let mut swizzle = mem::MaybeUninit::uninit();
        let res = from_glib(ffi::gst_gl_video_format_swizzle(
            video_format.into_glib(),
            swizzle.as_mut_ptr(),
        ));
        if res {
            Some(swizzle.assume_init())
        } else {
            None
        }
    }
}
