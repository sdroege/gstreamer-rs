// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};

use crate::{WebRTCICE, WebRTCICEStream};

pub trait WebRTCICEExtManual: 'static {
    #[doc(alias = "gst_webrtc_ice_add_candidate")]
    fn add_candidate(&self, stream: &impl IsA<WebRTCICEStream>, candidate: &str);
}

impl<O: IsA<WebRTCICE>> WebRTCICEExtManual for O {
    fn add_candidate(&self, stream: &impl IsA<WebRTCICEStream>, candidate: &str) {
        unsafe {
            use std::{mem, ptr};

            if gst::version() >= (1, 23, 0, 0) {
                let func = mem::transmute::<
                    unsafe extern "C" fn(
                        *mut ffi::GstWebRTCICE,
                        *mut ffi::GstWebRTCICEStream,
                        *const std::os::raw::c_char,
                    ),
                    unsafe extern "C" fn(
                        *mut ffi::GstWebRTCICE,
                        *mut ffi::GstWebRTCICEStream,
                        *const std::os::raw::c_char,
                        *mut gst::ffi::GstPromise,
                    ),
                >(ffi::gst_webrtc_ice_add_candidate);
                func(
                    self.as_ref().to_glib_none().0,
                    stream.as_ref().to_glib_none().0,
                    candidate.to_glib_none().0,
                    ptr::null_mut(),
                );
            } else {
                ffi::gst_webrtc_ice_add_candidate(
                    self.as_ref().to_glib_none().0,
                    stream.as_ref().to_glib_none().0,
                    candidate.to_glib_none().0,
                );
            }
        }
    }
}
