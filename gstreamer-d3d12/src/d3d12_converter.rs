// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ffi, D3D12CmdQueue, D3D12Converter, D3D12Device, D3D12FenceData};
use glib::{prelude::*, translate::*};

use windows::{
    core::Interface,
    Win32::Graphics::Direct3D12::{ID3D12GraphicsCommandList, D3D12_BLEND_DESC},
};

impl D3D12Converter {
    #[doc(alias = "gst_d3d12_converter_new")]
    pub fn new(
        device: &impl IsA<D3D12Device>,
        queue: Option<&impl IsA<D3D12CmdQueue>>,
        in_info: &gst_video::VideoInfo,
        out_info: &gst_video::VideoInfo,
        blend_desc: Option<&D3D12_BLEND_DESC>,
        blend_factor: Option<[f32; 4]>,
        config: Option<gst::Structure>,
    ) -> Option<D3D12Converter> {
        assert_initialized_main_thread!();
        unsafe {
            let blend_factor_ptr: *const f32 = match &blend_factor {
                Some(bf) => bf.as_ptr(),
                None => std::ptr::null(),
            };

            from_glib_full(ffi::gst_d3d12_converter_new(
                device.as_ref().to_glib_none().0,
                queue.map_or(std::ptr::null_mut(), |queue| {
                    queue.as_ref().to_glib_none().0
                }),
                in_info.to_glib_none().0,
                out_info.to_glib_none().0,
                blend_desc.map_or(std::ptr::null_mut(), |desc| {
                    desc as *const D3D12_BLEND_DESC as *const std::ffi::c_void
                }),
                blend_factor_ptr,
                config.to_glib_full(),
            ))
        }
    }
}

pub trait D3D12ConverterExtManual: IsA<D3D12Converter> + 'static {
    #[doc(alias = "gst_d3d12_converter_convert_buffer")]
    fn convert_buffer(
        &self,
        in_buf: &gst::Buffer,
        out_buf: &gst::Buffer,
        fence_data: &D3D12FenceData,
        command_list: &ID3D12GraphicsCommandList,
        execute_gpu_wait: bool,
    ) -> Result<(), glib::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_d3d12_converter_convert_buffer(
                    self.as_ref().to_glib_none().0,
                    in_buf.to_glib_none().0,
                    out_buf.to_glib_none().0,
                    fence_data.to_glib_none().0,
                    command_list.as_raw(),
                    execute_gpu_wait.into_glib(),
                ),
                "Failed to convert buffer"
            )
        }
    }

    #[doc(alias = "gst_d3d12_converter_update_blend_state")]
    fn update_blend_state(
        &self,
        blend_desc: Option<&D3D12_BLEND_DESC>,
        blend_factor: Option<[f32; 4]>,
    ) -> Result<(), glib::BoolError> {
        let blend_factor_ptr: *const f32 = match &blend_factor {
            Some(bf) => bf.as_ptr(),
            None => std::ptr::null(),
        };

        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_d3d12_converter_update_blend_state(
                    self.as_ref().to_glib_none().0,
                    blend_desc.map_or(std::ptr::null_mut(), |desc| {
                        desc as *const D3D12_BLEND_DESC as *const std::ffi::c_void
                    }),
                    blend_factor_ptr,
                ),
                "Failed to update blend state"
            )
        }
    }
}

impl<O: IsA<D3D12Converter>> D3D12ConverterExtManual for O {}
