// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{ffi, D3D12CmdAlloc};
use windows::{core::Interface, Win32::Graphics::Direct3D12::ID3D12CommandAllocator};

impl D3D12CmdAlloc {
    #[doc(alias = "gst_d3d12_cmd_alloc_get_handle")]
    #[doc(alias = "get_handle")]
    pub fn handle(&self) -> ID3D12CommandAllocator {
        unsafe {
            let raw = ffi::gst_d3d12_cmd_alloc_get_handle(self.to_glib_none().0);
            ID3D12CommandAllocator::from_raw_borrowed(&raw)
                .unwrap()
                .clone()
        }
    }
}
