// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::object::ObjectType;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys::gpointer;
use gst;
use gst_app_sys;
use gst_sys;
use std::boxed::Box as Box_;
use std::cell::RefCell;
use std::mem::transmute;
use std::ptr;
use AppSink;

#[allow(clippy::type_complexity)]
pub struct AppSinkCallbacks {
    eos: Option<RefCell<Box<dyn FnMut(&AppSink) + Send + 'static>>>,
    new_preroll: Option<
        RefCell<
            Box<dyn FnMut(&AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static>,
        >,
    >,
    new_sample: Option<
        RefCell<
            Box<dyn FnMut(&AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static>,
        >,
    >,
    callbacks: gst_app_sys::GstAppSinkCallbacks,
}

unsafe impl Send for AppSinkCallbacks {}
unsafe impl Sync for AppSinkCallbacks {}

impl AppSinkCallbacks {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> AppSinkCallbacksBuilder {
        skip_assert_initialized!();
        AppSinkCallbacksBuilder {
            eos: None,
            new_preroll: None,
            new_sample: None,
        }
    }
}

#[allow(clippy::type_complexity)]
pub struct AppSinkCallbacksBuilder {
    eos: Option<RefCell<Box<dyn FnMut(&AppSink) + Send + 'static>>>,
    new_preroll: Option<
        RefCell<
            Box<dyn FnMut(&AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static>,
        >,
    >,
    new_sample: Option<
        RefCell<
            Box<dyn FnMut(&AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static>,
        >,
    >,
}

impl AppSinkCallbacksBuilder {
    pub fn eos<F: Fn(&AppSink) + Send + 'static>(self, eos: F) -> Self {
        Self {
            eos: Some(RefCell::new(Box::new(eos))),
            ..self
        }
    }

    pub fn new_preroll<
        F: Fn(&AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static,
    >(
        self,
        new_preroll: F,
    ) -> Self {
        Self {
            new_preroll: Some(RefCell::new(Box::new(new_preroll))),
            ..self
        }
    }

    pub fn new_sample<
        F: Fn(&AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static,
    >(
        self,
        new_sample: F,
    ) -> Self {
        Self {
            new_sample: Some(RefCell::new(Box::new(new_sample))),
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
            callbacks: gst_app_sys::GstAppSinkCallbacks {
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

unsafe extern "C" fn trampoline_eos(appsink: *mut gst_app_sys::GstAppSink, callbacks: gpointer) {
    let callbacks = &*(callbacks as *const AppSinkCallbacks);

    if let Some(ref eos) = callbacks.eos {
        (&mut *eos.borrow_mut())(&from_glib_borrow(appsink))
    }
}

unsafe extern "C" fn trampoline_new_preroll(
    appsink: *mut gst_app_sys::GstAppSink,
    callbacks: gpointer,
) -> gst_sys::GstFlowReturn {
    let callbacks = &*(callbacks as *const AppSinkCallbacks);

    let ret = if let Some(ref new_preroll) = callbacks.new_preroll {
        (&mut *new_preroll.borrow_mut())(&from_glib_borrow(appsink)).into()
    } else {
        gst::FlowReturn::Error
    };

    ret.to_glib()
}

unsafe extern "C" fn trampoline_new_sample(
    appsink: *mut gst_app_sys::GstAppSink,
    callbacks: gpointer,
) -> gst_sys::GstFlowReturn {
    let callbacks = &*(callbacks as *const AppSinkCallbacks);

    let ret = if let Some(ref new_sample) = callbacks.new_sample {
        (&mut *new_sample.borrow_mut())(&from_glib_borrow(appsink)).into()
    } else {
        gst::FlowReturn::Error
    };

    ret.to_glib()
}

unsafe extern "C" fn destroy_callbacks(ptr: gpointer) {
    Box::<AppSinkCallbacks>::from_raw(ptr as *mut _);
}

impl AppSink {
    pub fn set_callbacks(&self, callbacks: AppSinkCallbacks) {
        unsafe {
            gst_app_sys::gst_app_sink_set_callbacks(
                self.to_glib_none().0,
                mut_override(&callbacks.callbacks),
                Box::into_raw(Box::new(callbacks)) as *mut _,
                Some(destroy_callbacks),
            );
        }
    }

    pub fn connect_new_sample<
        F: Fn(&AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"new-sample\0".as_ptr() as *const _,
                Some(transmute(new_sample_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_new_preroll<
        F: Fn(&AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"new-preroll\0".as_ptr() as *const _,
                Some(transmute(new_preroll_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe extern "C" fn new_sample_trampoline<
    F: Fn(&AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static,
>(
    this: *mut gst_app_sys::GstAppSink,
    f: glib_sys::gpointer,
) -> gst_sys::GstFlowReturn {
    let f: &F = &*(f as *const F);
    let ret: gst::FlowReturn = f(&from_glib_borrow(this)).into();
    ret.to_glib()
}

unsafe extern "C" fn new_preroll_trampoline<
    F: Fn(&AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static,
>(
    this: *mut gst_app_sys::GstAppSink,
    f: glib_sys::gpointer,
) -> gst_sys::GstFlowReturn {
    let f: &F = &*(f as *const F);
    let ret: gst::FlowReturn = f(&from_glib_borrow(this)).into();
    ret.to_glib()
}
