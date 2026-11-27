// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{D3D12Device, D3D12StagingPoolAllocator, ffi};
use glib::{prelude::*, translate::*};
use windows::Win32::Graphics::Direct3D12::D3D12_PLACED_SUBRESOURCE_FOOTPRINT;

impl D3D12StagingPoolAllocator {
    #[doc(alias = "gst_d3d12_staging_pool_allocator_new")]
    pub fn new(
        device: &impl IsA<D3D12Device>,
        layouts: &[D3D12_PLACED_SUBRESOURCE_FOOTPRINT],
        total_bytes: usize,
    ) -> Option<Self> {
        assert_initialized_main_thread!();
        let num_layouts = u32::try_from(layouts.len()).ok()?;
        unsafe {
            from_glib_full(ffi::gst_d3d12_staging_pool_allocator_new(
                device.as_ref().to_glib_none().0,
                num_layouts,
                layouts.as_ptr().cast(),
                total_bytes,
            ))
        }
    }

    #[doc(alias = "gst_d3d12_staging_pool_allocator_acquire_memory")]
    pub fn acquire_memory(&self) -> Result<gst::Memory, gst::FlowError> {
        unsafe {
            let mut mem = std::ptr::null_mut();
            gst::FlowSuccess::try_from_glib(ffi::gst_d3d12_staging_pool_allocator_acquire_memory(
                self.to_glib_none().0,
                &mut mem,
            ))
            .map(|_| from_glib_full(mem))
        }
    }
}
