// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::fmt;

use gst_sdp_sys;

#[repr(C)]
pub struct SDPOrigin(pub(crate) gst_sdp_sys::GstSDPOrigin);

impl SDPOrigin {
    pub fn username(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.username).to_str().unwrap() }
    }

    pub fn sess_id(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.sess_id).to_str().unwrap() }
    }

    pub fn sess_version(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.sess_version).to_str().unwrap() }
    }

    pub fn nettype(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.nettype).to_str().unwrap() }
    }

    pub fn addrtype(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.addrtype).to_str().unwrap() }
    }

    pub fn addr(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.addr).to_str().unwrap() }
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
