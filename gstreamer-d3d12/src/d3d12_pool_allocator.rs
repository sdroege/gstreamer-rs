// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ffi, D3D12Device, D3D12PoolAllocator};
use glib::{prelude::*, translate::*};

use windows::Win32::Graphics::Direct3D12::{
    D3D12_CLEAR_VALUE, D3D12_HEAP_FLAGS, D3D12_HEAP_PROPERTIES, D3D12_RESOURCE_DESC,
    D3D12_RESOURCE_STATES,
};

impl D3D12PoolAllocator {
    #[doc(alias = "gst_d3d12_pool_allocator_new")]
    pub fn new(
        device: &impl IsA<D3D12Device>,
        heap_props: &D3D12_HEAP_PROPERTIES,
        heap_flags: D3D12_HEAP_FLAGS,
        desc: &D3D12_RESOURCE_DESC,
        initial_state: D3D12_RESOURCE_STATES,
        optimized_clear_value: Option<D3D12_CLEAR_VALUE>,
    ) -> Option<D3D12PoolAllocator> {
        assert_initialized_main_thread!();
        unsafe {
            let heap_props_ptr =
                heap_props as *const D3D12_HEAP_PROPERTIES as *const std::ffi::c_void;
            let desc_ptr = desc as *const D3D12_RESOURCE_DESC as *const std::ffi::c_void;
            let clear_value_ptr: *const std::ffi::c_void = optimized_clear_value
                .as_ref()
                .map_or(std::ptr::null_mut(), |value| {
                    value as *const D3D12_CLEAR_VALUE as *const std::ffi::c_void
                });

            from_glib_full(ffi::gst_d3d12_pool_allocator_new(
                device.as_ref().to_glib_none().0,
                heap_props_ptr,
                heap_flags.0,
                desc_ptr,
                initial_state.0,
                clear_value_ptr,
            ))
        }
    }

    #[doc(alias = "gst_d3d12_pool_allocator_acquire_memory")]
    pub fn acquire_memory(&self) -> std::result::Result<gst::Memory, gst::FlowError> {
        unsafe {
            let mut mem = std::ptr::null_mut();
            gst::FlowSuccess::try_from_glib(ffi::gst_d3d12_pool_allocator_acquire_memory(
                self.to_glib_none().0,
                &mut mem,
            ))
            .map(|_| from_glib_full(mem))
        }
    }
}
