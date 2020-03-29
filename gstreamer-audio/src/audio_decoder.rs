// Copyright (C) 2019 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::object::IsA;
use glib::translate::*;
use gst;
use gst_audio_sys;
use std::mem;
use std::ptr;
use AudioDecoder;
use AudioInfo;

extern "C" {
    fn _gst_audio_decoder_error(
        dec: *mut gst_audio_sys::GstAudioDecoder,
        weight: i32,
        domain: glib_sys::GQuark,
        code: i32,
        txt: *mut libc::c_char,
        debug: *mut libc::c_char,
        file: *const libc::c_char,
        function: *const libc::c_char,
        line: i32,
    ) -> gst_sys::GstFlowReturn;
}

pub trait AudioDecoderExtManual: 'static {
    fn finish_frame(
        &self,
        buffer: Option<gst::Buffer>,
        frames: i32,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn finish_subframe(
        &self,
        buffer: Option<gst::Buffer>,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn negotiate(&self) -> Result<(), gst::FlowError>;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_output_caps(&self, caps: &gst::Caps) -> Result<(), gst::FlowError>;

    fn set_output_format(&self, info: &AudioInfo) -> Result<(), gst::FlowError>;

    fn get_allocator(&self) -> (Option<gst::Allocator>, gst::AllocationParams);

    #[allow(clippy::too_many_arguments)]
    fn error<T: gst::MessageErrorDomain>(
        &self,
        weight: i32,
        code: T,
        message: Option<&str>,
        debug: Option<&str>,
        file: &str,
        function: &str,
        line: u32,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;
}

impl<O: IsA<AudioDecoder>> AudioDecoderExtManual for O {
    fn finish_frame(
        &self,
        buffer: Option<gst::Buffer>,
        frames: i32,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(gst_audio_sys::gst_audio_decoder_finish_frame(
                self.as_ref().to_glib_none().0,
                buffer.map(|b| b.into_ptr()).unwrap_or(ptr::null_mut()),
                frames,
            ))
        };
        ret.into_result()
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn finish_subframe(
        &self,
        buffer: Option<gst::Buffer>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(gst_audio_sys::gst_audio_decoder_finish_subframe(
                self.as_ref().to_glib_none().0,
                buffer.map(|b| b.into_ptr()).unwrap_or(ptr::null_mut()),
            ))
        };
        ret.into_result()
    }

    fn negotiate(&self) -> Result<(), gst::FlowError> {
        unsafe {
            let ret = from_glib(gst_audio_sys::gst_audio_decoder_negotiate(
                self.as_ref().to_glib_none().0,
            ));
            if ret {
                Ok(())
            } else {
                Err(gst::FlowError::NotNegotiated)
            }
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_output_caps(&self, caps: &gst::Caps) -> Result<(), gst::FlowError> {
        unsafe {
            let ret = from_glib(gst_audio_sys::gst_audio_decoder_set_output_caps(
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

    fn set_output_format(&self, info: &AudioInfo) -> Result<(), gst::FlowError> {
        unsafe {
            let ret = from_glib(gst_audio_sys::gst_audio_decoder_set_output_format(
                self.as_ref().to_glib_none().0,
                info.to_glib_none().0,
            ));
            if ret {
                Ok(())
            } else {
                Err(gst::FlowError::NotNegotiated)
            }
        }
    }

    fn get_allocator(&self) -> (Option<gst::Allocator>, gst::AllocationParams) {
        unsafe {
            let mut allocator = ptr::null_mut();
            let mut params = mem::zeroed();
            gst_audio_sys::gst_audio_decoder_get_allocator(
                self.as_ref().to_glib_none().0,
                &mut allocator,
                &mut params,
            );
            (from_glib_full(allocator), params.into())
        }
    }

    fn error<T: gst::MessageErrorDomain>(
        &self,
        weight: i32,
        code: T,
        message: Option<&str>,
        debug: Option<&str>,
        file: &str,
        function: &str,
        line: u32,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(_gst_audio_decoder_error(
                self.as_ref().to_glib_none().0,
                weight,
                T::domain().to_glib(),
                code.code(),
                message.to_glib_full(),
                debug.to_glib_full(),
                file.to_glib_none().0,
                function.to_glib_none().0,
                line as i32,
            ))
        };
        ret.into_result()
    }
}

#[macro_export]
macro_rules! gst_audio_decoder_error(
    ($obj:expr, $weight:expr, $err:expr, ($msg:expr), [$debug:expr]) => { {
        use $crate::AudioDecoderExtManual;
        $obj.error(
            $weight,
            $err,
            Some($msg),
            Some($debug),
            file!(),
            module_path!(),
            line!(),
        )
    }};
    ($obj:expr, $weight:expr, $err:expr, ($msg:expr)) => { {
        use $crate::AudioDecoderExtManual;
        $obj.error(
            $weight,
            $err,
            Some($msg),
            None,
            file!(),
            module_path!(),
            line!(),
        )
    }};
    ($obj:expr, $weight:expr, $err:expr, [$debug:expr]) => { {
        use $crate::AudioDecoderExtManual;
        $obj.error(
            $weight,
            $err,
            None,
            Some($debug),
            file!(),
            module_path!(),
            line!(),
        )
    }};
    ($obj:expr, $weight:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*]) => { {
        use $crate::AudioDecoderExtManual;
        $obj.error(
            $weight,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
        )
    }};
    ($obj:expr, $weight:expr, $err:expr, ($($msg:tt)*)) => { {
        use $crate::AudioDecoderExtManual;
        $obj.error(
            $weight,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            module_path!(),
            line!(),
        )
    }};
    ($obj:expr, $weight:expr, $err:expr, [$($debug:tt)*]) => { {
        use $crate::AudioDecoderExtManual;
        $obj.error(
            $weight,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
        )
    }};
);
