// Take a look at the license at the top of the repository in the LICENSE file.

use crate::AudioEncoder;
use glib::prelude::*;
use glib::translate::*;
use std::mem;
use std::ptr;

pub trait AudioEncoderExtManual: 'static {
    #[doc(alias = "gst_audio_encoder_finish_frame")]
    fn finish_frame(
        &self,
        buffer: Option<gst::Buffer>,
        frames: i32,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[doc(alias = "gst_audio_encoder_negotiate")]
    fn negotiate(&self) -> Result<(), gst::FlowError>;

    #[doc(alias = "gst_audio_encoder_set_output_format")]
    fn set_output_format(&self, caps: &gst::Caps) -> Result<(), gst::FlowError>;

    #[doc(alias = "get_allocator")]
    #[doc(alias = "gst_audio_encoder_get_allocator")]
    fn allocator(&self) -> (Option<gst::Allocator>, gst::AllocationParams);

    fn sink_pad(&self) -> gst::Pad;

    fn src_pad(&self) -> gst::Pad;
}

impl<O: IsA<AudioEncoder>> AudioEncoderExtManual for O {
    fn finish_frame(
        &self,
        buffer: Option<gst::Buffer>,
        frames: i32,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_audio_encoder_finish_frame(
                self.as_ref().to_glib_none().0,
                buffer.map(|b| b.into_ptr()).unwrap_or(ptr::null_mut()),
                frames,
            ))
        }
    }

    fn negotiate(&self) -> Result<(), gst::FlowError> {
        unsafe {
            let ret = from_glib(ffi::gst_audio_encoder_negotiate(
                self.as_ref().to_glib_none().0,
            ));
            if ret {
                Ok(())
            } else {
                Err(gst::FlowError::NotNegotiated)
            }
        }
    }

    fn set_output_format(&self, caps: &gst::Caps) -> Result<(), gst::FlowError> {
        unsafe {
            let ret = from_glib(ffi::gst_audio_encoder_set_output_format(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
            ));
            if ret {
                Ok(())
            } else {
                Err(gst::FlowError::NotNegotiated)
            }
        }
    }

    fn allocator(&self) -> (Option<gst::Allocator>, gst::AllocationParams) {
        unsafe {
            let mut allocator = ptr::null_mut();
            let mut params = mem::zeroed();
            ffi::gst_audio_encoder_get_allocator(
                self.as_ref().to_glib_none().0,
                &mut allocator,
                &mut params,
            );
            (from_glib_full(allocator), params.into())
        }
    }

    fn sink_pad(&self) -> gst::Pad {
        unsafe {
            let elt: &ffi::GstAudioEncoder = &*(self.as_ptr() as *const _);
            from_glib_none(elt.sinkpad)
        }
    }

    fn src_pad(&self) -> gst::Pad {
        unsafe {
            let elt: &ffi::GstAudioEncoder = &*(self.as_ptr() as *const _);
            from_glib_none(elt.srcpad)
        }
    }
}
