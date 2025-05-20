// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ffi, D3D12AllocationParams, D3D12BufferPool, D3D12Device};
use glib::translate::*;

impl D3D12BufferPool {
    pub fn device(&self) -> D3D12Device {
        let obj: *const ffi::GstD3D12BufferPool = self.to_glib_none().0;
        unsafe { from_glib_none((*obj).device) }
    }
}

pub trait D3D12BufferPoolConfig {
    fn d3d12_allocation_params(&self) -> Option<D3D12AllocationParams>;

    fn set_d3d12_allocation_params(&mut self, params: &D3D12AllocationParams);
}

impl D3D12BufferPoolConfig for gst::BufferPoolConfigRef {
    #[doc(alias = "gst_buffer_pool_config_get_d3d12_allocation_params")]
    fn d3d12_allocation_params(&self) -> Option<D3D12AllocationParams> {
        unsafe {
            let params =
                ffi::gst_buffer_pool_config_get_d3d12_allocation_params(self.as_ref().as_mut_ptr());
            if params.is_null() {
                None
            } else {
                from_glib_full(params)
            }
        }
    }

    #[doc(alias = "gst_buffer_pool_config_set_d3d12_allocation_params")]
    fn set_d3d12_allocation_params(&mut self, params: &D3D12AllocationParams) {
        unsafe {
            ffi::gst_buffer_pool_config_set_d3d12_allocation_params(
                self.as_mut().as_mut_ptr(),
                params.to_glib_none().0,
            )
        }
    }
}
