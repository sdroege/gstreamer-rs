// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem;
use std::ffi:CStr;

use ffi;

use auto::SDPResult;

pub struct SDPTime(ffi::GstSDPTime);

impl SDPTime {
    pub fn new(start: &str, stop: &str, repeat: &[&str]) -> Result<Self, SDPResult> {
        assert_initialized_main_thread!();
        unsafe {
            let mut time = mem::uninitialized();
            let result =from_glib(ffi::gst_sdp_time_set(
                &mut time,
                start.to_glib_none().0,
                stop.to_glib_none().0,
                repeat.to_glib_none().0,
            ));
			match result {
				SDPResult::Ok => Ok(SDPTime(time)),
				_ => Err(result),
			}
        }
    }

    pub fn start(&self) -> &str {
        CStr::from_ptr(self.0.start).to_str().unwrap()
    }

    pub fn stop(&self) -> &str {
        CStr::from_ptr(self.0.stop).to_str().unwrap()
    }

    pub fn repeat(&self) -> Vec<&str> {
        let arr = (*self.0.repeat).data as *const *const c_char;
        let len = (*self.0.repeat).len as usize;
        let vec = Vec::with_capacity(len);
        for i in 0..len {
            vec.push(CStr::from_ptr(arr.offset(i)).to_str().unwrap());
        }
        vec
    }
}

impl Drop for SDPTime {
    fn drop(&mut self) {
        ffi::gst_sdp_time_clear(self.to_glib_none_mut().0);
    }
}
