// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]
#![doc = include_str!("../README.md")]

pub use glib;
pub use gst;
pub use gst_video;
pub use gstreamer_d3d12_sys as ffi;

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
pub use crate::auto::*;

mod d3d12_allocation_params;
mod d3d12_allocator;
mod d3d12_buffer_pool;
mod d3d12_cmd_alloc;
mod d3d12_cmd_alloc_pool;
mod d3d12_cmd_queue;
mod d3d12_converter;
mod d3d12_desc_heap;
mod d3d12_desc_heap_pool;
mod d3d12_device;
mod d3d12_fence_data;
mod d3d12_memory;
pub use crate::d3d12_memory::*;
mod d3d12_pool_allocator;

pub mod prelude {
    #[doc(hidden)]
    pub use gst_video::prelude::*;

    pub use crate::{
        auto::traits::*, d3d12_buffer_pool::D3D12BufferPoolConfig,
        d3d12_cmd_queue::D3D12CmdQueueExtManual, d3d12_converter::D3D12ConverterExtManual,
        d3d12_device::D3D12DeviceExtManual,
    };
}
