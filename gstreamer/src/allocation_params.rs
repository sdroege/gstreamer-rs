// Take a look at the license at the top of the repository in the LICENSE file.

use std::mem;

use glib::translate::*;

use crate::MemoryFlags;

#[derive(Debug, Clone)]
pub struct AllocationParams(ffi::GstAllocationParams);

unsafe impl Send for AllocationParams {}
unsafe impl Sync for AllocationParams {}

impl AllocationParams {
    pub fn get_flags(&self) -> MemoryFlags {
        unsafe { from_glib(self.0.flags) }
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
            let mut allocationparams: ffi::GstAllocationParams = mem::zeroed();

            allocationparams.flags = flags.to_glib();
            allocationparams.align = align;
            allocationparams.prefix = prefix;
            allocationparams.padding = padding;

            allocationparams
        };

        AllocationParams(allocationparams)
    }

    pub fn as_ptr(&self) -> *const ffi::GstAllocationParams {
        &self.0
    }
}

impl From<ffi::GstAllocationParams> for AllocationParams {
    fn from(params: ffi::GstAllocationParams) -> Self {
        skip_assert_initialized!();
        AllocationParams(params)
    }
}
