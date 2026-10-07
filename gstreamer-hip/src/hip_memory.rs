// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{HipDevice, HipEvent, HipStream, ffi, ffi::GstHipMemory};
use glib::translate::*;
use gst::{Memory, MemoryRef};
use std::{ffi::c_void, fmt, marker::PhantomData, mem};

gst::memory_object_wrapper!(
    HipMemory,
    HipMemoryRef,
    GstHipMemory,
    |mem: &MemoryRef| { unsafe { from_glib(ffi::gst_is_hip_memory(mem.as_mut_ptr())) } },
    Memory,
    MemoryRef
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum HipTextureFilterMode {
    Point = 0,
    Linear = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum HipTextureAddressMode {
    Wrap = 0,
    Clamp = 1,
    Mirror = 2,
    Border = 3,
}

// rustdoc-stripper-ignore-next
/// Non-owning texture handle, valid while its HIP memory remains alive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HipTextureHandle(u64);

impl HipTextureHandle {
    pub fn as_raw(self) -> u64 {
        self.0
    }
}

pub enum Readable {}
pub enum Writable {}

impl HipMemoryRef {
    pub fn device(&self) -> HipDevice {
        unsafe { from_glib_none(self.0.device) }
    }

    #[doc(alias = "gst_hip_memory_get_video_info")]
    pub fn video_info(&self) -> Option<gst_video::VideoInfo> {
        unsafe {
            let mut info = mem::MaybeUninit::uninit();
            if from_glib(ffi::gst_hip_memory_get_video_info(
                mut_override(&self.0),
                info.as_mut_ptr(),
            )) {
                Some(from_glib_none(info.as_ptr()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_hip_memory_get_stream")]
    pub fn stream(&self) -> Option<HipStream> {
        unsafe { from_glib_none(ffi::gst_hip_memory_get_stream(mut_override(&self.0))) }
    }

    #[doc(alias = "gst_hip_memory_get_event")]
    pub fn event(&self) -> Option<HipEvent> {
        unsafe {
            let mut event = std::ptr::null_mut();
            if from_glib(ffi::gst_hip_memory_get_event(
                mut_override(&self.0),
                &mut event,
            )) {
                from_glib_full(event)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_hip_memory_set_event")]
    pub fn set_event(&self, event: Option<&HipEvent>) {
        unsafe { ffi::gst_hip_memory_set_event(mut_override(&self.0), event.to_glib_none().0) }
    }

    #[doc(alias = "gst_hip_memory_get_texture")]
    pub fn texture(
        &self,
        plane: u32,
        filter: HipTextureFilterMode,
        address: HipTextureAddressMode,
    ) -> Result<HipTextureHandle, glib::BoolError> {
        unsafe {
            let mut texture = std::ptr::null_mut();
            glib::result_from_gboolean!(
                ffi::gst_hip_memory_get_texture(
                    mut_override(&self.0),
                    plane,
                    filter as i32,
                    address as i32,
                    &mut texture,
                ),
                "Failed to get HIP texture"
            )?;
            Ok(HipTextureHandle(texture as usize as u64))
        }
    }

    pub fn map_readable_hip(&self) -> Result<HipMemoryMap<'_, Readable>, glib::BoolError> {
        unsafe { self.map_hip(ffi::GST_MAP_READ_HIP) }
    }

    pub fn map_writable_hip(&mut self) -> Result<HipMemoryMap<'_, Writable>, glib::BoolError> {
        unsafe { self.map_hip(ffi::GST_MAP_WRITE_HIP) }
    }

    unsafe fn map_hip<T>(
        &self,
        flags: gst::ffi::GstMapFlags,
    ) -> Result<HipMemoryMap<'_, T>, glib::BoolError> {
        unsafe {
            let mut map_info = mem::MaybeUninit::uninit();
            glib::result_from_gboolean!(
                gst::ffi::gst_memory_map(
                    self.upcast_memory_ref::<Memory>().as_mut_ptr(),
                    map_info.as_mut_ptr(),
                    flags
                ),
                "Failed to map HIP memory"
            )?;

            Ok(HipMemoryMap {
                map_info: map_info.assume_init(),
                phantom: PhantomData,
            })
        }
    }
}

impl fmt::Debug for HipMemoryRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("HipMemory")
            .field(&self.upcast_memory_ref::<Memory>())
            .finish()
    }
}

pub struct HipMemoryMap<'a, T> {
    map_info: gst::ffi::GstMapInfo,
    phantom: PhantomData<(&'a HipMemoryRef, T)>,
}

impl<T> HipMemoryMap<'_, T> {
    pub fn memory(&self) -> &HipMemoryRef {
        unsafe { HipMemoryRef::from_ptr(self.map_info.memory.cast()) }
    }

    pub fn device_ptr(&self) -> *mut c_void {
        self.map_info.data.cast()
    }
}

impl<T> fmt::Debug for HipMemoryMap<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("HipMemoryMap").field(&self.memory()).finish()
    }
}

impl<T> Drop for HipMemoryMap<'_, T> {
    fn drop(&mut self) {
        unsafe { gst::ffi::gst_memory_unmap(self.map_info.memory, &mut self.map_info) }
    }
}
