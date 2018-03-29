// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem;
use std::ffi:CStr;

use auto::SDPResult;

use ffi;

pub struct SDPZone(ffi::GstSDPZone);

impl SDPZone {
    pub fn new(time: &str, typed_time: &str) -> Result<Self, SDPResult> {
        assert_initialized_main_thread!();
        unsafe {
            let mut zone = mem::uninitialized();
            let result = from_glib(ffi::gst_sdp_zone_set(&mut zone, time.to_glib_none().0, typed_time.to_glib_none().0));
			match result {
				SDPResult::Ok => Ok(SDPZone(zone)),
				_ => Err(result),
			}
        }
    }

    pub fn time(&self) -> &str {
        CStr::from_ptr(self.0.time).to_str().unwrap()
    }

    pub fn typed_time(&self) -> &str {
        CStr::from_ptr(self.0.typed_time).to_str().unwrap()
    }
}

impl Drop for SDPZone {
    fn drop(&mut self) {
        ffi::gst_sdp_zone_clear(self.to_glib_none_mut().0);
    }
}
