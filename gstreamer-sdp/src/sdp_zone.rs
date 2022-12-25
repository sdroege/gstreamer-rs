// Take a look at the license at the top of the repository in the LICENSE file.

use std::{ffi::CStr, fmt, mem};

use glib::translate::*;

#[repr(transparent)]
#[doc(alias = "GstSDPZone")]
pub struct SDPZone(pub(crate) ffi::GstSDPZone);

unsafe impl Send for SDPZone {}
unsafe impl Sync for SDPZone {}

impl SDPZone {
    pub fn new(time: &str, typed_time: &str) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut zone = mem::MaybeUninit::uninit();
            ffi::gst_sdp_zone_set(
                zone.as_mut_ptr(),
                time.to_glib_none().0,
                typed_time.to_glib_none().0,
            );
            SDPZone(zone.assume_init())
        }
    }

    pub fn time(&self) -> Option<&str> {
        unsafe {
            if self.0.time.is_null() {
                None
            } else {
                Some(CStr::from_ptr(self.0.time).to_str().unwrap())
            }
        }
    }

    pub fn typed_time(&self) -> Option<&str> {
        unsafe {
            if self.0.typed_time.is_null() {
                None
            } else {
                Some(CStr::from_ptr(self.0.typed_time).to_str().unwrap())
            }
        }
    }
}

impl Clone for SDPZone {
    fn clone(&self) -> Self {
        skip_assert_initialized!();
        unsafe {
            let mut zone = mem::MaybeUninit::uninit();
            ffi::gst_sdp_zone_set(zone.as_mut_ptr(), self.0.time, self.0.typed_time);
            SDPZone(zone.assume_init())
        }
    }
}

impl Drop for SDPZone {
    fn drop(&mut self) {
        unsafe {
            ffi::gst_sdp_zone_clear(&mut self.0);
        }
    }
}

impl fmt::Debug for SDPZone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SDPZone")
            .field("time", &self.time())
            .field("typed_time", &self.typed_time())
            .finish()
    }
}
