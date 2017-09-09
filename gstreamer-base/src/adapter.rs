// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib::translate::*;
use gst;
use Adapter;

impl Adapter {
    pub fn copy(&self, offset: usize, dest: &mut [u8]) {
        unsafe {
            let size = dest.len();
            ffi::gst_adapter_copy(
                self.to_glib_none().0,
                dest.as_mut_ptr() as *mut _,
                offset,
                size,
            );
        }
    }

    pub fn push(&self, buf: gst::Buffer) {
        unsafe {
            ffi::gst_adapter_push(self.to_glib_none().0, buf.into_ptr());
        }
    }
}
