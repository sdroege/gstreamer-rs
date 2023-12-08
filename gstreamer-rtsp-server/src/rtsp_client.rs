// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RTSPClient, RTSPSession};
use glib::{prelude::*, source::SourceId, translate::*};
use gst_rtsp::rtsp_message::RTSPMessage;

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::RTSPClient>> Sealed for T {}
}

pub trait RTSPClientExtManual: sealed::Sealed + IsA<RTSPClient> + 'static {
    #[doc(alias = "gst_rtsp_client_attach")]
    fn attach(&self, context: Option<&glib::MainContext>) -> SourceId {
        unsafe {
            from_glib(ffi::gst_rtsp_client_attach(
                self.as_ref().to_glib_none().0,
                context.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_client_send_message")]
    fn send_message(
        &self,
        message: &RTSPMessage,
        session: Option<&RTSPSession>,
    ) -> gst_rtsp::RTSPResult {
        unsafe {
            from_glib(ffi::gst_rtsp_client_send_message(
                self.as_ref().to_glib_none().0,
                session.to_glib_none().0,
                message.to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<RTSPClient>> RTSPClientExtManual for O {}
