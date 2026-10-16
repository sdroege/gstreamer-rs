// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{D3D12Device, Readable, Writable, ffi, ffi::GstD3D12StagingMemory};
use glib::translate::*;
use gst::{Memory, MemoryRef, ffi as gst_ffi};
use std::{fmt, marker::PhantomData, mem};
use windows::{
    Win32::Graphics::Direct3D12::{
        D3D12_PLACED_SUBRESOURCE_FOOTPRINT, ID3D12Fence, ID3D12Resource,
    },
    core::Interface,
};

gst::memory_object_wrapper!(
    D3D12StagingMemory,
    D3D12StagingMemoryRef,
    GstD3D12StagingMemory,
    |mem: &MemoryRef| { unsafe { from_glib(ffi::gst_is_d3d12_staging_memory(mem.as_mut_ptr())) } },
    Memory,
    MemoryRef
);

impl D3D12StagingMemoryRef {
    #[doc(alias = "gst_d3d12_staging_memory_get_fence")]
    #[doc(alias = "get_fence")]
    pub fn fence(&self) -> Option<(ID3D12Fence, u64)> {
        unsafe {
            let mut raw_fence: *mut std::ffi::c_void = std::ptr::null_mut();
            let mut fence_val = mem::MaybeUninit::uninit();
            if from_glib(ffi::gst_d3d12_staging_memory_get_fence(
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

    #[doc(alias = "gst_d3d12_staging_memory_get_layout")]
    #[doc(alias = "get_layout")]
    pub fn layout(
        &self,
        index: u32,
    ) -> Result<D3D12_PLACED_SUBRESOURCE_FOOTPRINT, glib::BoolError> {
        unsafe {
            let mut layout = mem::MaybeUninit::<D3D12_PLACED_SUBRESOURCE_FOOTPRINT>::uninit();
            glib::result_from_gboolean!(
                ffi::gst_d3d12_staging_memory_get_layout(
                    mut_override(&self.0),
                    index,
                    layout.as_mut_ptr().cast(),
                ),
                "Failed to get staging memory layout {}",
                index
            )?;

            Ok(layout.assume_init())
        }
    }

    #[doc(alias = "gst_d3d12_staging_memory_set_fence")]
    pub fn set_fence(&mut self, fence: Option<&ID3D12Fence>, fence_value: u64, wait: bool) {
        unsafe {
            let fence_ptr = match fence {
                Some(f) => f.as_raw(),
                None => std::ptr::null_mut(),
            };

            ffi::gst_d3d12_staging_memory_set_fence(
                mut_override(&self.0),
                fence_ptr,
                fence_value,
                wait.into_glib(),
            )
        }
    }

    #[doc(alias = "gst_d3d12_staging_memory_sync")]
    pub fn sync(&self) -> Result<(), glib::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_d3d12_staging_memory_sync(mut_override(&self.0)),
                "Failed to sync memory",
            )
        }
    }

    pub fn device(&self) -> D3D12Device {
        unsafe { from_glib_none(self.0.device) }
    }

    #[inline]
    pub fn map_readable_d3d12(
        &self,
    ) -> Result<D3D12StagingMemoryMap<'_, Readable>, glib::BoolError> {
        unsafe {
            let mut map_info = mem::MaybeUninit::uninit();
            let res = gst_ffi::gst_memory_map(
                self.upcast_memory_ref::<gst::Memory>().as_mut_ptr(),
                map_info.as_mut_ptr(),
                ffi::GST_MAP_READ_D3D12,
            );
            if res == glib::ffi::GTRUE {
                let map_info = map_info.assume_init();
                let raw = map_info.data.wrapping_sub(self.offset()).cast();
                let resource = ID3D12Resource::from_raw_borrowed(&raw).unwrap().clone();
                Ok(D3D12StagingMemoryMap {
                    resource,
                    map_info,
                    phantom: PhantomData,
                })
            } else {
                Err(glib::bool_error!("Failed to map memory readable"))
            }
        }
    }

    #[inline]
    pub fn map_writable_d3d12(
        &mut self,
    ) -> Result<D3D12StagingMemoryMap<'_, Writable>, glib::BoolError> {
        unsafe {
            let mut map_info = mem::MaybeUninit::uninit();
            let res = gst_ffi::gst_memory_map(
                self.upcast_memory_ref::<gst::Memory>().as_mut_ptr(),
                map_info.as_mut_ptr(),
                ffi::GST_MAP_WRITE_D3D12,
            );
            if res == glib::ffi::GTRUE {
                let map_info = map_info.assume_init();
                let raw = map_info.data.wrapping_sub(self.offset()).cast();
                let resource = ID3D12Resource::from_raw_borrowed(&raw).unwrap().clone();
                Ok(D3D12StagingMemoryMap {
                    resource,
                    map_info,
                    phantom: PhantomData,
                })
            } else {
                Err(glib::bool_error!("Failed to map memory writable"))
            }
        }
    }
}

impl fmt::Debug for D3D12StagingMemoryRef {
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

pub struct D3D12StagingMemoryMap<'a, T> {
    resource: ID3D12Resource,
    map_info: gst_ffi::GstMapInfo,
    phantom: PhantomData<(&'a D3D12StagingMemoryRef, T)>,
}

impl<T> D3D12StagingMemoryMap<'_, T> {
    #[inline]
    pub fn memory(&self) -> &D3D12StagingMemoryRef {
        unsafe { D3D12StagingMemoryRef::from_ptr(self.map_info.memory as _) }
    }

    #[inline]
    pub fn resource(&self) -> &ID3D12Resource {
        &self.resource
    }
}

impl<T> fmt::Debug for D3D12StagingMemoryMap<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("MemoryMap").field(&self.memory()).finish()
    }
}

impl<'a, T> PartialEq for D3D12StagingMemoryMap<'a, T> {
    fn eq(&self, other: &D3D12StagingMemoryMap<'a, T>) -> bool {
        self.resource().eq(other.resource())
    }
}

impl<T> Eq for D3D12StagingMemoryMap<'_, T> {}

impl<T> Drop for D3D12StagingMemoryMap<'_, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gst_ffi::gst_memory_unmap(self.map_info.memory, &mut self.map_info);
        }
    }
}
