use glib::translate::{from_glib, from_glib_full, FromGlibPtrFull, ToGlib};
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::slice;

use gst_rtp_sys;

pub enum Readable {}
pub enum Writable {}

pub struct RTPBuffer<'a, T> {
    rtp_buffer: gst_rtp_sys::GstRTPBuffer,
    buffer: &'a gst::Buffer,
    phantom: PhantomData<T>,
}

unsafe impl<'a, T> Send for RTPBuffer<'a, T> {}
unsafe impl<'a, T> Sync for RTPBuffer<'a, T> {}

impl<'a, T> fmt::Debug for RTPBuffer<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RTPBuffer")
            .field("rtp_buffer", &self.rtp_buffer)
            .field("buffer", &self.buffer)
            .field("phantom", &self.phantom)
            .finish()
    }
}

impl<'a> RTPBuffer<'a, Readable> {
    pub fn from_buffer_readable(
        buffer: &gst::Buffer,
    ) -> Result<RTPBuffer<Readable>, glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            let mut rtp_buffer = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(gst_rtp_sys::gst_rtp_buffer_map(
                buffer.as_mut_ptr(),
                gst_sys::GST_MAP_READ,
                rtp_buffer.as_mut_ptr(),
            ));

            if res {
                Ok(RTPBuffer {
                    rtp_buffer: rtp_buffer.assume_init(),
                    buffer,
                    phantom: PhantomData,
                })
            } else {
                Err(glib_bool_error!("Failed to map RTP buffer readable"))
            }
        }
    }
}

impl<'a> RTPBuffer<'a, Writable> {
    pub fn from_buffer_writable(
        buffer: &mut gst::Buffer,
    ) -> Result<RTPBuffer<Writable>, glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            let mut rtp_buffer = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(gst_rtp_sys::gst_rtp_buffer_map(
                buffer.as_mut_ptr(),
                gst_sys::GST_MAP_READWRITE,
                rtp_buffer.as_mut_ptr(),
            ));

            if res {
                Ok(RTPBuffer {
                    rtp_buffer: rtp_buffer.assume_init(),
                    buffer,
                    phantom: PhantomData,
                })
            } else {
                Err(glib_bool_error!("Failed to map RTP buffer writable"))
            }
        }
    }

    pub fn set_seq(&mut self, seq: u16) {
        unsafe {
            gst_rtp_sys::gst_rtp_buffer_set_seq(&mut self.rtp_buffer, seq);
        }
    }

    pub fn set_marker(&mut self, m: bool) {
        unsafe {
            gst_rtp_sys::gst_rtp_buffer_set_marker(&mut self.rtp_buffer, m.to_glib());
        }
    }

    pub fn set_payload_type(&mut self, pt: u8) {
        unsafe {
            gst_rtp_sys::gst_rtp_buffer_set_payload_type(&mut self.rtp_buffer, pt);
        }
    }

    pub fn set_ssrc(&mut self, ssrc: u32) {
        unsafe { gst_rtp_sys::gst_rtp_buffer_set_ssrc(&mut self.rtp_buffer, ssrc) }
    }

    pub fn set_csrc(&mut self, idx: u8, ssrc: u32) {
        unsafe { gst_rtp_sys::gst_rtp_buffer_set_csrc(&mut self.rtp_buffer, idx, ssrc) }
    }

    pub fn set_timestamp(&mut self, rtptime: u32) {
        unsafe {
            gst_rtp_sys::gst_rtp_buffer_set_timestamp(&mut self.rtp_buffer, rtptime);
        }
    }

    pub fn set_extension(&mut self, extension: bool) {
        unsafe {
            gst_rtp_sys::gst_rtp_buffer_set_extension(&mut self.rtp_buffer, extension.to_glib())
        }
    }

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
            let result: bool = from_glib(gst_rtp_sys::gst_rtp_buffer_add_extension_onebyte_header(
                &mut self.rtp_buffer,
                id,
                data.as_ptr() as glib_sys::gconstpointer,
                data.len() as u32,
            ));
            if result {
                Ok(())
            } else {
                Err(glib_bool_error!("Failed to add onebyte header extension"))
            }
        }
    }

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
            let result: bool =
                from_glib(gst_rtp_sys::gst_rtp_buffer_add_extension_twobytes_header(
                    &mut self.rtp_buffer,
                    appbits,
                    id,
                    data.as_ptr() as glib_sys::gconstpointer,
                    data.len() as u32,
                ));
            if result {
                Ok(())
            } else {
                Err(glib_bool_error!("Failed to add twobytes header extension"))
            }
        }
    }
}

impl<'a, T> RTPBuffer<'a, T> {
    pub fn get_seq(&self) -> u16 {
        unsafe {
            gst_rtp_sys::gst_rtp_buffer_get_seq(glib::translate::mut_override(&self.rtp_buffer))
        }
    }

    pub fn get_payload_type(&self) -> u8 {
        unsafe {
            gst_rtp_sys::gst_rtp_buffer_get_payload_type(glib::translate::mut_override(
                &self.rtp_buffer,
            ))
        }
    }

    pub fn get_ssrc(&self) -> u32 {
        unsafe {
            gst_rtp_sys::gst_rtp_buffer_get_ssrc(glib::translate::mut_override(&self.rtp_buffer))
        }
    }

    pub fn get_timestamp(&self) -> u32 {
        unsafe {
            gst_rtp_sys::gst_rtp_buffer_get_timestamp(glib::translate::mut_override(
                &self.rtp_buffer,
            ))
        }
    }

    pub fn get_csrc(&self, idx: u8) -> Option<u32> {
        if idx < self.get_csrc_count() {
            unsafe {
                Some(gst_rtp_sys::gst_rtp_buffer_get_csrc(
                    glib::translate::mut_override(&self.rtp_buffer),
                    idx,
                ))
            }
        } else {
            None
        }
    }

    pub fn get_csrc_count(&self) -> u8 {
        unsafe {
            gst_rtp_sys::gst_rtp_buffer_get_csrc_count(glib::translate::mut_override(
                &self.rtp_buffer,
            ))
        }
    }

    pub fn get_marker(&self) -> bool {
        unsafe {
            from_glib(gst_rtp_sys::gst_rtp_buffer_get_marker(
                glib::translate::mut_override(&self.rtp_buffer),
            ))
        }
    }

    pub fn get_payload_size(&self) -> u32 {
        unsafe {
            gst_rtp_sys::gst_rtp_buffer_get_payload_len(glib::translate::mut_override(
                &self.rtp_buffer,
            )) as u32
        }
    }

    pub fn get_payload(&self) -> Result<&[u8], glib::error::BoolError> {
        let size = self.get_payload_size();
        if size == 0 {
            return Ok(&[]);
        }
        unsafe {
            let pointer = gst_rtp_sys::gst_rtp_buffer_get_payload(glib::translate::mut_override(
                &self.rtp_buffer,
            ));
            if pointer.is_null() {
                Err(glib_bool_error!("Failed to get payload data"))
            } else {
                Ok(slice::from_raw_parts(pointer as *const u8, size as usize))
            }
        }
    }

    pub fn get_extension(&self) -> bool {
        unsafe {
            from_glib(gst_rtp_sys::gst_rtp_buffer_get_extension(
                glib::translate::mut_override(&self.rtp_buffer),
            ))
        }
    }

    pub fn get_extension_bytes(&self) -> Option<(u16, glib::Bytes)> {
        unsafe {
            let mut bits: u16 = 0;
            match from_glib_full(gst_rtp_sys::gst_rtp_buffer_get_extension_bytes(
                glib::translate::mut_override(&self.rtp_buffer),
                &mut bits,
            )) {
                Some(bytes) => Some((bits, bytes)),
                None => None,
            }
        }
    }

    pub fn get_extension_onebyte_header(&self, id: u8, nth: u32) -> Option<&[u8]> {
        unsafe {
            let mut data = ptr::null_mut();
            // FIXME: Workaround for gstreamer-rtp-sys having the wrong type for this parameter
            let data_ptr = &mut data as *mut *mut u8 as *mut u8;
            let mut size: u32 = 0;
            let result: bool = from_glib(gst_rtp_sys::gst_rtp_buffer_get_extension_onebyte_header(
                glib::translate::mut_override(&self.rtp_buffer),
                id,
                nth,
                data_ptr,
                &mut size,
            ));
            if result {
                Some(slice::from_raw_parts(data as *const u8, size as usize))
            } else {
                None
            }
        }
    }

    pub fn get_extension_twobytes_header(&self, id: u8, nth: u32) -> Option<(u8, &[u8])> {
        unsafe {
            let mut data = ptr::null_mut();
            // FIXME: Workaround for gstreamer-rtp-sys having the wrong type for this parameter
            let data_ptr = &mut data as *mut *mut u8 as *mut u8;
            let mut size: u32 = 0;
            let mut appbits = 0;
            let result: bool =
                from_glib(gst_rtp_sys::gst_rtp_buffer_get_extension_twobytes_header(
                    glib::translate::mut_override(&self.rtp_buffer),
                    &mut appbits,
                    id,
                    nth,
                    data_ptr,
                    &mut size,
                ));
            if result {
                Some((
                    appbits,
                    slice::from_raw_parts(data as *const u8, size as usize),
                ))
            } else {
                None
            }
        }
    }
}

impl<'a, T> Drop for RTPBuffer<'a, T> {
    fn drop(&mut self) {
        unsafe {
            gst_rtp_sys::gst_rtp_buffer_unmap(&mut self.rtp_buffer);
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
            Option::<_>::from_glib_full(gst_rtp_sys::gst_rtp_buffer_new_allocate(
                payload_len,
                pad_len,
                csrc_count,
            ))
            .ok_or_else(|| glib_bool_error!("Failed to allocate new RTP buffer"))
        }
    }
}

pub fn compare_seqnum(seqnum1: u16, seqnum2: u16) -> i32 {
    skip_assert_initialized!();
    unsafe { gst_rtp_sys::gst_rtp_buffer_compare_seqnum(seqnum1, seqnum2) }
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
        let mut rtp_buffer = RTPBuffer::from_buffer_writable(&mut buffer).unwrap();

        rtp_buffer.set_seq(42);
        assert_eq!(rtp_buffer.get_seq(), 42);

        rtp_buffer.set_marker(true);
        assert!(rtp_buffer.get_marker());

        rtp_buffer.set_payload_type(43);
        assert_eq!(rtp_buffer.get_payload_type(), 43);

        rtp_buffer.set_timestamp(44);
        assert_eq!(rtp_buffer.get_timestamp(), 44);

        rtp_buffer.set_ssrc(45);
        assert_eq!(rtp_buffer.get_ssrc(), 45);

        assert_eq!(rtp_buffer.get_payload_size(), payload_size);
        let payload = rtp_buffer.get_payload();
        assert!(payload.is_ok());
        let payload = payload.unwrap();
        assert_eq!(payload.len(), payload_size as usize);

        assert_eq!(rtp_buffer.get_csrc_count(), csrc_count);
        rtp_buffer.set_csrc(0, 12);
        rtp_buffer.set_csrc(1, 15);
        assert_eq!(rtp_buffer.get_csrc(0).unwrap(), 12);
        assert_eq!(rtp_buffer.get_csrc(1).unwrap(), 15);
        assert!(rtp_buffer.get_csrc(2).is_none());

        rtp_buffer.set_extension(true);
        assert_eq!(rtp_buffer.get_extension(), true);

        assert_eq!(rtp_buffer.get_extension_bytes(), None);
    }

    #[test]
    fn test_empty_payload() {
        gst::init().unwrap();

        let csrc_count = 0;
        let payload_size = 0;
        let buffer = gst::Buffer::new_rtp_with_sizes(payload_size, 4, csrc_count).unwrap();
        let rtp_buffer = RTPBuffer::from_buffer_readable(&buffer).unwrap();

        assert_eq!(rtp_buffer.get_payload_size(), payload_size);
        let payload = rtp_buffer.get_payload();
        assert!(payload.is_ok());
        assert_eq!(payload.unwrap().len(), payload_size as usize);
    }

    #[test]
    fn test_extension_header_onebyte() {
        gst::init().unwrap();

        let mut buffer = gst::Buffer::new_rtp_with_sizes(16, 4, 0).unwrap();
        let mut rtp_buffer = RTPBuffer::from_buffer_writable(&mut buffer).unwrap();

        assert_eq!(rtp_buffer.get_extension_bytes(), None);

        let extension_data: [u8; 4] = [100, 101, 102, 103];
        let result = rtp_buffer.add_extension_onebyte_header(1, &extension_data);
        assert!(result.is_ok());

        let bytes_option = rtp_buffer.get_extension_bytes();
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

        let result = rtp_buffer.get_extension_onebyte_header(2, 0);
        assert!(result.is_none());

        let result = rtp_buffer.get_extension_onebyte_header(1, 0);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), &extension_data);
    }

    #[test]
    fn test_extension_header_twobytes() {
        gst::init().unwrap();

        let mut buffer = gst::Buffer::new_rtp_with_sizes(16, 4, 0).unwrap();
        let mut rtp_buffer = RTPBuffer::from_buffer_writable(&mut buffer).unwrap();

        assert_eq!(rtp_buffer.get_extension_bytes(), None);

        let extension_data: [u8; 4] = [100, 101, 102, 103];
        let appbits = 5;
        let id = 1;
        let result = rtp_buffer.add_extension_twobytes_header(appbits, id, &extension_data);
        assert!(result.is_ok());

        let bytes_option = rtp_buffer.get_extension_bytes();
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

        let result = rtp_buffer.get_extension_twobytes_header(2, 0);
        assert!(result.is_none());

        let result = rtp_buffer.get_extension_twobytes_header(id, 0);
        assert!(result.is_some());
        let (extracted_appbits, data) = result.unwrap();
        assert_eq!(appbits, extracted_appbits);
        assert_eq!(data, &extension_data);
    }
}
