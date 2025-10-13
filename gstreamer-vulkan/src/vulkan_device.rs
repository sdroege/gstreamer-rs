use crate::VulkanDevice;

use glib::prelude::*;
use glib::translate::*;

pub trait VulkanDeviceExtManual: IsA<VulkanDevice> + 'static {
    fn create_shader(&self, code: &[u8]) -> Result<crate::VulkanHandle, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let shader = crate::ffi::gst_vulkan_create_shader(
                self.as_ref().to_glib_none().0,
                code.as_ptr() as *const i8,
                code.len(),
                &mut error,
            );
            debug_assert_eq!(shader.is_null(), !error.is_null());
            if error.is_null() {
                Ok(crate::VulkanHandle::from_glib_full(shader))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
impl<O: IsA<VulkanDevice>> VulkanDeviceExtManual for O {}
