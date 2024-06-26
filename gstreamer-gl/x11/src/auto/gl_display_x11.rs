// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ffi;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GstGLDisplayX11")]
    pub struct GLDisplayX11(Object<ffi::GstGLDisplayX11, ffi::GstGLDisplayX11Class>) @extends gst_gl::GLDisplay, gst::Object;

    match fn {
        type_ => || ffi::gst_gl_display_x11_get_type(),
    }
}

impl GLDisplayX11 {
    pub const NONE: Option<&'static GLDisplayX11> = None;

    #[doc(alias = "gst_gl_display_x11_new")]
    pub fn new(name: Option<&str>) -> Result<GLDisplayX11, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_gl_display_x11_new(name.to_glib_none().0))
                .ok_or_else(|| glib::bool_error!("Failed to create X11 display"))
        }
    }

    //#[doc(alias = "gst_gl_display_x11_new_with_display")]
    //#[doc(alias = "new_with_display")]
    //pub fn with_display(display: /*Unimplemented*/Basic: Pointer) -> GLDisplayX11 {
    //    unsafe { TODO: call ffi:gst_gl_display_x11_new_with_display() }
    //}
}

unsafe impl Send for GLDisplayX11 {}
unsafe impl Sync for GLDisplayX11 {}
