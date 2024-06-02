mod vulkan_video_filter;

pub mod prelude {
    #[doc(hidden)]
    pub use gst_video::subclass::prelude::*;

    pub use super::vulkan_video_filter::{VulkanVideoFilterImpl, VulkanVideoFilterImplExt};
}
