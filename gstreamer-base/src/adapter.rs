// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use gst;
use gst_base_sys;
use std::io;
use std::ops;
use Adapter;

impl Adapter {
    pub fn copy(&self, offset: usize, dest: &mut [u8]) {
        unsafe {
            let size = dest.len();
            gst_base_sys::gst_adapter_copy(
                self.to_glib_none().0,
                dest.as_mut_ptr() as *mut _,
                offset,
                size,
            );
        }
    }

    pub fn push(&self, buf: gst::Buffer) {
        unsafe {
            gst_base_sys::gst_adapter_push(self.to_glib_none().0, buf.into_ptr());
        }
    }
}

impl io::Read for Adapter {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        let mut len = self.available();

        if len == 0 {
            return Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                format!(
                    "Missing data: requesting {} but only got {}.",
                    buf.len(),
                    len
                ),
            ));
        }

        if buf.len() < len {
            len = buf.len();
        }

        self.copy(0, &mut buf[0..len]);
        self.flush(len);
        Ok(len)
    }
}

#[derive(Debug)]
pub struct UniqueAdapter(Adapter);

unsafe impl Send for UniqueAdapter {}
unsafe impl Sync for UniqueAdapter {}

impl UniqueAdapter {
    pub fn new() -> UniqueAdapter {
        UniqueAdapter(Adapter::new())
    }

    pub fn available(&self) -> usize {
        self.0.available()
    }

    pub fn available_fast(&self) -> usize {
        self.0.available_fast()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn copy_bytes(&self, offset: usize, size: usize) -> Result<glib::Bytes, glib::BoolError> {
        self.0.copy_bytes(offset, size)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn distance_from_discont(&self) -> u64 {
        self.0.distance_from_discont()
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn dts_at_discont(&self) -> gst::ClockTime {
        self.0.dts_at_discont()
    }

    pub fn flush(&mut self, flush: usize) {
        self.0.flush(flush);
    }

    pub fn get_buffer(&self, nbytes: usize) -> Result<gst::Buffer, glib::BoolError> {
        self.0.get_buffer(nbytes)
    }

    pub fn get_buffer_fast(&self, nbytes: usize) -> Result<gst::Buffer, glib::BoolError> {
        self.0.get_buffer_fast(nbytes)
    }

    pub fn get_buffer_list(&self, nbytes: usize) -> Result<gst::BufferList, glib::BoolError> {
        self.0.get_buffer_list(nbytes)
    }

    pub fn get_list(&self, nbytes: usize) -> Vec<gst::Buffer> {
        self.0.get_list(nbytes)
    }

    pub fn masked_scan_uint32(&self, mask: u32, pattern: u32, offset: usize, size: usize) -> isize {
        self.0.masked_scan_uint32(mask, pattern, offset, size)
    }

    pub fn masked_scan_uint32_peek(
        &self,
        mask: u32,
        pattern: u32,
        offset: usize,
        size: usize,
    ) -> (isize, u32) {
        self.0.masked_scan_uint32_peek(mask, pattern, offset, size)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn offset_at_discont(&self) -> u64 {
        self.0.offset_at_discont()
    }

    pub fn prev_dts(&self) -> (gst::ClockTime, u64) {
        self.0.prev_dts()
    }

    pub fn prev_dts_at_offset(&self, offset: usize) -> (gst::ClockTime, u64) {
        self.0.prev_dts_at_offset(offset)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn prev_offset(&self) -> (u64, u64) {
        self.0.prev_offset()
    }

    pub fn prev_pts(&self) -> (gst::ClockTime, u64) {
        self.0.prev_pts()
    }

    pub fn prev_pts_at_offset(&self, offset: usize) -> (gst::ClockTime, u64) {
        self.0.prev_pts_at_offset(offset)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn pts_at_discont(&self) -> gst::ClockTime {
        self.0.pts_at_discont()
    }

    pub fn take_buffer(&mut self, nbytes: usize) -> Result<gst::Buffer, glib::BoolError> {
        self.0.take_buffer(nbytes)
    }

    pub fn take_buffer_fast(&mut self, nbytes: usize) -> Result<gst::Buffer, glib::BoolError> {
        self.0.take_buffer_fast(nbytes)
    }

    pub fn take_buffer_list(&mut self, nbytes: usize) -> Result<gst::BufferList, glib::BoolError> {
        self.0.take_buffer_list(nbytes)
    }

    pub fn take_list(&mut self, nbytes: usize) -> Vec<gst::Buffer> {
        self.0.take_list(nbytes)
    }

    pub fn copy(&self, offset: usize, dest: &mut [u8]) {
        self.0.copy(offset, dest);
    }

    pub fn push(&mut self, buf: gst::Buffer) {
        self.0.push(buf);
    }

    pub fn map(&mut self, nbytes: usize) -> Result<UniqueAdapterMap, glib::error::BoolError> {
        use std::slice;

        unsafe {
            let ptr = gst_base_sys::gst_adapter_map(self.0.to_glib_none().0, nbytes);
            if ptr.is_null() {
                Err(glib_bool_error!("size bytes are not available"))
            } else {
                Ok(UniqueAdapterMap(
                    self,
                    slice::from_raw_parts(ptr as *const u8, nbytes),
                ))
            }
        }
    }
}

#[derive(Debug)]
pub struct UniqueAdapterMap<'a>(&'a UniqueAdapter, &'a [u8]);

impl<'a> Drop for UniqueAdapterMap<'a> {
    fn drop(&mut self) {
        unsafe {
            gst_base_sys::gst_adapter_unmap((self.0).0.to_glib_none().0);
        }
    }
}

impl<'a> ops::Deref for UniqueAdapterMap<'a> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.1
    }
}

impl<'a> AsRef<[u8]> for UniqueAdapterMap<'a> {
    fn as_ref(&self) -> &[u8] {
        self.1
    }
}

impl Default for UniqueAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl io::Read for UniqueAdapter {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        self.0.read(buf)
    }
}
