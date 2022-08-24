// Take a look at the license at the top of the repository in the LICENSE file.

use crate::PromiseResult;
use crate::Structure;
use crate::StructureRef;
use glib::translate::*;

use std::ptr;
use std::task::{Context, Poll};
use std::{ops::Deref, pin::Pin};

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GstPromise")]
    pub struct Promise(Shared<ffi::GstPromise>);

    match fn {
        ref => |ptr| ffi::gst_mini_object_ref(ptr as *mut _),
        unref => |ptr| ffi::gst_mini_object_unref(ptr as *mut _),
        type_ => || ffi::gst_promise_get_type(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PromiseError {
    Interrupted,
    Expired,
    Other(PromiseResult),
}

impl Promise {
    #[doc(alias = "gst_promise_new")]
    pub fn new() -> Promise {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_promise_new()) }
    }

    #[doc(alias = "gst_promise_new_with_change_func")]
    pub fn with_change_func<F>(func: F) -> Promise
    where
        F: FnOnce(Result<Option<&StructureRef>, PromiseError>) + Send + 'static,
    {
        assert_initialized_main_thread!();
        let user_data: Box<Option<F>> = Box::new(Some(func));

        unsafe extern "C" fn trampoline<
            F: FnOnce(Result<Option<&StructureRef>, PromiseError>) + Send + 'static,
        >(
            promise: *mut ffi::GstPromise,
            user_data: glib::ffi::gpointer,
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
            user_data: glib::ffi::gpointer,
        ) {
            let _: Box<Option<F>> = Box::from_raw(user_data as *mut _);
        }

        unsafe {
            from_glib_full(ffi::gst_promise_new_with_change_func(
                Some(trampoline::<F>),
                Box::into_raw(user_data) as *mut _,
                Some(free_user_data::<F>),
            ))
        }
    }

    pub fn new_future() -> (Self, PromiseFuture) {
        use futures_channel::oneshot;

        // We only use the channel as a convenient waker
        let (sender, receiver) = oneshot::channel();
        let promise = Self::with_change_func(move |_res| {
            let _ = sender.send(());
        });

        (promise.clone(), PromiseFuture(promise, receiver))
    }

    #[doc(alias = "gst_promise_expire")]
    pub fn expire(&self) {
        unsafe {
            ffi::gst_promise_expire(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_promise_get_reply")]
    pub fn get_reply(&self) -> Option<&StructureRef> {
        unsafe {
            let s = ffi::gst_promise_get_reply(self.to_glib_none().0);
            if s.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(s))
            }
        }
    }

    #[doc(alias = "gst_promise_interrupt")]
    pub fn interrupt(&self) {
        unsafe {
            ffi::gst_promise_interrupt(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_promise_reply")]
    pub fn reply(&self, s: Option<Structure>) {
        unsafe {
            ffi::gst_promise_reply(
                self.to_glib_none().0,
                s.map(|s| s.into_glib_ptr()).unwrap_or(ptr::null_mut()),
            );
        }
    }

    #[doc(alias = "gst_promise_wait")]
    pub fn wait(&self) -> PromiseResult {
        unsafe { from_glib(ffi::gst_promise_wait(self.to_glib_none().0)) }
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
pub struct PromiseFuture(Promise, futures_channel::oneshot::Receiver<()>);

pub struct PromiseReply(Promise);

impl std::future::Future for PromiseFuture {
    type Output = Result<Option<PromiseReply>, PromiseError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<Self::Output> {
        match Pin::new(&mut self.1).poll(context) {
            Poll::Ready(Err(_)) => panic!("Sender dropped before callback was called"),
            Poll::Ready(Ok(())) => {
                let res = match self.0.wait() {
                    PromiseResult::Replied => {
                        if self.0.get_reply().is_none() {
                            Ok(None)
                        } else {
                            Ok(Some(PromiseReply(self.0.clone())))
                        }
                    }
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

impl futures_core::future::FusedFuture for PromiseFuture {
    fn is_terminated(&self) -> bool {
        self.1.is_terminated()
    }
}

impl Deref for PromiseReply {
    type Target = StructureRef;

    fn deref(&self) -> &StructureRef {
        self.0.get_reply().expect("Promise without reply")
    }
}

impl std::fmt::Debug for PromiseReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_tuple("PromiseReply");

        match self.0.get_reply() {
            Some(reply) => debug.field(reply),
            None => debug.field(&"<no reply>"),
        }
        .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;
    use std::thread;

    #[test]
    fn test_change_func() {
        crate::init().unwrap();

        let (sender, receiver) = channel();
        let promise = Promise::with_change_func(move |res| {
            sender.send(res.map(|s| s.map(ToOwned::to_owned))).unwrap();
        });

        thread::spawn(move || {
            promise.reply(Some(crate::Structure::new_empty("foo/bar")));
        });

        let res = receiver.recv().unwrap();
        let res = res.expect("promise failed").expect("promise returned None");
        assert_eq!(res.name(), "foo/bar");
    }
}
