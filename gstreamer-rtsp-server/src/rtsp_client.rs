// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RTSPClient;
use glib::object::IsA;
use glib::source::SourceId;
use glib::translate::*;

pub trait RTSPClientExtManual: 'static {
    fn attach(&self, context: Option<&glib::MainContext>) -> SourceId;
}

impl<O: IsA<RTSPClient>> RTSPClientExtManual for O {
    fn attach(&self, context: Option<&glib::MainContext>) -> SourceId {
        unsafe {
            from_glib(ffi::gst_rtsp_client_attach(
                self.as_ref().to_glib_none().0,
                context.to_glib_none().0,
            ))
        }
    }
}
