// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::mem;

use ffi;
use glib::translate::*;

#[repr(C)]
pub struct SDPZone(pub(crate) ffi::GstSDPZone);

impl SDPZone {
    pub fn new(time: &str, typed_time: &str) -> Result<Self, ()> {
        assert_initialized_main_thread!();
        unsafe {
            let mut zone = mem::zeroed();
            let result = ffi::gst_sdp_zone_set(
                &mut zone,
                time.to_glib_none().0,
                typed_time.to_glib_none().0,
            );
            match result {
                ffi::GST_SDP_OK => Ok(SDPZone(zone)),
                _ => Err(()),
            }
        }
    }

    pub fn time(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.time).to_str().unwrap() }
    }

    pub fn typed_time(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.typed_time).to_str().unwrap() }
    }
}

impl Drop for SDPZone {
    fn drop(&mut self) {
        unsafe {
            ffi::gst_sdp_zone_clear(&mut self.0);
        }
    }
}
