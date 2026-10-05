// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{ffi::gpointer, prelude::*, translate::*};
use std::mem;
use windows::{
    Win32::Graphics::Direct3D12::{
        D3D12_COMMAND_LIST_TYPE, D3D12_PLACED_SUBRESOURCE_FOOTPRINT, ID3D12CommandList,
        ID3D12Device, ID3D12Fence,
    },
    Win32::Graphics::Dxgi::{IDXGIAdapter1, IDXGIFactory2},
    core::{HRESULT, Interface, Result},
};

#[cfg(feature = "v1_30")]
use crate::{D3D12CmdAllocPool, D3D12FenceData};
use crate::{D3D12CmdQueue, D3D12Device, ffi};
#[cfg(feature = "v1_30")]
use windows::Win32::Graphics::Direct3D12::{
    D3D12_RESIDENCY_FLAGS, ID3D12GraphicsCommandList, ID3D12Pageable, ID3D12PipelineState,
};

pub trait D3D12DeviceExtManual: IsA<D3D12Device> + 'static {
    #[doc(alias = "gst_d3d12_context_new")]
    fn create_context(&self) -> gst::Context {
        unsafe { from_glib_full(ffi::gst_d3d12_context_new(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gst_d3d12_get_copyable_footprints")]
    fn copyable_footprints(
        &self,
        info: &gst_video::VideoInfo,
    ) -> std::result::Result<
        (
            [D3D12_PLACED_SUBRESOURCE_FOOTPRINT; gst_video::ffi::GST_VIDEO_MAX_PLANES as usize],
            u64,
        ),
        glib::BoolError,
    > {
        unsafe {
            let mut layouts = [D3D12_PLACED_SUBRESOURCE_FOOTPRINT::default();
                gst_video::ffi::GST_VIDEO_MAX_PLANES as usize];
            let mut size = 0;
            glib::result_from_gboolean!(
                ffi::gst_d3d12_get_copyable_footprints(
                    self.as_ref().to_glib_none().0,
                    info.to_glib_none().0,
                    layouts.as_mut_ptr() as gpointer,
                    &mut size
                ),
                "Failed to get D3D12 copyable footprints"
            )?;
            Ok((layouts, size))
        }
    }

    #[cfg(feature = "v1_30")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
    #[doc(alias = "gst_d3d12_device_prepare_graphics_cmd_list")]
    fn prepare_graphics_cmd_list(
        &self,
        cl: &mut Option<ID3D12GraphicsCommandList>,
        ca_pool: &D3D12CmdAllocPool,
        initial_state: Option<&ID3D12PipelineState>,
        fence_data: &D3D12FenceData,
    ) -> std::result::Result<(), glib::BoolError> {
        unsafe {
            let mut raw = cl.as_ref().map_or(std::ptr::null_mut(), Interface::as_raw);
            glib::result_from_gboolean!(
                ffi::gst_d3d12_device_prepare_graphics_cmd_list(
                    self.as_ref().to_glib_none().0,
                    &mut raw,
                    ca_pool.to_glib_none().0,
                    initial_state.map_or(std::ptr::null_mut(), Interface::as_raw),
                    fence_data.to_glib_none().0,
                ),
                "Failed to prepare D3D12 graphics command list"
            )?;

            if cl.is_none() {
                *cl = Some(ID3D12GraphicsCommandList::from_raw(raw));
            }
            Ok(())
        }
    }

    #[cfg(feature = "v1_30")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
    #[doc(alias = "gst_d3d12_device_enqueue_make_resident")]
    fn enqueue_make_resident(
        &self,
        flags: D3D12_RESIDENCY_FLAGS,
        objects: &[ID3D12Pageable],
    ) -> Result<(ID3D12Fence, u64)> {
        assert!(!objects.is_empty());
        let num_objects = u32::try_from(objects.len()).expect("Too many pageable objects");
        let mut objects: Vec<_> = objects.iter().map(Interface::as_raw).collect();
        unsafe {
            let mut fence = std::ptr::null_mut();
            let mut fence_value = 0;
            HRESULT(ffi::gst_d3d12_device_enqueue_make_resident(
                self.as_ref().to_glib_none().0,
                flags.0,
                num_objects,
                objects.as_mut_ptr(),
                &mut fence,
                &mut fence_value,
            ))
            .ok()?;
            Ok((ID3D12Fence::from_raw(fence), fence_value))
        }
    }

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
