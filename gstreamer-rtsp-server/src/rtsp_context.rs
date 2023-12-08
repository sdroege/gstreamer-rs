// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    marker::PhantomData,
    ptr::{self, addr_of},
};

use glib::{translate::*, ObjectType};
use gst_rtsp::{rtsp_message::RTSPMessage, RTSPUrl};

use crate::{RTSPClient, RTSPSession, RTSPToken};

#[derive(Debug, PartialEq, Eq)]
#[doc(alias = "GstRTSPContext")]
#[repr(transparent)]
pub struct RTSPContext(ptr::NonNull<ffi::GstRTSPContext>);

impl RTSPContext {
    #[inline]
    pub fn with_current_context<F: FnOnce(&RTSPContext) -> T, T>(func: F) -> Option<T> {
        unsafe {
            let ptr = ffi::gst_rtsp_context_get_current();
            if ptr.is_null() {
                return None;
            }

            let ctx = RTSPContext(ptr::NonNull::new_unchecked(ptr));
            Some(func(&ctx))
        }
    }

    #[inline]
    pub fn uri(&self) -> Option<&RTSPUrl> {
        unsafe {
            let ptr = self.0.as_ptr();
            if (*ptr).uri.is_null() {
                None
            } else {
                let uri = RTSPUrl::from_glib_ptr_borrow(
                    addr_of!((*ptr).uri) as *const *const gst_rtsp::ffi::GstRTSPUrl
                );
                Some(uri)
            }
        }
    }

    #[inline]
    pub fn client(&self) -> Option<&RTSPClient> {
        unsafe {
            let ptr = self.0.as_ptr();
            if (*ptr).client.is_null() {
                None
            } else {
                let client = RTSPClient::from_glib_ptr_borrow(
                    addr_of!((*ptr).client) as *const *const ffi::GstRTSPClient
                );
                Some(client)
            }
        }
    }

    #[inline]
    pub fn request(&self) -> Option<&RTSPMessage> {
        unsafe {
            let ptr = self.0.as_ptr();
            if (*ptr).request.is_null() {
                None
            } else {
                let msg = RTSPMessage::from_glib_ptr_borrow(
                    addr_of!((*ptr).request) as *const *const gst_rtsp::ffi::GstRTSPMessage
                );
                Some(msg)
            }
        }
    }

    #[inline]
    pub fn response(&self) -> Option<&RTSPMessage> {
        unsafe {
            let ptr = self.0.as_ptr();
            if (*ptr).response.is_null() {
                None
            } else {
                let msg = RTSPMessage::from_glib_ptr_borrow(
                    addr_of!((*ptr).response) as *const *const gst_rtsp::ffi::GstRTSPMessage
                );
                Some(msg)
            }
        }
    }

    #[inline]
    pub fn session(&self) -> Option<&RTSPSession> {
        unsafe {
            let ptr = self.0.as_ptr();
            if (*ptr).session.is_null() {
                None
            } else {
                let sess = RTSPSession::from_glib_ptr_borrow(
                    addr_of!((*ptr).session) as *const *const ffi::GstRTSPSession
                );
                Some(sess)
            }
        }
    }

    #[inline]
    pub fn token(&self) -> Option<RTSPToken> {
        unsafe {
            let ptr = self.0.as_ptr();
            if (*ptr).token.is_null() {
                None
            } else {
                let token = RTSPToken::from_glib_none((*ptr).token as *const ffi::GstRTSPToken);
                Some(token)
            }
        }
    }

    #[cfg(feature = "v1_22")]
    #[doc(alias = "gst_rtsp_context_set_token")]
    pub fn set_token(&self, token: RTSPToken) {
        unsafe {
            ffi::gst_rtsp_context_set_token(self.0.as_ptr(), token.into_glib_ptr());
        }
    }

    // TODO: Add additional getters for all the contained fields as needed
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::GstRTSPContext> for RTSPContext {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::GstRTSPContext) -> Borrowed<Self> {
        debug_assert!(!ptr.is_null());
        Borrowed::new(RTSPContext(ptr::NonNull::new_unchecked(ptr)))
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut ffi::GstRTSPContext> for RTSPContext {
    type Storage = PhantomData<&'a RTSPContext>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GstRTSPContext, Self> {
        Stash(self.0.as_ptr(), PhantomData)
    }

    fn to_glib_full(&self) -> *mut ffi::GstRTSPContext {
        unimplemented!()
    }
}
