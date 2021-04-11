// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ops;
use std::ptr;
use std::slice;
use std::u64;
use std::usize;

use crate::meta::*;
use crate::BufferCursor;
use crate::BufferFlags;
use crate::BufferRefCursor;
use crate::ClockTime;
use crate::Memory;
use crate::MemoryRef;

use glib::translate::{from_glib, from_glib_full, FromGlib, FromGlibPtrFull, ToGlib};

pub enum Readable {}
pub enum Writable {}

mini_object_wrapper!(Buffer, BufferRef, ffi::GstBuffer, || {
    ffi::gst_buffer_get_type()
});

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

impl Buffer {
    pub fn new() -> Self {
        assert_initialized_main_thread!();

        unsafe { from_glib_full(ffi::gst_buffer_new()) }
    }

    pub fn with_size(size: usize) -> Result<Self, glib::BoolError> {
        assert_initialized_main_thread!();

        unsafe {
            Option::<_>::from_glib_full(ffi::gst_buffer_new_allocate(
                ptr::null_mut(),
                size,
                ptr::null_mut(),
            ))
            .ok_or_else(|| glib::bool_error!("Failed to allocate buffer"))
        }
    }

    unsafe extern "C" fn drop_box<T>(vec: glib::ffi::gpointer) {
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
            from_glib_full(ffi::gst_buffer_new_wrapped_full(
                0,
                data as glib::ffi::gpointer,
                size,
                0,
                size,
                user_data as glib::ffi::gpointer,
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
            from_glib_full(ffi::gst_buffer_new_wrapped_full(
                ffi::GST_MEMORY_FLAG_READONLY,
                data as glib::ffi::gpointer,
                size,
                0,
                size,
                user_data as glib::ffi::gpointer,
                Some(Self::drop_box::<T>),
            ))
        }
    }

    pub fn into_mapped_buffer_readable(self) -> Result<MappedBuffer<Readable>, Self> {
        unsafe {
            let mut map_info = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(ffi::gst_buffer_map(
                self.as_mut_ptr(),
                map_info.as_mut_ptr(),
                ffi::GST_MAP_READ,
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
            let res: bool = from_glib(ffi::gst_buffer_map(
                self.as_mut_ptr(),
                map_info.as_mut_ptr(),
                ffi::GST_MAP_READWRITE,
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

    pub fn into_cursor_readable(self) -> BufferCursor<Readable> {
        BufferCursor::new_readable(self)
    }

    pub fn into_cursor_writable(self) -> Result<BufferCursor<Writable>, glib::BoolError> {
        BufferCursor::new_writable(self)
    }

    pub fn append(&mut self, other: Self) {
        skip_assert_initialized!();
        unsafe {
            let ptr = ffi::gst_buffer_append(self.as_mut_ptr(), other.into_ptr());
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
            let res =
                ffi::gst_buffer_map(self.as_mut_ptr(), map_info.as_mut_ptr(), ffi::GST_MAP_READ);
            if res == glib::ffi::GTRUE {
                Ok(BufferMap {
                    buffer: self,
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(glib::bool_error!("Failed to map buffer readable"))
            }
        }
    }

    pub fn map_writable(&mut self) -> Result<BufferMap<Writable>, glib::BoolError> {
        unsafe {
            let mut map_info = mem::MaybeUninit::zeroed();
            let res = ffi::gst_buffer_map(
                self.as_mut_ptr(),
                map_info.as_mut_ptr(),
                ffi::GST_MAP_READWRITE,
            );
            if res == glib::ffi::GTRUE {
                Ok(BufferMap {
                    buffer: self,
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(glib::bool_error!("Failed to map buffer writable"))
            }
        }
    }

    pub fn copy_region(
        &self,
        flags: crate::BufferCopyFlags,
        offset: usize,
        size: Option<usize>,
    ) -> Result<Buffer, glib::BoolError> {
        let size_real = size.unwrap_or(usize::MAX);
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_buffer_copy_region(
                self.as_mut_ptr(),
                flags.to_glib(),
                offset,
                size_real,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to copy region of buffer"))
        }
    }

    pub fn copy_into(
        &self,
        dest: &mut BufferRef,
        flags: crate::BufferCopyFlags,
        offset: usize,
        size: Option<usize>,
    ) -> Result<(), glib::BoolError> {
        let size_real = size.unwrap_or(usize::MAX);
        unsafe {
            glib::result_from_gboolean!(
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
        let maxsize = self.maxsize();
        let size = slice.len();

        assert!(maxsize >= offset && maxsize - offset >= size);

        let copied = unsafe {
            let src = slice.as_ptr();
            ffi::gst_buffer_fill(
                self.as_mut_ptr(),
                offset,
                src as glib::ffi::gconstpointer,
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
        let maxsize = self.size();
        let size = slice.len();

        assert!(maxsize >= offset && maxsize - offset >= size);

        let copied = unsafe {
            let dest = slice.as_mut_ptr();
            ffi::gst_buffer_extract(self.as_mut_ptr(), offset, dest as glib::ffi::gpointer, size)
        };

        if copied == size {
            Ok(())
        } else {
            Err(copied)
        }
    }

    pub fn copy_deep(&self) -> Result<Buffer, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_buffer_copy_deep(self.as_ptr()))
                .ok_or_else(|| glib::bool_error!("Failed to deep copy buffer"))
        }
    }

    pub fn size(&self) -> usize {
        unsafe { ffi::gst_buffer_get_size(self.as_mut_ptr()) }
    }

    pub fn maxsize(&self) -> usize {
        unsafe {
            let mut maxsize = mem::MaybeUninit::uninit();
            ffi::gst_buffer_get_sizes_range(
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
        assert!(self.maxsize() >= size);

        unsafe {
            ffi::gst_buffer_set_size(self.as_mut_ptr(), size as isize);
        }
    }

    pub fn offset(&self) -> u64 {
        self.0.offset
    }

    pub fn set_offset(&mut self, offset: u64) {
        self.0.offset = offset;
    }

    pub fn offset_end(&self) -> u64 {
        self.0.offset_end
    }

    pub fn set_offset_end(&mut self, offset_end: u64) {
        self.0.offset_end = offset_end;
    }

    pub fn pts(&self) -> ClockTime {
        unsafe { from_glib(self.0.pts) }
    }

    pub fn set_pts(&mut self, pts: ClockTime) {
        self.0.pts = pts.to_glib();
    }

    pub fn dts(&self) -> ClockTime {
        unsafe { from_glib(self.0.dts) }
    }

    pub fn set_dts(&mut self, dts: ClockTime) {
        self.0.dts = dts.to_glib();
    }

    pub fn dts_or_pts(&self) -> ClockTime {
        let val = self.dts();
        if val.is_none() {
            self.pts()
        } else {
            val
        }
    }

    pub fn duration(&self) -> ClockTime {
        unsafe { from_glib(self.0.duration) }
    }

    pub fn set_duration(&mut self, duration: ClockTime) {
        self.0.duration = duration.to_glib();
    }

    pub fn flags(&self) -> BufferFlags {
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
            let meta = ffi::gst_buffer_get_meta(self.as_mut_ptr(), T::get_meta_api().to_glib());
            if meta.is_null() {
                None
            } else {
                Some(T::from_ptr(self, meta as *const <T as MetaAPI>::GstType))
            }
        }
    }

    pub fn get_meta_mut<T: MetaAPI>(&mut self) -> Option<MetaRefMut<T, crate::meta::Standalone>> {
        unsafe {
            let meta = ffi::gst_buffer_get_meta(self.as_mut_ptr(), T::get_meta_api().to_glib());
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

    pub fn foreach_meta<F: FnMut(MetaRef<Meta>) -> bool>(&self, func: F) -> bool {
        unsafe extern "C" fn trampoline<F: FnMut(MetaRef<Meta>) -> bool>(
            buffer: *mut ffi::GstBuffer,
            meta: *mut *mut ffi::GstMeta,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let func = user_data as *const _ as usize as *mut F;
            let res = (*func)(Meta::from_ptr(BufferRef::from_ptr(buffer), *meta));

            res.to_glib()
        }

        unsafe {
            let func_ptr: &F = &func;

            from_glib(ffi::gst_buffer_foreach_meta(
                self.as_ptr() as *mut _,
                Some(trampoline::<F>),
                func_ptr as *const _ as usize as *mut _,
            ))
        }
    }

    pub fn foreach_meta_mut<
        F: FnMut(MetaRefMut<Meta, crate::meta::Iterated>) -> Result<bool, bool>,
    >(
        &mut self,
        func: F,
    ) -> bool {
        unsafe extern "C" fn trampoline<
            F: FnMut(MetaRefMut<Meta, crate::meta::Iterated>) -> Result<bool, bool>,
        >(
            buffer: *mut ffi::GstBuffer,
            meta: *mut *mut ffi::GstMeta,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let func = user_data as *const _ as usize as *mut F;
            let res = (*func)(Meta::from_mut_ptr(BufferRef::from_mut_ptr(buffer), *meta));

            match res {
                Ok(false) | Err(false) => {
                    *meta = ptr::null_mut();
                    res.is_ok().to_glib()
                }
                Ok(true) | Err(true) => res.is_ok().to_glib(),
            }
        }

        unsafe {
            let func_ptr: &F = &func;

            from_glib(ffi::gst_buffer_foreach_meta(
                self.as_ptr() as *mut _,
                Some(trampoline::<F>),
                func_ptr as *const _ as usize as *mut _,
            ))
        }
    }

    pub fn append_memory(&mut self, mem: Memory) {
        unsafe { ffi::gst_buffer_append_memory(self.as_mut_ptr(), mem.into_ptr()) }
    }

    pub fn find_memory(&self, offset: usize, size: Option<usize>) -> Option<(u32, u32, usize)> {
        unsafe {
            let mut idx = mem::MaybeUninit::uninit();
            let mut length = mem::MaybeUninit::uninit();
            let mut skip = mem::MaybeUninit::uninit();

            let res = from_glib(ffi::gst_buffer_find_memory(
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

    pub fn all_memory(&self) -> Option<Memory> {
        unsafe {
            let res = ffi::gst_buffer_get_all_memory(self.as_mut_ptr());
            if res.is_null() {
                None
            } else {
                Some(from_glib_full(res))
            }
        }
    }

    pub fn get_max_memory() -> u32 {
        unsafe { ffi::gst_buffer_get_max_memory() }
    }

    pub fn get_memory(&self, idx: u32) -> Option<Memory> {
        if idx >= self.n_memory() {
            None
        } else {
            unsafe {
                let res = ffi::gst_buffer_get_memory(self.as_mut_ptr(), idx);
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
            let res = ffi::gst_buffer_get_memory_range(
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
            ffi::gst_buffer_insert_memory(
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
        unsafe { from_glib(ffi::gst_buffer_is_all_memory_writable(self.as_mut_ptr())) }
    }

    pub fn is_memory_range_writable(&self, idx: u32, length: Option<u16>) -> bool {
        unsafe {
            from_glib(ffi::gst_buffer_is_memory_range_writable(
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
        unsafe { ffi::gst_buffer_n_memory(self.as_ptr() as *mut _) }
    }

    pub fn peek_memory(&self, idx: u32) -> &MemoryRef {
        assert!(idx < self.n_memory());
        unsafe { MemoryRef::from_ptr(ffi::gst_buffer_peek_memory(self.as_mut_ptr(), idx)) }
    }

    pub fn peek_memory_mut(&mut self, idx: u32) -> Result<&mut MemoryRef, glib::BoolError> {
        assert!(idx < self.n_memory());
        unsafe {
            let mem = ffi::gst_buffer_peek_memory(self.as_mut_ptr(), idx);
            if ffi::gst_mini_object_is_writable(mem as *mut _) == glib::ffi::GFALSE {
                Err(glib::bool_error!("Memory not writable"))
            } else {
                Ok(MemoryRef::from_mut_ptr(ffi::gst_buffer_peek_memory(
                    self.as_mut_ptr(),
                    idx,
                )))
            }
        }
    }

    pub fn prepend_memory(&mut self, mem: Memory) {
        unsafe { ffi::gst_buffer_prepend_memory(self.as_mut_ptr(), mem.into_ptr()) }
    }

    pub fn remove_all_memory(&mut self) {
        unsafe { ffi::gst_buffer_remove_all_memory(self.as_mut_ptr()) }
    }

    pub fn remove_memory(&mut self, idx: u32) {
        assert!(idx < self.n_memory());
        unsafe { ffi::gst_buffer_remove_memory(self.as_mut_ptr(), idx) }
    }

    pub fn remove_memory_range(&mut self, idx: u32, length: Option<u32>) {
        assert!(idx + length.unwrap_or(0) < self.n_memory());
        unsafe {
            ffi::gst_buffer_remove_memory_range(
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
        unsafe { ffi::gst_buffer_replace_all_memory(self.as_mut_ptr(), mem.into_ptr()) }
    }

    pub fn replace_memory(&mut self, idx: u32, mem: Memory) {
        assert!(idx < self.n_memory());
        unsafe { ffi::gst_buffer_replace_memory(self.as_mut_ptr(), idx, mem.into_ptr()) }
    }

    pub fn replace_memory_range(&mut self, idx: u32, length: Option<u32>, mem: Memory) {
        assert!(idx + length.unwrap_or(0) < self.n_memory());
        unsafe {
            ffi::gst_buffer_replace_memory_range(
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
            Err(glib::bool_error!("Not all memory are writable"))
        } else {
            Ok(IterMut::new(self))
        }
    }

    pub fn iter_memories_owned(&self) -> IterOwned {
        IterOwned::new(self)
    }

    pub fn as_cursor_readable(&self) -> BufferRefCursor<&BufferRef> {
        BufferRefCursor::new_readable(self)
    }

    pub fn as_cursor_writable(
        &mut self,
    ) -> Result<BufferRefCursor<&mut BufferRef>, glib::BoolError> {
        BufferRefCursor::new_writable(self)
    }
}

macro_rules! define_meta_iter(
    ($name:ident, $typ:ty, $mtyp:ty, $prepare_buffer:expr, $from_ptr:expr) => {
    pub struct $name<'a, T: MetaAPI + 'a> {
        buffer: $typ,
        state: glib::ffi::gpointer,
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
                    let meta = ffi::gst_buffer_iterate_meta(self.buffer.as_mut_ptr(), &mut self.state);

                    if meta.is_null() {
                        return None;
                    } else if self.meta_api == glib::Type::INVALID || glib::Type::from_glib((*(*meta).info).api) == self.meta_api {
                        // FIXME: Workaround for a lifetime issue with the mutable iterator only
                        let buffer = $prepare_buffer(self.buffer.as_mut_ptr());
                        let item = $from_ptr(buffer, meta);
                        return Some(item);
                    }
                }
            }
        }
    }
    }
);

define_meta_iter!(
    MetaIter,
    &'a BufferRef,
    MetaRef<'a, T>,
    |buffer: *const ffi::GstBuffer| BufferRef::from_ptr(buffer),
    |buffer, meta| T::from_ptr(buffer, meta as *const <T as MetaAPI>::GstType)
);
define_meta_iter!(
    MetaIterMut,
    &'a mut BufferRef,
    MetaRefMut<'a, T, crate::meta::Iterated>,
    |buffer: *mut ffi::GstBuffer| BufferRef::from_mut_ptr(buffer),
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
        let ptr = ffi::gst_buffer_peek_memory(buffer.as_mut_ptr(), idx);
        if ptr.is_null() {
            None
        } else {
            Some(MemoryRef::from_ptr(ptr as *const ffi::GstMemory))
        }
    }
);

define_iter!(
    IterMut,
    &'a mut BufferRef,
    &'a mut MemoryRef,
    |buffer: &mut BufferRef, idx| {
        let ptr = ffi::gst_buffer_peek_memory(buffer.as_mut_ptr(), idx);
        if ptr.is_null() {
            None
        } else {
            Some(MemoryRef::from_mut_ptr(ptr as *mut ffi::GstMemory))
        }
    }
);

define_iter!(
    IterOwned,
    &'a BufferRef,
    Memory,
    |buffer: &BufferRef, idx| { buffer.get_memory(idx) }
);

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        BufferRef::fmt(self, f)
    }
}

impl PartialEq for Buffer {
    fn eq(&self, other: &Buffer) -> bool {
        BufferRef::eq(self, other)
    }
}

impl Eq for Buffer {}

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
            .field("pts", &self.pts().to_string())
            .field("dts", &self.dts().to_string())
            .field("duration", &self.duration().to_string())
            .field("size", &self.size())
            .field("offset", &self.offset())
            .field("offset_end", &self.offset_end())
            .field("flags", &self.flags())
            .field(
                "metas",
                &DebugIter(RefCell::new(
                    self.iter_meta::<crate::Meta>().map(|m| m.api()),
                )),
            )
            .finish()
    }
}

impl PartialEq for BufferRef {
    fn eq(&self, other: &BufferRef) -> bool {
        if self.size() != other.size() {
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
    pub fn size(&self) -> usize {
        self.map_info.size
    }

    pub fn buffer(&self) -> &BufferRef {
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
        f.debug_tuple("BufferMap").field(&self.buffer()).finish()
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

unsafe impl<'a, T> Send for BufferMap<'a, T> {}
unsafe impl<'a, T> Sync for BufferMap<'a, T> {}

impl<T> MappedBuffer<T> {
    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.map_info.data as *const u8, self.map_info.size) }
    }

    pub fn size(&self) -> usize {
        self.map_info.size
    }

    pub fn buffer(&self) -> &BufferRef {
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
        f.debug_tuple("MappedBuffer").field(&self.buffer()).finish()
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

pub const BUFFER_COPY_METADATA: crate::BufferCopyFlags =
    crate::BufferCopyFlags::from_bits_truncate(ffi::GST_BUFFER_COPY_METADATA);
pub const BUFFER_COPY_ALL: crate::BufferCopyFlags =
    crate::BufferCopyFlags::from_bits_truncate(ffi::GST_BUFFER_COPY_ALL);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fields() {
        crate::init().unwrap();

        let mut buffer = Buffer::new();

        {
            let buffer = buffer.get_mut().unwrap();
            buffer.set_pts(1.into());
            buffer.set_dts(2.into());
            buffer.set_offset(3);
            buffer.set_offset_end(4);
            buffer.set_duration(5.into());
        }
        assert_eq!(buffer.pts(), 1.into());
        assert_eq!(buffer.dts(), 2.into());
        assert_eq!(buffer.offset(), 3);
        assert_eq!(buffer.offset_end(), 4);
        assert_eq!(buffer.duration(), 5.into());
    }

    #[test]
    fn test_writability() {
        crate::init().unwrap();

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

        assert_eq!(buffer.pts(), 1.into());
        assert_eq!(buffer2.pts(), 2.into());

        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());

            let data = buffer2.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![0, 2, 3, 4].as_slice());
        }
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_memories() {
        crate::init().unwrap();

        let mut buffer = Buffer::new();
        {
            let buffer = buffer.get_mut().unwrap();
            buffer.append_memory(crate::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(crate::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(crate::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(crate::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(crate::Memory::from_mut_slice(vec![0; 10]));
        }

        assert!(buffer.is_all_memory_writable());
        assert_eq!(buffer.n_memory(), 5);
        assert_eq!(buffer.size(), 30);

        for i in 0..5 {
            {
                let mem = buffer.get_memory(i).unwrap();
                assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_readable().unwrap();
                assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
            }

            {
                let mem = buffer.peek_memory(i);
                assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_readable().unwrap();
                assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
            }

            {
                let buffer = buffer.get_mut().unwrap();
                let mem = buffer.peek_memory_mut(i).unwrap();
                assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_writable().unwrap();
                assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
            }
        }

        {
            let buffer = buffer.get_mut().unwrap();
            let mut last = 0;
            for (i, mem) in buffer.iter_memories_mut().unwrap().enumerate() {
                {
                    assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                    let map = mem.map_readable().unwrap();
                    assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
                }

                {
                    assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                    let map = mem.map_readable().unwrap();
                    assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
                }

                {
                    assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                    let map = mem.map_writable().unwrap();
                    assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
                }

                last = i;
            }

            assert_eq!(last, 4);
        }

        let mut last = 0;
        for (i, mem) in buffer.iter_memories().enumerate() {
            {
                assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_readable().unwrap();
                assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
            }

            {
                assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_readable().unwrap();
                assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
            }

            last = i;
        }

        assert_eq!(last, 4);

        let mut last = 0;
        for (i, mem) in buffer.iter_memories_owned().enumerate() {
            {
                assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_readable().unwrap();
                assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
            }

            {
                assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_readable().unwrap();
                assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
            }

            last = i;
        }

        assert_eq!(last, 4);
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[test]
    fn test_meta_foreach() {
        crate::init().unwrap();

        let mut buffer = Buffer::new();
        {
            let buffer = buffer.get_mut().unwrap();
            crate::ReferenceTimestampMeta::add(
                buffer,
                &crate::Caps::builder("foo/bar").build(),
                crate::ClockTime::from(0),
                crate::CLOCK_TIME_NONE,
            );
            crate::ReferenceTimestampMeta::add(
                buffer,
                &crate::Caps::builder("foo/bar").build(),
                crate::SECOND,
                crate::CLOCK_TIME_NONE,
            );
        }

        let mut res = vec![];
        buffer.foreach_meta(|meta| {
            let meta = meta
                .downcast_ref::<crate::ReferenceTimestampMeta>()
                .unwrap();
            res.push(meta.timestamp());
            true
        });

        assert_eq!(&[crate::ClockTime::from(0), crate::SECOND][..], &res[..]);
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[test]
    fn test_meta_foreach_mut() {
        crate::init().unwrap();

        let mut buffer = Buffer::new();
        {
            let buffer = buffer.get_mut().unwrap();
            crate::ReferenceTimestampMeta::add(
                buffer,
                &crate::Caps::builder("foo/bar").build(),
                crate::ClockTime::from(0),
                crate::CLOCK_TIME_NONE,
            );
            crate::ReferenceTimestampMeta::add(
                buffer,
                &crate::Caps::builder("foo/bar").build(),
                crate::SECOND,
                crate::CLOCK_TIME_NONE,
            );
        }

        let mut res = vec![];
        buffer.get_mut().unwrap().foreach_meta_mut(|mut meta| {
            let meta = meta
                .downcast_ref::<crate::ReferenceTimestampMeta>()
                .unwrap();
            res.push(meta.timestamp());
            if meta.timestamp() == crate::SECOND {
                Ok(false)
            } else {
                Ok(true)
            }
        });

        assert_eq!(&[crate::ClockTime::from(0), crate::SECOND][..], &res[..]);

        let mut res = vec![];
        buffer.foreach_meta(|meta| {
            let meta = meta
                .downcast_ref::<crate::ReferenceTimestampMeta>()
                .unwrap();
            res.push(meta.timestamp());
            true
        });

        assert_eq!(&[crate::ClockTime::from(0)][..], &res[..]);
    }
}
