// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{D3D12DescHeap, ffi};
use windows::{Win32::Graphics::Direct3D12::ID3D12DescriptorHeap, core::Interface};

impl D3D12DescHeap {
    #[doc(alias = "gst_d3d12_desc_heap_get_handle")]
    #[doc(alias = "get_handle")]
    pub fn handle(&self) -> ID3D12DescriptorHeap {
        unsafe {
            let raw = ffi::gst_d3d12_desc_heap_get_handle(self.to_glib_none().0);
            ID3D12DescriptorHeap::from_raw_borrowed(&raw)
                .unwrap()
                .clone()
        }
    }
}
