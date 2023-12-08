use crate::{RTSPAuthCredential, RTSPHeaderField, RTSPStatusCode};
use glib::translate::*;

glib::wrapper! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GstRTSPMessage")]
    pub struct RTSPMessage(Boxed<ffi::GstRTSPMessage>);

    match fn {
        copy => |ptr| {
            let mut copy = std::ptr::null_mut();
            let res = ffi::gst_rtsp_message_copy(ptr, &mut copy);
            debug_assert_eq!(res, ffi::GST_RTSP_OK);
            copy
        },
        free => |ptr| {
            let res = ffi::gst_rtsp_message_free(ptr);
            debug_assert_eq!(res, ffi::GST_RTSP_OK);
        },
        type_ => || ffi::gst_rtsp_msg_get_type(),
    }
}

impl RTSPMessage {
    pub const NONE: Option<&'static RTSPMessage> = None;

    #[doc(alias = "gst_rtsp_message_add_header")]
    pub fn add_header(&self, header: RTSPHeaderField, value: &str) {
        let ptr = self.to_glib_none().0;
        unsafe {
            ffi::gst_rtsp_message_add_header(ptr, header.into_glib(), value.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_rtsp_message_init_response")]
    pub fn init_response(&self, code: RTSPStatusCode, request: Option<&RTSPMessage>) {
        let ptr = self.to_glib_none().0;
        unsafe {
            ffi::gst_rtsp_message_init_response(
                ptr,
                code.into_glib(),
                ffi::gst_rtsp_status_as_text(code.into_glib()),
                request.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_rtsp_message_parse_auth_credentials")]
    pub fn parse_auth_credentials(&self) -> glib::collections::PtrSlice<RTSPAuthCredential> {
        unsafe {
            let credentials = ffi::gst_rtsp_message_parse_auth_credentials(
                self.to_glib_none().0,
                ffi::GST_RTSP_HDR_AUTHORIZATION,
            );
            FromGlibPtrContainer::from_glib_full(credentials)
        }
    }
}
