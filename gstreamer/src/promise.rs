// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib::translate::*;
use glib_ffi;
use PromiseResult;
use Structure;
use StructureRef;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Promise(Shared<ffi::GstPromise>);

    match fn {
        ref => |ptr| ffi::gst_mini_object_ref(ptr as *mut _),
        unref => |ptr| ffi::gst_mini_object_unref(ptr as *mut _),
        get_type => || ffi::gst_promise_get_type(),
    }
}

impl Promise {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn new() -> Promise {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_promise_new()) }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn new_with_change_func<F>(func: F) -> Promise
    where
        F: FnOnce(&Promise) + Send + 'static,
    {
        let user_data: Box<Option<F>> = Box::new(Some(func));

        unsafe extern "C" fn trampoline<F: FnOnce(&Promise) + Send + 'static>(
            promise: *mut ffi::GstPromise,
            user_data: glib_ffi::gpointer,
        ) {
            let user_data: &mut Option<F> = &mut *(user_data as *mut _);
            let callback = user_data.take().unwrap();

            callback(&from_glib_borrow(promise));
        }

        unsafe extern "C" fn free_user_data<F: FnOnce(&Promise) + Send + 'static>(
            user_data: glib_ffi::gpointer,
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

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn expire(&self) {
        unsafe {
            ffi::gst_promise_expire(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
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

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn interrupt(&self) {
        unsafe {
            ffi::gst_promise_interrupt(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn reply(&self, s: Structure) {
        unsafe {
            ffi::gst_promise_reply(self.to_glib_none().0, s.into_ptr());
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn wait(&self) -> PromiseResult {
        unsafe { from_glib(ffi::gst_promise_wait(self.to_glib_none().0)) }
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
impl Default for Promise {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for Promise {}
unsafe impl Sync for Promise {}
