// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CStr;

use crate::WebRTCICECandidateStats;

impl WebRTCICECandidateStats {
    pub fn ipaddr(&self) -> Option<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).ipaddr;
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    pub fn port(&self) -> u32 {
        unsafe { (*self.as_ptr()).port }
    }

    pub fn stream_id(&self) -> u32 {
        unsafe { (*self.as_ptr()).stream_id }
    }

    pub fn type_(&self) -> Option<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).type_;
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    pub fn proto(&self) -> Option<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).proto;
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    pub fn relay_proto(&self) -> Option<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).relay_proto;
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    pub fn prio(&self) -> u32 {
        unsafe { (*self.as_ptr()).prio }
    }

    pub fn url(&self) -> Option<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).url;
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }
}
