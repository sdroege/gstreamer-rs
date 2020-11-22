// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use std::ptr;

#[derive(Debug, PartialEq, Eq)]
pub struct RTSPContext(ptr::NonNull<ffi::GstRTSPContext>);

impl RTSPContext {
    pub fn with_current_context<F: FnOnce(&RTSPContext) -> T, T>(func: F) -> Option<T> {
        unsafe {
            let ptr = ffi::gst_rtsp_context_get_current();
            if ptr.is_null() {
                return None;
            }

            let ctx = RTSPContext(ptr::NonNull::new_unchecked(ptr));
            Some(func(&ctx))
        }
    }

    // TODO: Add various getters for all the contained fields as needed
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::GstRTSPContext> for RTSPContext {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::GstRTSPContext) -> Borrowed<Self> {
        assert!(!ptr.is_null());
        Borrowed::new(RTSPContext(ptr::NonNull::new_unchecked(ptr)))
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut ffi::GstRTSPContext> for RTSPContext {
    type Storage = &'a RTSPContext;

    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GstRTSPContext, Self> {
        Stash(self.0.as_ptr(), self)
    }

    fn to_glib_full(&self) -> *mut ffi::GstRTSPContext {
        unimplemented!()
    }
}
