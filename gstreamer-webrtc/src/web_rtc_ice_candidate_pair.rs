// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{WebRTCICECandidate, WebRTCICECandidatePair};

impl WebRTCICECandidatePair {
    pub fn local(&self) -> Option<&WebRTCICECandidate> {
        unsafe {
            if (*self.as_ptr()).local.is_null() {
                None
            } else {
                Some(WebRTCICECandidate::from_glib_ptr_borrow(
                    &(*self.as_ptr()).local,
                ))
            }
        }
    }

    pub fn remote(&self) -> Option<&WebRTCICECandidate> {
        unsafe {
            if (*self.as_ptr()).remote.is_null() {
                None
            } else {
                Some(WebRTCICECandidate::from_glib_ptr_borrow(
                    &(*self.as_ptr()).remote,
                ))
            }
        }
    }
}
