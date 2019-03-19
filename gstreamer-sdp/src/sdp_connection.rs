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
use gst_sdp_sys;

#[repr(C)]
pub struct SDPConnection(pub(crate) gst_sdp_sys::GstSDPConnection);

impl SDPConnection {
    pub fn new(nettype: &str, addrtype: &str, address: &str, ttl: u32, addr_number: u32) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut conn = mem::zeroed();
            gst_sdp_sys::gst_sdp_connection_set(
                &mut conn,
                nettype.to_glib_none().0,
                addrtype.to_glib_none().0,
                address.to_glib_none().0,
                ttl,
                addr_number,
            );
            SDPConnection(conn)
        }
    }

    pub fn nettype(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.nettype).to_str().unwrap() }
    }

    pub fn addrtype(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.addrtype).to_str().unwrap() }
    }

    pub fn address(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.address).to_str().unwrap() }
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
        SDPConnection::new(
            self.nettype(),
            self.addrtype(),
            self.address(),
            self.ttl(),
            self.addr_number(),
        )
    }
}

impl Drop for SDPConnection {
    fn drop(&mut self) {
        unsafe {
            gst_sdp_sys::gst_sdp_connection_clear(&mut self.0);
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
