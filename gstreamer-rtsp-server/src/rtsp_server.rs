// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RTSPServer;
use glib::prelude::*;
use glib::source::SourceId;
use glib::translate::*;

pub trait RTSPServerExtManual: 'static {
    #[doc(alias = "gst_rtsp_server_attach")]
    fn attach(
        &self,
        context: Option<&glib::MainContext>,
    ) -> Result<SourceId, glib::error::BoolError>;
}

impl<O: IsA<RTSPServer>> RTSPServerExtManual for O {
    fn attach(
        &self,
        context: Option<&glib::MainContext>,
    ) -> Result<SourceId, glib::error::BoolError> {
        unsafe {
            match ffi::gst_rtsp_server_attach(
                self.as_ref().to_glib_none().0,
                context.to_glib_none().0,
            ) {
                0 => Err(glib::bool_error!(
                    "Failed to attach main context to RTSP server"
                )),
                id => Ok(from_glib(id)),
            }
        }
    }
}
