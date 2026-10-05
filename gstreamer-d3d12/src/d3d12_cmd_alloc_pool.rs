// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};
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

pub trait D3D12CmdAllocPoolExtManual: IsA<D3D12CmdAllocPool> + 'static {
    #[cfg(feature = "v1_30")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
    #[doc(alias = "gst_d3d12_cmd_alloc_pool_get_cmd_list_type")]
    #[doc(alias = "get_cmd_list_type")]
    fn cmd_list_type(&self) -> D3D12_COMMAND_LIST_TYPE {
        unsafe {
            D3D12_COMMAND_LIST_TYPE(ffi::gst_d3d12_cmd_alloc_pool_get_cmd_list_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<D3D12CmdAllocPool>> D3D12CmdAllocPoolExtManual for O {}
