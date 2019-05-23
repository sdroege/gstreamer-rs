use glib;
use glib::object::IsA;
use glib::source::SourceId;
use glib::translate::*;
use gst_rtsp_server_sys;
use RTSPServer;

pub trait RTSPServerExtManual: 'static {
    fn attach(&self, context: Option<&glib::MainContext>) -> SourceId;
}

impl<O: IsA<RTSPServer>> RTSPServerExtManual for O {
    fn attach(&self, context: Option<&glib::MainContext>) -> SourceId {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_server_attach(
                self.as_ref().to_glib_none().0,
                context.to_glib_none().0,
            ))
        }
    }
}
