// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{D3D12Device, ffi, prelude::D3D12DeviceExt};
use glib::{prelude::*, translate::*};
use std::{cell::UnsafeCell, ptr};

// rustdoc-stripper-ignore-next
/// Helper for safely sharing a D3D12Device via `GstContext`.
///
/// The D3D12 context helpers can synchronously re-enter the element, so the
/// persistent device slot is protected by a recursive lock.
#[derive(Debug)]
pub struct D3D12DeviceState {
    lock: gst::TaskLock,
    device: UnsafeCell<*mut ffi::GstD3D12Device>,
}

// SAFETY: All accesses to the device slot are protected by the recursive lock
unsafe impl Send for D3D12DeviceState {}
unsafe impl Sync for D3D12DeviceState {}

impl Default for D3D12DeviceState {
    fn default() -> Self {
        Self::new()
    }
}

impl D3D12DeviceState {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        Self {
            lock: gst::TaskLock::new(),
            device: UnsafeCell::new(ptr::null_mut()),
        }
    }

    pub fn device(&self) -> Option<D3D12Device> {
        let _guard = self.lock.lock();
        unsafe { from_glib_none(*self.device.get()) }
    }

    pub fn set(&self, device: Option<&D3D12Device>) {
        let _guard = self.lock.lock();
        unsafe {
            let new_device = device
                .map(|device| device.to_glib_full())
                .unwrap_or(ptr::null_mut());
            let old_device = self.device.get().replace(new_device);

            if !old_device.is_null() {
                drop(D3D12Device::from_glib_full(old_device));
            }
        }
    }

    pub fn clear(&self) {
        self.set(None);
    }

    #[doc(alias = "gst_d3d12_ensure_element_data")]
    pub fn ensure_element_data(
        &self,
        element: &impl IsA<gst::Element>,
        adapter_index: i32,
    ) -> Result<(), glib::BoolError> {
        let _guard = self.lock.lock();
        // SAFETY: The device slot can be accessed recursively from set_context()
        unsafe {
            let device = *self.device.get();
            if adapter_index != -1
                && !device.is_null()
                && D3D12Device::from_glib_borrow(device).adapter_index() != adapter_index as u32
            {
                *self.device.get() = ptr::null_mut();
                drop(D3D12Device::from_glib_full(device));
            }

            glib::result_from_gboolean!(
                ffi::gst_d3d12_ensure_element_data(
                    element.as_ref().to_glib_none().0,
                    adapter_index,
                    self.device.get(),
                ),
                "Failed to ensure D3D12 device"
            )
        }
    }

    #[doc(alias = "gst_d3d12_handle_set_context")]
    pub fn handle_set_context(
        &self,
        element: &impl IsA<gst::Element>,
        context: &gst::ContextRef,
        adapter_index: i32,
    ) -> bool {
        let _guard = self.lock.lock();
        unsafe {
            from_glib(ffi::gst_d3d12_handle_set_context(
                element.as_ref().to_glib_none().0,
                context.as_mut_ptr(),
                adapter_index,
                self.device.get(),
            ))
        }
    }

    #[doc(alias = "gst_d3d12_ensure_element_data_for_adapter_luid")]
    pub fn ensure_element_data_for_adapter_luid(
        &self,
        element: &impl IsA<gst::Element>,
        adapter_luid: i64,
    ) -> Result<(), glib::BoolError> {
        let _guard = self.lock.lock();
        // SAFETY: The device slot can be accessed recursively from set_context()
        unsafe {
            let device = *self.device.get();
            if !device.is_null()
                && D3D12Device::from_glib_borrow(device).adapter_luid() != adapter_luid
            {
                *self.device.get() = ptr::null_mut();
                drop(D3D12Device::from_glib_full(device));
            }

            glib::result_from_gboolean!(
                ffi::gst_d3d12_ensure_element_data_for_adapter_luid(
                    element.as_ref().to_glib_none().0,
                    adapter_luid,
                    self.device.get(),
                ),
                "Failed to ensure D3D12 device"
            )
        }
    }

    #[doc(alias = "gst_d3d12_handle_set_context_for_adapter_luid")]
    pub fn handle_set_context_for_adapter_luid(
        &self,
        element: &impl IsA<gst::Element>,
        context: &gst::ContextRef,
        adapter_luid: i64,
    ) -> bool {
        let _guard = self.lock.lock();
        unsafe {
            from_glib(ffi::gst_d3d12_handle_set_context_for_adapter_luid(
                element.as_ref().to_glib_none().0,
                context.as_mut_ptr(),
                adapter_luid,
                self.device.get(),
            ))
        }
    }

    #[doc(alias = "gst_d3d12_handle_context_query")]
    pub fn handle_context_query(
        &self,
        element: &impl IsA<gst::Element>,
        query: &mut gst::query::Context,
    ) -> bool {
        let device = self.device();
        unsafe {
            from_glib(ffi::gst_d3d12_handle_context_query(
                element.as_ref().to_glib_none().0,
                query.as_mut_ptr(),
                device.to_glib_none().0,
            ))
        }
    }
}

impl Drop for D3D12DeviceState {
    fn drop(&mut self) {
        // The device slot owns one reference
        unsafe {
            let device = self.device.get_mut();
            let owned = std::mem::replace(device, ptr::null_mut());
            if !owned.is_null() {
                drop(D3D12Device::from_glib_full(owned));
            }
        }
    }
}
