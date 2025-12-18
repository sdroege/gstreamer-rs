// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CStr;

use crate::{WebRTCICECandidate, WebRTCICECandidateStats};

impl WebRTCICECandidate {
    pub fn candidate(&self) -> Option<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).candidate;
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    pub fn component(&self) -> i32 {
        unsafe { (*self.as_ptr()).component }
    }

    pub fn sdp_mid(&self) -> Option<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).sdp_mid;
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    pub fn sdp_mline_index(&self) -> i32 {
        unsafe { (*self.as_ptr()).sdp_mline_index }
    }

    pub fn stats(&self) -> Option<&WebRTCICECandidateStats> {
        unsafe {
            if (*self.as_ptr()).stats.is_null() {
                None
            } else {
                Some(WebRTCICECandidateStats::from_glib_ptr_borrow(
                    &(*self.as_ptr()).stats,
                ))
            }
        }
    }
}
