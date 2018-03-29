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
use gobject_ffi;

use auto::MIKEYPayload;

impl MIKEYPayload {

    pub fn kemac_add_sub(&mut self, newpay: MIKEYPayload) -> bool {
        unsafe {
            from_glib(ffi::gst_mikey_payload_kemac_add_sub(self.to_glib_none_mut().0, newpay.to_glib_full()))
        }
        mem::forget(newpay);
    }

    //pub fn sp_get_param(&self, idx: u32) -> /*Ignored*/Option<MIKEYPayloadSPParam> {
    //    unsafe { TODO: call ffi::gst_mikey_payload_sp_get_param() }
    //}

    //pub fn t_set(&mut self, type_: MIKEYTSType, ts_value: &[u8]) -> bool {
    //    unsafe { TODO: call ffi::gst_mikey_payload_t_set() }
    //}
}
