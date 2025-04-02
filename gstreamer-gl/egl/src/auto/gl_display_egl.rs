// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ffi;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstGLDisplayEGL")]
    pub struct GLDisplayEGL(Object<ffi::GstGLDisplayEGL, ffi::GstGLDisplayEGLClass>) @extends gst_gl::GLDisplay, gst::Object;

    match fn {
        type_ => || ffi::gst_gl_display_egl_get_type(),
    }
}

impl GLDisplayEGL {
    pub const NONE: Option<&'static GLDisplayEGL> = None;

    #[doc(alias = "gst_gl_display_egl_new")]
    pub fn new() -> Result<GLDisplayEGL, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_gl_display_egl_new())
                .ok_or_else(|| glib::bool_error!("Failed to create EGL display"))
        }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "gst_gl_display_egl_new_surfaceless")]
    pub fn new_surfaceless() -> Result<GLDisplayEGL, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_gl_display_egl_new_surfaceless())
                .ok_or_else(|| glib::bool_error!("Failed to create surfaceless EGL display"))
        }
    }

    #[doc(alias = "gst_gl_display_egl_from_gl_display")]
    pub fn from_gl_display(display: &impl IsA<gst_gl::GLDisplay>) -> Option<GLDisplayEGL> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_gl_display_egl_from_gl_display(
                display.as_ref().to_glib_none().0,
            ))
        }
    }
}

unsafe impl Send for GLDisplayEGL {}
unsafe impl Sync for GLDisplayEGL {}

pub trait GLDisplayEGLExt: IsA<GLDisplayEGL> + 'static {
    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_gl_display_egl_set_foreign")]
    fn set_foreign(&self, foreign: bool) {
        unsafe {
            ffi::gst_gl_display_egl_set_foreign(
                self.as_ref().to_glib_none().0,
                foreign.into_glib(),
            );
        }
    }
}

impl<O: IsA<GLDisplayEGL>> GLDisplayEGLExt for O {}
