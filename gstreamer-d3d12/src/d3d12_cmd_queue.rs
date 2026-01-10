// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{ffi::gpointer, prelude::*, translate::*};
use std::mem;
use windows::{
    Win32::Graphics::Direct3D12::{
        D3D12_COMMAND_QUEUE_DESC, D3D12_FENCE_FLAGS, ID3D12CommandList, ID3D12CommandQueue,
        ID3D12Device, ID3D12Fence,
    },
    core::{HRESULT, Interface, Result},
};

use crate::{D3D12CmdQueue, ffi};

impl D3D12CmdQueue {
    #[doc(alias = "gst_d3d12_cmd_queue_new")]
    pub fn new(
        device: &ID3D12Device,
        desc: &D3D12_COMMAND_QUEUE_DESC,
        flags: D3D12_FENCE_FLAGS,
        size: u32,
    ) -> D3D12CmdQueue {
        assert_initialized_main_thread!();
        unsafe {
            let desc_c_ptr = desc as *const D3D12_COMMAND_QUEUE_DESC as *const std::ffi::c_void;
            from_glib_full(ffi::gst_d3d12_cmd_queue_new(
                device.as_raw(),
                desc_c_ptr,
                flags.0,
                size,
            ))
        }
    }
}

pub trait D3D12CmdQueueExtManual: IsA<D3D12CmdQueue> + 'static {
    #[doc(alias = "gst_d3d12_cmd_queue_execute_command_lists")]
    fn execute_command_lists(&self, cmd_lists: &[Option<ID3D12CommandList>]) -> Result<u64> {
        unsafe {
            let mut fence_val = mem::MaybeUninit::uninit();
            let hr = HRESULT(ffi::gst_d3d12_cmd_queue_execute_command_lists(
                self.as_ref().to_glib_none().0,
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

    #[doc(alias = "gst_d3d12_cmd_queue_execute_wait")]
    fn execute_wait(&self, fence: &ID3D12Fence, fence_value: u64) -> Result<()> {
        unsafe {
            let hr = HRESULT(ffi::gst_d3d12_cmd_queue_execute_wait(
                self.as_ref().to_glib_none().0,
                fence.as_raw(),
                fence_value,
            ));

            if hr.is_ok() { Ok(()) } else { Err(hr.into()) }
        }
    }

    #[doc(alias = "gst_d3d12_cmd_queue_get_fence_handle")]
    #[doc(alias = "get_fence_handle")]
    fn fence_handle(&self) -> ID3D12Fence {
        unsafe {
            let raw = ffi::gst_d3d12_cmd_queue_get_fence_handle(self.as_ref().to_glib_none().0);
            ID3D12Fence::from_raw_borrowed(&raw).unwrap().clone()
        }
    }

    #[doc(alias = "gst_d3d12_cmd_queue_get_handle")]
    #[doc(alias = "get_handle")]
    fn handle(&self) -> ID3D12CommandQueue {
        unsafe {
            let raw = ffi::gst_d3d12_cmd_queue_get_handle(self.as_ref().to_glib_none().0);
            ID3D12CommandQueue::from_raw_borrowed(&raw).unwrap().clone()
        }
    }

    #[doc(alias = "gst_d3d12_cmd_queue_set_notify")]
    fn set_notify<F>(&self, fence_value: u64, func: F)
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
            ffi::gst_d3d12_cmd_queue_set_notify(
                self.as_ref().to_glib_none().0,
                fence_value,
                f as gpointer,
                Some(trampoline::<F>),
            );
        }
    }
}

impl<O: IsA<D3D12CmdQueue>> D3D12CmdQueueExtManual for O {}
