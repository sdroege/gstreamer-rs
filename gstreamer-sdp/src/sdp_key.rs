// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CStr;
use std::fmt;

#[repr(transparent)]
#[doc(alias = "GstSDPKey")]
pub struct SDPKey(ffi::GstSDPKey);

unsafe impl Send for SDPKey {}
unsafe impl Sync for SDPKey {}

impl SDPKey {
    pub fn type_(&self) -> Option<&str> {
        unsafe {
            if self.0.type_.is_null() {
                None
            } else {
                Some(CStr::from_ptr(self.0.type_).to_str().unwrap())
            }
        }
    }

    pub fn data(&self) -> Option<&str> {
        unsafe {
            if self.0.data.is_null() {
                None
            } else {
                Some(CStr::from_ptr(self.0.data).to_str().unwrap())
            }
        }
    }
}

impl fmt::Debug for SDPKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SDPKey")
            .field("type", &self.type_())
            .field("data", &self.data())
            .finish()
    }
}
