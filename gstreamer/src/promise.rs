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

use std::marker::PhantomData;
use std::pin::Pin;
use std::ptr;
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

    pub fn with_change_func<F>(func: F) -> Promise
    where
        F: FnOnce(Result<Option<&StructureRef>, PromiseError>) + Send + 'static,
    {
        assert_initialized_main_thread!();
        let user_data: Box<Option<F>> = Box::new(Some(func));

        unsafe extern "C" fn trampoline<
            F: FnOnce(Result<Option<&StructureRef>, PromiseError>) + Send + 'static,
        >(
            promise: *mut gst_sys::GstPromise,
            user_data: glib_sys::gpointer,
        ) {
            let user_data: &mut Option<F> = &mut *(user_data as *mut _);
            let callback = user_data.take().unwrap();

            let promise: Borrowed<Promise> = from_glib_borrow(promise);

            let res = match promise.wait() {
                PromiseResult::Replied => Ok(promise.get_reply()),
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
            F: FnOnce(Result<Option<&StructureRef>, PromiseError>) + Send + 'static,
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

    pub fn new_future<'a>() -> (Self, PromiseFuture<'a>) {
        use futures_channel::oneshot;

        // We only use the channel as a convenient waker
        let (sender, receiver) = oneshot::channel();
        let promise = Self::with_change_func(move |_res| {
            let _ = sender.send(());
        });

        (
            promise.clone(),
            PromiseFuture(promise, receiver, PhantomData),
        )
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

    pub fn reply(&self, s: Option<Structure>) {
        unsafe {
            gst_sys::gst_promise_reply(
                self.to_glib_none().0,
                s.map(|s| s.into_ptr()).unwrap_or(ptr::null_mut()),
            );
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
pub struct PromiseFuture<'a>(
    Promise,
    futures_channel::oneshot::Receiver<()>,
    PhantomData<&'a StructureRef>,
);

impl<'a> std::future::Future for PromiseFuture<'a> {
    type Output = Result<Option<&'a StructureRef>, PromiseError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<Self::Output> {
        match Pin::new(&mut self.1).poll(context) {
            Poll::Ready(Err(_)) => panic!("Sender dropped before callback was called"),
            Poll::Ready(Ok(())) => {
                let res = match self.0.wait() {
                    PromiseResult::Replied => unsafe {
                        let s = gst_sys::gst_promise_get_reply(self.0.to_glib_none().0);
                        if s.is_null() {
                            Ok(None)
                        } else {
                            Ok(Some(StructureRef::from_glib_borrow(s)))
                        }
                    },
                    PromiseResult::Interrupted => Err(PromiseError::Interrupted),
                    PromiseResult::Expired => Err(PromiseError::Expired),
                    PromiseResult::Pending => {
                        panic!("Promise resolved but returned Pending");
                    }
                    err => Err(PromiseError::Other(err)),
                };
                Poll::Ready(res)
            }
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
        let promise = Promise::with_change_func(move |res| {
            sender.send(res.map(|s| s.map(ToOwned::to_owned))).unwrap();
        });

        thread::spawn(move || {
            promise.reply(Some(crate::Structure::new("foo/bar", &[])));
        });

        let res = receiver.recv().unwrap();
        let res = res.expect("promise failed").expect("promise returned None");
        assert_eq!(res.get_name(), "foo/bar");
    }
}
