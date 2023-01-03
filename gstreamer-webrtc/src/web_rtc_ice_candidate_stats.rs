// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CStr;

use glib::translate::*;

use crate::WebRTCICECandidateStats;

impl WebRTCICECandidateStats {
    pub fn ipaddr(&self) -> Option<&str> {
        unsafe {
            let ptr = (*self.to_glib_none().0).ipaddr;
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    pub fn port(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).port }
    }

    pub fn stream_id(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).stream_id }
    }

    pub fn type_(&self) -> Option<&str> {
        unsafe {
            let ptr = (*self.to_glib_none().0).type_;
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    pub fn proto(&self) -> Option<&str> {
        unsafe {
            let ptr = (*self.to_glib_none().0).proto;
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    pub fn relay_proto(&self) -> Option<&str> {
        unsafe {
            let ptr = (*self.to_glib_none().0).relay_proto;
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    pub fn prio(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).prio }
    }

    pub fn url(&self) -> Option<&str> {
        unsafe {
            let ptr = (*self.to_glib_none().0).url;
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }
}
