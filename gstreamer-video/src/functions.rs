// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use gst_ffi;
use glib_ffi;

use gst;
use glib;
use glib::translate::{from_glib_full, from_glib_none, ToGlib, ToGlibPtr};

use std::ptr;
use std::mem;

pub fn convert_sample(
    sample: &gst::Sample,
    caps: &gst::Caps,
    timeout: gst::ClockTime,
) -> Result<gst::Sample, glib::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_video_convert_sample(
            sample.to_glib_none().0,
            caps.to_glib_none().0,
            timeout.to_glib(),
            &mut error,
        );

        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn convert_sample_async<F>(
    sample: &gst::Sample,
    caps: &gst::Caps,
    timeout: gst::ClockTime,
    func: F,
) where
    F: FnOnce(Result<gst::Sample, glib::Error>) + Send + 'static,
{
    unsafe extern "C" fn convert_sample_async_trampoline<F>(
        sample: *mut gst_ffi::GstSample,
        error: *mut glib_ffi::GError,
        user_data: glib_ffi::gpointer,
    ) where
        F: FnOnce(Result<gst::Sample, glib::Error>) + Send + 'static,
    {
        callback_guard!();
        let callback: &mut Option<Box<F>> = mem::transmute(user_data);
        let callback = callback.take().unwrap();

        if error.is_null() {
            callback(Ok(from_glib_none(sample)))
        } else {
            callback(Err(from_glib_none(error)))
        }
    }
    unsafe extern "C" fn convert_sample_async_free<F>(user_data: glib_ffi::gpointer)
    where
        F: FnOnce(Result<gst::Sample, glib::Error>) + Send + 'static,
    {
        callback_guard!();
        let _: Box<Option<Box<F>>> = Box::from_raw(user_data as *mut _);
    }

    unsafe {
        let user_data: Box<Option<Box<F>>> = Box::new(Some(Box::new(func)));

        ffi::gst_video_convert_sample_async(
            sample.to_glib_none().0,
            caps.to_glib_none().0,
            timeout.to_glib(),
            Some(convert_sample_async_trampoline::<F>),
            Box::into_raw(user_data) as glib_ffi::gpointer,
            Some(convert_sample_async_free::<F>),
        );
    }
}
