// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{HipDevice, ffi, ffi::GstHipHostMemory};
use glib::translate::*;
use gst::{Memory, MemoryRef};
use std::{ffi::c_void, fmt};

gst::memory_object_wrapper!(
    HipHostMemory,
    HipHostMemoryRef,
    GstHipHostMemory,
    |mem: &MemoryRef| { unsafe { from_glib(ffi::gst_is_hip_host_memory(mem.as_mut_ptr())) } },
    Memory,
    MemoryRef
);

impl HipHostMemoryRef {
    pub fn device(&self) -> HipDevice {
        unsafe { from_glib_none(self.0.device) }
    }

    #[doc(alias = "gst_hip_host_memory_get_device_pointer")]
    pub fn device_ptr(&self) -> *mut c_void {
        unsafe { ffi::gst_hip_host_memory_get_device_pointer(mut_override(&self.0)) }
    }
}

impl fmt::Debug for HipHostMemoryRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("HipHostMemory")
            .field(&self.upcast_memory_ref::<Memory>())
            .finish()
    }
}
