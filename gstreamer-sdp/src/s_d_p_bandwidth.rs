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

pub struct SDPBandwidth(ffi::GstSDPBandwidth);

impl SDPBandwidth {
    pub fn new(bwtype: &str, bandwidth: u32) -> Result<Self, SDPResult> {
        assert_initialized_main_thread!();
        unsafe {
            let mut bw = mem::uninitialized();
            let result = from_glib(ffi::gst_sdp_bandwidth_set(&mut bw, bwtype.to_glib_none().0, bandwidth));
            match result {
                SDPResult::Ok => Ok(SDPBandwidth(bw)),
                _ => Err(result),
            }
        }
    }

    pub fn bwtype(&self) -> &str {
        CStr::from_ptr(self.0.bwtype).to_str().unwrap()
    }

    pub fn value(&self) -> u32 {
        self.0.bandwidth as u32
    }
}

impl Drop for SDPBandwidth {
    fn drop(&mut self) {
        ffi::gst_sdp_bandwidth_clear(self.to_glib_none_mut().0);
    }
}
