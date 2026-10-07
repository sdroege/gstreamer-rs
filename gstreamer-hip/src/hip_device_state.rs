// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{HipDevice, HipVendor, ffi, prelude::HipDeviceExt};
use glib::{prelude::*, translate::*};
use std::{cell::UnsafeCell, ptr};

// rustdoc-stripper-ignore-next
/// Helper for safely sharing a HipDevice via `GstContext`.
///
/// The HIP context helpers can synchronously re-enter the element, so the
/// persistent device slot is protected by a recursive lock.
#[derive(Debug)]
pub struct HipDeviceState {
    lock: gst::TaskLock,
    device: UnsafeCell<*mut ffi::GstHipDevice>,
}

// SAFETY: All accesses to the device slot are protected by the recursive lock
unsafe impl Send for HipDeviceState {}
unsafe impl Sync for HipDeviceState {}

impl Default for HipDeviceState {
    fn default() -> Self {
        Self::new()
    }
}

impl HipDeviceState {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        Self {
            lock: gst::TaskLock::new(),
            device: UnsafeCell::new(ptr::null_mut()),
        }
    }

    pub fn device(&self) -> Option<HipDevice> {
        let _guard = self.lock.lock();
        unsafe { from_glib_none(*self.device.get()) }
    }

    pub fn set(&self, device: Option<&HipDevice>) {
        let _guard = self.lock.lock();
        unsafe {
            let new_device = device
                .map(|device| device.to_glib_full())
                .unwrap_or(ptr::null_mut());
            let old_device = self.device.get().replace(new_device);

            if !old_device.is_null() {
                drop(HipDevice::from_glib_full(old_device));
            }
        }
    }

    pub fn clear(&self) {
        self.set(None);
    }

    #[doc(alias = "gst_hip_ensure_element_data")]
    pub fn ensure_element_data(
        &self,
        element: &impl IsA<gst::Element>,
        vendor: HipVendor,
        device_id: i32,
    ) -> Result<(), glib::BoolError> {
        let _guard = self.lock.lock();
        // SAFETY: The device slot can be accessed recursively from set_context()
        unsafe {
            let device = *self.device.get();
            if !device.is_null()
                && ((vendor != HipVendor::Unknown
                    && HipDevice::from_glib_borrow(device).vendor() != vendor)
                    || (device_id != -1
                        && HipDevice::from_glib_borrow(device).device_id() != device_id as u32))
            {
                *self.device.get() = ptr::null_mut();
                drop(HipDevice::from_glib_full(device));
            }

            glib::result_from_gboolean!(
                ffi::gst_hip_ensure_element_data(
                    element.as_ref().to_glib_none().0,
                    vendor.into_glib(),
                    device_id,
                    self.device.get(),
                ),
                "Failed to ensure HIP device"
            )
        }
    }

    #[doc(alias = "gst_hip_handle_set_context")]
    pub fn handle_set_context(
        &self,
        element: &impl IsA<gst::Element>,
        context: &gst::ContextRef,
        vendor: HipVendor,
        device_id: i32,
    ) -> bool {
        let _guard = self.lock.lock();
        unsafe {
            from_glib(ffi::gst_hip_handle_set_context(
                element.as_ref().to_glib_none().0,
                context.as_mut_ptr(),
                vendor.into_glib(),
                device_id,
                self.device.get(),
            ))
        }
    }

    #[doc(alias = "gst_hip_ensure_element_data_for_adapter_luid")]
    pub fn ensure_element_data_for_adapter_luid(
        &self,
        element: &impl IsA<gst::Element>,
        vendor: HipVendor,
        adapter_luid: i64,
    ) -> Result<(), glib::BoolError> {
        let _guard = self.lock.lock();
        unsafe {
            let device = *self.device.get();
            if !device.is_null()
                && ((vendor != HipVendor::Unknown
                    && HipDevice::from_glib_borrow(device).vendor() != vendor)
                    || HipDevice::from_glib_borrow(device).adapter_luid() != adapter_luid)
            {
                *self.device.get() = ptr::null_mut();
                drop(HipDevice::from_glib_full(device));
            }

            glib::result_from_gboolean!(
                ffi::gst_hip_ensure_element_data_for_adapter_luid(
                    element.as_ref().to_glib_none().0,
                    vendor.into_glib(),
                    adapter_luid,
                    self.device.get(),
                ),
                "Failed to ensure HIP device"
            )
        }
    }

    #[doc(alias = "gst_hip_handle_set_context_for_adapter_luid")]
    pub fn handle_set_context_for_adapter_luid(
        &self,
        element: &impl IsA<gst::Element>,
        context: &gst::ContextRef,
        vendor: HipVendor,
        adapter_luid: i64,
    ) -> bool {
        let _guard = self.lock.lock();
        unsafe {
            from_glib(ffi::gst_hip_handle_set_context_for_adapter_luid(
                element.as_ref().to_glib_none().0,
                context.as_mut_ptr(),
                vendor.into_glib(),
                adapter_luid,
                self.device.get(),
            ))
        }
    }

    #[doc(alias = "gst_hip_ensure_element_data_for_pci_bus_id")]
    pub fn ensure_element_data_for_pci_bus_id(
        &self,
        element: &impl IsA<gst::Element>,
        vendor: HipVendor,
        pci_bus_id: &str,
    ) -> Result<(), glib::BoolError> {
        let _guard = self.lock.lock();
        unsafe {
            let device = *self.device.get();
            if !device.is_null()
                && ((vendor != HipVendor::Unknown
                    && HipDevice::from_glib_borrow(device).vendor() != vendor)
                    || HipDevice::from_glib_borrow(device).pci_bus_id().as_deref()
                        != Some(pci_bus_id))
            {
                *self.device.get() = ptr::null_mut();
                drop(HipDevice::from_glib_full(device));
            }

            glib::result_from_gboolean!(
                ffi::gst_hip_ensure_element_data_for_pci_bus_id(
                    element.as_ref().to_glib_none().0,
                    vendor.into_glib(),
                    pci_bus_id.to_glib_none().0,
                    self.device.get(),
                ),
                "Failed to ensure HIP device"
            )
        }
    }

    #[doc(alias = "gst_hip_handle_set_context_for_pci_bus_id")]
    pub fn handle_set_context_for_pci_bus_id(
        &self,
        element: &impl IsA<gst::Element>,
        context: &gst::ContextRef,
        vendor: HipVendor,
        pci_bus_id: &str,
    ) -> bool {
        let _guard = self.lock.lock();
        unsafe {
            from_glib(ffi::gst_hip_handle_set_context_for_pci_bus_id(
                element.as_ref().to_glib_none().0,
                context.as_mut_ptr(),
                vendor.into_glib(),
                pci_bus_id.to_glib_none().0,
                self.device.get(),
            ))
        }
    }

    #[doc(alias = "gst_hip_handle_context_query")]
    pub fn handle_context_query(
        &self,
        element: &impl IsA<gst::Element>,
        query: &mut gst::query::Context,
    ) -> bool {
        let device = self.device();
        unsafe {
            from_glib(ffi::gst_hip_handle_context_query(
                element.as_ref().to_glib_none().0,
                query.as_mut_ptr(),
                device.to_glib_none().0,
            ))
        }
    }
}

impl Drop for HipDeviceState {
    fn drop(&mut self) {
        // The device slot owns one reference
        unsafe {
            let device = self.device.get_mut();
            let owned = std::mem::replace(device, ptr::null_mut());
            if !owned.is_null() {
                drop(HipDevice::from_glib_full(owned));
            }
        }
    }
}
