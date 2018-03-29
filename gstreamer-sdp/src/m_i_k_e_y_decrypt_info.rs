// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem;
use std::ptr;

use ffi;
use glib::translate::*;
use glib_ffi;
use libc::c_void;

pub struct MIKEYDecryptInfo(ffi::GstMIKEYDecryptInfo);

impl MIKEYDecryptInfo {
    pub fn new() -> MIKEYDecryptInfo {
        MIKEYDecryptInfo(ffi::GstMIKEYDecryptInfo(ptr::null_mut() as *mut c_void))
    }
}
