use glib;
use glib::translate::*;
use gst_rtsp_server_sys;

use gst::prelude::*;

gst_define_mini_object_wrapper!(
    RTSPThread,
    RTSPThreadRef,
    gst_rtsp_server_sys::GstRTSPThread,
    [],
    || gst_rtsp_server_sys::gst_rtsp_thread_get_type()
);

impl RTSPThread {
    pub fn new(type_: ::RTSPThreadType) -> Option<Self> {
        unsafe { from_glib_full(gst_rtsp_server_sys::gst_rtsp_thread_new(type_.to_glib())) }
    }
}

impl RTSPThreadRef {
    pub fn reuse(&self) -> bool {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_thread_reuse(
                self.as_mut_ptr(),
            ))
        }
    }

    pub fn stop(&self) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_thread_stop(self.as_mut_ptr());
        }
    }

    pub fn type_(&self) -> ::RTSPThreadType {
        unsafe { from_glib((*self.as_ptr()).type_) }
    }

    pub fn context(&self) -> glib::MainContext {
        unsafe { from_glib_none((*self.as_ptr()).context) }
    }

    pub fn loop_(&self) -> glib::MainLoop {
        unsafe { from_glib_none((*self.as_ptr()).loop_) }
    }
}
