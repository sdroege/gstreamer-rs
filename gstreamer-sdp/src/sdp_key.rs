// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::fmt;

#[repr(transparent)]
pub struct SDPKey(ffi::GstSDPKey);

unsafe impl Send for SDPKey {}
unsafe impl Sync for SDPKey {}

impl SDPKey {
    pub fn type_(&self) -> Option<&str> {
        unsafe {
            if self.0.type_.is_null() {
                None
            } else {
                Some(CStr::from_ptr(self.0.type_).to_str().unwrap())
            }
        }
    }

    pub fn data(&self) -> Option<&str> {
        unsafe {
            if self.0.data.is_null() {
                None
            } else {
                Some(CStr::from_ptr(self.0.data).to_str().unwrap())
            }
        }
    }
}

impl fmt::Debug for SDPKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SDPKey")
            .field("type", &self.type_())
            .field("data", &self.data())
            .finish()
    }
}
