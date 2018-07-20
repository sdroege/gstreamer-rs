// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ops;
use std::ptr;
use std::slice;
use std::u64;
use std::usize;

use miniobject::*;
use BufferFlags;
use ClockTime;

use ffi;
use glib;
use glib::translate::{from_glib, from_glib_full, ToGlib};
use glib_ffi;

pub struct Readable;
pub struct Writable;

#[repr(C)]
pub struct BufferRef(ffi::GstBuffer);
pub type Buffer = GstRc<BufferRef>;

unsafe impl MiniObject for BufferRef {
    type GstType = ffi::GstBuffer;
}

pub struct BufferMap<'a, T> {
    buffer: &'a BufferRef,
    map_info: ffi::GstMapInfo,
    phantom: PhantomData<T>,
}

pub struct MappedBuffer<T> {
    buffer: Option<Buffer>,
    map_info: ffi::GstMapInfo,
    phantom: PhantomData<T>,
}

impl GstRc<BufferRef> {
    pub fn new() -> Self {
        assert_initialized_main_thread!();

        unsafe { from_glib_full(ffi::gst_buffer_new()) }
    }

    pub fn with_size(size: usize) -> Option<Self> {
        assert_initialized_main_thread!();

        unsafe {
            from_glib_full(ffi::gst_buffer_new_allocate(
                ptr::null_mut(),
                size,
                ptr::null_mut(),
            ))
        }
    }

    unsafe extern "C" fn drop_box<T>(vec: glib_ffi::gpointer) {
        let slice: Box<T> = Box::from_raw(vec as *mut T);
        drop(slice);
    }

    pub fn from_mut_slice<T: AsMut<[u8]> + Send + 'static>(slice: T) -> Option<Self> {
        assert_initialized_main_thread!();

        unsafe {
            let mut b = Box::new(slice);
            let (size, data) = {
                let slice = (*b).as_mut();
                (slice.len(), slice.as_mut_ptr())
            };
            let user_data = Box::into_raw(b);
            from_glib_full(ffi::gst_buffer_new_wrapped_full(
                0,
                data as glib_ffi::gpointer,
                size,
                0,
                size,
                user_data as glib_ffi::gpointer,
                Some(Self::drop_box::<T>),
            ))
        }
    }

    pub fn from_slice<T: AsRef<[u8]> + Send + 'static>(slice: T) -> Option<Self> {
        assert_initialized_main_thread!();

        unsafe {
            let b = Box::new(slice);
            let (size, data) = {
                let slice = (*b).as_ref();
                (slice.len(), slice.as_ptr())
            };
            let user_data = Box::into_raw(b);
            from_glib_full(ffi::gst_buffer_new_wrapped_full(
                ffi::GST_MEMORY_FLAG_READONLY,
                data as glib_ffi::gpointer,
                size,
                0,
                size,
                user_data as glib_ffi::gpointer,
                Some(Self::drop_box::<T>),
            ))
        }
    }

    pub fn into_mapped_buffer_readable(self) -> Result<MappedBuffer<Readable>, Self> {
        let mut map_info: ffi::GstMapInfo = unsafe { mem::zeroed() };
        let res: bool = unsafe {
            from_glib(ffi::gst_buffer_map(
                self.as_mut_ptr(),
                &mut map_info,
                ffi::GST_MAP_READ,
            ))
        };
        if res {
            Ok(MappedBuffer {
                buffer: Some(self),
                map_info,
                phantom: PhantomData,
            })
        } else {
            Err(self)
        }
    }

    pub fn into_mapped_buffer_writable(self) -> Result<MappedBuffer<Writable>, Self> {
        let mut map_info: ffi::GstMapInfo = unsafe { mem::zeroed() };
        let res: bool = unsafe {
            from_glib(ffi::gst_buffer_map(
                self.as_mut_ptr(),
                &mut map_info,
                ffi::GST_MAP_READWRITE,
            ))
        };
        if res {
            Ok(MappedBuffer {
                buffer: Some(self),
                map_info,
                phantom: PhantomData,
            })
        } else {
            Err(self)
        }
    }

    pub fn append(buffer: Self, other: Self) -> Self {
        skip_assert_initialized!();
        unsafe { from_glib_full(ffi::gst_buffer_append(buffer.into_ptr(), other.into_ptr())) }
    }
}

impl Default for GstRc<BufferRef> {
    fn default() -> Self {
        Self::new()
    }
}

impl BufferRef {
    pub fn map_readable(&self) -> Option<BufferMap<Readable>> {
        let mut map_info: ffi::GstMapInfo = unsafe { mem::zeroed() };
        let res =
            unsafe { ffi::gst_buffer_map(self.as_mut_ptr(), &mut map_info, ffi::GST_MAP_READ) };
        if res == glib_ffi::GTRUE {
            Some(BufferMap {
                buffer: self,
                map_info,
                phantom: PhantomData,
            })
        } else {
            None
        }
    }

    pub fn map_writable(&mut self) -> Option<BufferMap<Writable>> {
        let mut map_info: ffi::GstMapInfo = unsafe { mem::zeroed() };
        let res = unsafe {
            ffi::gst_buffer_map(self.as_mut_ptr(), &mut map_info, ffi::GST_MAP_READWRITE)
        };
        if res == glib_ffi::GTRUE {
            Some(BufferMap {
                buffer: self,
                map_info,
                phantom: PhantomData,
            })
        } else {
            None
        }
    }

    pub fn copy_region(
        &self,
        flags: ::BufferCopyFlags,
        offset: usize,
        size: Option<usize>,
    ) -> Option<Buffer> {
        let size_real = size.unwrap_or(usize::MAX);
        unsafe {
            from_glib_full(ffi::gst_buffer_copy_region(
                self.as_mut_ptr(),
                flags.to_glib(),
                offset,
                size_real,
            ))
        }
    }

    pub fn copy_into(
        &self,
        dest: &mut BufferRef,
        flags: ::BufferCopyFlags,
        offset: usize,
        size: Option<usize>,
    ) -> Result<(), glib::BoolError> {
        let size_real = size.unwrap_or(usize::MAX);
        unsafe {
            glib::BoolError::from_glib(
                ffi::gst_buffer_copy_into(
                    dest.as_mut_ptr(),
                    self.as_mut_ptr(),
                    flags.to_glib(),
                    offset,
                    size_real,
                ),
                "Failed to copy into destination buffer",
            )
        }
    }

    pub fn copy_from_slice(&mut self, offset: usize, slice: &[u8]) -> Result<(), usize> {
        let maxsize = self.get_maxsize();
        let size = slice.len();

        assert!(maxsize >= offset && maxsize - offset >= size);

        let copied = unsafe {
            let src = slice.as_ptr();
            ffi::gst_buffer_fill(
                self.as_mut_ptr(),
                offset,
                src as glib_ffi::gconstpointer,
                size,
            )
        };

        if copied == size {
            Ok(())
        } else {
            Err(copied)
        }
    }

    pub fn copy_to_slice(&self, offset: usize, slice: &mut [u8]) -> Result<(), usize> {
        let maxsize = self.get_size();
        let size = slice.len();

        assert!(maxsize >= offset && maxsize - offset >= size);

        let copied = unsafe {
            let dest = slice.as_mut_ptr();
            ffi::gst_buffer_extract(self.as_mut_ptr(), offset, dest as glib_ffi::gpointer, size)
        };

        if copied == size {
            Ok(())
        } else {
            Err(copied)
        }
    }

    pub fn copy_deep(&self) -> Option<Buffer> {
        unsafe { from_glib_full(ffi::gst_buffer_copy_deep(self.as_ptr())) }
    }

    pub fn get_size(&self) -> usize {
        unsafe { ffi::gst_buffer_get_size(self.as_mut_ptr()) }
    }

    pub fn get_maxsize(&self) -> usize {
        let mut maxsize: usize = 0;

        unsafe {
            ffi::gst_buffer_get_sizes_range(
                self.as_mut_ptr(),
                0,
                -1,
                ptr::null_mut(),
                &mut maxsize,
            );
        };

        maxsize
    }

    pub fn set_size(&mut self, size: usize) {
        assert!(self.get_maxsize() >= size);

        unsafe {
            ffi::gst_buffer_set_size(self.as_mut_ptr(), size as isize);
        }
    }

    pub fn get_offset(&self) -> u64 {
        self.0.offset
    }

    pub fn set_offset(&mut self, offset: u64) {
        self.0.offset = offset;
    }

    pub fn get_offset_end(&self) -> u64 {
        self.0.offset_end
    }

    pub fn set_offset_end(&mut self, offset_end: u64) {
        self.0.offset_end = offset_end;
    }

    pub fn get_pts(&self) -> ClockTime {
        from_glib(self.0.pts)
    }

    pub fn set_pts(&mut self, pts: ClockTime) {
        self.0.pts = pts.to_glib();
    }

    pub fn get_dts(&self) -> ClockTime {
        from_glib(self.0.dts)
    }

    pub fn set_dts(&mut self, dts: ClockTime) {
        self.0.dts = dts.to_glib();
    }

    pub fn get_dts_or_pts(&self) -> ClockTime {
        let val = self.get_dts();
        if val.is_none() {
            self.get_pts()
        } else {
            val
        }
    }

    pub fn get_duration(&self) -> ClockTime {
        from_glib(self.0.duration)
    }

    pub fn set_duration(&mut self, duration: ClockTime) {
        self.0.duration = duration.to_glib();
    }

    pub fn get_flags(&self) -> BufferFlags {
        BufferFlags::from_bits_truncate(self.0.mini_object.flags)
    }

    pub fn set_flags(&mut self, flags: BufferFlags) {
        self.0.mini_object.flags = flags.bits();
    }
}

unsafe impl Sync for BufferRef {}
unsafe impl Send for BufferRef {}

impl glib::types::StaticType for BufferRef {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_buffer_get_type()) }
    }
}

impl ToOwned for BufferRef {
    type Owned = GstRc<BufferRef>;

    fn to_owned(&self) -> GstRc<BufferRef> {
        unsafe { from_glib_full(ffi::gst_mini_object_copy(self.as_ptr() as *const _) as *mut _) }
    }
}

impl fmt::Debug for BufferRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Buffer")
            .field("ptr", unsafe { &self.as_ptr() } )
            .field("pts", &self.get_pts().to_string())
            .field("dts", &self.get_dts().to_string())
            .field("duration", &self.get_duration().to_string())
            .field("size", &self.get_size())
            .field("offset", &self.get_offset())
            .field("offset_end", &self.get_offset_end())
            .field("flags", &self.get_flags())
            .finish()
    }
}

impl PartialEq for BufferRef {
    fn eq(&self, other: &BufferRef) -> bool {
        if self.get_size() != other.get_size() {
            return false;
        }

        let self_map = self.map_readable();
        let other_map = other.map_readable();

        match (self_map, other_map) {
            (Some(self_map), Some(other_map)) => self_map.as_slice().eq(other_map.as_slice()),
            _ => false,
        }
    }
}

impl Eq for BufferRef {}

impl<'a, T> BufferMap<'a, T> {
    pub fn get_size(&self) -> usize {
        self.map_info.size
    }

    pub fn get_buffer(&self) -> &BufferRef {
        self.buffer
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.map_info.data as *const u8, self.map_info.size) }
    }
}

impl<'a> BufferMap<'a, Writable> {
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.map_info.data as *mut u8, self.map_info.size) }
    }
}

impl<'a, T> AsRef<[u8]> for BufferMap<'a, T> {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<'a> AsMut<[u8]> for BufferMap<'a, Writable> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<'a, T> ops::Deref for BufferMap<'a, T> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<'a> ops::DerefMut for BufferMap<'a, Writable> {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<'a, T> fmt::Debug for BufferMap<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("BufferMap")
            .field(&self.get_buffer())
            .finish()
    }
}

impl<'a, T> PartialEq for BufferMap<'a, T> {
    fn eq(&self, other: &BufferMap<'a, T>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<'a, T> Eq for BufferMap<'a, T> {}

impl<'a, T> Drop for BufferMap<'a, T> {
    fn drop(&mut self) {
        unsafe {
            ffi::gst_buffer_unmap(self.buffer.as_mut_ptr(), &mut self.map_info);
        }
    }
}

impl<T> MappedBuffer<T> {
    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.map_info.data as *const u8, self.map_info.size) }
    }

    pub fn get_size(&self) -> usize {
        self.map_info.size
    }

    pub fn get_buffer(&self) -> &BufferRef {
        self.buffer.as_ref().unwrap().as_ref()
    }

    pub fn into_buffer(mut self) -> Buffer {
        let buffer = self.buffer.take().unwrap();
        unsafe {
            ffi::gst_buffer_unmap(buffer.as_mut_ptr(), &mut self.map_info);
        }

        buffer
    }
}

impl MappedBuffer<Writable> {
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.map_info.data as *mut u8, self.map_info.size) }
    }
}

impl<T> AsRef<[u8]> for MappedBuffer<T> {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsMut<[u8]> for MappedBuffer<Writable> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<T> ops::Deref for MappedBuffer<T> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl ops::DerefMut for MappedBuffer<Writable> {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<T> Drop for MappedBuffer<T> {
    fn drop(&mut self) {
        if let Some(ref buffer) = self.buffer {
            unsafe {
                ffi::gst_buffer_unmap(buffer.as_mut_ptr(), &mut self.map_info);
            }
        }
    }
}

impl<T> fmt::Debug for MappedBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("MappedBuffer")
            .field(&self.get_buffer())
            .finish()
    }
}

impl<T> PartialEq for MappedBuffer<T> {
    fn eq(&self, other: &MappedBuffer<T>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<T> Eq for MappedBuffer<T> {}

unsafe impl<T> Send for MappedBuffer<T> {}

lazy_static! {
    pub static ref BUFFER_COPY_METADATA: ::BufferCopyFlags =
        ::BufferCopyFlags::FLAGS | ::BufferCopyFlags::TIMESTAMPS | ::BufferCopyFlags::META;
    pub static ref BUFFER_COPY_ALL: ::BufferCopyFlags =
        *BUFFER_COPY_METADATA | ::BufferCopyFlags::MEMORY;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fields() {
        ::init().unwrap();

        let mut buffer = Buffer::new();

        {
            let buffer = buffer.get_mut().unwrap();
            buffer.set_pts(1.into());
            buffer.set_dts(2.into());
            buffer.set_offset(3);
            buffer.set_offset_end(4);
            buffer.set_duration(5.into());
        }
        assert_eq!(buffer.get_pts(), 1.into());
        assert_eq!(buffer.get_dts(), 2.into());
        assert_eq!(buffer.get_offset(), 3);
        assert_eq!(buffer.get_offset_end(), 4);
        assert_eq!(buffer.get_duration(), 5.into());
    }

    #[test]
    fn test_writability() {
        ::init().unwrap();

        let mut buffer = Buffer::from_slice(vec![1, 2, 3, 4]).unwrap();
        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());
        }
        assert_ne!(buffer.get_mut(), None);
        {
            let buffer = buffer.get_mut().unwrap();
            buffer.set_pts(1.into());
        }

        let mut buffer2 = buffer.clone();
        assert_eq!(buffer.get_mut(), None);

        unsafe {
            assert_eq!(buffer2.as_ptr(), buffer.as_ptr());
        }

        {
            let buffer2 = buffer2.make_mut();
            unsafe {
                assert_ne!(buffer2.as_ptr(), buffer.as_ptr());
            }

            buffer2.set_pts(2.into());

            let mut data = buffer2.map_writable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());
            data.as_mut_slice()[0] = 0;
        }

        assert_eq!(buffer.get_pts(), 1.into());
        assert_eq!(buffer2.get_pts(), 2.into());

        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());

            let data = buffer2.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![0, 2, 3, 4].as_slice());
        }
    }
}
