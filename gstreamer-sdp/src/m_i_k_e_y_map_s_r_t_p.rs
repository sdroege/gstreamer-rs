// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;

#[repr(C)]
pub struct MIKEYMapSRTP(ffi::GstMIKEYMapSRTP);

impl MIKEYMapSRTP {
    pub fn new(policy: u8, ssrc: u32, roc: u32) -> MIKEYMapSRTP {
        MIKEYMapSRTP(ffi::GstMIKEYMapSRTP {
            policy: policy,
            ssrc: ssrc,
            roc: roc,
        })
    }

    pub fn policy(&self) -> u8 {
        self.0.policy
    }

    pub fn ssrc(&self) -> u32 {
        self.0.ssrc
    }

    pub fn roc(&self) -> u32 {
        self.0.roc
    }
}
