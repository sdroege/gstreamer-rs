// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use futures_sink::Sink;
use glib::translate::*;
use glib_sys::{gboolean, gpointer};
use gst;
use gst_app_sys;
use std::cell::RefCell;
use std::mem;
use std::pin::Pin;
use std::ptr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use AppSrc;

lazy_static! {
    static ref SET_ONCE_QUARK: glib::Quark =
        glib::Quark::from_string("gstreamer-rs-app-src-callbacks");
}

#[allow(clippy::type_complexity)]
pub struct AppSrcCallbacks {
    need_data: Option<RefCell<Box<dyn FnMut(&AppSrc, u32) + Send + 'static>>>,
    enough_data: Option<Box<dyn Fn(&AppSrc) + Send + Sync + 'static>>,
    seek_data: Option<Box<dyn Fn(&AppSrc, u64) -> bool + Send + Sync + 'static>>,
    callbacks: gst_app_sys::GstAppSrcCallbacks,
}

unsafe impl Send for AppSrcCallbacks {}
unsafe impl Sync for AppSrcCallbacks {}

impl AppSrcCallbacks {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> AppSrcCallbacksBuilder {
        skip_assert_initialized!();

        AppSrcCallbacksBuilder {
            need_data: None,
            enough_data: None,
            seek_data: None,
        }
    }
}

#[allow(clippy::type_complexity)]
pub struct AppSrcCallbacksBuilder {
    need_data: Option<RefCell<Box<dyn FnMut(&AppSrc, u32) + Send + 'static>>>,
    enough_data: Option<Box<dyn Fn(&AppSrc) + Send + Sync + 'static>>,
    seek_data: Option<Box<dyn Fn(&AppSrc, u64) -> bool + Send + Sync + 'static>>,
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
            callbacks: gst_app_sys::GstAppSrcCallbacks {
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
    appsrc: *mut gst_app_sys::GstAppSrc,
    length: u32,
    callbacks: gpointer,
) {
    let callbacks = &*(callbacks as *const AppSrcCallbacks);

    if let Some(ref need_data) = callbacks.need_data {
        (&mut *need_data.borrow_mut())(&from_glib_borrow(appsrc), length);
    }
}

unsafe extern "C" fn trampoline_enough_data(
    appsrc: *mut gst_app_sys::GstAppSrc,
    callbacks: gpointer,
) {
    let callbacks = &*(callbacks as *const AppSrcCallbacks);

    if let Some(ref enough_data) = callbacks.enough_data {
        (*enough_data)(&from_glib_borrow(appsrc));
    }
}

unsafe extern "C" fn trampoline_seek_data(
    appsrc: *mut gst_app_sys::GstAppSrc,
    offset: u64,
    callbacks: gpointer,
) -> gboolean {
    let callbacks = &*(callbacks as *const AppSrcCallbacks);

    let ret = if let Some(ref seek_data) = callbacks.seek_data {
        (*seek_data)(&from_glib_borrow(appsrc), offset)
    } else {
        false
    };

    ret.to_glib()
}

unsafe extern "C" fn destroy_callbacks(ptr: gpointer) {
    Box::<AppSrcCallbacks>::from_raw(ptr as *mut _);
}

impl AppSrc {
    pub fn end_of_stream(&self) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(gst_app_sys::gst_app_src_end_of_stream(
                self.to_glib_none().0,
            ))
        };
        ret.into_result()
    }

    pub fn push_buffer(&self, buffer: gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(gst_app_sys::gst_app_src_push_buffer(
                self.to_glib_none().0,
                buffer.into_ptr(),
            ))
        };
        ret.into_result()
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn push_buffer_list(
        &self,
        list: gst::BufferList,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(gst_app_sys::gst_app_src_push_buffer_list(
                self.to_glib_none().0,
                list.into_ptr(),
            ))
        };
        ret.into_result()
    }

    pub fn push_sample(&self, sample: &gst::Sample) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(gst_app_sys::gst_app_src_push_sample(
                self.to_glib_none().0,
                sample.to_glib_none().0,
            ))
        };
        ret.into_result()
    }

    pub fn set_callbacks(&self, callbacks: AppSrcCallbacks) {
        unsafe {
            let src = self.to_glib_none().0;
            if !gobject_sys::g_object_get_qdata(src as *mut _, SET_ONCE_QUARK.to_glib()).is_null() {
                panic!("AppSrc callbacks can only be set once");
            }

            gobject_sys::g_object_set_qdata(src as *mut _, SET_ONCE_QUARK.to_glib(), 1 as *mut _);

            gst_app_sys::gst_app_src_set_callbacks(
                src,
                mut_override(&callbacks.callbacks),
                Box::into_raw(Box::new(callbacks)) as *mut _,
                Some(destroy_callbacks),
            );
        }
    }

    pub fn set_latency(&self, min: gst::ClockTime, max: gst::ClockTime) {
        unsafe {
            gst_app_sys::gst_app_src_set_latency(
                self.to_glib_none().0,
                min.to_glib(),
                max.to_glib(),
            );
        }
    }

    pub fn get_latency(&self) -> (gst::ClockTime, gst::ClockTime) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            gst_app_sys::gst_app_src_get_latency(
                self.to_glib_none().0,
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );
            (from_glib(min.assume_init()), from_glib(max.assume_init()))
        }
    }

    pub fn sink(&self) -> AppSrcSink {
        AppSrcSink::new(self)
    }
}

#[derive(Debug)]
pub struct AppSrcSink {
    app_src: AppSrc,
    waker_reference: Arc<Mutex<Option<Waker>>>,
}

impl AppSrcSink {
    fn new(app_src: &AppSrc) -> Self {
        skip_assert_initialized!();

        let app_src = app_src.clone();
        let waker_reference = Arc::new(Mutex::new(None as Option<Waker>));

        app_src.set_callbacks(
            AppSrcCallbacks::new()
                .need_data({
                    let waker_reference = Arc::clone(&waker_reference);

                    move |_, _| {
                        if let Some(waker) = waker_reference.lock().unwrap().take() {
                            waker.wake();
                        }
                    }
                })
                .build(),
        );

        Self {
            app_src,
            waker_reference,
        }
    }
}

impl Sink<gst::Sample> for AppSrcSink {
    type Error = gst::FlowError;

    fn poll_ready(self: Pin<&mut Self>, context: &mut Context) -> Poll<Result<(), Self::Error>> {
        let mut waker = self.waker_reference.lock().unwrap();

        let current_level_bytes = self.app_src.get_current_level_bytes();
        let max_bytes = self.app_src.get_max_bytes();

        if current_level_bytes >= max_bytes && max_bytes != 0 {
            waker.replace(context.waker().to_owned());

            Poll::Pending
        } else {
            Poll::Ready(Ok(()))
        }
    }

    fn start_send(self: Pin<&mut Self>, sample: gst::Sample) -> Result<(), Self::Error> {
        self.app_src.push_sample(&sample)?;

        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.app_src.end_of_stream()?;

        Poll::Ready(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::{sink::SinkExt, stream::StreamExt};
    use gst::prelude::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_app_src_sink() {
        gst::init().unwrap();

        let appsrc = gst::ElementFactory::make("appsrc", None).unwrap();
        let fakesink = gst::ElementFactory::make("fakesink", None).unwrap();

        fakesink.set_property("signal-handoffs", &true).unwrap();

        let pipeline = gst::Pipeline::new(None);
        pipeline.add(&appsrc).unwrap();
        pipeline.add(&fakesink).unwrap();

        appsrc.link(&fakesink).unwrap();

        let mut bus_stream = pipeline.get_bus().unwrap().stream();
        let mut app_src_sink = appsrc.dynamic_cast::<AppSrc>().unwrap().sink();

        let sample_quantity = 5;

        let samples = (0..sample_quantity)
            .map(|_| gst::Sample::new().buffer(&gst::Buffer::new()).build())
            .collect::<Vec<gst::Sample>>();

        let mut sample_stream = futures_util::stream::iter(samples).map(Ok);

        let handoff_count_reference = Arc::new(AtomicUsize::new(0));

        fakesink
            .connect("handoff", false, {
                let handoff_count_reference = Arc::clone(&handoff_count_reference);

                move |_| {
                    handoff_count_reference.fetch_add(1, Ordering::AcqRel);

                    None
                }
            })
            .unwrap();

        pipeline.set_state(gst::State::Playing).unwrap();

        futures_executor::block_on(app_src_sink.send_all(&mut sample_stream)).unwrap();
        futures_executor::block_on(app_src_sink.close()).unwrap();

        while let Some(message) = futures_executor::block_on(bus_stream.next()) {
            match message.view() {
                gst::MessageView::Eos(_) => break,
                gst::MessageView::Error(_) => unreachable!(),
                _ => continue,
            }
        }

        pipeline.set_state(gst::State::Null).unwrap();

        assert_eq!(
            handoff_count_reference.load(Ordering::Acquire),
            sample_quantity
        );
    }
}
