// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use glib::translate::from_glib_full;
use glib::IsA;

use crate::AllocationParams;
use crate::Allocator;
use crate::Memory;

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
            let ret = ffi::gst_allocator_alloc(
                self.as_ptr() as *mut _,
                size,
                match params {
                    Some(val) => val.as_ptr() as *mut _,
                    None => ptr::null_mut(),
                },
            );
            if ret.is_null() {
                Err(glib::bool_error!("Failed to allocate memory"))
            } else {
                Ok(from_glib_full(ret))
            }
        }
    }
}
