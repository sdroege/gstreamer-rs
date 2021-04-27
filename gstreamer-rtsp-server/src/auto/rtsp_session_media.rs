// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::RTSPMedia;
use crate::RTSPStreamTransport;
use glib::object::IsA;
use glib::translate::*;
use std::mem;

glib::wrapper! {
    pub struct RTSPSessionMedia(Object<ffi::GstRTSPSessionMedia, ffi::GstRTSPSessionMediaClass>);

    match fn {
        type_ => || ffi::gst_rtsp_session_media_get_type(),
    }
}

impl RTSPSessionMedia {
    #[doc(alias = "gst_rtsp_session_media_new")]
    pub fn new<P: IsA<RTSPMedia>>(path: &str, media: &P) -> RTSPSessionMedia {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_rtsp_session_media_new(
                path.to_glib_none().0,
                media.as_ref().to_glib_full(),
            ))
        }
    }
}

unsafe impl Send for RTSPSessionMedia {}
unsafe impl Sync for RTSPSessionMedia {}

pub const NONE_RTSP_SESSION_MEDIA: Option<&RTSPSessionMedia> = None;

pub trait RTSPSessionMediaExt: 'static {
    //#[doc(alias = "gst_rtsp_session_media_alloc_channels")]
    //fn alloc_channels(&self, range: /*Ignored*/gst_rtsp::RTSPRange) -> bool;

    #[doc(alias = "gst_rtsp_session_media_get_base_time")]
    fn base_time(&self) -> gst::ClockTime;

    #[doc(alias = "gst_rtsp_session_media_get_media")]
    fn media(&self) -> Option<RTSPMedia>;

    #[doc(alias = "gst_rtsp_session_media_get_rtpinfo")]
    fn rtpinfo(&self) -> Option<glib::GString>;

    //#[doc(alias = "gst_rtsp_session_media_get_rtsp_state")]
    //fn rtsp_state(&self) -> /*Ignored*/gst_rtsp::RTSPState;

    #[doc(alias = "gst_rtsp_session_media_get_transport")]
    fn transport(&self, idx: u32) -> Option<RTSPStreamTransport>;

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[doc(alias = "gst_rtsp_session_media_get_transports")]
    fn transports(&self) -> Vec<RTSPStreamTransport>;

    #[doc(alias = "gst_rtsp_session_media_matches")]
    fn matches(&self, path: &str) -> Option<i32>;

    //#[doc(alias = "gst_rtsp_session_media_set_rtsp_state")]
    //fn set_rtsp_state(&self, state: /*Ignored*/gst_rtsp::RTSPState);

    #[doc(alias = "gst_rtsp_session_media_set_state")]
    fn set_state(&self, state: gst::State) -> Result<(), glib::error::BoolError>;

    //#[doc(alias = "gst_rtsp_session_media_set_transport")]
    //fn set_transport<P: IsA<RTSPStream>>(&self, stream: &P, tr: /*Ignored*/&mut gst_rtsp::RTSPTransport) -> Option<RTSPStreamTransport>;
}

impl<O: IsA<RTSPSessionMedia>> RTSPSessionMediaExt for O {
    //fn alloc_channels(&self, range: /*Ignored*/gst_rtsp::RTSPRange) -> bool {
    //    unsafe { TODO: call ffi:gst_rtsp_session_media_alloc_channels() }
    //}

    fn base_time(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_rtsp_session_media_get_base_time(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn media(&self) -> Option<RTSPMedia> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_session_media_get_media(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn rtpinfo(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_session_media_get_rtpinfo(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn rtsp_state(&self) -> /*Ignored*/gst_rtsp::RTSPState {
    //    unsafe { TODO: call ffi:gst_rtsp_session_media_get_rtsp_state() }
    //}

    fn transport(&self, idx: u32) -> Option<RTSPStreamTransport> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_session_media_get_transport(
                self.as_ref().to_glib_none().0,
                idx,
            ))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    fn transports(&self) -> Vec<RTSPStreamTransport> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_rtsp_session_media_get_transports(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn matches(&self, path: &str) -> Option<i32> {
        unsafe {
            let mut matched = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_rtsp_session_media_matches(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                matched.as_mut_ptr(),
            ));
            let matched = matched.assume_init();
            if ret {
                Some(matched)
            } else {
                None
            }
        }
    }

    //fn set_rtsp_state(&self, state: /*Ignored*/gst_rtsp::RTSPState) {
    //    unsafe { TODO: call ffi:gst_rtsp_session_media_set_rtsp_state() }
    //}

    fn set_state(&self, state: gst::State) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_rtsp_session_media_set_state(
                    self.as_ref().to_glib_none().0,
                    state.into_glib()
                ),
                "Failed to set state of session media"
            )
        }
    }

    //fn set_transport<P: IsA<RTSPStream>>(&self, stream: &P, tr: /*Ignored*/&mut gst_rtsp::RTSPTransport) -> Option<RTSPStreamTransport> {
    //    unsafe { TODO: call ffi:gst_rtsp_session_media_set_transport() }
    //}
}
