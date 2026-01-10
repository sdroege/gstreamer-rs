// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{D3D12Device, ffi, ffi::GstD3D12Memory};
use glib::translate::*;
use gst::{Memory, MemoryRef, ffi as gst_ffi};
use std::{fmt, marker::PhantomData, mem};

use windows::{
    Win32::{
        Foundation::{HANDLE, RECT},
        Graphics::{
            Direct3D11::{ID3D11Device, ID3D11Texture2D},
            Direct3D12::{ID3D12DescriptorHeap, ID3D12Fence, ID3D12Resource},
        },
    },
    core::Interface,
};

gst::memory_object_wrapper!(
    D3D12Memory,
    D3D12MemoryRef,
    GstD3D12Memory,
    |mem: &MemoryRef| { unsafe { from_glib(ffi::gst_is_d3d12_memory(mem.as_mut_ptr())) } },
    Memory,
    MemoryRef
);

pub enum Readable {}
pub enum Writable {}

impl D3D12MemoryRef {
    #[doc(alias = "gst_d3d12_memory_get_d3d11_texture")]
    #[doc(alias = "get_d3d11_texture")]
    pub fn d3d11_texture(&self, device11: &ID3D11Device) -> Option<ID3D11Texture2D> {
        unsafe {
            let raw =
                ffi::gst_d3d12_memory_get_d3d11_texture(mut_override(&self.0), device11.as_raw());
            if raw.is_null() {
                None
            } else {
                Some(ID3D11Texture2D::from_raw_borrowed(&raw).unwrap().clone())
            }
        }
    }

    #[doc(alias = "gst_d3d12_memory_get_fence")]
    #[doc(alias = "get_fence")]
    pub fn fence(&self) -> Option<(ID3D12Fence, u64)> {
        unsafe {
            let mut raw_fence: *mut std::ffi::c_void = std::ptr::null_mut();
            let mut fence_val = mem::MaybeUninit::uninit();
            if from_glib(ffi::gst_d3d12_memory_get_fence(
                mut_override(&self.0),
                &mut raw_fence,
                fence_val.as_mut_ptr(),
            )) {
                Some((ID3D12Fence::from_raw(raw_fence), fence_val.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_d3d12_memory_get_nt_handle")]
    #[doc(alias = "get_nt_handle")]
    pub fn nt_handle(&self) -> Option<HANDLE> {
        unsafe {
            let mut raw_handle: *mut std::ffi::c_void = std::ptr::null_mut();
            if from_glib(ffi::gst_d3d12_memory_get_nt_handle(
                mut_override(&self.0),
                &mut raw_handle,
            )) {
                Some(HANDLE(raw_handle))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_d3d12_memory_get_plane_count")]
    #[doc(alias = "get_plane_count")]
    pub fn plane_count(&self) -> u32 {
        unsafe { ffi::gst_d3d12_memory_get_plane_count(mut_override(&self.0)) }
    }

    #[doc(alias = "gst_d3d12_memory_get_plane_rectangle")]
    #[doc(alias = "get_plane_rectangle")]
    pub fn plane_rectangle(&self, plane: u32) -> Result<RECT, glib::BoolError> {
        unsafe {
            let mut rect = RECT::default();
            glib::result_from_gboolean!(
                ffi::gst_d3d12_memory_get_plane_rectangle(
                    mut_override(&self.0),
                    plane,
                    &mut rect as *mut _ as glib::ffi::gpointer
                ),
                "Failed to get plane {} rect",
                plane
            )?;

            Ok(rect)
        }
    }

    #[doc(alias = "gst_d3d12_memory_get_render_target_view_heap")]
    #[doc(alias = "get_render_target_view_heap")]
    pub fn render_target_view_heap(&self) -> Option<ID3D12DescriptorHeap> {
        unsafe {
            let raw = ffi::gst_d3d12_memory_get_render_target_view_heap(mut_override(&self.0));
            if raw.is_null() {
                None
            } else {
                Some(
                    ID3D12DescriptorHeap::from_raw_borrowed(&raw)
                        .unwrap()
                        .clone(),
                )
            }
        }
    }

    #[doc(alias = "gst_d3d12_memory_get_resource_handle")]
    #[doc(alias = "get_resource_handle")]
    pub fn resource_handle(&self) -> ID3D12Resource {
        unsafe {
            let raw = ffi::gst_d3d12_memory_get_resource_handle(mut_override(&self.0));
            ID3D12Resource::from_raw_borrowed(&raw).unwrap().clone()
        }
    }

    #[doc(alias = "gst_d3d12_memory_get_shader_resource_view_heap")]
    #[doc(alias = "get_shader_resource_view_heap")]
    pub fn shader_resource_view_heap(&self) -> Option<ID3D12DescriptorHeap> {
        unsafe {
            let raw = ffi::gst_d3d12_memory_get_shader_resource_view_heap(mut_override(&self.0));
            if raw.is_null() {
                None
            } else {
                Some(
                    ID3D12DescriptorHeap::from_raw_borrowed(&raw)
                        .unwrap()
                        .clone(),
                )
            }
        }
    }

    #[doc(alias = "gst_d3d12_memory_get_subresource_index")]
    #[doc(alias = "get_subresource_index")]
    pub fn subresource_index(&self, plane: u32) -> Option<u32> {
        unsafe {
            let mut index = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_d3d12_memory_get_subresource_index(
                mut_override(&self.0),
                plane,
                index.as_mut_ptr(),
            ));
            if ret { Some(index.assume_init()) } else { None }
        }
    }

    #[doc(alias = "gst_d3d12_memory_get_unordered_access_view_heap")]
    #[doc(alias = "get_unordered_access_view_heap")]
    pub fn unordered_access_view_heap(&self) -> Option<ID3D12DescriptorHeap> {
        unsafe {
            let raw = ffi::gst_d3d12_memory_get_unordered_access_view_heap(mut_override(&self.0));
            if raw.is_null() {
                None
            } else {
                Some(
                    ID3D12DescriptorHeap::from_raw_borrowed(&raw)
                        .unwrap()
                        .clone(),
                )
            }
        }
    }

    #[doc(alias = "gst_d3d12_memory_set_fence")]
    pub fn set_fence(&mut self, fence: Option<&ID3D12Fence>, fence_value: u64, wait: bool) {
        unsafe {
            let fence_ptr = match fence {
                Some(f) => f.as_raw(),
                None => std::ptr::null_mut(),
            };

            ffi::gst_d3d12_memory_set_fence(
                mut_override(&self.0),
                fence_ptr,
                fence_value,
                wait.into_glib(),
            )
        }
    }

    #[doc(alias = "gst_d3d12_memory_sync")]
    pub fn sync(&self) -> Result<(), glib::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_d3d12_memory_sync(mut_override(&self.0)),
                "Failed to sync memory",
            )
        }
    }

    pub fn device(&self) -> D3D12Device {
        unsafe { from_glib_none(self.0.device) }
    }

    #[inline]
    pub fn map_readable_d3d12(&self) -> Result<D3D12MemoryMap<'_, Readable>, glib::BoolError> {
        unsafe {
            let mut map_info = mem::MaybeUninit::uninit();
            let res = gst_ffi::gst_memory_map(
                self.upcast_memory_ref::<gst::Memory>().as_mut_ptr(),
                map_info.as_mut_ptr(),
                ffi::GST_MAP_READ_D3D12,
            );
            if res == glib::ffi::GTRUE {
                Ok(D3D12MemoryMap {
                    resource: self.resource_handle(),
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(glib::bool_error!("Failed to map memory readable"))
            }
        }
    }

    #[inline]
    pub fn map_writable_d3d12(&mut self) -> Result<D3D12MemoryMap<'_, Writable>, glib::BoolError> {
        unsafe {
            let mut map_info = mem::MaybeUninit::uninit();
            let res = gst_ffi::gst_memory_map(
                self.upcast_memory_ref::<gst::Memory>().as_mut_ptr(),
                map_info.as_mut_ptr(),
                ffi::GST_MAP_WRITE_D3D12,
            );
            if res == glib::ffi::GTRUE {
                Ok(D3D12MemoryMap {
                    resource: self.resource_handle(),
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(glib::bool_error!("Failed to map memory writable"))
            }
        }
    }
}

impl fmt::Debug for D3D12MemoryRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Memory")
            .field("ptr", &self.as_ptr())
            .field("allocator", &self.allocator())
            .field("parent", &self.parent())
            .field("maxsize", &self.maxsize())
            .field("align", &self.align())
            .field("offset", &self.offset())
            .field("size", &self.size())
            .field("flags", &self.flags())
            .finish()
    }
}

pub struct D3D12MemoryMap<'a, T> {
    resource: ID3D12Resource,
    map_info: gst_ffi::GstMapInfo,
    phantom: PhantomData<(&'a D3D12MemoryRef, T)>,
}

impl<T> D3D12MemoryMap<'_, T> {
    #[inline]
    pub fn memory(&self) -> &D3D12MemoryRef {
        unsafe { D3D12MemoryRef::from_ptr(self.map_info.memory as _) }
    }

    #[inline]
    pub fn resource(&self) -> &ID3D12Resource {
        &self.resource
    }
}

impl<T> fmt::Debug for D3D12MemoryMap<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("MemoryMap").field(&self.memory()).finish()
    }
}

impl<'a, T> PartialEq for D3D12MemoryMap<'a, T> {
    fn eq(&self, other: &D3D12MemoryMap<'a, T>) -> bool {
        self.resource().eq(other.resource())
    }
}

impl<T> Eq for D3D12MemoryMap<'_, T> {}

impl<T> Drop for D3D12MemoryMap<'_, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gst_ffi::gst_memory_unmap(self.map_info.memory, &mut self.map_info);
        }
    }
}
