// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Adapter;
use glib::translate::*;
use std::io;
use std::mem;
use std::ops;

impl Adapter {
    pub fn copy(&self, offset: usize, dest: &mut [u8]) -> Result<(), glib::BoolError> {
        assert!(
            offset
                .checked_add(dest.len())
                .map(|end| end <= self.available())
                == Some(true)
        );

        if dest.is_empty() {
            return Ok(());
        }

        unsafe {
            let size = dest.len();
            ffi::gst_adapter_copy(
                self.to_glib_none().0,
                dest.as_mut_ptr() as *mut _,
                offset,
                size,
            );
        }

        Ok(())
    }

    pub fn copy_bytes(&self, offset: usize, size: usize) -> Result<glib::Bytes, glib::BoolError> {
        assert!(offset.checked_add(size).map(|end| end <= self.available()) == Some(true));

        if size == 0 {
            return Ok(glib::Bytes::from_static(&[]));
        }

        unsafe {
            Ok(from_glib_full(ffi::gst_adapter_copy_bytes(
                self.to_glib_none().0,
                offset,
                size,
            )))
        }
    }

    pub fn flush(&self, flush: usize) {
        assert!(flush <= self.available());

        if flush == 0 {
            return;
        }

        unsafe {
            ffi::gst_adapter_flush(self.to_glib_none().0, flush);
        }
    }

    pub fn get_buffer(&self, nbytes: usize) -> Result<gst::Buffer, glib::BoolError> {
        assert!(nbytes <= self.available());
        assert!(nbytes != 0);

        unsafe {
            Option::<_>::from_glib_full(ffi::gst_adapter_get_buffer(self.to_glib_none().0, nbytes))
                .ok_or_else(|| glib::bool_error!("Failed to get buffer"))
        }
    }

    pub fn get_buffer_fast(&self, nbytes: usize) -> Result<gst::Buffer, glib::BoolError> {
        assert!(nbytes <= self.available());
        assert!(nbytes != 0);

        unsafe {
            Option::<_>::from_glib_full(ffi::gst_adapter_get_buffer_fast(
                self.to_glib_none().0,
                nbytes,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to get buffer"))
        }
    }

    pub fn get_buffer_list(&self, nbytes: usize) -> Result<gst::BufferList, glib::BoolError> {
        assert!(nbytes <= self.available());
        assert!(nbytes != 0);

        unsafe {
            Option::<_>::from_glib_full(ffi::gst_adapter_get_buffer_list(
                self.to_glib_none().0,
                nbytes,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to get buffer list"))
        }
    }

    pub fn get_list(&self, nbytes: usize) -> Result<Vec<gst::Buffer>, glib::BoolError> {
        assert!(nbytes <= self.available());
        assert!(nbytes != 0);

        unsafe {
            Ok(FromGlibPtrContainer::from_glib_full(
                ffi::gst_adapter_get_list(self.to_glib_none().0, nbytes),
            ))
        }
    }

    pub fn masked_scan_uint32(
        &self,
        mask: u32,
        pattern: u32,
        offset: usize,
        size: usize,
    ) -> Result<Option<usize>, glib::BoolError> {
        assert!(offset.checked_add(size).map(|end| end <= self.available()) == Some(true));
        assert!(size != 0);
        assert!(((!mask) & pattern) == 0);

        unsafe {
            let ret = ffi::gst_adapter_masked_scan_uint32(
                self.to_glib_none().0,
                mask,
                pattern,
                offset,
                size,
            );
            if ret == -1 {
                Ok(None)
            } else {
                assert!(ret >= 0);
                Ok(Some(ret as usize))
            }
        }
    }

    pub fn masked_scan_uint32_peek(
        &self,
        mask: u32,
        pattern: u32,
        offset: usize,
        size: usize,
    ) -> Result<Option<(usize, u32)>, glib::BoolError> {
        assert!(offset.checked_add(size).map(|end| end <= self.available()) == Some(true));
        assert!(size != 0);
        assert!(((!mask) & pattern) == 0);

        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let ret = ffi::gst_adapter_masked_scan_uint32_peek(
                self.to_glib_none().0,
                mask,
                pattern,
                offset,
                size,
                value.as_mut_ptr(),
            );

            if ret == -1 {
                Ok(None)
            } else {
                assert!(ret >= 0);
                let value = value.assume_init();
                Ok(Some((ret as usize, value)))
            }
        }
    }

    pub fn take_buffer(&self, nbytes: usize) -> Result<gst::Buffer, glib::BoolError> {
        assert!(nbytes <= self.available());
        assert!(nbytes != 0);

        unsafe {
            Option::<_>::from_glib_full(ffi::gst_adapter_take_buffer(self.to_glib_none().0, nbytes))
                .ok_or_else(|| glib::bool_error!("Failed to take buffer"))
        }
    }

    pub fn take_buffer_fast(&self, nbytes: usize) -> Result<gst::Buffer, glib::BoolError> {
        assert!(nbytes <= self.available());
        assert!(nbytes != 0);

        unsafe {
            Option::<_>::from_glib_full(ffi::gst_adapter_take_buffer_fast(
                self.to_glib_none().0,
                nbytes,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to take buffer"))
        }
    }

    pub fn take_buffer_list(&self, nbytes: usize) -> Result<gst::BufferList, glib::BoolError> {
        assert!(nbytes <= self.available());
        assert!(nbytes != 0);

        unsafe {
            Option::<_>::from_glib_full(ffi::gst_adapter_take_buffer_list(
                self.to_glib_none().0,
                nbytes,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to take buffer list"))
        }
    }

    pub fn take_list(&self, nbytes: usize) -> Result<Vec<gst::Buffer>, glib::BoolError> {
        assert!(nbytes <= self.available());
        assert!(nbytes != 0);

        unsafe {
            Ok(FromGlibPtrContainer::from_glib_full(
                ffi::gst_adapter_take_list(self.to_glib_none().0, nbytes),
            ))
        }
    }

    pub fn push(&self, buf: gst::Buffer) {
        unsafe {
            ffi::gst_adapter_push(self.to_glib_none().0, buf.into_ptr());
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

        self.copy(0, &mut buf[0..len])
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

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
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    pub fn distance_from_discont(&self) -> u64 {
        self.0.distance_from_discont()
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
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

    pub fn get_list(&self, nbytes: usize) -> Result<Vec<gst::Buffer>, glib::BoolError> {
        self.0.get_list(nbytes)
    }

    pub fn masked_scan_uint32(
        &self,
        mask: u32,
        pattern: u32,
        offset: usize,
        size: usize,
    ) -> Result<Option<usize>, glib::BoolError> {
        self.0.masked_scan_uint32(mask, pattern, offset, size)
    }

    pub fn masked_scan_uint32_peek(
        &self,
        mask: u32,
        pattern: u32,
        offset: usize,
        size: usize,
    ) -> Result<Option<(usize, u32)>, glib::BoolError> {
        self.0.masked_scan_uint32_peek(mask, pattern, offset, size)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
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
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
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
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
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

    pub fn take_list(&mut self, nbytes: usize) -> Result<Vec<gst::Buffer>, glib::BoolError> {
        self.0.take_list(nbytes)
    }

    pub fn copy(&self, offset: usize, dest: &mut [u8]) -> Result<(), glib::BoolError> {
        self.0.copy(offset, dest)
    }

    pub fn push(&mut self, buf: gst::Buffer) {
        self.0.push(buf);
    }

    pub fn map(&mut self, nbytes: usize) -> Result<UniqueAdapterMap, glib::error::BoolError> {
        assert!(nbytes <= self.available());
        assert!(nbytes != 0);

        use std::slice;

        unsafe {
            let ptr = ffi::gst_adapter_map(self.0.to_glib_none().0, nbytes);
            if ptr.is_null() {
                Err(glib::bool_error!("size bytes are not available"))
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
            ffi::gst_adapter_unmap((self.0).0.to_glib_none().0);
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
