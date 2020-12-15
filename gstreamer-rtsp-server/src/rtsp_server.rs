// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RTSPServer;
use glib::object::IsA;
use glib::source::SourceId;
use glib::translate::*;

pub trait RTSPServerExtManual: 'static {
    fn attach(&self, context: Option<&glib::MainContext>) -> SourceId;
}

impl<O: IsA<RTSPServer>> RTSPServerExtManual for O {
    fn attach(&self, context: Option<&glib::MainContext>) -> SourceId {
        unsafe {
            from_glib(ffi::gst_rtsp_server_attach(
                self.as_ref().to_glib_none().0,
                context.to_glib_none().0,
            ))
        }
    }
}
