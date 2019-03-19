use std::fmt;

use ffi;
use glib;
use glib::translate::*;
use gst;
use gst::prelude::*;

use GLContext;

#[repr(C)]
pub struct GLSyncMeta(ffi::GstGLSyncMeta);

impl GLSyncMeta {
    pub fn add<'a, C: IsA<GLContext>>(
        buffer: &'a mut gst::BufferRef,
        context: &C,
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        unsafe {
            let meta = ffi::gst_buffer_add_gl_sync_meta(
                context.as_ref().to_glib_none().0,
                buffer.as_mut_ptr(),
            );
            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn get_context(&self) -> GLContext {
        unsafe { from_glib_none(self.0.context) }
    }

    pub fn set_sync_point<C: IsA<GLContext>>(&self, context: &C) {
        unsafe {
            ffi::gst_gl_sync_meta_set_sync_point(
                &self.0 as *const _ as *mut _,
                context.as_ref().to_glib_none().0,
            );
        }
    }

    pub fn wait<C: IsA<GLContext>>(&self, context: &C) {
        unsafe {
            ffi::gst_gl_sync_meta_wait(
                &self.0 as *const _ as *mut _,
                context.as_ref().to_glib_none().0,
            );
        }
    }

    pub fn wait_cpu<C: IsA<GLContext>>(&self, context: &C) {
        unsafe {
            ffi::gst_gl_sync_meta_wait_cpu(
                &self.0 as *const _ as *mut _,
                context.as_ref().to_glib_none().0,
            );
        }
    }
}

unsafe impl MetaAPI for GLSyncMeta {
    type GstType = ffi::GstGLSyncMeta;

    fn get_meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_gl_sync_meta_api_get_type()) }
    }
}

impl fmt::Debug for GLSyncMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GLSyncMeta")
            .field("context", &self.get_context())
            .finish()
    }
}
