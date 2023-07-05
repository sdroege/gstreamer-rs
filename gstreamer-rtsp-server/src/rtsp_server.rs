// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, source::SourceId, translate::*};

use crate::RTSPServer;

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::RTSPServer>> Sealed for T {}
}

pub trait RTSPServerExtManual: sealed::Sealed + IsA<RTSPServer> + 'static {
    #[doc(alias = "gst_rtsp_server_attach")]
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

impl<O: IsA<RTSPServer>> RTSPServerExtManual for O {}
