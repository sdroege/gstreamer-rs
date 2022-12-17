// Take a look at the license at the top of the repository in the LICENSE file.

use std::marker::PhantomData;
use std::mem;

use glib::translate::*;

use crate::MemoryFlags;

#[derive(Debug, Clone)]
#[doc(alias = "GstAllocationParams")]
#[repr(transparent)]
pub struct AllocationParams(ffi::GstAllocationParams);

unsafe impl Send for AllocationParams {}
unsafe impl Sync for AllocationParams {}

impl AllocationParams {
    #[doc(alias = "get_flags")]
    pub fn flags(&self) -> MemoryFlags {
        unsafe { from_glib(self.0.flags) }
    }

    #[doc(alias = "get_align")]
    pub fn align(&self) -> usize {
        self.0.align
    }

    #[doc(alias = "get_prefix")]
    pub fn prefix(&self) -> usize {
        self.0.prefix
    }

    #[doc(alias = "get_padding")]
    pub fn padding(&self) -> usize {
        self.0.padding
    }

    pub fn new(flags: MemoryFlags, align: usize, prefix: usize, padding: usize) -> Self {
        assert_initialized_main_thread!();
        let params = unsafe {
            ffi::GstAllocationParams {
                flags: flags.into_glib(),
                align,
                prefix,
                padding,
                ..mem::zeroed()
            }
        };

        params.into()
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

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GstAllocationParams> for AllocationParams {
    type Storage = PhantomData<&'a Self>;

    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GstAllocationParams, Self> {
        Stash(&self.0, PhantomData)
    }
}

impl FromGlib<ffi::GstAllocationParams> for AllocationParams {
    #[allow(unused_unsafe)]
    unsafe fn from_glib(value: ffi::GstAllocationParams) -> Self {
        assert_initialized_main_thread!();
        Self::from(value)
    }
}
