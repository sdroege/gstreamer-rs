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
    eos: Option<Box<Fn(&AppSink) + Send + Sync + 'static>>,
    new_preroll: Option<Box<Fn(&AppSink) -> gst::FlowReturn + Send + Sync + 'static>>,
    new_sample: Option<Box<Fn(&AppSink) -> gst::FlowReturn + Send + Sync + 'static>>,
    callbacks: ffi::GstAppSinkCallbacks,
}

unsafe impl Send for AppSinkCallbacks {}
unsafe impl Sync for AppSinkCallbacks {}

impl AppSinkCallbacks {
    pub fn new() -> AppSinkCallbacksBuilder {
        skip_assert_initialized!();
        AppSinkCallbacksBuilder {
            eos: None,
            new_preroll: None,
            new_sample: None,
        }
    }
}

pub struct AppSinkCallbacksBuilder {
    eos: Option<Box<Fn(&AppSink) + Send + Sync + 'static>>,
    new_preroll: Option<Box<Fn(&AppSink) -> gst::FlowReturn + Send + Sync + 'static>>,
    new_sample: Option<Box<Fn(&AppSink) -> gst::FlowReturn + Send + Sync + 'static>>,
}

impl AppSinkCallbacksBuilder {
    pub fn eos<F: Fn(&AppSink) + Send + Sync + 'static>(self, eos: F) -> Self {
        Self {
            eos: Some(Box::new(eos)),
            ..self
        }
    }

    pub fn new_preroll<F: Fn(&AppSink) -> gst::FlowReturn + Send + Sync + 'static>(
        self,
        new_preroll: F,
    ) -> Self {
        Self {
            new_preroll: Some(Box::new(new_preroll)),
            ..self
        }
    }

    pub fn new_sample<F: Fn(&AppSink) -> gst::FlowReturn + Send + Sync + 'static>(
        self,
        new_sample: F,
    ) -> Self {
        Self {
            new_sample: Some(Box::new(new_sample)),
            ..self
        }
    }

    pub fn build(self) -> AppSinkCallbacks {
        let have_eos = self.eos.is_some();
        let have_new_preroll = self.new_preroll.is_some();
        let have_new_sample = self.new_sample.is_some();

        AppSinkCallbacks {
            eos: self.eos,
            new_preroll: self.new_preroll,
            new_sample: self.new_sample,
            callbacks: ffi::GstAppSinkCallbacks {
                eos: if have_eos { Some(trampoline_eos) } else { None },
                new_preroll: if have_new_preroll {
                    Some(trampoline_new_preroll)
                } else {
                    None
                },
                new_sample: if have_new_sample {
                    Some(trampoline_new_sample)
                } else {
                    None
                },
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

    callbacks
        .eos
        .as_ref()
        .map(|f| f(&from_glib_borrow(appsink)));
}

unsafe extern "C" fn trampoline_new_preroll(
    appsink: *mut ffi::GstAppSink,
    callbacks: gpointer,
) -> gst_ffi::GstFlowReturn {
    let _guard = CallbackGuard::new();
    let callbacks = &*(callbacks as *const AppSinkCallbacks);

    callbacks
        .new_preroll
        .as_ref()
        .map(|f| f(&from_glib_borrow(appsink)))
        .unwrap_or(gst::FlowReturn::Error)
        .to_glib()
}

unsafe extern "C" fn trampoline_new_sample(
    appsink: *mut ffi::GstAppSink,
    callbacks: gpointer,
) -> gst_ffi::GstFlowReturn {
    let _guard = CallbackGuard::new();
    let callbacks = &*(callbacks as *const AppSinkCallbacks);

    callbacks
        .new_sample
        .as_ref()
        .map(|f| f(&from_glib_borrow(appsink)))
        .unwrap_or(gst::FlowReturn::Error)
        .to_glib()
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
