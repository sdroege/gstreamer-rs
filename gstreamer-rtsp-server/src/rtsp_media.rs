use glib::object::IsA;
use glib::translate::*;
use gst;
use gst_rtsp_server_sys;

use RTSPMedia;

pub trait RTSPMediaExtManual: 'static {
    fn take_pipeline<P: IsA<gst::Pipeline>>(&self, pipeline: &P);
}

impl<O: IsA<RTSPMedia>> RTSPMediaExtManual for O {
    fn take_pipeline<P: IsA<gst::Pipeline>>(&self, pipeline: &P) {
        unsafe {
            let pipeline = pipeline.as_ref().to_glib_full();
            // See https://gitlab.freedesktop.org/gstreamer/gst-rtsp-server/merge_requests/109
            gobject_sys::g_object_force_floating(pipeline as *mut _);
            gst_rtsp_server_sys::gst_rtsp_media_take_pipeline(
                self.as_ref().to_glib_none().0,
                pipeline,
            );
            if gobject_sys::g_object_is_floating(pipeline as *mut _) != glib_sys::GFALSE {
                gobject_sys::g_object_ref_sink(pipeline as *mut _);
            }
        }
    }
}
