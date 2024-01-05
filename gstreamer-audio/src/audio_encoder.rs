// Take a look at the license at the top of the repository in the LICENSE file.

use std::{mem, ptr};

use glib::{prelude::*, translate::*};

use crate::AudioEncoder;

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::AudioEncoder>> Sealed for T {}
}

pub trait AudioEncoderExtManual: sealed::Sealed + IsA<AudioEncoder> + 'static {
    #[doc(alias = "gst_audio_encoder_negotiate")]
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

    #[doc(alias = "gst_audio_encoder_set_output_format")]
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

    #[doc(alias = "get_allocator")]
    #[doc(alias = "gst_audio_encoder_get_allocator")]
    fn allocator(&self) -> (Option<gst::Allocator>, gst::AllocationParams) {
        unsafe {
            let mut allocator = ptr::null_mut();
            let mut params = mem::MaybeUninit::uninit();
            ffi::gst_audio_encoder_get_allocator(
                self.as_ref().to_glib_none().0,
                &mut allocator,
                params.as_mut_ptr(),
            );
            (from_glib_full(allocator), params.assume_init().into())
        }
    }

    #[doc(alias = "gst_audio_encoder_set_headers")]
    fn set_headers(&self, headers: impl IntoIterator<Item = gst::Buffer>) {
        unsafe {
            ffi::gst_audio_encoder_set_headers(
                self.as_ref().to_glib_none().0,
                headers
                    .into_iter()
                    .collect::<glib::List<_>>()
                    .into_glib_ptr(),
            );
        }
    }

    fn sink_pad(&self) -> &gst::Pad {
        unsafe {
            let elt = &*(self.as_ptr() as *const ffi::GstAudioEncoder);
            &*(&elt.sinkpad as *const *mut gst::ffi::GstPad as *const gst::Pad)
        }
    }

    fn src_pad(&self) -> &gst::Pad {
        unsafe {
            let elt = &*(self.as_ptr() as *const ffi::GstAudioEncoder);
            &*(&elt.srcpad as *const *mut gst::ffi::GstPad as *const gst::Pad)
        }
    }

    fn input_segment(&self) -> gst::Segment {
        unsafe {
            let ptr: &ffi::GstAudioDecoder = &*(self.as_ptr() as *const _);
            glib::ffi::g_rec_mutex_lock(mut_override(&ptr.stream_lock));
            let segment = ptr.input_segment;
            glib::ffi::g_rec_mutex_unlock(mut_override(&ptr.stream_lock));
            from_glib_none(&segment as *const gst::ffi::GstSegment)
        }
    }

    fn output_segment(&self) -> gst::Segment {
        unsafe {
            let ptr: &ffi::GstAudioDecoder = &*(self.as_ptr() as *const _);
            glib::ffi::g_rec_mutex_lock(mut_override(&ptr.stream_lock));
            let segment = ptr.output_segment;
            glib::ffi::g_rec_mutex_unlock(mut_override(&ptr.stream_lock));
            from_glib_none(&segment as *const gst::ffi::GstSegment)
        }
    }
}

impl<O: IsA<AudioEncoder>> AudioEncoderExtManual for O {}
