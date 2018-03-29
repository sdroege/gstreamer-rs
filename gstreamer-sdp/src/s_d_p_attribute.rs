// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem;
use std::ffi::CStr;

use ffi;

use auto::SDPResult;

pub struct SDPAttribute(ffi::GstSDPAttribute);

impl SDPAttribute {
    pub fn new(key: &str, value: &str) -> Result<Self, SDPResult> {
        assert_initialized_main_thread!();
        unsafe {
            let mut attr = mem::uninitialized();
            let result = from_glib(ffi::gst_sdp_attribute_set(&mut attr, key.to_glib_none().0, value.to_glib_none().0));
            match result {
                SDPResult::Ok => Ok(SDPAttribute(attr)),
                _ => Err(result),
            }
        }
    }

    pub fn key(&self) -> &str {
        CStr::from_ptr(self.0.key).to_str().unwrap()
    }

    pub fn value(&self) -> &str {
        CStr::from_ptr(self.0.value).to_str().unwrap()
    }
}

impl Drop for SDPAttribute {
    fn drop(&mut self) {
        ffi::gst_sdp_attribute_clear(self.to_glib_none_mut().0);
    }
}
