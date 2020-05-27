// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use glib_sys;
use gst_sys;
use PromiseResult;
use Structure;
use StructureRef;

use std::pin::Pin;
use std::task::{Context, Poll};

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Promise(Shared<gst_sys::GstPromise>);

    match fn {
        ref => |ptr| gst_sys::gst_mini_object_ref(ptr as *mut _),
        unref => |ptr| gst_sys::gst_mini_object_unref(ptr as *mut _),
        get_type => || gst_sys::gst_promise_get_type(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PromiseError {
    Interrupted,
    Expired,
    Other(PromiseResult),
}

impl Promise {
    pub fn new() -> Promise {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gst_sys::gst_promise_new()) }
    }

    pub fn new_with_change_func<F>(func: F) -> Promise
    where
        F: FnOnce(Result<&StructureRef, PromiseError>) + Send + 'static,
    {
        let user_data: Box<Option<F>> = Box::new(Some(func));

        unsafe extern "C" fn trampoline<
            F: FnOnce(Result<&StructureRef, PromiseError>) + Send + 'static,
        >(
            promise: *mut gst_sys::GstPromise,
            user_data: glib_sys::gpointer,
        ) {
            lazy_static! {
                static ref EMPTY: Structure = Structure::new_empty("EMPTY");
            }

            let user_data: &mut Option<F> = &mut *(user_data as *mut _);
            let callback = user_data.take().unwrap();

            let promise: Promise = from_glib_borrow(promise);

            let res = match promise.wait() {
                // Return an empty structure if it's None as workaround for
                // https://gitlab.freedesktop.org/gstreamer/gst-plugins-bad/-/issues/1300
                PromiseResult::Replied => Ok(promise.get_reply().unwrap_or(&EMPTY)),
                PromiseResult::Interrupted => Err(PromiseError::Interrupted),
                PromiseResult::Expired => Err(PromiseError::Expired),
                PromiseResult::Pending => {
                    panic!("Promise resolved but returned Pending");
                }
                err => Err(PromiseError::Other(err)),
            };

            callback(res);
        }

        unsafe extern "C" fn free_user_data<
            F: FnOnce(Result<&StructureRef, PromiseError>) + Send + 'static,
        >(
            user_data: glib_sys::gpointer,
        ) {
            let _: Box<Option<F>> = Box::from_raw(user_data as *mut _);
        }

        unsafe {
            from_glib_full(gst_sys::gst_promise_new_with_change_func(
                Some(trampoline::<F>),
                Box::into_raw(user_data) as *mut _,
                Some(free_user_data::<F>),
            ))
        }
    }

    pub fn new_future() -> (Self, PromiseFuture) {
        use futures_channel::oneshot;

        let (sender, receiver) = oneshot::channel();

        let promise = Self::new_with_change_func(move |res| {
            let _ = sender.send(res.map(|s| s.to_owned()));
        });

        (promise, PromiseFuture(receiver))
    }

    pub fn expire(&self) {
        unsafe {
            gst_sys::gst_promise_expire(self.to_glib_none().0);
        }
    }

    pub fn get_reply(&self) -> Option<&StructureRef> {
        unsafe {
            let s = gst_sys::gst_promise_get_reply(self.to_glib_none().0);
            if s.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(s))
            }
        }
    }

    pub fn interrupt(&self) {
        unsafe {
            gst_sys::gst_promise_interrupt(self.to_glib_none().0);
        }
    }

    pub fn reply(&self, s: Structure) {
        unsafe {
            gst_sys::gst_promise_reply(self.to_glib_none().0, s.into_ptr());
        }
    }

    pub fn wait(&self) -> PromiseResult {
        unsafe { from_glib(gst_sys::gst_promise_wait(self.to_glib_none().0)) }
    }
}

impl Default for Promise {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for Promise {}
unsafe impl Sync for Promise {}

#[derive(Debug)]
pub struct PromiseFuture(futures_channel::oneshot::Receiver<Result<Structure, PromiseError>>);

impl std::future::Future for PromiseFuture {
    type Output = Result<Structure, PromiseError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<Self::Output> {
        match Pin::new(&mut self.0).poll(context) {
            Poll::Ready(Err(_)) => panic!("Sender dropped before callback was called"),
            Poll::Ready(Ok(res)) => Poll::Ready(res),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;
    use std::thread;

    #[test]
    fn test_change_func() {
        ::init().unwrap();

        let (sender, receiver) = channel();
        let promise = Promise::new_with_change_func(move |res| {
            sender.send(res.map(|s| s.to_owned())).unwrap();
        });

        thread::spawn(move || {
            promise.reply(crate::Structure::new("foo/bar", &[]));
        });

        let res = receiver.recv().unwrap();
        let res = res.expect("promise failed");
        assert_eq!(res.get_name(), "foo/bar");
    }
}
