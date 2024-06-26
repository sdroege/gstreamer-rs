// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, RTSPServer};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstRTSPOnvifServer")]
    pub struct RTSPOnvifServer(Object<ffi::GstRTSPOnvifServer, ffi::GstRTSPOnvifServerClass>) @extends RTSPServer;

    match fn {
        type_ => || ffi::gst_rtsp_onvif_server_get_type(),
    }
}

impl RTSPOnvifServer {
    pub const NONE: Option<&'static RTSPOnvifServer> = None;

    #[doc(alias = "gst_rtsp_onvif_server_new")]
    pub fn new() -> RTSPOnvifServer {
        assert_initialized_main_thread!();
        unsafe { RTSPServer::from_glib_full(ffi::gst_rtsp_onvif_server_new()).unsafe_cast() }
    }
}

impl Default for RTSPOnvifServer {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPOnvifServer {}
unsafe impl Sync for RTSPOnvifServer {}
