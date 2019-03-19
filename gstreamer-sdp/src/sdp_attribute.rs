// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::fmt;
use std::mem;

use glib::translate::*;
use gst_sdp_sys;

#[repr(C)]
pub struct SDPAttribute(pub(crate) gst_sdp_sys::GstSDPAttribute);

impl SDPAttribute {
    pub fn new(key: &str, value: Option<&str>) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut attr = mem::zeroed();
            gst_sdp_sys::gst_sdp_attribute_set(
                &mut attr,
                key.to_glib_none().0,
                value.to_glib_none().0,
            );
            SDPAttribute(attr)
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
            gst_sdp_sys::gst_sdp_attribute_clear(&mut self.0);
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
