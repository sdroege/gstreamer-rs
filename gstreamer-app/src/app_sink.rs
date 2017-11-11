// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use AppSink;
use ffi;
use gst_ffi;
use glib::translate::*;
use gst;
use glib::source::CallbackGuard;
use glib_ffi::gpointer;
use std::ptr;

pub struct AppSinkCallbacks {
    eos: Box<Fn(&AppSink) + Send + Sync + 'static>,
    new_preroll: Box<Fn(&AppSink) -> gst::FlowReturn + Send + Sync + 'static>,
    new_sample: Box<Fn(&AppSink) -> gst::FlowReturn + Send + Sync + 'static>,
    callbacks: ffi::GstAppSinkCallbacks,
}

impl AppSinkCallbacks {
    pub fn new<F, G, H>(eos: F, new_preroll: G, new_sample: H) -> Self
    where
        F: Fn(&AppSink) + Send + Sync + 'static,
        G: Fn(&AppSink) -> gst::FlowReturn + Send + Sync + 'static,
        H: Fn(&AppSink) -> gst::FlowReturn + Send + Sync + 'static,
    {
        skip_assert_initialized!();

        AppSinkCallbacks {
            eos: Box::new(eos),
            new_preroll: Box::new(new_preroll),
            new_sample: Box::new(new_sample),
            callbacks: ffi::GstAppSinkCallbacks {
                eos: Some(trampoline_eos),
                new_preroll: Some(trampoline_new_preroll),
                new_sample: Some(trampoline_new_sample),
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

unsafe extern "C" fn trampoline_eos(appsink: *mut ffi::GstAppSink, callbacks: gpointer) {
    let _guard = CallbackGuard::new();
    let callbacks = &*(callbacks as *const AppSinkCallbacks);

    (callbacks.eos)(&from_glib_borrow(appsink));
}

unsafe extern "C" fn trampoline_new_preroll(
    appsink: *mut ffi::GstAppSink,
    callbacks: gpointer,
) -> gst_ffi::GstFlowReturn {
    let _guard = CallbackGuard::new();
    let callbacks = &*(callbacks as *const AppSinkCallbacks);

    (callbacks.new_preroll)(&from_glib_borrow(appsink)).to_glib()
}

unsafe extern "C" fn trampoline_new_sample(
    appsink: *mut ffi::GstAppSink,
    callbacks: gpointer,
) -> gst_ffi::GstFlowReturn {
    let _guard = CallbackGuard::new();
    let callbacks = &*(callbacks as *const AppSinkCallbacks);

    (callbacks.new_sample)(&from_glib_borrow(appsink)).to_glib()
}

unsafe extern "C" fn destroy_callbacks(ptr: gpointer) {
    let _guard = CallbackGuard::new();
    Box::<AppSinkCallbacks>::from_raw(ptr as *mut _);
}

impl AppSink {
    pub fn set_callbacks(&self, callbacks: AppSinkCallbacks) {
        unsafe {
            ffi::gst_app_sink_set_callbacks(
                self.to_glib_none().0,
                mut_override(&callbacks.callbacks),
                Box::into_raw(Box::new(callbacks)) as *mut _,
                Some(destroy_callbacks),
            );
        }
    }
}
