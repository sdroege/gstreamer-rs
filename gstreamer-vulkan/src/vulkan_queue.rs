use crate::ffi;
use crate::VulkanQueue;

use glib::{prelude::*, translate::*};

// rustdoc-stripper-ignore-next
/// Represents a locked vulkan queue that can be submitted too. The queue is unlock when this struct is dropped.
#[derive(Debug)]
pub struct VulkanQueueSubmitGuard<'a> {
    obj: &'a VulkanQueue,
}

impl Drop for VulkanQueueSubmitGuard<'_> {
    fn drop(&mut self) {
        unsafe {
            ffi::gst_vulkan_queue_submit_unlock(self.obj.to_glib_none().0);
        }
    }
}
impl PartialEq for VulkanQueueSubmitGuard<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for VulkanQueueSubmitGuard<'_> {}

pub trait VulkanQueueExtManual: IsA<VulkanQueue> + 'static {
    // rustdoc-stripper-ignore-next
    /// Locks the vulkan queue for submission. A struct similar to `MutexGuard` is retured that unlocks the queue once dropped.
    #[doc(alias = "gst_vulkan_queue_submit_lock")]
    fn submit_lock<'a>(&'a self) -> VulkanQueueSubmitGuard<'a> {
        unsafe {
            ffi::gst_vulkan_queue_submit_lock(self.as_ref().to_glib_none().0);
        }
        VulkanQueueSubmitGuard {
            obj: self.upcast_ref(),
        }
    }
}
impl<O: IsA<VulkanQueue>> VulkanQueueExtManual for O {}
