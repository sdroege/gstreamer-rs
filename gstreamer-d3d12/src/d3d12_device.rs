// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{ffi::gpointer, prelude::*, translate::*};
use std::mem;
use windows::{
    Win32::Graphics::Direct3D12::{
        D3D12_COMMAND_LIST_TYPE, ID3D12CommandList, ID3D12Device, ID3D12Fence,
    },
    Win32::Graphics::Dxgi::{IDXGIAdapter1, IDXGIFactory2},
    core::{HRESULT, Interface, Result},
};

use crate::{D3D12CmdQueue, D3D12Device, ffi};

pub trait D3D12DeviceExtManual: IsA<D3D12Device> + 'static {
    #[doc(alias = "gst_d3d12_device_execute_command_lists")]
    fn execute_command_lists(
        &self,
        queue_type: D3D12_COMMAND_LIST_TYPE,
        cmd_lists: &[Option<ID3D12CommandList>],
    ) -> Result<u64> {
        unsafe {
            let mut fence_val = mem::MaybeUninit::uninit();
            let hr = HRESULT(ffi::gst_d3d12_device_execute_command_lists(
                self.as_ref().to_glib_none().0,
                queue_type.0,
                cmd_lists.len() as u32,
                core::mem::transmute::<*const Option<ID3D12CommandList>, *mut *mut std::ffi::c_void>(
                    cmd_lists.as_ptr(),
                ),
                fence_val.as_mut_ptr(),
            ));

            if hr.is_ok() {
                Ok(fence_val.assume_init())
            } else {
                Err(hr.into())
            }
        }
    }

    #[doc(alias = "gst_d3d12_device_get_adapter_handle")]
    #[doc(alias = "get_adapter_handle")]
    fn adapter_handle(&self) -> IDXGIAdapter1 {
        unsafe {
            let raw = ffi::gst_d3d12_device_get_adapter_handle(self.as_ref().to_glib_none().0);
            IDXGIAdapter1::from_raw_borrowed(&raw).unwrap().clone()
        }
    }

    #[doc(alias = "gst_d3d12_device_get_device_handle")]
    #[doc(alias = "get_device_handle")]
    fn device_handle(&self) -> ID3D12Device {
        unsafe {
            let raw = ffi::gst_d3d12_device_get_device_handle(self.as_ref().to_glib_none().0);
            ID3D12Device::from_raw_borrowed(&raw).unwrap().clone()
        }
    }

    #[doc(alias = "gst_d3d12_device_get_factory_handle")]
    #[doc(alias = "get_factory_handle")]
    fn factory_handle(&self) -> IDXGIFactory2 {
        unsafe {
            let raw = ffi::gst_d3d12_device_get_factory_handle(self.as_ref().to_glib_none().0);
            IDXGIFactory2::from_raw_borrowed(&raw).unwrap().clone()
        }
    }

    #[doc(alias = "gst_d3d12_device_get_fence_handle")]
    #[doc(alias = "get_fence_handle")]
    fn fence_handle(&self, queue_type: D3D12_COMMAND_LIST_TYPE) -> Option<ID3D12Fence> {
        unsafe {
            let raw = ffi::gst_d3d12_device_get_fence_handle(
                self.as_ref().to_glib_none().0,
                queue_type.0,
            );

            if raw.is_null() {
                None
            } else {
                Some(ID3D12Fence::from_raw_borrowed(&raw).unwrap().clone())
            }
        }
    }

    #[doc(alias = "gst_d3d12_device_fence_wait")]
    fn fence_wait(&self, queue_type: D3D12_COMMAND_LIST_TYPE, fence_value: u64) -> Result<()> {
        unsafe {
            let hr = HRESULT(ffi::gst_d3d12_device_fence_wait(
                self.as_ref().to_glib_none().0,
                queue_type.0,
                fence_value,
            ));

            if hr.is_ok() { Ok(()) } else { Err(hr.into()) }
        }
    }

    #[doc(alias = "gst_d3d12_device_get_cmd_queue")]
    #[doc(alias = "get_cmd_queue")]
    fn cmd_queue(&self, queue_type: D3D12_COMMAND_LIST_TYPE) -> Option<D3D12CmdQueue> {
        unsafe {
            from_glib_none(ffi::gst_d3d12_device_get_cmd_queue(
                self.as_ref().to_glib_none().0,
                queue_type.0,
            ))
        }
    }

    #[doc(alias = "gst_d3d12_device_get_completed_value")]
    #[doc(alias = "get_completed_value")]
    fn completed_value(&self, queue_type: D3D12_COMMAND_LIST_TYPE) -> u64 {
        unsafe {
            ffi::gst_d3d12_device_get_completed_value(self.as_ref().to_glib_none().0, queue_type.0)
        }
    }

    #[doc(alias = "gst_d3d12_device_set_fence_notify")]
    fn set_fence_notify<F>(
        &self,
        queue_type: D3D12_COMMAND_LIST_TYPE,
        fence_value: u64,
        func: F,
    ) -> std::result::Result<(), glib::error::BoolError>
    where
        F: FnOnce() + Send + 'static,
    {
        let f: Box<F> = Box::new(func);
        let f = Box::into_raw(f);

        unsafe extern "C" fn trampoline<F: FnOnce() + Send + 'static>(data: glib::ffi::gpointer) {
            unsafe {
                let func = Box::from_raw(data as *mut F);
                func()
            }
        }

        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_d3d12_device_set_fence_notify(
                    self.as_ref().to_glib_none().0,
                    queue_type.0,
                    fence_value,
                    f as gpointer,
                    Some(trampoline::<F>),
                ),
                "Failed to set fence notify"
            )
        }
    }
}

impl<O: IsA<D3D12Device>> D3D12DeviceExtManual for O {}
