// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use AppSrc;
use ffi;
use glib::source::CallbackGuard;
use glib::translate::*;
use glib_ffi::{gboolean, gpointer};
use gst;
use std::mem;
use std::ptr;
use std::cell::RefCell;

pub struct AppSrcCallbacks {
    need_data: Option<RefCell<Box<FnMut(&AppSrc, u32) + Send + 'static>>>,
    enough_data: Option<Box<Fn(&AppSrc) + Send + Sync + 'static>>,
    seek_data: Option<Box<Fn(&AppSrc, u64) -> bool + Send + Sync + 'static>>,
    callbacks: ffi::GstAppSrcCallbacks,
}

unsafe impl Send for AppSrcCallbacks {}
unsafe impl Sync for AppSrcCallbacks {}

impl AppSrcCallbacks {
    pub fn new() -> AppSrcCallbacksBuilder {
        skip_assert_initialized!();

        AppSrcCallbacksBuilder {
            need_data: None,
            enough_data: None,
            seek_data: None,
        }
    }
}

pub struct AppSrcCallbacksBuilder {
    need_data: Option<RefCell<Box<FnMut(&AppSrc, u32) + Send + 'static>>>,
    enough_data: Option<Box<Fn(&AppSrc) + Send + Sync + 'static>>,
    seek_data: Option<Box<Fn(&AppSrc, u64) -> bool + Send + Sync + 'static>>,
}

impl AppSrcCallbacksBuilder {
    pub fn need_data<F: FnMut(&AppSrc, u32) + Send + 'static>(self, need_data: F) -> Self {
        Self {
            need_data: Some(RefCell::new(Box::new(need_data))),
            ..self
        }
    }

    pub fn enough_data<F: Fn(&AppSrc) + Send + Sync + 'static>(self, enough_data: F) -> Self {
        Self {
            enough_data: Some(Box::new(enough_data)),
            ..self
        }
    }

    pub fn seek_data<F: Fn(&AppSrc, u64) -> bool + Send + Sync + 'static>(
        self,
        seek_data: F,
    ) -> Self {
        Self {
            seek_data: Some(Box::new(seek_data)),
            ..self
        }
    }

    pub fn build(self) -> AppSrcCallbacks {
        let have_need_data = self.need_data.is_some();
        let have_enough_data = self.enough_data.is_some();
        let have_seek_data = self.seek_data.is_some();

        AppSrcCallbacks {
            need_data: self.need_data,
            enough_data: self.enough_data,
            seek_data: self.seek_data,
            callbacks: ffi::GstAppSrcCallbacks {
                need_data: if have_need_data {
                    Some(trampoline_need_data)
                } else {
                    None
                },
                enough_data: if have_enough_data {
                    Some(trampoline_enough_data)
                } else {
                    None
                },
                seek_data: if have_seek_data {
                    Some(trampoline_seek_data)
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

unsafe extern "C" fn trampoline_need_data(
    appsrc: *mut ffi::GstAppSrc,
    length: u32,
    callbacks: gpointer,
) {
    let _guard = CallbackGuard::new();
    let callbacks = &*(callbacks as *const AppSrcCallbacks);

    callbacks
        .need_data
        .as_ref()
        .map(|f| (&mut *f.borrow_mut())(&from_glib_borrow(appsrc), length));
}

unsafe extern "C" fn trampoline_enough_data(appsrc: *mut ffi::GstAppSrc, callbacks: gpointer) {
    let _guard = CallbackGuard::new();
    let callbacks = &*(callbacks as *const AppSrcCallbacks);

    callbacks
        .enough_data
        .as_ref()
        .map(|f| f(&from_glib_borrow(appsrc)));
}

unsafe extern "C" fn trampoline_seek_data(
    appsrc: *mut ffi::GstAppSrc,
    offset: u64,
    callbacks: gpointer,
) -> gboolean {
    let _guard = CallbackGuard::new();
    let callbacks = &*(callbacks as *const AppSrcCallbacks);

    callbacks
        .seek_data
        .as_ref()
        .map(|f| f(&from_glib_borrow(appsrc), offset))
        .unwrap_or(false)
        .to_glib()
}

unsafe extern "C" fn destroy_callbacks(ptr: gpointer) {
    let _guard = CallbackGuard::new();
    Box::<AppSrcCallbacks>::from_raw(ptr as *mut _);
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

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn push_buffer_list(&self, list: gst::BufferList) -> gst::FlowReturn {
        unsafe {
            from_glib(ffi::gst_app_src_push_buffer_list(
                self.to_glib_none().0,
                list.into_ptr(),
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

    pub fn set_latency(&self, min: gst::ClockTime, max: gst::ClockTime) {
        unsafe {
            ffi::gst_app_src_set_latency(self.to_glib_none().0, min.to_glib(), max.to_glib());
        }
    }

    pub fn get_latency(&self) -> (gst::ClockTime, gst::ClockTime) {
        unsafe {
            let mut min = mem::uninitialized();
            let mut max = mem::uninitialized();
            ffi::gst_app_src_get_latency(self.to_glib_none().0, &mut min, &mut max);
            (from_glib(min), from_glib(max))
        }
    }
}
