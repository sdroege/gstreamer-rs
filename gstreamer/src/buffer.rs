// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::io;
use std::marker::PhantomData;
use std::mem;
use std::ops;
use std::ptr;
use std::slice;
use std::u64;
use std::usize;

use meta::*;
use miniobject::*;
use BufferFlags;
use ClockTime;
use Memory;
use MemoryRef;

use glib;
use glib::translate::{from_glib, from_glib_full, FromGlib, FromGlibPtrFull, ToGlib};
use glib_sys;
use gst_sys;

pub enum Readable {}
pub enum Writable {}

gst_define_mini_object_wrapper!(
    Buffer,
    BufferRef,
    gst_sys::GstBuffer,
    [Debug, PartialEq, Eq,],
    || gst_sys::gst_buffer_get_type()
);

pub struct BufferMap<'a, T> {
    buffer: &'a BufferRef,
    map_info: gst_sys::GstMapInfo,
    phantom: PhantomData<T>,
}

pub struct MappedBuffer<T> {
    buffer: Option<Buffer>,
    map_info: gst_sys::GstMapInfo,
    phantom: PhantomData<T>,
}

pub struct BufferCursor<T> {
    buffer: Option<Buffer>,
    size: u64,
    num_mem: u32,
    cur_mem_idx: u32,
    cur_offset: u64,
    cur_mem_offset: usize,
    map_info: gst_sys::GstMapInfo,
    phantom: PhantomData<T>,
}

pub struct BufferCursorRef<T> {
    buffer: T,
    size: u64,
    num_mem: u32,
    cur_mem_idx: u32,
    cur_offset: u64,
    cur_mem_offset: usize,
    map_info: gst_sys::GstMapInfo,
}

impl Buffer {
    pub fn new() -> Self {
        assert_initialized_main_thread!();

        unsafe { from_glib_full(gst_sys::gst_buffer_new()) }
    }

    pub fn with_size(size: usize) -> Result<Self, glib::BoolError> {
        assert_initialized_main_thread!();

        unsafe {
            Option::<_>::from_glib_full(gst_sys::gst_buffer_new_allocate(
                ptr::null_mut(),
                size,
                ptr::null_mut(),
            ))
            .ok_or_else(|| glib_bool_error!("Failed to allocate buffer"))
        }
    }

    unsafe extern "C" fn drop_box<T>(vec: glib_sys::gpointer) {
        let slice: Box<T> = Box::from_raw(vec as *mut T);
        drop(slice);
    }

    pub fn from_mut_slice<T: AsMut<[u8]> + Send + 'static>(slice: T) -> Self {
        assert_initialized_main_thread!();

        unsafe {
            let mut b = Box::new(slice);
            let (size, data) = {
                let slice = (*b).as_mut();
                (slice.len(), slice.as_mut_ptr())
            };
            let user_data = Box::into_raw(b);
            from_glib_full(gst_sys::gst_buffer_new_wrapped_full(
                0,
                data as glib_sys::gpointer,
                size,
                0,
                size,
                user_data as glib_sys::gpointer,
                Some(Self::drop_box::<T>),
            ))
        }
    }

    pub fn from_slice<T: AsRef<[u8]> + Send + 'static>(slice: T) -> Self {
        assert_initialized_main_thread!();

        unsafe {
            let b = Box::new(slice);
            let (size, data) = {
                let slice = (*b).as_ref();
                (slice.len(), slice.as_ptr())
            };
            let user_data = Box::into_raw(b);
            from_glib_full(gst_sys::gst_buffer_new_wrapped_full(
                gst_sys::GST_MEMORY_FLAG_READONLY,
                data as glib_sys::gpointer,
                size,
                0,
                size,
                user_data as glib_sys::gpointer,
                Some(Self::drop_box::<T>),
            ))
        }
    }

    pub fn into_mapped_buffer_readable(self) -> Result<MappedBuffer<Readable>, Self> {
        unsafe {
            let mut map_info = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(gst_sys::gst_buffer_map(
                self.as_mut_ptr(),
                map_info.as_mut_ptr(),
                gst_sys::GST_MAP_READ,
            ));
            if res {
                Ok(MappedBuffer {
                    buffer: Some(self),
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(self)
            }
        }
    }

    pub fn into_mapped_buffer_writable(self) -> Result<MappedBuffer<Writable>, Self> {
        unsafe {
            let mut map_info = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(gst_sys::gst_buffer_map(
                self.as_mut_ptr(),
                map_info.as_mut_ptr(),
                gst_sys::GST_MAP_READWRITE,
            ));
            if res {
                Ok(MappedBuffer {
                    buffer: Some(self),
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(self)
            }
        }
    }

    pub fn into_buffer_cursor_readable(self) -> BufferCursor<Readable> {
        BufferCursor::new_readable(self)
    }

    pub fn into_buffer_cursor_writable(self) -> Result<BufferCursor<Writable>, glib::BoolError> {
        BufferCursor::new_writable(self)
    }

    pub fn append(&mut self, other: Self) {
        skip_assert_initialized!();
        unsafe {
            let ptr = gst_sys::gst_buffer_append(self.as_mut_ptr(), other.into_ptr());
            self.replace_ptr(ptr);
        }
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl BufferRef {
    pub fn map_readable(&self) -> Result<BufferMap<Readable>, glib::BoolError> {
        unsafe {
            let mut map_info = mem::MaybeUninit::zeroed();
            let res = gst_sys::gst_buffer_map(
                self.as_mut_ptr(),
                map_info.as_mut_ptr(),
                gst_sys::GST_MAP_READ,
            );
            if res == glib_sys::GTRUE {
                Ok(BufferMap {
                    buffer: self,
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(glib_bool_error!("Failed to map buffer readable"))
            }
        }
    }

    pub fn map_writable(&mut self) -> Result<BufferMap<Writable>, glib::BoolError> {
        unsafe {
            let mut map_info = mem::MaybeUninit::zeroed();
            let res = gst_sys::gst_buffer_map(
                self.as_mut_ptr(),
                map_info.as_mut_ptr(),
                gst_sys::GST_MAP_READWRITE,
            );
            if res == glib_sys::GTRUE {
                Ok(BufferMap {
                    buffer: self,
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(glib_bool_error!("Failed to map buffer writable"))
            }
        }
    }

    pub fn copy_region(
        &self,
        flags: ::BufferCopyFlags,
        offset: usize,
        size: Option<usize>,
    ) -> Result<Buffer, glib::BoolError> {
        let size_real = size.unwrap_or(usize::MAX);
        unsafe {
            Option::<_>::from_glib_full(gst_sys::gst_buffer_copy_region(
                self.as_mut_ptr(),
                flags.to_glib(),
                offset,
                size_real,
            ))
            .ok_or_else(|| glib_bool_error!("Failed to copy region of buffer"))
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
            glib_result_from_gboolean!(
                gst_sys::gst_buffer_copy_into(
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
            gst_sys::gst_buffer_fill(
                self.as_mut_ptr(),
                offset,
                src as glib_sys::gconstpointer,
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
            gst_sys::gst_buffer_extract(self.as_mut_ptr(), offset, dest as glib_sys::gpointer, size)
        };

        if copied == size {
            Ok(())
        } else {
            Err(copied)
        }
    }

    pub fn copy_deep(&self) -> Result<Buffer, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(gst_sys::gst_buffer_copy_deep(self.as_ptr()))
                .ok_or_else(|| glib_bool_error!("Failed to deep copy buffer"))
        }
    }

    pub fn get_size(&self) -> usize {
        unsafe { gst_sys::gst_buffer_get_size(self.as_mut_ptr()) }
    }

    pub fn get_maxsize(&self) -> usize {
        unsafe {
            let mut maxsize = mem::MaybeUninit::uninit();
            gst_sys::gst_buffer_get_sizes_range(
                self.as_mut_ptr(),
                0,
                -1,
                ptr::null_mut(),
                maxsize.as_mut_ptr(),
            );

            maxsize.assume_init()
        }
    }

    pub fn set_size(&mut self, size: usize) {
        assert!(self.get_maxsize() >= size);

        unsafe {
            gst_sys::gst_buffer_set_size(self.as_mut_ptr(), size as isize);
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
        self.0.mini_object.flags |= flags.bits();
    }

    pub fn unset_flags(&mut self, flags: BufferFlags) {
        self.0.mini_object.flags &= !flags.bits();
    }

    pub fn get_meta<T: MetaAPI>(&self) -> Option<MetaRef<T>> {
        unsafe {
            let meta = gst_sys::gst_buffer_get_meta(self.as_mut_ptr(), T::get_meta_api().to_glib());
            if meta.is_null() {
                None
            } else {
                Some(T::from_ptr(self, meta as *const <T as MetaAPI>::GstType))
            }
        }
    }

    pub fn get_meta_mut<T: MetaAPI>(&mut self) -> Option<MetaRefMut<T, ::meta::Standalone>> {
        unsafe {
            let meta = gst_sys::gst_buffer_get_meta(self.as_mut_ptr(), T::get_meta_api().to_glib());
            if meta.is_null() {
                None
            } else {
                Some(T::from_mut_ptr(self, meta as *mut <T as MetaAPI>::GstType))
            }
        }
    }

    pub fn iter_meta<T: MetaAPI>(&self) -> MetaIter<T> {
        MetaIter::new(self)
    }

    pub fn iter_meta_mut<T: MetaAPI>(&mut self) -> MetaIterMut<T> {
        MetaIterMut::new(self)
    }

    pub fn append_memory(&mut self, mem: Memory) {
        unsafe { gst_sys::gst_buffer_append_memory(self.as_mut_ptr(), mem.into_ptr()) }
    }

    pub fn find_memory(&self, offset: usize, size: Option<usize>) -> Option<(u32, u32, usize)> {
        unsafe {
            let mut idx = mem::MaybeUninit::uninit();
            let mut length = mem::MaybeUninit::uninit();
            let mut skip = mem::MaybeUninit::uninit();

            let res = from_glib(gst_sys::gst_buffer_find_memory(
                self.as_mut_ptr(),
                offset,
                size.unwrap_or(usize::MAX),
                idx.as_mut_ptr(),
                length.as_mut_ptr(),
                skip.as_mut_ptr(),
            ));

            if res {
                Some((idx.assume_init(), length.assume_init(), skip.assume_init()))
            } else {
                None
            }
        }
    }

    pub fn get_all_memory(&self) -> Option<Memory> {
        unsafe {
            let res = gst_sys::gst_buffer_get_all_memory(self.as_mut_ptr());
            if res.is_null() {
                None
            } else {
                Some(from_glib_full(res))
            }
        }
    }

    pub fn get_max_memory() -> u32 {
        unsafe { gst_sys::gst_buffer_get_max_memory() }
    }

    pub fn get_memory(&self, idx: u32) -> Option<Memory> {
        if idx >= self.n_memory() {
            None
        } else {
            unsafe {
                let res = gst_sys::gst_buffer_get_memory(self.as_mut_ptr(), idx);
                if res.is_null() {
                    None
                } else {
                    Some(from_glib_full(res))
                }
            }
        }
    }

    pub fn get_memory_range(&self, idx: u32, length: Option<u32>) -> Option<Memory> {
        assert!(idx + length.unwrap_or(0) < self.n_memory());
        unsafe {
            let res = gst_sys::gst_buffer_get_memory_range(
                self.as_mut_ptr(),
                idx,
                match length {
                    Some(val) => val as i32,
                    None => -1,
                },
            );
            if res.is_null() {
                None
            } else {
                Some(from_glib_full(res))
            }
        }
    }

    pub fn insert_memory(&mut self, idx: Option<u32>, mem: Memory) {
        unsafe {
            gst_sys::gst_buffer_insert_memory(
                self.as_mut_ptr(),
                match idx {
                    Some(val) => val as i32,
                    None => -1,
                },
                mem.into_ptr(),
            )
        }
    }

    pub fn is_all_memory_writable(&self) -> bool {
        unsafe {
            from_glib(gst_sys::gst_buffer_is_all_memory_writable(
                self.as_mut_ptr(),
            ))
        }
    }

    pub fn is_memory_range_writable(&self, idx: u32, length: Option<u16>) -> bool {
        unsafe {
            from_glib(gst_sys::gst_buffer_is_memory_range_writable(
                self.as_mut_ptr(),
                idx,
                match length {
                    Some(val) => val as i32,
                    None => -1,
                },
            ))
        }
    }

    pub fn n_memory(&self) -> u32 {
        unsafe { gst_sys::gst_buffer_n_memory(self.as_ptr() as *mut _) }
    }

    pub fn peek_memory(&self, idx: u32) -> &MemoryRef {
        assert!(idx < self.n_memory());
        unsafe { MemoryRef::from_ptr(gst_sys::gst_buffer_peek_memory(self.as_mut_ptr(), idx)) }
    }

    pub fn peek_memory_mut(&mut self, idx: u32) -> Result<&mut MemoryRef, glib::BoolError> {
        assert!(idx < self.n_memory());
        unsafe {
            let mem = gst_sys::gst_buffer_peek_memory(self.as_mut_ptr(), idx);
            if gst_sys::gst_mini_object_is_writable(mem as *mut _) == glib_sys::GFALSE {
                Err(glib_bool_error!("Memory not writable"))
            } else {
                Ok(MemoryRef::from_mut_ptr(gst_sys::gst_buffer_peek_memory(
                    self.as_mut_ptr(),
                    idx,
                )))
            }
        }
    }

    pub fn prepend_memory(&mut self, mem: Memory) {
        unsafe { gst_sys::gst_buffer_prepend_memory(self.as_mut_ptr(), mem.into_ptr()) }
    }

    pub fn remove_all_memory(&mut self) {
        unsafe { gst_sys::gst_buffer_remove_all_memory(self.as_mut_ptr()) }
    }

    pub fn remove_memory(&mut self, idx: u32) {
        assert!(idx < self.n_memory());
        unsafe { gst_sys::gst_buffer_remove_memory(self.as_mut_ptr(), idx) }
    }

    pub fn remove_memory_range(&mut self, idx: u32, length: Option<u32>) {
        assert!(idx + length.unwrap_or(0) < self.n_memory());
        unsafe {
            gst_sys::gst_buffer_remove_memory_range(
                self.as_mut_ptr(),
                idx,
                match length {
                    Some(val) => val as i32,
                    None => -1,
                },
            )
        }
    }

    pub fn replace_all_memory(&mut self, mem: Memory) {
        unsafe { gst_sys::gst_buffer_replace_all_memory(self.as_mut_ptr(), mem.into_ptr()) }
    }

    pub fn replace_memory(&mut self, idx: u32, mem: Memory) {
        assert!(idx < self.n_memory());
        unsafe { gst_sys::gst_buffer_replace_memory(self.as_mut_ptr(), idx, mem.into_ptr()) }
    }

    pub fn replace_memory_range(&mut self, idx: u32, length: Option<u32>, mem: Memory) {
        assert!(idx + length.unwrap_or(0) < self.n_memory());
        unsafe {
            gst_sys::gst_buffer_replace_memory_range(
                self.as_mut_ptr(),
                idx,
                match length {
                    Some(val) => val as i32,
                    None => -1,
                },
                mem.into_ptr(),
            )
        }
    }

    pub fn iter_memories(&self) -> Iter {
        Iter::new(self)
    }

    pub fn iter_memories_mut(&mut self) -> Result<IterMut, glib::BoolError> {
        if !self.is_all_memory_writable() {
            Err(glib_bool_error!("Not all memory are writable"))
        } else {
            Ok(IterMut::new(self))
        }
    }

    pub fn iter_memories_owned(&self) -> IterOwned {
        IterOwned::new(self)
    }

    pub fn as_buffer_cursor_ref_readable<'a>(&'a self) -> BufferCursorRef<&'a BufferRef> {
        BufferCursorRef::new_readable(self)
    }

    pub fn as_buffer_cursor_ref_writable<'a>(
        &'a mut self,
    ) -> Result<BufferCursorRef<&'a mut BufferRef>, glib::BoolError> {
        BufferCursorRef::new_writable(self)
    }
}

macro_rules! define_meta_iter(
    ($name:ident, $typ:ty, $mtyp:ty, $prepare_buffer:expr, $from_ptr:expr) => {
    pub struct $name<'a, T: MetaAPI + 'a> {
        buffer: $typ,
        state: glib_sys::gpointer,
        meta_api: glib::Type,
        items: PhantomData<$mtyp>,
    }

    unsafe impl<'a, T: MetaAPI> Send for $name<'a, T> { }
    unsafe impl<'a, T: MetaAPI> Sync for $name<'a, T> { }

    impl<'a, T: MetaAPI> fmt::Debug for $name<'a, T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct(stringify!($name))
                .field("buffer", &self.buffer)
                .field("state", &self.state)
                .field("meta_api", &self.meta_api)
                .field("items", &self.items)
                .finish()
        }
    }

    impl<'a, T: MetaAPI> $name<'a, T> {
        fn new(buffer: $typ) -> $name<'a, T> {
            skip_assert_initialized!();

            $name {
                buffer,
                state: ptr::null_mut(),
                meta_api: T::get_meta_api(),
                items: PhantomData,
            }
        }
    }

    impl<'a, T: MetaAPI> Iterator for $name<'a, T> {
        type Item = $mtyp;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                unsafe {
                    let meta = gst_sys::gst_buffer_iterate_meta(self.buffer.as_mut_ptr(), &mut self.state);

                    if meta.is_null() {
                        return None;
                    } else if self.meta_api == glib::Type::Invalid || glib::Type::from_glib((*(*meta).info).api) == self.meta_api {
                        // FIXME: Workaround for a lifetime issue with the mutable iterator only
                        let buffer = $prepare_buffer(self.buffer.as_mut_ptr());
                        let item = $from_ptr(buffer, meta);
                        return Some(item);
                    }
                }
            }
        }
    }

    impl<'a, T: MetaAPI> ExactSizeIterator for $name<'a, T> {}
    }
);

define_meta_iter!(
    MetaIter,
    &'a BufferRef,
    MetaRef<'a, T>,
    |buffer: *const gst_sys::GstBuffer| BufferRef::from_ptr(buffer),
    |buffer, meta| T::from_ptr(buffer, meta as *const <T as MetaAPI>::GstType)
);
define_meta_iter!(
    MetaIterMut,
    &'a mut BufferRef,
    MetaRefMut<'a, T, ::meta::Iterated>,
    |buffer: *mut gst_sys::GstBuffer| BufferRef::from_mut_ptr(buffer),
    |buffer: &'a mut BufferRef, meta| T::from_mut_ptr(buffer, meta as *mut <T as MetaAPI>::GstType)
);

macro_rules! define_iter(
    ($name:ident, $typ:ty, $mtyp:ty, $get_item:expr) => {
    pub struct $name<'a> {
        buffer: $typ,
        idx: u32,
        n_memory: u32,
    }

    impl<'a> fmt::Debug for $name<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct(stringify!($name))
                .field("buffer", &self.buffer)
                .field("idx", &self.idx)
                .field("n_memory", &self.n_memory)
                .finish()
        }
    }

    impl<'a> $name<'a> {
        fn new(buffer: $typ) -> $name<'a> {
            skip_assert_initialized!();

            let n_memory = buffer.n_memory();

            $name {
                buffer,
                idx: 0,
                n_memory,
            }
        }
    }

    impl<'a> Iterator for $name<'a> {
        type Item = $mtyp;

        fn next(&mut self) -> Option<Self::Item> {
            if self.idx >= self.n_memory {
                return None;
            }

            #[allow(unused_unsafe)]
            unsafe {
                let item = $get_item(self.buffer, self.idx)?;
                self.idx += 1;
                Some(item)
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            if self.idx == self.n_memory {
                return (0, Some(0));
            }

            let remaining = (self.n_memory - self.idx) as usize;

            (remaining, Some(remaining))
        }
    }

    impl<'a> DoubleEndedIterator for $name<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.idx == self.n_memory {
                return None;
            }

            self.n_memory -= 1;

            #[allow(unused_unsafe)]
            unsafe {
                $get_item(self.buffer, self.n_memory)
            }
        }
    }

    impl<'a> ExactSizeIterator for $name<'a> {}
    }
);

define_iter!(
    Iter,
    &'a BufferRef,
    &'a MemoryRef,
    |buffer: &BufferRef, idx| {
        let ptr = gst_sys::gst_buffer_peek_memory(buffer.as_mut_ptr(), idx);
        if ptr.is_null() {
            None
        } else {
            Some(MemoryRef::from_ptr(ptr as *const gst_sys::GstMemory))
        }
    }
);

define_iter!(
    IterMut,
    &'a mut BufferRef,
    &'a mut MemoryRef,
    |buffer: &mut BufferRef, idx| {
        let ptr = gst_sys::gst_buffer_peek_memory(buffer.as_mut_ptr(), idx);
        if ptr.is_null() {
            None
        } else {
            Some(MemoryRef::from_mut_ptr(ptr as *mut gst_sys::GstMemory))
        }
    }
);

define_iter!(
    IterOwned,
    &'a BufferRef,
    Memory,
    |buffer: &BufferRef, idx| { buffer.get_memory(idx) }
);

impl fmt::Debug for BufferRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::cell::RefCell;

        struct DebugIter<I>(RefCell<I>);
        impl<I: Iterator> fmt::Debug for DebugIter<I>
        where
            I::Item: fmt::Debug,
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_list().entries(&mut *self.0.borrow_mut()).finish()
            }
        }

        f.debug_struct("Buffer")
            .field("ptr", unsafe { &self.as_ptr() })
            .field("pts", &self.get_pts().to_string())
            .field("dts", &self.get_dts().to_string())
            .field("duration", &self.get_duration().to_string())
            .field("size", &self.get_size())
            .field("offset", &self.get_offset())
            .field("offset_end", &self.get_offset_end())
            .field("flags", &self.get_flags())
            .field(
                "metas",
                &DebugIter(RefCell::new(
                    self.iter_meta::<::Meta>().map(|m| m.get_api()),
                )),
            )
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
            (Ok(self_map), Ok(other_map)) => self_map.as_slice().eq(other_map.as_slice()),
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
            gst_sys::gst_buffer_unmap(self.buffer.as_mut_ptr(), &mut self.map_info);
        }
    }
}

unsafe impl<'a, T> Send for BufferMap<'a, T> {}
unsafe impl<'a, T> Sync for BufferMap<'a, T> {}

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
            gst_sys::gst_buffer_unmap(buffer.as_mut_ptr(), &mut self.map_info);
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
                gst_sys::gst_buffer_unmap(buffer.as_mut_ptr(), &mut self.map_info);
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
unsafe impl<T> Sync for MappedBuffer<T> {}

impl<T> fmt::Debug for BufferCursor<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BufferCursor")
            .field("buffer", &self.buffer)
            .field("size", &self.size)
            .field("num_mem", &self.num_mem)
            .field("cur_mem_idx", &self.cur_mem_idx)
            .field("cur_offset", &self.cur_offset)
            .field("cur_mem_offset", &self.cur_mem_offset)
            .field("map_info", &self.map_info)
            .finish()
    }
}

impl<T> Drop for BufferCursor<T> {
    fn drop(&mut self) {
        if !self.map_info.memory.is_null() {
            unsafe {
                gst_sys::gst_memory_unmap(self.map_info.memory, &mut self.map_info);
            }
        }
    }
}

impl io::Read for BufferCursor<Readable> {
    fn read(&mut self, mut data: &mut [u8]) -> Result<usize, io::Error> {
        let mut copied = 0;

        while !data.is_empty() && self.cur_mem_idx < self.num_mem {
            // Map memory if needed. cur_mem_idx, cur_mem_offset and cur_offset are required to be
            // set correctly here already (from constructor, seek and the bottom of the loop)
            if self.map_info.memory.is_null() {
                unsafe {
                    let memory = gst_sys::gst_buffer_peek_memory(
                        self.buffer.as_ref().unwrap().as_mut_ptr(),
                        self.cur_mem_idx,
                    );
                    assert!(!memory.is_null());

                    if gst_sys::gst_memory_map(memory, &mut self.map_info, gst_sys::GST_MAP_READ)
                        == glib_sys::GFALSE
                    {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Failed to map memory readable",
                        ));
                    }
                }

                assert!(self.cur_mem_offset < self.map_info.size);
            }

            assert!(!self.map_info.memory.is_null());

            // Copy all data we can currently copy
            let data_left = self.map_info.size - self.cur_mem_offset;
            let to_copy = std::cmp::min(data.len(), data_left);
            unsafe {
                ptr::copy_nonoverlapping(
                    (self.map_info.data as *const u8).add(self.cur_mem_offset),
                    data.as_mut_ptr(),
                    to_copy,
                );
            }
            copied += to_copy;
            self.cur_offset += to_copy as u64;
            self.cur_mem_offset += to_copy;
            data = &mut data[to_copy..];

            // If we're at the end of the current memory, unmap and advance to the next memory
            if self.cur_mem_offset == self.map_info.size {
                unsafe {
                    gst_sys::gst_memory_unmap(self.map_info.memory, &mut self.map_info);
                }
                self.map_info.memory = ptr::null_mut();
                self.cur_mem_idx += 1;
                self.cur_mem_offset = 0;
            }
        }

        Ok(copied)
    }
}

impl io::Write for BufferCursor<Writable> {
    fn write(&mut self, mut data: &[u8]) -> Result<usize, io::Error> {
        let mut copied = 0;

        while !data.is_empty() && self.cur_mem_idx < self.num_mem {
            // Map memory if needed. cur_mem_idx, cur_mem_offset and cur_offset are required to be
            // set correctly here already (from constructor, seek and the bottom of the loop)
            if self.map_info.memory.is_null() {
                unsafe {
                    let memory = gst_sys::gst_buffer_peek_memory(
                        self.buffer.as_ref().unwrap().as_mut_ptr(),
                        self.cur_mem_idx,
                    );
                    assert!(!memory.is_null());

                    if gst_sys::gst_memory_map(memory, &mut self.map_info, gst_sys::GST_MAP_WRITE)
                        == glib_sys::GFALSE
                    {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Failed to map memory writable",
                        ));
                    }
                }

                assert!(self.cur_mem_offset < self.map_info.size);
            }

            assert!(!self.map_info.memory.is_null());

            // Copy all data we can currently copy
            let data_left = self.map_info.size - self.cur_mem_offset;
            let to_copy = std::cmp::min(data.len(), data_left);
            unsafe {
                ptr::copy_nonoverlapping(
                    data.as_ptr(),
                    (self.map_info.data as *mut u8).add(self.cur_mem_offset),
                    to_copy,
                );
            }
            copied += to_copy;
            self.cur_offset += to_copy as u64;
            self.cur_mem_offset += to_copy;
            data = &data[to_copy..];

            // If we're at the end of the current memory, unmap and advance to the next memory
            if self.cur_mem_offset == self.map_info.size {
                unsafe {
                    gst_sys::gst_memory_unmap(self.map_info.memory, &mut self.map_info);
                }
                self.map_info.memory = ptr::null_mut();
                self.cur_mem_idx += 1;
                self.cur_mem_offset = 0;
            }
        }

        Ok(copied)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}

impl<T> io::Seek for BufferCursor<T> {
    fn seek(&mut self, pos: io::SeekFrom) -> Result<u64, io::Error> {
        if !self.map_info.memory.is_null() {
            unsafe {
                gst_sys::gst_memory_unmap(self.map_info.memory, &mut self.map_info);
                self.map_info.memory = ptr::null_mut();
            }
        }

        match pos {
            io::SeekFrom::Start(off) => {
                self.cur_offset = std::cmp::min(self.size, off);
            }
            io::SeekFrom::End(off) if off <= 0 => {
                self.cur_offset = self.size;
            }
            io::SeekFrom::End(off) => {
                self.cur_offset = self.size.checked_sub(off as u64).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "Seek before start of buffer")
                })?;
            }
            io::SeekFrom::Current(std::i64::MIN) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Seek before start of buffer",
                ));
            }
            io::SeekFrom::Current(off) => {
                if off <= 0 {
                    self.cur_offset =
                        self.cur_offset.checked_sub((-off) as u64).ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::InvalidInput,
                                "Seek before start of buffer",
                            )
                        })?;
                } else {
                    self.cur_offset = std::cmp::min(
                        self.size,
                        self.cur_offset.checked_add(off as u64).unwrap_or(self.size),
                    );
                }
            }
        }

        let (idx, _, skip) = self
            .buffer
            .as_ref()
            .unwrap()
            .find_memory(self.cur_offset as usize, None)
            .expect("Failed to find memory");
        self.cur_mem_idx = idx;
        self.cur_mem_offset = skip;

        Ok(self.cur_offset)
    }

    // Once stabilized
    //    fn stream_len(&mut self) -> Result<u64, io::Error> {
    //        Ok(self.size)
    //    }
    //
    //    fn stream_position(&mut self) -> Result<u64, io::Error> {
    //        Ok(self.current_offset)
    //    }
}

impl<T> BufferCursor<T> {
    pub fn stream_len(&mut self) -> Result<u64, io::Error> {
        Ok(self.size)
    }

    pub fn stream_position(&mut self) -> Result<u64, io::Error> {
        Ok(self.cur_offset)
    }

    pub fn get_buffer(&self) -> &BufferRef {
        self.buffer.as_ref().unwrap().as_ref()
    }

    pub fn into_buffer(mut self) -> Buffer {
        self.buffer.take().unwrap()
    }
}

impl BufferCursor<Readable> {
    fn new_readable(buffer: Buffer) -> BufferCursor<Readable> {
        let size = buffer.get_size() as u64;
        let num_mem = buffer.n_memory();

        BufferCursor {
            buffer: Some(buffer),
            size,
            num_mem,
            cur_mem_idx: 0,
            cur_offset: 0,
            cur_mem_offset: 0,
            map_info: unsafe { mem::zeroed() },
            phantom: PhantomData,
        }
    }
}

impl BufferCursor<Writable> {
    fn new_writable(buffer: Buffer) -> Result<BufferCursor<Writable>, glib::BoolError> {
        if !buffer.is_writable() || !buffer.is_all_memory_writable() {
            return Err(glib_bool_error!("Not all memories are writable"));
        }

        let size = buffer.get_size() as u64;
        let num_mem = buffer.n_memory();

        Ok(BufferCursor {
            buffer: Some(buffer),
            size,
            num_mem,
            cur_mem_idx: 0,
            cur_offset: 0,
            cur_mem_offset: 0,
            map_info: unsafe { mem::zeroed() },
            phantom: PhantomData,
        })
    }
}

unsafe impl<T> Send for BufferCursor<T> {}
unsafe impl<T> Sync for BufferCursor<T> {}

impl<T: fmt::Debug> fmt::Debug for BufferCursorRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BufferCursorRef")
            .field("buffer", &self.buffer)
            .field("size", &self.size)
            .field("num_mem", &self.num_mem)
            .field("cur_mem_idx", &self.cur_mem_idx)
            .field("cur_offset", &self.cur_offset)
            .field("cur_mem_offset", &self.cur_mem_offset)
            .field("map_info", &self.map_info)
            .finish()
    }
}

impl<T> Drop for BufferCursorRef<T> {
    fn drop(&mut self) {
        if !self.map_info.memory.is_null() {
            unsafe {
                gst_sys::gst_memory_unmap(self.map_info.memory, &mut self.map_info);
            }
        }
    }
}

impl<'a> io::Read for BufferCursorRef<&'a BufferRef> {
    fn read(&mut self, mut data: &mut [u8]) -> Result<usize, io::Error> {
        let mut copied = 0;

        while !data.is_empty() && self.cur_mem_idx < self.num_mem {
            // Map memory if needed. cur_mem_idx, cur_mem_offset and cur_offset are required to be
            // set correctly here already (from constructor, seek and the bottom of the loop)
            if self.map_info.memory.is_null() {
                unsafe {
                    let memory =
                        gst_sys::gst_buffer_peek_memory(self.buffer.as_mut_ptr(), self.cur_mem_idx);
                    assert!(!memory.is_null());

                    if gst_sys::gst_memory_map(memory, &mut self.map_info, gst_sys::GST_MAP_READ)
                        == glib_sys::GFALSE
                    {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Failed to map memory readable",
                        ));
                    }
                }

                assert!(self.cur_mem_offset < self.map_info.size);
            }

            assert!(!self.map_info.memory.is_null());

            // Copy all data we can currently copy
            let data_left = self.map_info.size - self.cur_mem_offset;
            let to_copy = std::cmp::min(data.len(), data_left);
            unsafe {
                ptr::copy_nonoverlapping(
                    (self.map_info.data as *const u8).add(self.cur_mem_offset),
                    data.as_mut_ptr(),
                    to_copy,
                );
            }
            copied += to_copy;
            self.cur_offset += to_copy as u64;
            self.cur_mem_offset += to_copy;
            data = &mut data[to_copy..];

            // If we're at the end of the current memory, unmap and advance to the next memory
            if self.cur_mem_offset == self.map_info.size {
                unsafe {
                    gst_sys::gst_memory_unmap(self.map_info.memory, &mut self.map_info);
                }
                self.map_info.memory = ptr::null_mut();
                self.cur_mem_idx += 1;
                self.cur_mem_offset = 0;
            }
        }

        Ok(copied)
    }
}

impl<'a> io::Write for BufferCursorRef<&'a mut BufferRef> {
    fn write(&mut self, mut data: &[u8]) -> Result<usize, io::Error> {
        let mut copied = 0;

        while !data.is_empty() && self.cur_mem_idx < self.num_mem {
            // Map memory if needed. cur_mem_idx, cur_mem_offset and cur_offset are required to be
            // set correctly here already (from constructor, seek and the bottom of the loop)
            if self.map_info.memory.is_null() {
                unsafe {
                    let memory =
                        gst_sys::gst_buffer_peek_memory(self.buffer.as_mut_ptr(), self.cur_mem_idx);
                    assert!(!memory.is_null());

                    if gst_sys::gst_memory_map(memory, &mut self.map_info, gst_sys::GST_MAP_WRITE)
                        == glib_sys::GFALSE
                    {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Failed to map memory writable",
                        ));
                    }
                }

                assert!(self.cur_mem_offset < self.map_info.size);
            }

            assert!(!self.map_info.memory.is_null());

            // Copy all data we can currently copy
            let data_left = self.map_info.size - self.cur_mem_offset;
            let to_copy = std::cmp::min(data.len(), data_left);
            unsafe {
                ptr::copy_nonoverlapping(
                    data.as_ptr(),
                    (self.map_info.data as *mut u8).add(self.cur_mem_offset),
                    to_copy,
                );
            }
            copied += to_copy;
            self.cur_offset += to_copy as u64;
            self.cur_mem_offset += to_copy;
            data = &data[to_copy..];

            // If we're at the end of the current memory, unmap and advance to the next memory
            if self.cur_mem_offset == self.map_info.size {
                unsafe {
                    gst_sys::gst_memory_unmap(self.map_info.memory, &mut self.map_info);
                }
                self.map_info.memory = ptr::null_mut();
                self.cur_mem_idx += 1;
                self.cur_mem_offset = 0;
            }
        }

        Ok(copied)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}

impl<'a> io::Seek for BufferCursorRef<&'a BufferRef> {
    fn seek(&mut self, pos: io::SeekFrom) -> Result<u64, io::Error> {
        if !self.map_info.memory.is_null() {
            unsafe {
                gst_sys::gst_memory_unmap(self.map_info.memory, &mut self.map_info);
                self.map_info.memory = ptr::null_mut();
            }
        }

        match pos {
            io::SeekFrom::Start(off) => {
                self.cur_offset = std::cmp::min(self.size, off);
            }
            io::SeekFrom::End(off) if off <= 0 => {
                self.cur_offset = self.size;
            }
            io::SeekFrom::End(off) => {
                self.cur_offset = self.size.checked_sub(off as u64).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "Seek before start of buffer")
                })?;
            }
            io::SeekFrom::Current(std::i64::MIN) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Seek before start of buffer",
                ));
            }
            io::SeekFrom::Current(off) => {
                if off <= 0 {
                    self.cur_offset =
                        self.cur_offset.checked_sub((-off) as u64).ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::InvalidInput,
                                "Seek before start of buffer",
                            )
                        })?;
                } else {
                    self.cur_offset = std::cmp::min(
                        self.size,
                        self.cur_offset.checked_add(off as u64).unwrap_or(self.size),
                    );
                }
            }
        }

        let (idx, _, skip) = self
            .buffer
            .find_memory(self.cur_offset as usize, None)
            .expect("Failed to find memory");
        self.cur_mem_idx = idx;
        self.cur_mem_offset = skip;

        Ok(self.cur_offset)
    }

    // Once stabilized
    //    fn stream_len(&mut self) -> Result<u64, io::Error> {
    //        Ok(self.size)
    //    }
    //
    //    fn stream_position(&mut self) -> Result<u64, io::Error> {
    //        Ok(self.current_offset)
    //    }
}

impl<'a> io::Seek for BufferCursorRef<&'a mut BufferRef> {
    fn seek(&mut self, pos: io::SeekFrom) -> Result<u64, io::Error> {
        if !self.map_info.memory.is_null() {
            unsafe {
                gst_sys::gst_memory_unmap(self.map_info.memory, &mut self.map_info);
                self.map_info.memory = ptr::null_mut();
            }
        }

        match pos {
            io::SeekFrom::Start(off) => {
                self.cur_offset = std::cmp::min(self.size, off);
            }
            io::SeekFrom::End(off) if off <= 0 => {
                self.cur_offset = self.size;
            }
            io::SeekFrom::End(off) => {
                self.cur_offset = self.size.checked_sub(off as u64).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "Seek before start of buffer")
                })?;
            }
            io::SeekFrom::Current(std::i64::MIN) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Seek before start of buffer",
                ));
            }
            io::SeekFrom::Current(off) => {
                if off <= 0 {
                    self.cur_offset =
                        self.cur_offset.checked_sub((-off) as u64).ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::InvalidInput,
                                "Seek before start of buffer",
                            )
                        })?;
                } else {
                    self.cur_offset = std::cmp::min(
                        self.size,
                        self.cur_offset.checked_add(off as u64).unwrap_or(self.size),
                    );
                }
            }
        }

        let (idx, _, skip) = self
            .buffer
            .find_memory(self.cur_offset as usize, None)
            .expect("Failed to find memory");
        self.cur_mem_idx = idx;
        self.cur_mem_offset = skip;

        Ok(self.cur_offset)
    }

    // Once stabilized
    //    fn stream_len(&mut self) -> Result<u64, io::Error> {
    //        Ok(self.size)
    //    }
    //
    //    fn stream_position(&mut self) -> Result<u64, io::Error> {
    //        Ok(self.current_offset)
    //    }
}

impl<T> BufferCursorRef<T> {
    pub fn stream_len(&mut self) -> Result<u64, io::Error> {
        Ok(self.size)
    }

    pub fn stream_position(&mut self) -> Result<u64, io::Error> {
        Ok(self.cur_offset)
    }
}

impl<'a> BufferCursorRef<&'a BufferRef> {
    pub fn get_buffer(&self) -> &BufferRef {
        self.buffer
    }

    fn new_readable(buffer: &'a BufferRef) -> BufferCursorRef<&'a BufferRef> {
        let size = buffer.get_size() as u64;
        let num_mem = buffer.n_memory();

        BufferCursorRef {
            buffer,
            size,
            num_mem,
            cur_mem_idx: 0,
            cur_offset: 0,
            cur_mem_offset: 0,
            map_info: unsafe { mem::zeroed() },
        }
    }
}

impl<'a> BufferCursorRef<&'a mut BufferRef> {
    pub fn get_buffer(&self) -> &BufferRef {
        self.buffer
    }

    fn new_writable(
        buffer: &'a mut BufferRef,
    ) -> Result<BufferCursorRef<&'a mut BufferRef>, glib::BoolError> {
        if !buffer.is_all_memory_writable() {
            return Err(glib_bool_error!("Not all memories are writable"));
        }

        let size = buffer.get_size() as u64;
        let num_mem = buffer.n_memory();

        Ok(BufferCursorRef {
            buffer,
            size,
            num_mem,
            cur_mem_idx: 0,
            cur_offset: 0,
            cur_mem_offset: 0,
            map_info: unsafe { mem::zeroed() },
        })
    }
}

unsafe impl<T> Send for BufferCursorRef<T> {}
unsafe impl<T> Sync for BufferCursorRef<T> {}

pub const BUFFER_COPY_METADATA: ::BufferCopyFlags =
    ::BufferCopyFlags::from_bits_truncate(gst_sys::GST_BUFFER_COPY_METADATA);
pub const BUFFER_COPY_ALL: ::BufferCopyFlags =
    ::BufferCopyFlags::from_bits_truncate(gst_sys::GST_BUFFER_COPY_ALL);

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

        let mut buffer = Buffer::from_slice(vec![1, 2, 3, 4]);
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

    #[test]
    fn test_memories() {
        ::init().unwrap();

        let mut buffer = Buffer::new();
        {
            let buffer = buffer.get_mut().unwrap();
            buffer.append_memory(::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(::Memory::from_mut_slice(vec![0; 10]));
        }

        assert!(buffer.is_all_memory_writable());
        assert_eq!(buffer.n_memory(), 5);
        assert_eq!(buffer.get_size(), 30);

        for i in 0..5 {
            {
                let mem = buffer.get_memory(i).unwrap();
                assert_eq!(mem.get_size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_readable().unwrap();
                assert_eq!(map.get_size(), if i < 4 { 5 } else { 10 });
            }

            {
                let mem = buffer.peek_memory(i);
                assert_eq!(mem.get_size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_readable().unwrap();
                assert_eq!(map.get_size(), if i < 4 { 5 } else { 10 });
            }

            {
                let buffer = buffer.get_mut().unwrap();
                let mem = buffer.peek_memory_mut(i).unwrap();
                assert_eq!(mem.get_size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_writable().unwrap();
                assert_eq!(map.get_size(), if i < 4 { 5 } else { 10 });
            }
        }

        {
            let buffer = buffer.get_mut().unwrap();
            let mut last = 0;
            for (i, mem) in buffer.iter_memories_mut().unwrap().enumerate() {
                {
                    assert_eq!(mem.get_size(), if i < 4 { 5 } else { 10 });
                    let map = mem.map_readable().unwrap();
                    assert_eq!(map.get_size(), if i < 4 { 5 } else { 10 });
                }

                {
                    assert_eq!(mem.get_size(), if i < 4 { 5 } else { 10 });
                    let map = mem.map_readable().unwrap();
                    assert_eq!(map.get_size(), if i < 4 { 5 } else { 10 });
                }

                {
                    assert_eq!(mem.get_size(), if i < 4 { 5 } else { 10 });
                    let map = mem.map_writable().unwrap();
                    assert_eq!(map.get_size(), if i < 4 { 5 } else { 10 });
                }

                last = i;
            }

            assert_eq!(last, 4);
        }

        let mut last = 0;
        for (i, mem) in buffer.iter_memories().enumerate() {
            {
                assert_eq!(mem.get_size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_readable().unwrap();
                assert_eq!(map.get_size(), if i < 4 { 5 } else { 10 });
            }

            {
                assert_eq!(mem.get_size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_readable().unwrap();
                assert_eq!(map.get_size(), if i < 4 { 5 } else { 10 });
            }

            last = i;
        }

        assert_eq!(last, 4);

        let mut last = 0;
        for (i, mem) in buffer.iter_memories_owned().enumerate() {
            {
                assert_eq!(mem.get_size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_readable().unwrap();
                assert_eq!(map.get_size(), if i < 4 { 5 } else { 10 });
            }

            {
                assert_eq!(mem.get_size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_readable().unwrap();
                assert_eq!(map.get_size(), if i < 4 { 5 } else { 10 });
            }

            last = i;
        }

        assert_eq!(last, 4);
    }

    #[test]
    fn test_buffer_cursor() {
        use std::io::{self, Read, Seek, Write};

        ::init().unwrap();

        let mut buffer = Buffer::new();
        {
            let buffer = buffer.get_mut().unwrap();
            buffer.append_memory(::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(::Memory::from_mut_slice(vec![0; 10]));
        }

        assert!(buffer.is_all_memory_writable());
        assert_eq!(buffer.n_memory(), 5);
        assert_eq!(buffer.get_size(), 30);

        let mut cursor = buffer.into_buffer_cursor_writable().unwrap();
        assert_eq!(cursor.stream_position().unwrap(), 0);
        cursor.write_all(b"01234567").unwrap();
        assert_eq!(cursor.stream_position().unwrap(), 8);
        cursor.write_all(b"890123").unwrap();
        assert_eq!(cursor.stream_position().unwrap(), 14);
        cursor.write_all(b"456").unwrap();
        assert_eq!(cursor.stream_position().unwrap(), 17);
        cursor.write_all(b"78901234567").unwrap();
        assert_eq!(cursor.stream_position().unwrap(), 28);
        cursor.write_all(b"89").unwrap();
        assert_eq!(cursor.stream_position().unwrap(), 30);
        assert!(cursor.write_all(b"0").is_err());

        assert_eq!(cursor.seek(io::SeekFrom::Start(5)).unwrap(), 5);
        assert_eq!(cursor.stream_position().unwrap(), 5);
        cursor.write_all(b"A").unwrap();

        assert_eq!(cursor.seek(io::SeekFrom::End(5)).unwrap(), 25);
        assert_eq!(cursor.stream_position().unwrap(), 25);
        cursor.write_all(b"B").unwrap();

        assert_eq!(cursor.seek(io::SeekFrom::Current(-1)).unwrap(), 25);
        assert_eq!(cursor.stream_position().unwrap(), 25);
        cursor.write_all(b"C").unwrap();

        assert_eq!(cursor.seek(io::SeekFrom::Current(1)).unwrap(), 27);
        assert_eq!(cursor.stream_position().unwrap(), 27);
        cursor.write_all(b"D").unwrap();

        let buffer = cursor.into_buffer();

        let mut cursor = buffer.into_buffer_cursor_readable();
        let mut data = [0; 30];

        assert_eq!(cursor.stream_position().unwrap(), 0);
        cursor.read_exact(&mut data[0..7]).unwrap();
        assert_eq!(cursor.stream_position().unwrap(), 7);
        assert_eq!(&data[0..7], b"01234A6");
        cursor.read_exact(&mut data[0..5]).unwrap();
        assert_eq!(cursor.stream_position().unwrap(), 12);
        assert_eq!(&data[0..5], b"78901");
        cursor.read_exact(&mut data[0..10]).unwrap();
        assert_eq!(cursor.stream_position().unwrap(), 22);
        assert_eq!(&data[0..10], b"2345678901");
        cursor.read_exact(&mut data[0..8]).unwrap();
        assert_eq!(cursor.stream_position().unwrap(), 30);
        assert_eq!(&data[0..8], b"234C6D89");
        assert!(cursor.read_exact(&mut data[0..1]).is_err());

        assert_eq!(cursor.seek(io::SeekFrom::Start(5)).unwrap(), 5);
        assert_eq!(cursor.stream_position().unwrap(), 5);
        cursor.read_exact(&mut data[0..1]).unwrap();
        assert_eq!(&data[0..1], b"A");

        assert_eq!(cursor.seek(io::SeekFrom::End(5)).unwrap(), 25);
        assert_eq!(cursor.stream_position().unwrap(), 25);
        cursor.read_exact(&mut data[0..1]).unwrap();
        assert_eq!(&data[0..1], b"C");

        assert_eq!(cursor.seek(io::SeekFrom::Current(-1)).unwrap(), 25);
        assert_eq!(cursor.stream_position().unwrap(), 25);
        cursor.read_exact(&mut data[0..1]).unwrap();
        assert_eq!(&data[0..1], b"C");

        assert_eq!(cursor.seek(io::SeekFrom::Current(1)).unwrap(), 27);
        assert_eq!(cursor.stream_position().unwrap(), 27);
        cursor.read_exact(&mut data[0..1]).unwrap();
        assert_eq!(&data[0..1], b"D");
    }

    #[test]
    fn test_buffer_cursor_ref() {
        use std::io::{self, Read, Seek, Write};

        ::init().unwrap();

        let mut buffer = Buffer::new();
        {
            let buffer = buffer.get_mut().unwrap();
            buffer.append_memory(::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(::Memory::from_mut_slice(vec![0; 10]));
        }

        assert!(buffer.is_all_memory_writable());
        assert_eq!(buffer.n_memory(), 5);
        assert_eq!(buffer.get_size(), 30);

        {
            let buffer = buffer.get_mut().unwrap();

            let mut cursor = buffer.as_buffer_cursor_ref_writable().unwrap();
            assert_eq!(cursor.stream_position().unwrap(), 0);
            cursor.write_all(b"01234567").unwrap();
            assert_eq!(cursor.stream_position().unwrap(), 8);
            cursor.write_all(b"890123").unwrap();
            assert_eq!(cursor.stream_position().unwrap(), 14);
            cursor.write_all(b"456").unwrap();
            assert_eq!(cursor.stream_position().unwrap(), 17);
            cursor.write_all(b"78901234567").unwrap();
            assert_eq!(cursor.stream_position().unwrap(), 28);
            cursor.write_all(b"89").unwrap();
            assert_eq!(cursor.stream_position().unwrap(), 30);
            assert!(cursor.write_all(b"0").is_err());

            assert_eq!(cursor.seek(io::SeekFrom::Start(5)).unwrap(), 5);
            assert_eq!(cursor.stream_position().unwrap(), 5);
            cursor.write_all(b"A").unwrap();

            assert_eq!(cursor.seek(io::SeekFrom::End(5)).unwrap(), 25);
            assert_eq!(cursor.stream_position().unwrap(), 25);
            cursor.write_all(b"B").unwrap();

            assert_eq!(cursor.seek(io::SeekFrom::Current(-1)).unwrap(), 25);
            assert_eq!(cursor.stream_position().unwrap(), 25);
            cursor.write_all(b"C").unwrap();

            assert_eq!(cursor.seek(io::SeekFrom::Current(1)).unwrap(), 27);
            assert_eq!(cursor.stream_position().unwrap(), 27);
            cursor.write_all(b"D").unwrap();
        }

        let mut cursor = buffer.as_buffer_cursor_ref_readable();
        let mut data = [0; 30];

        assert_eq!(cursor.stream_position().unwrap(), 0);
        cursor.read_exact(&mut data[0..7]).unwrap();
        assert_eq!(cursor.stream_position().unwrap(), 7);
        assert_eq!(&data[0..7], b"01234A6");
        cursor.read_exact(&mut data[0..5]).unwrap();
        assert_eq!(cursor.stream_position().unwrap(), 12);
        assert_eq!(&data[0..5], b"78901");
        cursor.read_exact(&mut data[0..10]).unwrap();
        assert_eq!(cursor.stream_position().unwrap(), 22);
        assert_eq!(&data[0..10], b"2345678901");
        cursor.read_exact(&mut data[0..8]).unwrap();
        assert_eq!(cursor.stream_position().unwrap(), 30);
        assert_eq!(&data[0..8], b"234C6D89");
        assert!(cursor.read_exact(&mut data[0..1]).is_err());

        assert_eq!(cursor.seek(io::SeekFrom::Start(5)).unwrap(), 5);
        assert_eq!(cursor.stream_position().unwrap(), 5);
        cursor.read_exact(&mut data[0..1]).unwrap();
        assert_eq!(&data[0..1], b"A");

        assert_eq!(cursor.seek(io::SeekFrom::End(5)).unwrap(), 25);
        assert_eq!(cursor.stream_position().unwrap(), 25);
        cursor.read_exact(&mut data[0..1]).unwrap();
        assert_eq!(&data[0..1], b"C");

        assert_eq!(cursor.seek(io::SeekFrom::Current(-1)).unwrap(), 25);
        assert_eq!(cursor.stream_position().unwrap(), 25);
        cursor.read_exact(&mut data[0..1]).unwrap();
        assert_eq!(&data[0..1], b"C");

        assert_eq!(cursor.seek(io::SeekFrom::Current(1)).unwrap(), 27);
        assert_eq!(cursor.stream_position().unwrap(), 27);
        cursor.read_exact(&mut data[0..1]).unwrap();
        assert_eq!(&data[0..1], b"D");
    }
}
