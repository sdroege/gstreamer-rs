// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

gst::mini_object_wrapper!(RTSPThread, RTSPThreadRef, ffi::GstRTSPThread, || {
    ffi::gst_rtsp_thread_get_type()
});

impl RTSPThread {
    #[doc(alias = "gst_rtsp_thread_new")]
    pub fn new(type_: crate::RTSPThreadType) -> Option<Self> {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_rtsp_thread_new(type_.into_glib())) }
    }
}

impl RTSPThreadRef {
    #[doc(alias = "gst_rtsp_thread_reuse")]
    pub fn reuse(&self) -> bool {
        unsafe { from_glib(ffi::gst_rtsp_thread_reuse(self.as_mut_ptr())) }
    }

    #[doc(alias = "gst_rtsp_thread_stop")]
    pub fn stop(&self) {
        unsafe {
            ffi::gst_rtsp_thread_stop(self.as_mut_ptr());
        }
    }

    pub fn type_(&self) -> crate::RTSPThreadType {
        unsafe { from_glib((*self.as_ptr()).type_) }
    }

    pub fn context(&self) -> glib::MainContext {
        unsafe { from_glib_none((*self.as_ptr()).context) }
    }

    pub fn loop_(&self) -> glib::MainLoop {
        unsafe { from_glib_none((*self.as_ptr()).loop_) }
    }
}
