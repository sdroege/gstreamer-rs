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

    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    pub fn foundation(&self) -> Option<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).ABI.abi.foundation;
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    pub fn related_address(&self) -> Option<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).ABI.abi.related_address;
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    pub fn related_port(&self) -> u32 {
        unsafe { (*self.as_ptr()).ABI.abi.related_port }
    }

    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    pub fn username_fragment(&self) -> Option<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).ABI.abi.username_fragment;
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    pub fn tcp_type(&self) -> crate::WebRTCICETcpCandidateType {
        unsafe { glib::translate::from_glib((*self.as_ptr()).ABI.abi.tcp_type) }
    }
}
