// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use AppSrc;
use ffi;
use glib::translate::*;
use gst;
use glib::source::CallbackGuard;
use glib_ffi::{gboolean, gpointer};
use std::ptr;

pub struct AppSrcCallbacks {
    need_data: Box<Fn(&AppSrc, u32) + Send + Sync + 'static>,
    enough_data: Box<Fn(&AppSrc) + Send + Sync + 'static>,
    seek_data: Box<Fn(&AppSrc, u64) -> bool + Send + Sync + 'static>,
    callbacks: ffi::GstAppSrcCallbacks,
}

impl AppSrcCallbacks {
    pub fn new<F, G, H>(need_data: F, enough_data: G, seek_data: H) -> Self
    where
        F: Fn(&AppSrc, u32) + Send + Sync + 'static,
        G: Fn(&AppSrc) + Send + Sync + 'static,
        H: Fn(&AppSrc, u64) -> bool + Send + Sync + 'static,
    {
        AppSrcCallbacks {
            need_data: Box::new(need_data),
            enough_data: Box::new(enough_data),
            seek_data: Box::new(seek_data),
            callbacks: ffi::GstAppSrcCallbacks {
                need_data: Some(trampoline_need_data),
                enough_data: Some(trampoline_enough_data),
                seek_data: Some(trampoline_seek_data),
                _gst_reserved: [
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                ],
            },
        }
    }
}

unsafe extern "C" fn trampoline_need_data(
    appsrc: *mut ffi::GstAppSrc,
    length: u32,
    callbacks: gpointer,
) {
    let _guard = CallbackGuard::new();
    let callbacks = &*(callbacks as *const AppSrcCallbacks);

    (callbacks.need_data)(&from_glib_none(appsrc), length);
}

unsafe extern "C" fn trampoline_enough_data(appsrc: *mut ffi::GstAppSrc, callbacks: gpointer) {
    let _guard = CallbackGuard::new();
    let callbacks = &*(callbacks as *const AppSrcCallbacks);

    (callbacks.enough_data)(&from_glib_none(appsrc));
}

unsafe extern "C" fn trampoline_seek_data(
    appsrc: *mut ffi::GstAppSrc,
    offset: u64,
    callbacks: gpointer,
) -> gboolean {
    let _guard = CallbackGuard::new();
    let callbacks = &*(callbacks as *const AppSrcCallbacks);

    (callbacks.seek_data)(&from_glib_none(appsrc), offset).to_glib()
}

unsafe extern "C" fn destroy_callbacks(ptr: gpointer) {
    let _guard = CallbackGuard::new();
    Box::<Box<AppSrcCallbacks>>::from_raw(ptr as *mut _);
}

impl AppSrc {
    pub fn push_buffer(&self, buffer: gst::Buffer) -> gst::FlowReturn {
        unsafe {
            from_glib(ffi::gst_app_src_push_buffer(
                self.to_glib_none().0,
                buffer.into_ptr(),
            ))
        }
    }

    pub fn set_callbacks(&self, callbacks: AppSrcCallbacks) {
        unsafe {
            ffi::gst_app_src_set_callbacks(
                self.to_glib_none().0,
                mut_override(&callbacks.callbacks),
                Box::into_raw(Box::new(callbacks)) as *mut _,
                Some(destroy_callbacks),
            );
        }
    }
}
