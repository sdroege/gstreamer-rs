// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::mem;
use std::os::raw::c_char;

use ffi;
use glib::translate::*;

#[repr(C)]
pub struct SDPTime(pub(crate) ffi::GstSDPTime);

impl SDPTime {
    pub fn new(start: &str, stop: &str, repeat: &[&str]) -> Result<Self, ()> {
        assert_initialized_main_thread!();
        unsafe {
            let mut time = mem::zeroed();
            let result = ffi::gst_sdp_time_set(
                &mut time,
                start.to_glib_none().0,
                stop.to_glib_none().0,
                repeat.to_glib_none().0,
            );
            match result {
                ffi::GST_SDP_OK => Ok(SDPTime(time)),
                _ => Err(()),
            }
        }
    }

    pub fn start(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.start).to_str().unwrap() }
    }

    pub fn stop(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.stop).to_str().unwrap() }
    }

    pub fn repeat(&self) -> Vec<&str> {
        unsafe {
            let arr = (*self.0.repeat).data as *const *const c_char;
            let len = (*self.0.repeat).len as usize;
            let mut vec = Vec::with_capacity(len);
            for i in 0..len {
                vec.push(CStr::from_ptr(*arr.offset(i as isize)).to_str().unwrap());
            }
            vec
        }
    }
}

impl Drop for SDPTime {
    fn drop(&mut self) {
        unsafe {
            ffi::gst_sdp_time_clear(&mut self.0);
        }
    }
}
