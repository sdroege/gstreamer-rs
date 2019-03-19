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
use std::os::raw::c_char;

use glib::translate::*;
use gst_sdp_sys;

#[repr(C)]
pub struct SDPTime(pub(crate) gst_sdp_sys::GstSDPTime);

impl SDPTime {
    pub fn new(start: &str, stop: &str, repeat: &[&str]) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut time = mem::zeroed();
            gst_sdp_sys::gst_sdp_time_set(
                &mut time,
                start.to_glib_none().0,
                stop.to_glib_none().0,
                repeat.to_glib_none().0,
            );
            SDPTime(time)
        }
    }

    pub fn start(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.start).to_str().unwrap() }
    }

    pub fn stop(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.stop).to_str().unwrap() }
    }

    pub fn repeat(&self) -> Vec<&str> {
        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            let arr = (*self.0.repeat).data as *const *const c_char;
            let len = (*self.0.repeat).len as usize;
            let mut vec = Vec::with_capacity(len);
            for i in 0..len {
                vec.push(CStr::from_ptr(*arr.add(i)).to_str().unwrap());
            }
            vec
        }
    }
}

impl Clone for SDPTime {
    fn clone(&self) -> Self {
        SDPTime::new(self.start(), self.stop(), self.repeat().as_slice())
    }
}

impl Drop for SDPTime {
    fn drop(&mut self) {
        unsafe {
            gst_sdp_sys::gst_sdp_time_clear(&mut self.0);
        }
    }
}

impl fmt::Debug for SDPTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SDPTime")
            .field("start", &self.start())
            .field("stop", &self.stop())
            .field("repeat", &self.repeat())
            .finish()
    }
}
