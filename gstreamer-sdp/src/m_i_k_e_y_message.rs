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
use gst;

use auto::{MIKEYMessage, MIKEYTSType};
use m_i_k_e_y_decrypt_info::MIKEYDecryptInfo;
use m_i_k_e_y_encrypt_info::MIKEYEncryptInfo;
use m_i_k_e_y_map_s_r_t_p::MIKEYMapSRTP;
use m_i_k_e_y_payload::MIKEYPayload;

impl MIKEYMessage {
    pub fn new_from_bytes<'a, P: Into<Option<&'a MIKEYDecryptInfo>>>(
        bytes: &glib::Bytes,
        info: P,
    ) -> Result<MIKEYMessage, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gst_mikey_message_new_from_bytes(
                bytes.to_glib_none(),
                info.to_glib_full(),
                &mut error,
            );
            mem::forget(bytes);
            mem::forget(info);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn new_from_data<'a, P: Into<Option<&'a MIKEYDecryptInfo>>>(
        data: &[u8],
        info: P,
    ) -> Result<MIKEYMessage, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gst_mikey_message_new_from_data(
                data.to_glib_none().0,
                data.len(),
                info.to_glib_full(),
                &mut err,
            );
            mem::forget(info);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn add_t(&mut self, type_: MIKEYTSType, ts_value: &[u8]) -> bool {
        unsafe {
            from_glib(ffi::gst_mikey_message_add_t(
                self.to_glib_none_mut().0,
                type_.to_glib(),
                ts_value.to_glib_none().0,
            ))
        }
    }

    pub fn get_cs_srtp(&self, idx: u32) -> Option<&MIKEYMapSRTP> {
        unsafe {
            &*(from_glib_none(ffi::gst_mikey_message_get_cs_srtp(
                self.to_glib_none().0,
                idx,
            )) as *mut MIKEYMapSRTP)
        }
    }

    pub fn insert_cs_srtp(&mut self, idx: i32, map: MIKEYMapSRTP) -> bool {
        unsafe {
            let ret = from_glib(ffi::gst_mikey_message_insert_cs_srtp(
                self.to_glib_none_mut().0,
                idx,
                map.to_glib_full(),
            ));
            mem::forget(map);
            ret
        }
    }

    pub fn get_payload(&self, idx: u32) -> Option<&MIKEYPayload> {
        unsafe {
            &*(from_glib_none(ffi::gst_mikey_message_get_payload(
                self.to_glib_none().0,
                idx,
            )) as *mut MIKEYPayload)
        }
    }

    pub fn add_payload(&mut self, payload: MIKEYPayload) -> bool {
        unsafe {
            let ret = from_glib(ffi::gst_mikey_message_add_payload(
                self.to_glib_none_mut().0,
                payload.to_glib_full(),
            ));
            mem::forget(payload);
            ret
        }
    }

    pub fn insert_payload(&mut self, idx: u32, payload: MIKEYPayload) -> bool {
        unsafe {
            let ret = from_glib(ffi::gst_mikey_message_insert_payload(
                self.to_glib_none_mut().0,
                idx,
                payload.to_glib_full(),
            ));
            mem::forget(payload);
            ret
        }
    }

    pub fn replace_cs_srtp(&mut self, idx: i32, map: MIKEYMapSRTP) -> bool {
        unsafe {
            let ret = from_glib(ffi::gst_mikey_message_replace_cs_srtp(
                self.to_glib_none_mut().0,
                idx,
                map.to_glib_full(),
            ));
            mem::forget(map);
            ret
        }
    }

    pub fn replace_payload(&mut self, idx: u32, payload: MIKEYPayload) -> bool {
        unsafe {
            let ret = from_glib(ffi::gst_mikey_message_replace_payload(
                self.to_glib_none_mut().0,
                idx,
                payload.to_glib_full(),
            ));
            mem::forget(payload);
            ret
        }
    }

    pub fn base64_encode(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_mikey_message_base64_encode(
                self.to_glib_none_mut().0,
            ))
        }
    }
}
