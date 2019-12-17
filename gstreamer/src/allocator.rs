// Copyright (C) 2019 Vivia Nikolaidou <vivia@ahiru.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr;

use gst_sys;

use glib::translate::from_glib_full;
use glib::IsA;

use AllocationParams;
use Allocator;
use Memory;

pub trait AllocatorExtManual: 'static {
    fn alloc(
        &self,
        size: usize,
        params: Option<&AllocationParams>,
    ) -> Result<Memory, glib::BoolError>;
}

impl<O: IsA<Allocator>> AllocatorExtManual for O {
    fn alloc(
        &self,
        size: usize,
        params: Option<&AllocationParams>,
    ) -> Result<Memory, glib::BoolError> {
        unsafe {
            let ret = gst_sys::gst_allocator_alloc(
                self.as_ptr() as *mut _,
                size,
                match params {
                    Some(val) => val.as_ptr() as *mut _,
                    None => ptr::null_mut(),
                },
            );
            if ret.is_null() {
                Err(glib_bool_error!("Failed to allocate memory"))
            } else {
                Ok(from_glib_full(ret))
            }
        }
    }
}
