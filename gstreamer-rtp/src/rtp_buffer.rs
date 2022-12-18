// Take a look at the license at the top of the repository in the LICENSE file.

use std::{fmt, marker::PhantomData, mem, ptr, slice};

use glib::translate::{from_glib, mut_override, FromGlibPtrFull, IntoGlib};

pub enum Readable {}
pub enum Writable {}

pub struct RTPBuffer<'a, T> {
    rtp_buffer: ffi::GstRTPBuffer,
    phantom: PhantomData<&'a T>,
}

unsafe impl<'a, T> Send for RTPBuffer<'a, T> {}
unsafe impl<'a, T> Sync for RTPBuffer<'a, T> {}

impl<'a, T> fmt::Debug for RTPBuffer<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RTPBuffer")
            .field("rtp_buffer", &self.rtp_buffer)
            .finish()
    }
}

impl<'a> RTPBuffer<'a, Readable> {
    #[inline]
    pub fn from_buffer_readable(
        buffer: &'a gst::BufferRef,
    ) -> Result<RTPBuffer<Readable>, glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            let mut rtp_buffer = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(ffi::gst_rtp_buffer_map(
                mut_override(buffer.as_ptr()),
                gst::ffi::GST_MAP_READ,
                rtp_buffer.as_mut_ptr(),
            ));

            if res {
                Ok(RTPBuffer {
                    rtp_buffer: rtp_buffer.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(glib::bool_error!("Failed to map RTP buffer readable"))
            }
        }
    }

    #[inline]
    pub unsafe fn from_glib_borrow<'b>(
        rtp_buffer: *mut ffi::GstRTPBuffer,
    ) -> glib::translate::Borrowed<RTPBuffer<'b, Readable>> {
        glib::translate::Borrowed::new(RTPBuffer {
            rtp_buffer: *rtp_buffer,
            phantom: PhantomData,
        })
    }
}

impl<'a> RTPBuffer<'a, Writable> {
    #[inline]
    pub fn from_buffer_writable(
        buffer: &'a mut gst::BufferRef,
    ) -> Result<RTPBuffer<Writable>, glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            let mut rtp_buffer = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(ffi::gst_rtp_buffer_map(
                buffer.as_mut_ptr(),
                gst::ffi::GST_MAP_READWRITE,
                rtp_buffer.as_mut_ptr(),
            ));

            if res {
                Ok(RTPBuffer {
                    rtp_buffer: rtp_buffer.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(glib::bool_error!("Failed to map RTP buffer writable"))
            }
        }
    }

    #[doc(alias = "gst_rtp_buffer_set_seq")]
    pub fn set_seq(&mut self, seq: u16) {
        unsafe {
            ffi::gst_rtp_buffer_set_seq(&mut self.rtp_buffer, seq);
        }
    }

    #[doc(alias = "gst_rtp_buffer_set_marker")]
    pub fn set_marker(&mut self, m: bool) {
        unsafe {
            ffi::gst_rtp_buffer_set_marker(&mut self.rtp_buffer, m.into_glib());
        }
    }

    #[doc(alias = "gst_rtp_buffer_set_payload_type")]
    pub fn set_payload_type(&mut self, pt: u8) {
        unsafe {
            ffi::gst_rtp_buffer_set_payload_type(&mut self.rtp_buffer, pt);
        }
    }

    #[doc(alias = "gst_rtp_buffer_set_ssrc")]
    pub fn set_ssrc(&mut self, ssrc: u32) {
        unsafe { ffi::gst_rtp_buffer_set_ssrc(&mut self.rtp_buffer, ssrc) }
    }

    #[doc(alias = "gst_rtp_buffer_set_csrc")]
    pub fn set_csrc(&mut self, idx: u8, ssrc: u32) {
        unsafe { ffi::gst_rtp_buffer_set_csrc(&mut self.rtp_buffer, idx, ssrc) }
    }

    #[doc(alias = "gst_rtp_buffer_set_timestamp")]
    pub fn set_timestamp(&mut self, rtptime: u32) {
        unsafe {
            ffi::gst_rtp_buffer_set_timestamp(&mut self.rtp_buffer, rtptime);
        }
    }

    #[doc(alias = "gst_rtp_buffer_set_extension")]
    pub fn set_extension(&mut self, extension: bool) {
        unsafe { ffi::gst_rtp_buffer_set_extension(&mut self.rtp_buffer, extension.into_glib()) }
    }

    #[doc(alias = "gst_rtp_buffer_add_extension_onebyte_header")]
    pub fn add_extension_onebyte_header(
        &mut self,
        id: u8,
        data: &[u8],
    ) -> Result<(), glib::BoolError> {
        assert!(
            id >= 1 && id <= 14,
            "id should be between 1 and 14 (inclusive)"
        );
        assert!(
            !data.is_empty() && data.len() <= 16,
            "data size should be between 1 and 16 (inclusive"
        );
        unsafe {
            let result: bool = from_glib(ffi::gst_rtp_buffer_add_extension_onebyte_header(
                &mut self.rtp_buffer,
                id,
                data.as_ptr() as glib::ffi::gconstpointer,
                data.len() as u32,
            ));
            if result {
                Ok(())
            } else {
                Err(glib::bool_error!("Failed to add onebyte header extension"))
            }
        }
    }

    #[doc(alias = "gst_rtp_buffer_add_extension_twobytes_header")]
    pub fn add_extension_twobytes_header(
        &mut self,
        appbits: u8,
        id: u8,
        data: &[u8],
    ) -> Result<(), glib::BoolError> {
        assert_eq!(
            appbits & 0xF0,
            0,
            "appbits must use only 4 bits (max value is 15)"
        );
        assert!(data.len() < 256, "data size should be smaller than 256");
        unsafe {
            let result: bool = from_glib(ffi::gst_rtp_buffer_add_extension_twobytes_header(
                &mut self.rtp_buffer,
                appbits,
                id,
                data.as_ptr() as glib::ffi::gconstpointer,
                data.len() as u32,
            ));
            if result {
                Ok(())
            } else {
                Err(glib::bool_error!("Failed to add twobytes header extension"))
            }
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_rtp_buffer_remove_extension_data")]
    pub fn remove_extension_data(&mut self) {
        unsafe {
            ffi::gst_rtp_buffer_remove_extension_data(&mut self.rtp_buffer);
        }
    }

    #[doc(alias = "gst_rtp_buffer_set_padding")]
    pub fn set_padding(&mut self, padding: bool) {
        unsafe { ffi::gst_rtp_buffer_set_padding(&mut self.rtp_buffer, padding.into_glib()) }
    }
}

impl<'a, T> RTPBuffer<'a, T> {
    #[doc(alias = "get_seq")]
    #[doc(alias = "gst_rtp_buffer_get_seq")]
    pub fn seq(&self) -> u16 {
        unsafe { ffi::gst_rtp_buffer_get_seq(glib::translate::mut_override(&self.rtp_buffer)) }
    }

    #[doc(alias = "get_payload_type")]
    #[doc(alias = "gst_rtp_buffer_get_payload_type")]
    pub fn payload_type(&self) -> u8 {
        unsafe {
            ffi::gst_rtp_buffer_get_payload_type(glib::translate::mut_override(&self.rtp_buffer))
        }
    }

    #[doc(alias = "get_ssrc")]
    #[doc(alias = "gst_rtp_buffer_get_ssrc")]
    pub fn ssrc(&self) -> u32 {
        unsafe { ffi::gst_rtp_buffer_get_ssrc(glib::translate::mut_override(&self.rtp_buffer)) }
    }

    #[doc(alias = "get_timestamp")]
    #[doc(alias = "gst_rtp_buffer_get_timestamp")]
    pub fn timestamp(&self) -> u32 {
        unsafe {
            ffi::gst_rtp_buffer_get_timestamp(glib::translate::mut_override(&self.rtp_buffer))
        }
    }

    #[doc(alias = "get_csrc")]
    #[doc(alias = "gst_rtp_buffer_get_csrc")]
    pub fn csrc(&self, idx: u8) -> Option<u32> {
        if idx < self.csrc_count() {
            unsafe {
                Some(ffi::gst_rtp_buffer_get_csrc(
                    glib::translate::mut_override(&self.rtp_buffer),
                    idx,
                ))
            }
        } else {
            None
        }
    }

    #[doc(alias = "get_csrc_count")]
    #[doc(alias = "gst_rtp_buffer_get_csrc_count")]
    pub fn csrc_count(&self) -> u8 {
        unsafe {
            ffi::gst_rtp_buffer_get_csrc_count(glib::translate::mut_override(&self.rtp_buffer))
        }
    }

    #[doc(alias = "get_marker")]
    pub fn is_marker(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtp_buffer_get_marker(
                glib::translate::mut_override(&self.rtp_buffer),
            ))
        }
    }

    #[doc(alias = "get_payload_size")]
    pub fn payload_size(&self) -> u32 {
        unsafe {
            ffi::gst_rtp_buffer_get_payload_len(glib::translate::mut_override(&self.rtp_buffer))
        }
    }

    #[doc(alias = "get_payload")]
    #[doc(alias = "gst_rtp_buffer_get_payload")]
    pub fn payload(&self) -> Result<&[u8], glib::error::BoolError> {
        let size = self.payload_size();
        if size == 0 {
            return Ok(&[]);
        }
        unsafe {
            let pointer =
                ffi::gst_rtp_buffer_get_payload(glib::translate::mut_override(&self.rtp_buffer));
            if pointer.is_null() {
                Err(glib::bool_error!("Failed to get payload data"))
            } else {
                Ok(slice::from_raw_parts(pointer as *const u8, size as usize))
            }
        }
    }

    #[doc(alias = "get_payload")]
    #[doc(alias = "gst_rtp_buffer_get_payload")]
    pub fn payload_mut(&mut self) -> Result<&mut [u8], glib::error::BoolError> {
        let size = self.payload_size();
        if size == 0 {
            return Ok(&mut []);
        }
        unsafe {
            let pointer = ffi::gst_rtp_buffer_get_payload(&mut self.rtp_buffer);
            if pointer.is_null() {
                Err(glib::bool_error!("Failed to get payload data"))
            } else {
                Ok(slice::from_raw_parts_mut(pointer as *mut u8, size as usize))
            }
        }
    }

    #[doc(alias = "get_payload_buffer")]
    #[doc(alias = "gst_rtp_buffer_get_payload_buffer")]
    pub fn payload_buffer(&self) -> Result<gst::Buffer, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_rtp_buffer_get_payload_buffer(
                glib::translate::mut_override(&self.rtp_buffer),
            ))
            .ok_or_else(|| glib::bool_error!("Failed to get payload buffer"))
        }
    }

    #[inline]
    pub fn buffer(&self) -> &gst::BufferRef {
        unsafe {
            let ptr = self.rtp_buffer.buffer;

            assert!(!ptr.is_null());

            gst::BufferRef::from_ptr(ptr)
        }
    }

    #[doc(alias = "get_extension")]
    pub fn is_extension(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtp_buffer_get_extension(
                glib::translate::mut_override(&self.rtp_buffer),
            ))
        }
    }

    #[doc(alias = "get_extension_bytes")]
    #[doc(alias = "gst_rtp_buffer_get_extension_bytes")]
    pub fn extension_bytes(&self) -> Option<(u16, glib::Bytes)> {
        unsafe {
            let mut bits: u16 = 0;
            Option::<glib::Bytes>::from_glib_full(ffi::gst_rtp_buffer_get_extension_bytes(
                glib::translate::mut_override(&self.rtp_buffer),
                &mut bits,
            ))
            .map(|bytes| (bits, bytes))
        }
    }

    #[doc(alias = "get_extension_onebyte_header")]
    #[doc(alias = "gst_rtp_buffer_get_extension_onebyte_header")]
    pub fn extension_onebyte_header(&self, id: u8, nth: u32) -> Option<&[u8]> {
        unsafe {
            let mut data = ptr::null_mut();
            // FIXME: Workaround for gstreamer-rtp-sys having the wrong type for this parameter
            let data_ptr = &mut data as *mut *mut u8 as *mut u8;
            let mut size: u32 = 0;
            let result: bool = from_glib(ffi::gst_rtp_buffer_get_extension_onebyte_header(
                glib::translate::mut_override(&self.rtp_buffer),
                id,
                nth,
                data_ptr,
                &mut size,
            ));
            if result {
                if size == 0 {
                    Some(&[])
                } else {
                    Some(slice::from_raw_parts(data as *const u8, size as usize))
                }
            } else {
                None
            }
        }
    }

    #[doc(alias = "get_extension_twobytes_header")]
    #[doc(alias = "gst_rtp_buffer_get_extension_twobytes_header")]
    pub fn extension_twobytes_header(&self, id: u8, nth: u32) -> Option<(u8, &[u8])> {
        unsafe {
            let mut data = ptr::null_mut();
            // FIXME: Workaround for gstreamer-rtp-sys having the wrong type for this parameter
            let data_ptr = &mut data as *mut *mut u8 as *mut u8;
            let mut size: u32 = 0;
            let mut appbits = 0;
            let result: bool = from_glib(ffi::gst_rtp_buffer_get_extension_twobytes_header(
                glib::translate::mut_override(&self.rtp_buffer),
                &mut appbits,
                id,
                nth,
                data_ptr,
                &mut size,
            ));
            if result {
                if size == 0 {
                    Some((appbits, &[]))
                } else {
                    Some((
                        appbits,
                        slice::from_raw_parts(data as *const u8, size as usize),
                    ))
                }
            } else {
                None
            }
        }
    }

    #[doc(alias = "get_padding")]
    #[doc(alias = "gst_rtp_buffer_get_padding")]
    pub fn has_padding(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtp_buffer_get_padding(
                glib::translate::mut_override(&self.rtp_buffer),
            ))
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ffi::GstRTPBuffer {
        &self.rtp_buffer as *const ffi::GstRTPBuffer
    }

    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::GstRTPBuffer {
        &self.rtp_buffer as *const ffi::GstRTPBuffer as *mut ffi::GstRTPBuffer
    }
}

impl<'a, T> Drop for RTPBuffer<'a, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::gst_rtp_buffer_unmap(&mut self.rtp_buffer);
        }
    }
}

pub trait RTPBufferExt {
    fn new_rtp_with_sizes(
        payload_len: u32,
        pad_len: u8,
        csrc_count: u8,
    ) -> Result<gst::Buffer, glib::BoolError>;
}

impl RTPBufferExt for gst::Buffer {
    fn new_rtp_with_sizes(
        payload_len: u32,
        pad_len: u8,
        csrc_count: u8,
    ) -> Result<gst::Buffer, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_rtp_buffer_new_allocate(
                payload_len,
                pad_len,
                csrc_count,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to allocate new RTP buffer"))
        }
    }
}

#[doc(alias = "gst_rtp_buffer_compare_seqnum")]
pub fn compare_seqnum(seqnum1: u16, seqnum2: u16) -> i32 {
    skip_assert_initialized!();
    unsafe { ffi::gst_rtp_buffer_compare_seqnum(seqnum1, seqnum2) }
}

#[doc(alias = "gst_rtp_buffer_calc_header_len")]
pub fn calc_header_len(csrc_count: u8) -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gst_rtp_buffer_calc_header_len(csrc_count) }
}

#[doc(alias = "gst_rtp_buffer_calc_packet_len")]
pub fn calc_packet_len(payload_len: u32, pad_len: u8, csrc_count: u8) -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gst_rtp_buffer_calc_packet_len(payload_len, pad_len, csrc_count) }
}

#[doc(alias = "gst_rtp_buffer_calc_payload_len")]
pub fn calc_payload_len(packet_len: u32, pad_len: u8, csrc_count: u8) -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gst_rtp_buffer_calc_payload_len(packet_len, pad_len, csrc_count) }
}

#[doc(alias = "gst_rtp_buffer_ext_timestamp")]
pub fn ext_timestamp(exttimestamp: &mut u64, timestamp: u32) -> u64 {
    skip_assert_initialized!();
    unsafe { ffi::gst_rtp_buffer_ext_timestamp(exttimestamp, timestamp) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        gst::init().unwrap();

        let csrc_count = 2;
        let payload_size = 16;
        let mut buffer = gst::Buffer::new_rtp_with_sizes(payload_size, 4, csrc_count).unwrap();
        {
            let buffer = buffer.get_mut().unwrap();
            let mut rtp_buffer = RTPBuffer::from_buffer_writable(buffer).unwrap();

            rtp_buffer.set_seq(42);
            assert_eq!(rtp_buffer.seq(), 42);

            rtp_buffer.set_marker(true);
            assert!(rtp_buffer.is_marker());

            rtp_buffer.set_payload_type(43);
            assert_eq!(rtp_buffer.payload_type(), 43);

            rtp_buffer.set_timestamp(44);
            assert_eq!(rtp_buffer.timestamp(), 44);

            rtp_buffer.set_ssrc(45);
            assert_eq!(rtp_buffer.ssrc(), 45);

            assert_eq!(rtp_buffer.payload_size(), payload_size);
            let payload = rtp_buffer.payload();
            assert!(payload.is_ok());
            let payload = payload.unwrap();
            assert_eq!(payload.len(), payload_size as usize);

            assert_eq!(rtp_buffer.csrc_count(), csrc_count);
            rtp_buffer.set_csrc(0, 12);
            rtp_buffer.set_csrc(1, 15);
            assert_eq!(rtp_buffer.csrc(0).unwrap(), 12);
            assert_eq!(rtp_buffer.csrc(1).unwrap(), 15);
            assert!(rtp_buffer.csrc(2).is_none());

            rtp_buffer.set_extension(true);
            assert!(rtp_buffer.is_extension());

            assert_eq!(rtp_buffer.extension_bytes(), None);
        }
    }

    #[test]
    fn test_empty_payload() {
        gst::init().unwrap();

        let csrc_count = 0;
        let payload_size = 0;
        let buffer = gst::Buffer::new_rtp_with_sizes(payload_size, 4, csrc_count).unwrap();
        let rtp_buffer = RTPBuffer::from_buffer_readable(&buffer).unwrap();

        assert_eq!(rtp_buffer.payload_size(), payload_size);
        let payload = rtp_buffer.payload();
        assert!(payload.is_ok());
        assert_eq!(payload.unwrap().len(), payload_size as usize);
    }

    #[test]
    fn test_mut_payload() {
        gst::init().unwrap();

        let csrc_count = 2;
        let payload_size = 8;
        let mut buffer = gst::Buffer::new_rtp_with_sizes(payload_size, 4, csrc_count).unwrap();
        {
            let buffer = buffer.get_mut().unwrap();
            let mut rtp_buffer = RTPBuffer::from_buffer_writable(buffer).unwrap();

            let payload = rtp_buffer.payload_mut();
            assert!(payload.is_ok());

            let payload = payload.unwrap();
            payload[3] = 42;
        }

        let rtp_buffer = RTPBuffer::from_buffer_readable(&buffer).unwrap();
        let payload = rtp_buffer.payload();

        assert!(payload.is_ok());
        assert_eq!(payload.unwrap()[3], 42);
    }

    #[test]
    fn test_extension_header_onebyte() {
        gst::init().unwrap();

        let extension_data: [u8; 4] = [100, 101, 102, 103];
        let mut buffer = gst::Buffer::new_rtp_with_sizes(16, 4, 0).unwrap();
        {
            let buffer = buffer.get_mut().unwrap();
            let mut rtp_buffer = RTPBuffer::from_buffer_writable(buffer).unwrap();

            assert_eq!(rtp_buffer.extension_bytes(), None);

            let result = rtp_buffer.add_extension_onebyte_header(1, &extension_data);
            assert!(result.is_ok());
        }

        let rtp_buffer = RTPBuffer::from_buffer_readable(&buffer).unwrap();
        let bytes_option = rtp_buffer.extension_bytes();
        assert!(bytes_option.is_some());
        let (bits, bytes) = bytes_option.unwrap();
        // 0xBEDE is the onebyte extension header marker: https://tools.ietf.org/html/rfc5285 (4.2)
        assert_eq!(bits, 0xbede);
        /*
         * bytes is:
         * * id (4 bits)
         * * size-1 (4 bits)
         * * data (with padded length to multiples of 4)
         */
        assert_eq!(bytes[0] >> 4, 1);
        assert_eq!(bytes[0] & 0xF, 3);
        for i in 0..extension_data.len() {
            assert_eq!(bytes[i + 1], extension_data[i]);
        }

        let result = rtp_buffer.extension_onebyte_header(2, 0);
        assert!(result.is_none());

        let result = rtp_buffer.extension_onebyte_header(1, 0);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), &extension_data);
    }

    #[test]
    fn test_extension_header_twobytes() {
        gst::init().unwrap();

        let extension_data: [u8; 4] = [100, 101, 102, 103];
        let appbits = 5;
        let id = 1;

        let mut buffer = gst::Buffer::new_rtp_with_sizes(16, 4, 0).unwrap();
        {
            let buffer = buffer.get_mut().unwrap();
            let mut rtp_buffer = RTPBuffer::from_buffer_writable(buffer).unwrap();

            assert_eq!(rtp_buffer.extension_bytes(), None);

            let result = rtp_buffer.add_extension_twobytes_header(appbits, id, &extension_data);
            assert!(result.is_ok());
        }

        let rtp_buffer = RTPBuffer::from_buffer_readable(&buffer).unwrap();

        let bytes_option = rtp_buffer.extension_bytes();
        assert!(bytes_option.is_some());
        let (bits, bytes) = bytes_option.unwrap();
        // 0x100 + appbits is the twobyte extension header marker:
        // https://tools.ietf.org/html/rfc5285 (4.3)
        assert_eq!(bits, 0x1000 | appbits as u16);
        /*
         * bytes is:
         * * id (1 byte)
         * * size-2 (1 byte)
         * * data (with padded length to multiples of 4)
         */
        assert_eq!(bytes[0], id);
        assert_eq!(bytes[1], extension_data.len() as u8);
        for i in 0..extension_data.len() {
            assert_eq!(bytes[i + 2], extension_data[i]);
        }

        let result = rtp_buffer.extension_twobytes_header(2, 0);
        assert!(result.is_none());

        let result = rtp_buffer.extension_twobytes_header(id, 0);
        assert!(result.is_some());
        let (extracted_appbits, data) = result.unwrap();
        assert_eq!(appbits, extracted_appbits);
        assert_eq!(data, &extension_data);
    }

    #[test]
    fn test_padding() {
        gst::init().unwrap();

        let csrc_count = 2;
        let payload_size = 16;
        let mut buffer = gst::Buffer::new_rtp_with_sizes(payload_size, 4, csrc_count).unwrap();
        {
            let rtp_buffer = RTPBuffer::from_buffer_readable(&buffer).unwrap();
            assert!(rtp_buffer.has_padding());
        }
        {
            let buffer = buffer.get_mut().unwrap();
            let mut rtp_buffer = RTPBuffer::from_buffer_writable(buffer).unwrap();

            rtp_buffer.set_padding(false);
        }

        {
            let rtp_buffer = RTPBuffer::from_buffer_readable(&buffer).unwrap();
            assert!(!rtp_buffer.has_padding());
        }
    }

    #[test]
    fn test_calc_functions() {
        let res = super::calc_header_len(0);
        assert_eq!(res, 12);
        let res = super::calc_packet_len(100, 10, 2);
        assert_eq!(res, 130);
        let res = super::calc_payload_len(100, 5, 4);
        assert_eq!(res, 67);
    }
}
