// Copyright (C) 2020 Sebastian Dröge <sebastian@centricular.com>
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
use std::ptr;
use std::u64;
use std::usize;

use Buffer;
use BufferRef;

use glib;
use glib_sys;
use gst_sys;

use buffer::{Readable, Writable};

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

pub struct BufferRefCursor<T> {
    buffer: T,
    size: u64,
    num_mem: u32,
    cur_mem_idx: u32,
    cur_offset: u64,
    cur_mem_offset: usize,
    map_info: gst_sys::GstMapInfo,
}

macro_rules! define_seek_impl(
    ($get_buffer_ref:expr) => {
        fn seek(&mut self, pos: io::SeekFrom) -> Result<u64, io::Error> {
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

            // Work around lifetime annotation issues with closures
            let get_buffer_ref: fn(&Self) -> &BufferRef = $get_buffer_ref;
            let (idx, _, skip) = get_buffer_ref(self)
                .find_memory(self.cur_offset as usize, None)
                .expect("Failed to find memory");

            if idx != self.cur_mem_idx && !self.map_info.memory.is_null() {
                unsafe {
                    gst_sys::gst_memory_unmap(self.map_info.memory, &mut self.map_info);
                    self.map_info.memory = ptr::null_mut();
                }
            }

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
);

macro_rules! define_read_write_fn_impl(
    ($self:ident, $data:ident, $data_type:ty, $get_buffer_ref:expr, $map_flags:path, $copy:expr, $split:expr) => {{
        let mut copied = 0;

        while !$data.is_empty() && $self.cur_mem_idx < $self.num_mem {
            // Map memory if needed. cur_mem_idx, cur_mem_offset and cur_offset are required to be
            // set correctly here already (from constructor, seek and the bottom of the loop)
            if $self.map_info.memory.is_null() {
                unsafe {
                    // Work around lifetime annotation issues with closures
                    let get_buffer_ref: fn(&Self) -> &BufferRef = $get_buffer_ref;
                    let memory = gst_sys::gst_buffer_peek_memory(
                        get_buffer_ref($self).as_mut_ptr(),
                        $self.cur_mem_idx,
                    );
                    assert!(!memory.is_null());

                    if gst_sys::gst_memory_map(memory, &mut $self.map_info, $map_flags)
                        == glib_sys::GFALSE
                    {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Failed to map memory readable",
                        ));
                    }
                }

                assert!($self.cur_mem_offset < $self.map_info.size);
            }

            assert!(!$self.map_info.memory.is_null());

            // Copy all data we can currently copy
            let data_left = $self.map_info.size - $self.cur_mem_offset;
            let to_copy = std::cmp::min($data.len(), data_left);
            $copy(&$self.map_info, $self.cur_mem_offset, $data, to_copy);
            copied += to_copy;
            $self.cur_offset += to_copy as u64;
            $self.cur_mem_offset += to_copy;
            // Work around lifetime annotation issues with closures
            let split: fn($data_type, usize) -> $data_type = $split;
            #[allow(clippy::redundant_closure_call)]
            {
                $data = split($data, to_copy);
            }

            // If we're at the end of the current memory, unmap and advance to the next memory
            if $self.cur_mem_offset == $self.map_info.size {
                unsafe {
                    gst_sys::gst_memory_unmap($self.map_info.memory, &mut $self.map_info);
                }
                $self.map_info.memory = ptr::null_mut();
                $self.cur_mem_idx += 1;
                $self.cur_mem_offset = 0;
            }
        }

        Ok(copied)
    }}
);

macro_rules! define_read_impl(
    ($get_buffer_ref:expr) => {
        fn read(&mut self, mut data: &mut [u8]) -> Result<usize, io::Error> {
            define_read_write_fn_impl!(
                self,
                data,
                &mut [u8],
                $get_buffer_ref,
                gst_sys::GST_MAP_READ,
                |map_info: &gst_sys::GstMapInfo, off, data: &mut [u8], to_copy| unsafe {
                    ptr::copy_nonoverlapping(
                        (map_info.data as *const u8).add(off),
                        data.as_mut_ptr(),
                        to_copy,
                    );
                },
                |data, to_copy| &mut data[to_copy..]
            )
        }
    }
);

macro_rules! define_write_impl(
    ($get_buffer_ref:expr) => {
        fn write(&mut self, mut data: &[u8]) -> Result<usize, io::Error> {
            define_read_write_fn_impl!(
                self,
                data,
                &[u8],
                $get_buffer_ref,
                gst_sys::GST_MAP_WRITE,
                |map_info: &gst_sys::GstMapInfo, off, data: &[u8], to_copy| unsafe {
                    ptr::copy_nonoverlapping(
                        data.as_ptr(),
                        (map_info.data as *mut u8).add(off),
                        to_copy,
                    );
                },
                |data, to_copy| &data[to_copy..]
            )
        }

        fn flush(&mut self) -> Result<(), io::Error> {
            if !self.map_info.memory.is_null() {
                unsafe {
                    gst_sys::gst_memory_unmap(self.map_info.memory, &mut self.map_info);
                    self.map_info.memory = ptr::null_mut();
                }
            }

            Ok(())
        }
    }
);

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
    define_read_impl!(|s| s.buffer.as_ref().unwrap());
}

impl io::Write for BufferCursor<Writable> {
    define_write_impl!(|s| s.buffer.as_ref().unwrap());
}

impl<T> io::Seek for BufferCursor<T> {
    define_seek_impl!(|s| s.buffer.as_ref().unwrap());
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
    pub(crate) fn new_readable(buffer: Buffer) -> BufferCursor<Readable> {
        skip_assert_initialized!();
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
    pub(crate) fn new_writable(buffer: Buffer) -> Result<BufferCursor<Writable>, glib::BoolError> {
        skip_assert_initialized!();
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

impl<T: fmt::Debug> fmt::Debug for BufferRefCursor<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BufferRefCursor")
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

impl<T> Drop for BufferRefCursor<T> {
    fn drop(&mut self) {
        if !self.map_info.memory.is_null() {
            unsafe {
                gst_sys::gst_memory_unmap(self.map_info.memory, &mut self.map_info);
            }
        }
    }
}

impl<'a> io::Read for BufferRefCursor<&'a BufferRef> {
    define_read_impl!(|s| s.buffer);
}

impl<'a> io::Write for BufferRefCursor<&'a mut BufferRef> {
    define_write_impl!(|s| s.buffer);
}

impl<'a> io::Seek for BufferRefCursor<&'a BufferRef> {
    define_seek_impl!(|s| s.buffer);
}

impl<'a> io::Seek for BufferRefCursor<&'a mut BufferRef> {
    define_seek_impl!(|s| s.buffer);
}

impl<T> BufferRefCursor<T> {
    pub fn stream_len(&mut self) -> Result<u64, io::Error> {
        Ok(self.size)
    }

    pub fn stream_position(&mut self) -> Result<u64, io::Error> {
        Ok(self.cur_offset)
    }
}

impl<'a> BufferRefCursor<&'a BufferRef> {
    pub fn get_buffer(&self) -> &BufferRef {
        self.buffer
    }

    pub(crate) fn new_readable(buffer: &'a BufferRef) -> BufferRefCursor<&'a BufferRef> {
        skip_assert_initialized!();
        let size = buffer.get_size() as u64;
        let num_mem = buffer.n_memory();

        BufferRefCursor {
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

impl<'a> BufferRefCursor<&'a mut BufferRef> {
    pub fn get_buffer(&self) -> &BufferRef {
        self.buffer
    }

    pub(crate) fn new_writable(
        buffer: &'a mut BufferRef,
    ) -> Result<BufferRefCursor<&'a mut BufferRef>, glib::BoolError> {
        skip_assert_initialized!();
        if !buffer.is_all_memory_writable() {
            return Err(glib_bool_error!("Not all memories are writable"));
        }

        let size = buffer.get_size() as u64;
        let num_mem = buffer.n_memory();

        Ok(BufferRefCursor {
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

unsafe impl<T> Send for BufferRefCursor<T> {}
unsafe impl<T> Sync for BufferRefCursor<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::cognitive_complexity)]
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

        let mut cursor = buffer.into_cursor_writable().unwrap();
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

        let mut cursor = buffer.into_cursor_readable();
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
    #[allow(clippy::cognitive_complexity)]
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

            let mut cursor = buffer.as_cursor_writable().unwrap();
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

        let mut cursor = buffer.as_cursor_readable();
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
