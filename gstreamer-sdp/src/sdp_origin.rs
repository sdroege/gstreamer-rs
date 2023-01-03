// Take a look at the license at the top of the repository in the LICENSE file.

use std::{ffi::CStr, fmt};

#[repr(transparent)]
#[doc(alias = "GstSDPOrigin")]
pub struct SDPOrigin(pub(crate) ffi::GstSDPOrigin);

unsafe impl Send for SDPOrigin {}
unsafe impl Sync for SDPOrigin {}

impl SDPOrigin {
    pub fn username(&self) -> Option<&str> {
        unsafe {
            if self.0.username.is_null() {
                None
            } else {
                Some(CStr::from_ptr(self.0.username).to_str().unwrap())
            }
        }
    }

    pub fn sess_id(&self) -> Option<&str> {
        unsafe {
            if self.0.sess_id.is_null() {
                None
            } else {
                Some(CStr::from_ptr(self.0.sess_id).to_str().unwrap())
            }
        }
    }

    pub fn sess_version(&self) -> Option<&str> {
        unsafe {
            if self.0.sess_version.is_null() {
                None
            } else {
                Some(CStr::from_ptr(self.0.sess_version).to_str().unwrap())
            }
        }
    }

    pub fn nettype(&self) -> Option<&str> {
        unsafe {
            if self.0.nettype.is_null() {
                None
            } else {
                Some(CStr::from_ptr(self.0.nettype).to_str().unwrap())
            }
        }
    }

    pub fn addrtype(&self) -> Option<&str> {
        unsafe {
            if self.0.addrtype.is_null() {
                None
            } else {
                Some(CStr::from_ptr(self.0.addrtype).to_str().unwrap())
            }
        }
    }

    pub fn addr(&self) -> Option<&str> {
        unsafe {
            if self.0.addr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(self.0.addr).to_str().unwrap())
            }
        }
    }
}

impl fmt::Debug for SDPOrigin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SDPOrigin")
            .field("username", &self.username())
            .field("sess_id", &self.sess_id())
            .field("sess_version", &self.sess_version())
            .field("nettype", &self.nettype())
            .field("addrtype", &self.addrtype())
            .field("addr", &self.addr())
            .finish()
    }
}
