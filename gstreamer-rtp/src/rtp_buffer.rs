use glib::translate::{from_glib, from_glib_full};
use std::marker::PhantomData;
use std::mem;

use gst::MiniObject;
use gst_rtp_sys;

pub enum Readable {}
pub enum Writable {}

#[repr(C)]
pub struct RTPBuffer<'a, T>(gst_rtp_sys::GstRTPBuffer, &'a gst::Buffer, PhantomData<T>);

impl<'a> RTPBuffer<'a, Readable> {
    pub fn from_buffer_readable(
        buffer: &gst::Buffer,
    ) -> Result<RTPBuffer<Readable>, glib::BoolError> {
        unsafe {
            let mut rtp_buffer = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(gst_rtp_sys::gst_rtp_buffer_map(
                buffer.as_mut_ptr(),
                gst_sys::GST_MAP_READ,
                rtp_buffer.as_mut_ptr(),
            ));

            if res {
                Ok(RTPBuffer(rtp_buffer.assume_init(), buffer, PhantomData))
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
        unsafe {
            let mut rtp_buffer = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(gst_rtp_sys::gst_rtp_buffer_map(
                buffer.as_mut_ptr(),
                gst_sys::GST_MAP_READWRITE,
                rtp_buffer.as_mut_ptr(),
            ));

            if res {
                Ok(RTPBuffer(rtp_buffer.assume_init(), buffer, PhantomData))
            } else {
                Err(glib_bool_error!("Failed to map RTP buffer writable"))
            }
        }
    }

    pub fn set_seq(&mut self, seq: u16) {
        unsafe {
            gst_rtp_sys::gst_rtp_buffer_set_seq(&mut self.0, seq);
        }
    }

    pub fn set_payload_type(&mut self, pt: u8) {
        unsafe {
            gst_rtp_sys::gst_rtp_buffer_set_payload_type(&mut self.0, pt);
        }
    }

    pub fn set_timestamp(&mut self, rtptime: u32) {
        unsafe {
            gst_rtp_sys::gst_rtp_buffer_set_timestamp(&mut self.0, rtptime);
        }
    }
}

impl<'a, T> RTPBuffer<'a, T> {
    pub fn get_seq(&mut self) -> u16 {
        unsafe { gst_rtp_sys::gst_rtp_buffer_get_seq(&mut self.0) }
    }

    pub fn get_payload_type(&mut self) -> u8 {
        unsafe { gst_rtp_sys::gst_rtp_buffer_get_payload_type(&mut self.0) }
    }

    pub fn get_timestamp(&mut self) -> u32 {
        unsafe { gst_rtp_sys::gst_rtp_buffer_get_timestamp(&mut self.0) }
    }
}

impl<'a, T> Drop for RTPBuffer<'a, T> {
    fn drop(&mut self) {
        unsafe {
            gst_rtp_sys::gst_rtp_buffer_unmap(&mut self.0);
        }
    }
}

pub trait RTPBufferExt {
    fn new_rtp_with_sizes(payload_len: u32, pad_len: u8, csrc_count: u8) -> Option<gst::Buffer>;
}

impl RTPBufferExt for gst::Buffer {
    fn new_rtp_with_sizes(payload_len: u32, pad_len: u8, csrc_count: u8) -> Option<gst::Buffer> {
        unsafe {
            from_glib_full(gst_rtp_sys::gst_rtp_buffer_new_allocate(
                payload_len,
                pad_len,
                csrc_count,
            ))
        }
    }
}

pub fn compare_seqnum(seqnum1: u16, seqnum2: u16) -> i32 {
    unsafe { gst_rtp_sys::gst_rtp_buffer_compare_seqnum(seqnum1, seqnum2) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        gst::init().unwrap();

        let mut buffer = gst::Buffer::new_rtp_with_sizes(16, 4, 0).unwrap();
        let mut rtp_buffer = RTPBuffer::from_buffer_writable(&mut buffer).unwrap();

        rtp_buffer.set_seq(42);
        assert_eq!(rtp_buffer.get_seq(), 42);

        rtp_buffer.set_payload_type(43);
        assert_eq!(rtp_buffer.get_payload_type(), 43);

        rtp_buffer.set_timestamp(44);
        assert_eq!(rtp_buffer.get_timestamp(), 44);
    }
}
