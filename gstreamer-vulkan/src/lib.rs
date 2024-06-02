// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(unused_imports)]
#![doc = include_str!("../README.md")]

pub use ffi;
pub use glib;
pub use gst;
pub use gst_base;
pub use gst_video;

macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            gst::assert_initialized();
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

mod auto;
pub use crate::auto::*;

mod vulkan_device;
mod vulkan_full_screen_quad;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_vulkan::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst_video::prelude::*;

    pub use super::vulkan_device::VulkanDeviceExtManual;
    pub use super::vulkan_full_screen_quad::VulkanFullScreenQuadExtManual;
    pub use crate::auto::traits::*;
}

pub mod subclass;

mod caps_features;
pub use caps_features::*;
