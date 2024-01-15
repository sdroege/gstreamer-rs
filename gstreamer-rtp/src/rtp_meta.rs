// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;
use std::ptr;

use glib::translate::*;
use gst::prelude::*;

#[repr(transparent)]
#[doc(alias = "GstRTPSourceMeta")]
pub struct RTPSourceMeta(ffi::GstRTPSourceMeta);

unsafe impl Send for RTPSourceMeta {}
unsafe impl Sync for RTPSourceMeta {}

impl RTPSourceMeta {
    #[doc(alias = "gst_buffer_add_rtp_source_meta")]
    pub fn add<'a>(
        buffer: &'a mut gst::BufferRef,
        ssrc: Option<u32>,
        csrc: &[u32],
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = ffi::gst_buffer_add_rtp_source_meta(
                buffer.as_mut_ptr(),
                ssrc.as_ref()
                    .map(|ssrc: &u32| ssrc as *const u32)
                    .unwrap_or(ptr::null()),
                csrc.as_ptr(),
                csrc.len() as u32,
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    #[inline]
    pub fn ssrc(&self) -> Option<u32> {
        unsafe {
            if from_glib(self.0.ssrc_valid) {
                Some(self.0.ssrc)
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn csrc(&self) -> &[u32] {
        &self.0.csrc[0..self.0.csrc_count as usize]
    }

    #[doc(alias = "gst_rtp_source_meta_set_ssrc")]
    #[inline]
    pub fn set_ssrc(&mut self, ssrc: Option<u32>) {
        unsafe {
            ffi::gst_rtp_source_meta_set_ssrc(
                &mut self.0,
                mut_override(
                    ssrc.as_ref()
                        .map(|ssrc: &u32| ssrc as *const u32)
                        .unwrap_or(ptr::null()),
                ),
            );
        }
    }

    #[inline]
    #[doc(alias = "gst_rtp_source_meta_append_csrc")]
    pub fn append_csrc(&mut self, csrc: &[u32]) -> Result<(), glib::BoolError> {
        unsafe {
            let res: bool = from_glib(ffi::gst_rtp_source_meta_append_csrc(
                &mut self.0,
                csrc.as_ptr(),
                csrc.len() as u32,
            ));

            if res {
                Ok(())
            } else {
                Err(glib::bool_error!("Failed adding another CSRC"))
            }
        }
    }
}

unsafe impl MetaAPI for RTPSourceMeta {
    type GstType = ffi::GstRTPSourceMeta;

    #[doc(alias = "gst_rtp_source_meta_api_get_type")]
    #[inline]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_rtp_source_meta_api_get_type()) }
    }
}

impl fmt::Debug for RTPSourceMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RTPSourceMeta")
            .field("ssrc", &self.ssrc())
            .field("csrc", &self.csrc())
            .finish()
    }
}
