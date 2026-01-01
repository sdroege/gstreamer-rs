use glib::prelude::*;
use glib::translate::*;

use crate::ffi;
use crate::VulkanDevice;
use crate::VulkanDisplay;
use crate::VulkanInstance;

#[doc(alias = "gst_vulkan_handle_context_query")]
pub fn context_query(
    element: &impl IsA<gst::Element>,
    query: &gst::Query,
    display: Option<&impl IsA<VulkanDisplay>>,
    instance: Option<&impl IsA<VulkanInstance>>,
    device: Option<&impl IsA<VulkanDevice>>,
) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_vulkan_handle_context_query(
            element.as_ref().to_glib_none().0,
            query.to_glib_none().0,
            display.map(|p| p.as_ref()).to_glib_none().0,
            instance.map(|p| p.as_ref()).to_glib_none().0,
            device.map(|p| p.as_ref()).to_glib_none().0,
        ))
    }
}
