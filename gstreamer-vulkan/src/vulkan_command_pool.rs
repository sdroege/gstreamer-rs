use crate::ffi;
use crate::VulkanCommandPool;

use glib::{prelude::*, translate::*};

// rustdoc-stripper-ignore-next
/// Represents a locked VulkanCommandPool. The command pool is unlocked when this struct is dropped.
#[derive(Debug)]
pub struct VulkanCommandPoolGuard<'a> {
    obj: &'a VulkanCommandPool,
}

impl Drop for VulkanCommandPoolGuard<'_> {
    fn drop(&mut self) {
        unsafe {
            ffi::gst_vulkan_command_pool_unlock(self.obj.to_glib_none().0);
        }
    }
}
impl PartialEq for VulkanCommandPoolGuard<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for VulkanCommandPoolGuard<'_> {}

pub trait VulkanCommandPoolExtManual: IsA<VulkanCommandPool> + 'static {
    // rustdoc-stripper-ignore-next
    /// Locks the command pool. A struct similar to `MutexGuard` is retured that unlocks the command pool once dropped.
    #[doc(alias = "gst_vulkan_command_pool_lock")]
    fn lock<'a>(&'a self) -> VulkanCommandPoolGuard<'a> {
        unsafe {
            ffi::gst_vulkan_command_pool_lock(self.as_ref().to_glib_none().0);
        }
        VulkanCommandPoolGuard {
            obj: self.upcast_ref(),
        }
    }
}
impl<O: IsA<VulkanCommandPool>> VulkanCommandPoolExtManual for O {}
