// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CStr;
use std::fmt;
use std::mem;

use glib::translate::*;

#[repr(transparent)]
pub struct SDPAttribute(pub(crate) ffi::GstSDPAttribute);

unsafe impl Send for SDPAttribute {}
unsafe impl Sync for SDPAttribute {}

impl SDPAttribute {
    pub fn new(key: &str, value: Option<&str>) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut attr = mem::MaybeUninit::zeroed();
            ffi::gst_sdp_attribute_set(
                attr.as_mut_ptr(),
                key.to_glib_none().0,
                value.to_glib_none().0,
            );
            SDPAttribute(attr.assume_init())
        }
    }

    pub fn key(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.key).to_str().unwrap() }
    }

    pub fn value(&self) -> Option<&str> {
        unsafe {
            let ptr = self.0.value;

            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }
}

impl Clone for SDPAttribute {
    fn clone(&self) -> Self {
        SDPAttribute::new(self.key(), self.value())
    }
}

impl Drop for SDPAttribute {
    fn drop(&mut self) {
        unsafe {
            ffi::gst_sdp_attribute_clear(&mut self.0);
        }
    }
}

impl fmt::Debug for SDPAttribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SDPAttribute")
            .field("key", &self.key())
            .field("value", &self.value())
            .finish()
    }
}
