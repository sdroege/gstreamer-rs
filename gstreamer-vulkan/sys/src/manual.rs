pub mod vulkan {
    pub use ash::vk::BaseInStructure as VkBaseInStructure;
    pub use ash::vk::BlendFactor as VkBlendFactor;
    pub use ash::vk::BlendOp as VkBlendOp;
    pub use ash::vk::Buffer as VkBuffer;
    pub use ash::vk::BufferCreateInfo as VkBufferCreateInfo;
    pub use ash::vk::BufferUsageFlags as VkBufferUsageFlags;
    pub use ash::vk::CommandBuffer as VkCommandBuffer;
    pub use ash::vk::CommandBufferLevel as VkCommandBufferLevel;
    pub use ash::vk::DescriptorPool as VkDescriptorPool;
    pub use ash::vk::DescriptorSet as VkDescriptorSet;
    pub use ash::vk::Format as VkFormat;
    pub use ash::vk::Image as VkImage;
    pub use ash::vk::ImageCreateInfo as VkImageCreateInfo;
    pub use ash::vk::ImageLayout as VkImageLayout;
    pub use ash::vk::ImageTiling as VkImageTiling;
    pub use ash::vk::ImageUsageFlags as VkImageUsageFlags;
    pub use ash::vk::ImageViewCreateInfo as VkImageViewCreateInfo;
    pub use ash::vk::MemoryHeapFlags as VkMemoryHeapFlags;
    pub use ash::vk::MemoryPropertyFlags as VkMemoryPropertyFlags;
    pub use ash::vk::MemoryRequirements as VkMemoryRequirements;
    pub use ash::vk::PhysicalDevice as VkPhysicalDevice;
    pub use ash::vk::PhysicalDeviceType as VkPhysicalDeviceType;
    pub use ash::vk::PresentModeKHR as VkPresentModeKHR;
    pub use ash::vk::Queue as VkQueue;
    pub use ash::vk::QueueFlags as VkQueueFlags;
    pub type VkQueueFlagBits = u32;
    pub use ash::vk::Result as VkResult;
    pub use ash::vk::SampleCountFlags as VkSampleCountFlags;
    pub use ash::vk::Semaphore as VkSemaphore;
    pub use ash::vk::SurfaceKHR as VkSurfaceKHR;

    #[cfg(feature = "v1_30")]
    pub type VkPhysicalDeviceFeatures2 = ash::vk::PhysicalDeviceFeatures2<'static>;
    #[cfg(feature = "v1_30")]
    pub type VkPhysicalDeviceProperties2 = ash::vk::PhysicalDeviceProperties2<'static>;
}
