#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
mod dma_buf_allocator;
mod fd_allocator;

pub mod prelude {
    #[doc(hidden)]
    pub use gst::subclass::prelude::*;

    #[cfg(any(target_os = "linux", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
    pub use super::dma_buf_allocator::DmaBufAllocatorImpl;
    pub use super::fd_allocator::FdAllocatorImpl;
}
