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
use AudioEncoder;

pub trait AudioEncoderExtManual: 'static {
    fn finish_frame(
        &self,
        buffer: Option<gst::Buffer>,
        frames: i32,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn negotiate(&self) -> Result<(), gst::FlowError>;

    fn set_output_format(&self, caps: &gst::Caps) -> Result<(), gst::FlowError>;

    fn get_allocator(&self) -> (gst::Allocator, gst::AllocationParams);

    fn get_latency(&self) -> (gst::ClockTime, gst::ClockTime);
}

impl<O: IsA<AudioEncoder>> AudioEncoderExtManual for O {
    fn finish_frame(
        &self,
        buffer: Option<gst::Buffer>,
        frames: i32,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(gst_audio_sys::gst_audio_encoder_finish_frame(
                self.as_ref().to_glib_none().0,
                buffer.map(|b| b.into_ptr()).unwrap_or(ptr::null_mut()),
                frames,
            ))
        };
        ret.into_result()
    }

    fn negotiate(&self) -> Result<(), gst::FlowError> {
        unsafe {
            let ret = from_glib(gst_audio_sys::gst_audio_encoder_negotiate(
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
            let ret = from_glib(gst_audio_sys::gst_audio_encoder_set_output_format(
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

    fn get_allocator(&self) -> (gst::Allocator, gst::AllocationParams) {
        unsafe {
            let mut allocator = ptr::null_mut();
            let mut params = mem::zeroed();
            gst_audio_sys::gst_audio_encoder_get_allocator(
                self.as_ref().to_glib_none().0,
                &mut allocator,
                &mut params,
            );
            (from_glib_full(allocator), params.into())
        }
    }

    fn get_latency(&self) -> (gst::ClockTime, gst::ClockTime) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            gst_audio_sys::gst_audio_encoder_get_latency(
                self.as_ref().to_glib_none().0,
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );
            let min = min.assume_init();
            let max = max.assume_init();
            (from_glib(min), from_glib(max))
        }
    }
}
