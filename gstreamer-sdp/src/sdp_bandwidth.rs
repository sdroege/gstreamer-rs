// Take a look at the license at the top of the repository in the LICENSE file.

use std::{ffi::CStr, fmt, mem};

use glib::translate::*;

#[repr(transparent)]
#[doc(alias = "GstSDPBandwidth")]
pub struct SDPBandwidth(pub(crate) ffi::GstSDPBandwidth);

unsafe impl Send for SDPBandwidth {}
unsafe impl Sync for SDPBandwidth {}

impl SDPBandwidth {
    pub fn new(bwtype: &str, bandwidth: u32) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut bw = mem::MaybeUninit::zeroed();
            ffi::gst_sdp_bandwidth_set(bw.as_mut_ptr(), bwtype.to_glib_none().0, bandwidth);
            SDPBandwidth(bw.assume_init())
        }
    }

    pub fn bwtype(&self) -> Option<&str> {
        unsafe {
            if self.0.bwtype.is_null() {
                None
            } else {
                Some(CStr::from_ptr(self.0.bwtype).to_str().unwrap())
            }
        }
    }

    pub fn value(&self) -> u32 {
        self.0.bandwidth
    }
}

impl Clone for SDPBandwidth {
    fn clone(&self) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut bw = mem::MaybeUninit::zeroed();
            ffi::gst_sdp_bandwidth_set(bw.as_mut_ptr(), self.0.bwtype, self.0.bandwidth);
            SDPBandwidth(bw.assume_init())
        }
    }
}

impl Drop for SDPBandwidth {
    fn drop(&mut self) {
        unsafe {
            ffi::gst_sdp_bandwidth_clear(&mut self.0);
        }
    }
}

impl fmt::Debug for SDPBandwidth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SDPBandwidth")
            .field("bwtype", &self.bwtype())
            .field("value", &self.value())
            .finish()
    }
}
