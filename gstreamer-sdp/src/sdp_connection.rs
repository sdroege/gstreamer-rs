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

use glib::translate::*;

#[repr(transparent)]
pub struct SDPConnection(pub(crate) ffi::GstSDPConnection);

unsafe impl Send for SDPConnection {}
unsafe impl Sync for SDPConnection {}

impl SDPConnection {
    pub fn new(nettype: &str, addrtype: &str, address: &str, ttl: u32, addr_number: u32) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut conn = mem::MaybeUninit::zeroed();
            ffi::gst_sdp_connection_set(
                conn.as_mut_ptr(),
                nettype.to_glib_none().0,
                addrtype.to_glib_none().0,
                address.to_glib_none().0,
                ttl,
                addr_number,
            );
            SDPConnection(conn.assume_init())
        }
    }

    pub fn nettype(&self) -> Option<&str> {
        unsafe {
            if self.0.nettype.is_null() {
                None
            } else {
                Some(CStr::from_ptr(self.0.nettype).to_str().unwrap())
            }
        }
    }

    pub fn addrtype(&self) -> Option<&str> {
        unsafe {
            if self.0.addrtype.is_null() {
                None
            } else {
                Some(CStr::from_ptr(self.0.addrtype).to_str().unwrap())
            }
        }
    }

    pub fn address(&self) -> Option<&str> {
        unsafe {
            if self.0.address.is_null() {
                None
            } else {
                Some(CStr::from_ptr(self.0.address).to_str().unwrap())
            }
        }
    }

    pub fn ttl(&self) -> u32 {
        self.0.ttl as u32
    }

    pub fn addr_number(&self) -> u32 {
        self.0.addr_number as u32
    }
}

impl Clone for SDPConnection {
    fn clone(&self) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut conn = mem::MaybeUninit::zeroed();
            ffi::gst_sdp_connection_set(
                conn.as_mut_ptr(),
                self.0.nettype,
                self.0.addrtype,
                self.0.address,
                self.0.ttl,
                self.0.addr_number,
            );
            SDPConnection(conn.assume_init())
        }
    }
}

impl Drop for SDPConnection {
    fn drop(&mut self) {
        unsafe {
            ffi::gst_sdp_connection_clear(&mut self.0);
        }
    }
}

impl fmt::Debug for SDPConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SDPConnection")
            .field("nettype", &self.nettype())
            .field("addrtype", &self.addrtype())
            .field("address", &self.address())
            .field("ttl", &self.ttl())
            .field("addr_number", &self.addr_number())
            .finish()
    }
}
