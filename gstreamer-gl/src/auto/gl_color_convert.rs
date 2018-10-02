// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use GLContext;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use gst_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct GLColorConvert(Object<ffi::GstGLColorConvert, ffi::GstGLColorConvertClass>): [
        gst::Object => gst_ffi::GstObject,
    ];

    match fn {
        get_type => || ffi::gst_gl_color_convert_get_type(),
    }
}

impl GLColorConvert {
    pub fn new(context: &GLContext) -> GLColorConvert {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_gl_color_convert_new(context.to_glib_none().0))
        }
    }

    pub fn set_caps(&self, in_caps: &gst::Caps, out_caps: &gst::Caps) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_color_convert_set_caps(self.to_glib_none().0, in_caps.to_glib_none().0, out_caps.to_glib_none().0))
        }
    }

    pub fn transform_caps(context: &GLContext, direction: gst::PadDirection, caps: &gst::Caps, filter: &gst::Caps) -> Option<gst::Caps> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_gl_color_convert_transform_caps(context.to_glib_none().0, direction.to_glib(), caps.to_glib_none().0, filter.to_glib_none().0))
        }
    }
}

unsafe impl Send for GLColorConvert {}
unsafe impl Sync for GLColorConvert {}
