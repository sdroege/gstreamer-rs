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
use m_i_k_e_y_map_s_r_t_p::MIKEYPayloadSPParam;

impl MIKEYPayload {
    pub fn kemac_add_sub(&mut self, newpay: MIKEYPayload) -> bool {
        unsafe {
            let ret = from_glib(ffi::gst_mikey_payload_kemac_add_sub(
                self.to_glib_none_mut().0,
                newpay.to_glib_full(),
            ));
            mem::forget(newpay);
            ret
        }
    }

    pub fn kemac_get_sub(&self, idx: u32) -> Option<&MIKEYPayload> {
        unsafe {
            &*(from_glib_none(ffi::gst_mikey_payload_kemac_get_sub(
                self.to_glib_none().0,
                idx,
            )) as *mut MIKEYPayload)
        }
    }

    pub fn sp_get_param(&self, idx: u32) -> Option<&MIKEYPayloadSPParam> {
        unsafe {
            &*(from_glib_none(ffi::gst_mikey_payload_sp_get_param(
                self.to_glib_none().0,
                idx,
            )) as *mut MIKEYPayloadSPParam)
        }
    }

    pub fn t_set(&mut self, type_: MIKEYTSType, ts_value: &[u8]) -> bool {
        unsafe {
            from_glib(ffi::gst_mikey_payload_t_set(
                self.to_glib_none_mut().0,
                type_.to_glib(),
                ts_value.to_glib_none().0,
            ))
        }
    }

    pub fn key_data_set_interval(&mut self, vf_data: &[u8], vt_data: &[u8]) -> bool {
        let vf_len = vf_data.len() as u8;
        let vt_len = vt_data.len() as u8;
        unsafe {
            from_glib(ffi::gst_mikey_payload_key_data_set_interval(
                self.to_glib_none_mut().0,
                vf_len,
                vf_data.to_glib_none().0,
                vt_len,
                vt_data.to_glib_none().0,
            ))
        }
    }
}
