// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use windows::{
    Win32::Graphics::Direct3D12::{D3D12_COMMAND_LIST_TYPE, ID3D12Device},
    core::Interface,
};

use crate::{D3D12CmdAllocPool, ffi};

impl D3D12CmdAllocPool {
    #[doc(alias = "gst_d3d12_cmd_alloc_pool_new")]
    pub fn new(device: &ID3D12Device, type_: D3D12_COMMAND_LIST_TYPE) -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_d3d12_cmd_alloc_pool_new(device.as_raw(), type_.0)) }
    }
}
