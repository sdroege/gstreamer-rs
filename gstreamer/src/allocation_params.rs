// Copyright (C) 2019 Vivia Nikolaidou <vivia@ahiru.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem;

use glib::translate::*;

use MemoryFlags;

#[derive(Debug, Clone)]
pub struct AllocationParams(gst_sys::GstAllocationParams);

unsafe impl Send for AllocationParams {}
unsafe impl Sync for AllocationParams {}

impl AllocationParams {
    pub fn get_flags(&self) -> MemoryFlags {
        from_glib(self.0.flags)
    }

    pub fn get_align(&self) -> usize {
        self.0.align
    }

    pub fn get_prefix(&self) -> usize {
        self.0.prefix
    }

    pub fn get_padding(&self) -> usize {
        self.0.padding
    }

    pub fn new(flags: MemoryFlags, align: usize, prefix: usize, padding: usize) -> Self {
        assert_initialized_main_thread!();
        let allocationparams = unsafe {
            let mut allocationparams: gst_sys::GstAllocationParams = mem::zeroed();

            allocationparams.flags = flags.to_glib();
            allocationparams.align = align;
            allocationparams.prefix = prefix;
            allocationparams.padding = padding;

            allocationparams
        };

        AllocationParams(allocationparams)
    }

    pub fn as_ptr(&self) -> *const gst_sys::GstAllocationParams {
        &self.0
    }
}

impl From<gst_sys::GstAllocationParams> for AllocationParams {
    fn from(params: gst_sys::GstAllocationParams) -> Self {
        skip_assert_initialized!();
        AllocationParams(params)
    }
}
