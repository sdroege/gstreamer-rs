// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ffi;

pub trait HipHostBufferPoolConfig {
    fn hip_host_alloc_flags(&self) -> u32;

    fn set_hip_host_alloc_flags(&mut self, flags: u32);
}

impl HipHostBufferPoolConfig for gst::BufferPoolConfigRef {
    #[doc(alias = "gst_buffer_pool_config_get_hip_host_alloc_flags")]
    fn hip_host_alloc_flags(&self) -> u32 {
        unsafe { ffi::gst_buffer_pool_config_get_hip_host_alloc_flags(self.as_ref().as_mut_ptr()) }
    }

    #[doc(alias = "gst_buffer_pool_config_set_hip_host_alloc_flags")]
    fn set_hip_host_alloc_flags(&mut self, flags: u32) {
        unsafe {
            ffi::gst_buffer_pool_config_set_hip_host_alloc_flags(self.as_mut().as_mut_ptr(), flags)
        }
    }
}
