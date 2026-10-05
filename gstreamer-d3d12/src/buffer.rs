// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
#[cfg(feature = "v1_28")]
use windows::Win32::Graphics::Direct3D12::D3D12_COMMAND_LIST_TYPE;
use windows::{Win32::Graphics::Direct3D12::ID3D12Fence, core::Interface};

use crate::ffi;

#[doc(alias = "gst_d3d12_buffer_copy_into")]
pub fn buffer_copy_into(
    dest: &mut gst::BufferRef,
    src: &gst::BufferRef,
    info: &gst_video::VideoInfo,
) -> Result<(), glib::BoolError> {
    skip_assert_initialized!();
    unsafe {
        glib::result_from_gboolean!(
            ffi::gst_d3d12_buffer_copy_into(
                dest.as_mut_ptr(),
                src.as_mut_ptr(),
                info.to_glib_none().0
            ),
            "Failed to copy D3D12 buffer"
        )
    }
}

#[cfg(feature = "v1_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
#[doc(alias = "gst_d3d12_buffer_copy_into_full")]
pub fn buffer_copy_into_full(
    dest: &mut gst::BufferRef,
    src: &gst::BufferRef,
    info: &gst_video::VideoInfo,
    queue_type: D3D12_COMMAND_LIST_TYPE,
) -> Result<(), glib::BoolError> {
    skip_assert_initialized!();
    unsafe {
        glib::result_from_gboolean!(
            ffi::gst_d3d12_buffer_copy_into_full(
                dest.as_mut_ptr(),
                src.as_mut_ptr(),
                info.to_glib_none().0,
                queue_type.0
            ),
            "Failed to copy D3D12 buffer"
        )
    }
}

#[doc(alias = "gst_d3d12_buffer_set_fence")]
pub fn buffer_set_fence(
    buffer: &mut gst::BufferRef,
    fence: Option<&ID3D12Fence>,
    fence_value: u64,
    wait: bool,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_d3d12_buffer_set_fence(
            buffer.as_mut_ptr(),
            fence.map_or(std::ptr::null_mut(), |f| f.as_raw()),
            fence_value,
            wait.into_glib(),
        );
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
#[doc(alias = "gst_d3d12_buffer_make_resident")]
pub fn buffer_make_resident(buffer: &mut gst::BufferRef) -> Result<(), glib::BoolError> {
    skip_assert_initialized!();
    unsafe {
        glib::result_from_gboolean!(
            ffi::gst_d3d12_buffer_make_resident(buffer.as_mut_ptr()),
            "Failed to make resident D3D12 buffer"
        )
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
#[doc(alias = "gst_d3d12_buffer_evict")]
pub fn buffer_evict(buffer: &mut gst::BufferRef) -> Result<(), glib::BoolError> {
    skip_assert_initialized!();
    unsafe {
        glib::result_from_gboolean!(
            ffi::gst_d3d12_buffer_evict(buffer.as_mut_ptr()),
            "Failed to evict D3D12 buffer"
        )
    }
}
