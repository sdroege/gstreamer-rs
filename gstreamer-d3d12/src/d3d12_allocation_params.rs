// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};
use windows::Win32::Graphics::Direct3D12::{D3D12_HEAP_FLAGS, D3D12_RESOURCE_FLAGS};

use crate::{D3D12AllocationFlags, D3D12AllocationParams, D3D12Device, ffi};

impl D3D12AllocationParams {
    #[doc(alias = "gst_d3d12_allocation_params_new")]
    pub fn new(
        device: &impl IsA<D3D12Device>,
        info: &gst_video::VideoInfo,
        flags: D3D12AllocationFlags,
        resource_flags: D3D12_RESOURCE_FLAGS,
        heap_flags: D3D12_HEAP_FLAGS,
    ) -> Option<D3D12AllocationParams> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_d3d12_allocation_params_new(
                device.as_ref().to_glib_none().0,
                info.to_glib_none().0,
                flags.into_glib(),
                resource_flags.0,
                heap_flags.0,
            ))
        }
    }

    #[doc(alias = "gst_d3d12_allocation_params_alignment")]
    pub fn alignment(&mut self, align: &gst_video::VideoAlignment) -> bool {
        unsafe {
            from_glib(ffi::gst_d3d12_allocation_params_alignment(
                self.to_glib_none_mut().0,
                align.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_d3d12_allocation_params_set_heap_flags")]
    pub fn set_heap_flags(&mut self, heap_flags: D3D12_HEAP_FLAGS) -> bool {
        unsafe {
            from_glib(ffi::gst_d3d12_allocation_params_set_heap_flags(
                self.to_glib_none_mut().0,
                heap_flags.0,
            ))
        }
    }

    #[doc(alias = "gst_d3d12_allocation_params_set_resource_flags")]
    pub fn set_resource_flags(&mut self, resource_flags: D3D12_RESOURCE_FLAGS) -> bool {
        unsafe {
            from_glib(ffi::gst_d3d12_allocation_params_set_resource_flags(
                self.to_glib_none_mut().0,
                resource_flags.0,
            ))
        }
    }

    #[doc(alias = "gst_d3d12_allocation_params_unset_resource_flags")]
    pub fn unset_resource_flags(&mut self, resource_flags: D3D12_RESOURCE_FLAGS) -> bool {
        unsafe {
            from_glib(ffi::gst_d3d12_allocation_params_unset_resource_flags(
                self.to_glib_none_mut().0,
                resource_flags.0,
            ))
        }
    }
}
