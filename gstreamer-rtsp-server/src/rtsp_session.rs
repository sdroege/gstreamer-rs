// Take a look at the license at the top of the repository in the LICENSE file.

use std::mem;

use glib::{prelude::*, translate::*};

use crate::{RTSPSession, RTSPSessionMedia};

pub trait RTSPSessionExtManual: 'static {
    #[doc(alias = "gst_rtsp_session_dup_media")]
    #[doc(alias = "gst_rtsp_session_get_media")]
    fn media(&self, path: &str) -> (Option<RTSPSessionMedia>, i32);
}

impl<O: IsA<RTSPSession>> RTSPSessionExtManual for O {
    fn media(&self, path: &str) -> (Option<RTSPSessionMedia>, i32) {
        #[cfg(feature = "v1_20")]
        unsafe {
            let mut matched = mem::MaybeUninit::uninit();
            let ret = from_glib_full(ffi::gst_rtsp_session_dup_media(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                matched.as_mut_ptr(),
            ));
            (ret, matched.assume_init())
        }
        #[cfg(not(any(feature = "v1_20", docsrs)))]
        unsafe {
            let mut matched = mem::MaybeUninit::uninit();
            let ret = from_glib_none(ffi::gst_rtsp_session_get_media(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                matched.as_mut_ptr(),
            ));
            (ret, matched.assume_init())
        }
    }
}
