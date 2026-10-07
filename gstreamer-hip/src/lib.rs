// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]
#![doc = include_str!("../README.md")]

pub use glib;
pub use gst;
pub use gst_video;
pub use gstreamer_hip_sys as ffi;

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

#[allow(unused_imports)]
mod auto;
pub use crate::auto::functions::*;
pub use crate::auto::*;

mod buffer;
pub use crate::buffer::*;

mod hip_allocator;
mod hip_converter;
mod hip_device_state;
mod hip_event;
mod hip_fence_data;
mod hip_host_buffer_pool;
mod hip_host_memory;
mod hip_host_pool_allocator;
mod hip_kernel_args;
mod hip_kernel_module;
mod hip_memory;
mod hip_pool_allocator;
mod hip_rtc;

pub use hip_rtc::{HipCompiledBlob, rtc_compile};

pub use hip_device_state::HipDeviceState;
pub use hip_host_memory::{HipHostMemory, HipHostMemoryRef};
pub use hip_kernel_args::HipKernelArg;
pub use hip_memory::{
    HipMemory, HipMemoryMap, HipMemoryRef, HipTextureAddressMode, HipTextureFilterMode,
    HipTextureHandle, Readable, Writable,
};

pub mod prelude {
    pub use crate::hip_converter::HipConverterExtManual;
    pub use crate::hip_host_buffer_pool::HipHostBufferPoolConfig;
    pub use crate::hip_host_pool_allocator::HipHostPoolAllocatorExtManual;
    pub use crate::{auto::traits::*, hip_pool_allocator::HipPoolAllocatorExtManual};
    #[doc(hidden)]
    pub use gst_video::prelude::*;
}
