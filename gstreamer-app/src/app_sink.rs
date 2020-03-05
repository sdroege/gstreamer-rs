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
use gst::gst_element_error;
use gst_app_sys;
use gst_sys;
use std::boxed::Box as Box_;
use std::cell::RefCell;
use std::mem::transmute;
use std::panic;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use AppSink;

#[cfg(any(feature = "v1_10"))]
use {
    futures_core::Stream,
    std::{
        pin::Pin,
        sync::{Arc, Mutex},
        task::{Context, Poll, Waker},
    },
};

lazy_static! {
    static ref SET_ONCE_QUARK: glib::Quark =
        glib::Quark::from_string("gstreamer-rs-app-sink-callbacks");
}

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
    panicked: AtomicBool,
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
    pub fn eos<F: FnMut(&AppSink) + Send + 'static>(self, eos: F) -> Self {
        Self {
            eos: Some(RefCell::new(Box::new(eos))),
            ..self
        }
    }

    pub fn new_preroll<
        F: FnMut(&AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static,
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
        F: FnMut(&AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static,
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
            panicked: AtomicBool::new(false),
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

fn post_panic_error_message(element: &AppSink, err: &dyn std::any::Any) {
    if let Some(cause) = err.downcast_ref::<&str>() {
        gst_element_error!(&element, gst::LibraryError::Failed, ["Panicked: {}", cause]);
    } else if let Some(cause) = err.downcast_ref::<String>() {
        gst_element_error!(&element, gst::LibraryError::Failed, ["Panicked: {}", cause]);
    } else {
        gst_element_error!(&element, gst::LibraryError::Failed, ["Panicked"]);
    }
}

unsafe extern "C" fn trampoline_eos(appsink: *mut gst_app_sys::GstAppSink, callbacks: gpointer) {
    let callbacks = &*(callbacks as *const AppSinkCallbacks);
    let element: AppSink = from_glib_borrow(appsink);

    if callbacks.panicked.load(Ordering::Relaxed) {
        let element: AppSink = from_glib_borrow(appsink);
        gst_element_error!(&element, gst::LibraryError::Failed, ["Panicked"]);
        return;
    }

    if let Some(ref eos) = callbacks.eos {
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            (&mut *eos.borrow_mut())(&element)
        }));
        match result {
            Ok(result) => result,
            Err(err) => {
                callbacks.panicked.store(true, Ordering::Relaxed);
                post_panic_error_message(&element, &err);
            }
        }
    }
}

unsafe extern "C" fn trampoline_new_preroll(
    appsink: *mut gst_app_sys::GstAppSink,
    callbacks: gpointer,
) -> gst_sys::GstFlowReturn {
    let callbacks = &*(callbacks as *const AppSinkCallbacks);
    let element: AppSink = from_glib_borrow(appsink);

    if callbacks.panicked.load(Ordering::Relaxed) {
        let element: AppSink = from_glib_borrow(appsink);
        gst_element_error!(&element, gst::LibraryError::Failed, ["Panicked"]);
        return gst::FlowReturn::Error.to_glib();
    }

    let ret = if let Some(ref new_preroll) = callbacks.new_preroll {
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            (&mut *new_preroll.borrow_mut())(&element).into()
        }));
        match result {
            Ok(result) => result,
            Err(err) => {
                callbacks.panicked.store(true, Ordering::Relaxed);
                post_panic_error_message(&element, &err);

                gst::FlowReturn::Error
            }
        }
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
    let element: AppSink = from_glib_borrow(appsink);

    if callbacks.panicked.load(Ordering::Relaxed) {
        let element: AppSink = from_glib_borrow(appsink);
        gst_element_error!(&element, gst::LibraryError::Failed, ["Panicked"]);
        return gst::FlowReturn::Error.to_glib();
    }

    let ret = if let Some(ref new_sample) = callbacks.new_sample {
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            (&mut *new_sample.borrow_mut())(&element).into()
        }));
        match result {
            Ok(result) => result,
            Err(err) => {
                callbacks.panicked.store(true, Ordering::Relaxed);
                post_panic_error_message(&element, &err);

                gst::FlowReturn::Error
            }
        }
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
            let sink = self.to_glib_none().0;

            // This is not thread-safe before 1.16.3, see
            // https://gitlab.freedesktop.org/gstreamer/gst-plugins-base/merge_requests/570
            if gst::version() < (1, 16, 3, 0) {
                if !gobject_sys::g_object_get_qdata(sink as *mut _, SET_ONCE_QUARK.to_glib())
                    .is_null()
                {
                    panic!("AppSink callbacks can only be set once");
                }

                gobject_sys::g_object_set_qdata(
                    sink as *mut _,
                    SET_ONCE_QUARK.to_glib(),
                    1 as *mut _,
                );
            }

            gst_app_sys::gst_app_sink_set_callbacks(
                sink,
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

    #[cfg(any(feature = "v1_10"))]
    pub fn stream(&self) -> AppSinkStream {
        AppSinkStream::new(self)
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

#[cfg(any(feature = "v1_10"))]
#[derive(Debug)]
pub struct AppSinkStream {
    app_sink: AppSink,
    waker_reference: Arc<Mutex<Option<Waker>>>,
}

#[cfg(any(feature = "v1_10"))]
impl AppSinkStream {
    fn new(app_sink: &AppSink) -> Self {
        skip_assert_initialized!();

        let app_sink = app_sink.clone();
        let waker_reference = Arc::new(Mutex::new(None as Option<Waker>));

        app_sink.set_callbacks(
            AppSinkCallbacks::new()
                .new_sample({
                    let waker_reference = Arc::clone(&waker_reference);

                    move |_| {
                        if let Some(waker) = waker_reference.lock().unwrap().take() {
                            waker.wake();
                        }

                        Ok(gst::FlowSuccess::Ok)
                    }
                })
                .eos({
                    let waker_reference = Arc::clone(&waker_reference);

                    move |_| {
                        if let Some(waker) = waker_reference.lock().unwrap().take() {
                            waker.wake();
                        }
                    }
                })
                .build(),
        );

        Self {
            app_sink,
            waker_reference,
        }
    }
}

#[cfg(any(feature = "v1_10"))]
impl Drop for AppSinkStream {
    fn drop(&mut self) {
        // This is not thread-safe before 1.16.3, see
        // https://gitlab.freedesktop.org/gstreamer/gst-plugins-base/merge_requests/570
        if gst::version() >= (1, 16, 3, 0) {
            self.app_sink.set_callbacks(AppSinkCallbacks::new().build());
        }
    }
}

#[cfg(any(feature = "v1_10"))]
impl Stream for AppSinkStream {
    type Item = gst::Sample;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context) -> Poll<Option<Self::Item>> {
        let mut waker = self.waker_reference.lock().unwrap();

        self.app_sink
            .try_pull_sample(gst::ClockTime::from_mseconds(0))
            .map(|sample| Poll::Ready(Some(sample)))
            .unwrap_or_else(|| {
                if self.app_sink.is_eos() {
                    return Poll::Ready(None);
                }

                waker.replace(context.waker().to_owned());

                Poll::Pending
            })
    }
}

#[cfg(any(feature = "v1_10"))]
#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;
    use gst::prelude::*;

    #[test]
    fn test_app_sink_stream() {
        gst::init().unwrap();

        let videotestsrc = gst::ElementFactory::make("videotestsrc", None).unwrap();
        let appsink = gst::ElementFactory::make("appsink", None).unwrap();

        videotestsrc.set_property("num-buffers", &5).unwrap();

        let pipeline = gst::Pipeline::new(None);
        pipeline.add(&videotestsrc).unwrap();
        pipeline.add(&appsink).unwrap();

        videotestsrc.link(&appsink).unwrap();

        let app_sink_stream = appsink.dynamic_cast::<AppSink>().unwrap().stream();
        let samples_future = app_sink_stream.collect::<Vec<gst::Sample>>();

        pipeline.set_state(gst::State::Playing).unwrap();
        let samples = futures_executor::block_on(samples_future);
        pipeline.set_state(gst::State::Null).unwrap();

        assert_eq!(samples.len(), 5);
    }
}
