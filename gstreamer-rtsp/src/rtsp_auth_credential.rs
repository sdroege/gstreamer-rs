use crate::{RTSPAuthCredential, RTSPAuthMethod, RTSPAuthParam};
use ffi::GstRTSPAuthCredential;
use glib::translate::*;

impl RTSPAuthCredential {
    pub fn scheme(&self) -> RTSPAuthMethod {
        let ptr: *mut GstRTSPAuthCredential = self.to_glib_none().0;
        unsafe { from_glib((*ptr).scheme) }
    }

    pub fn authorization(&self) -> Option<&str> {
        let ptr: *mut GstRTSPAuthCredential = self.to_glib_none().0;
        unsafe {
            if (*ptr).authorization.is_null() {
                None
            } else {
                std::ffi::CStr::from_ptr((*ptr).authorization).to_str().ok()
            }
        }
    }

    pub fn params(&self) -> glib::collections::PtrSlice<RTSPAuthParam> {
        let ptr: *mut GstRTSPAuthCredential = self.to_glib_none().0;
        unsafe { FromGlibPtrContainer::from_glib_none((*ptr).params) }
    }
}
