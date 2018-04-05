// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::slice;

use ffi;

#[repr(C)]
pub struct MIKEYPayloadSPParam(ffi::GstMIKEYPayloadSPParam);

impl MIKEYPayloadSPParam {
    pub fn type_(&self) -> u8 {
        self.0.type_
    }

    pub fn len(&self) -> u8 {
        self.0.len
    }

    pub fn val(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.0.val as *const u8, self.0.len as usize) }
    }
}
