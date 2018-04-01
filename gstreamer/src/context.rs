// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::fmt;

use ffi;

use glib;
use glib::StaticType;
use glib::translate::{from_glib, from_glib_full, ToGlib, ToGlibPtr};

use StructureRef;
use miniobject::*;

pub type Context = GstRc<ContextRef>;
pub struct ContextRef(ffi::GstContext);

unsafe impl MiniObject for ContextRef {
    type GstType = ffi::GstContext;
}

impl GstRc<ContextRef> {
    pub fn new(context_type: &str, persistent: bool) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_context_new(
                context_type.to_glib_none().0,
                persistent.to_glib(),
            ))
        }
    }
}

impl ContextRef {
    pub fn get_context_type(&self) -> &str {
        unsafe {
            let raw = ffi::gst_context_get_context_type(self.as_mut_ptr());
            CStr::from_ptr(raw).to_str().unwrap()
        }
    }

    pub fn has_context_type(&self, context_type: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_context_has_context_type(
                self.as_mut_ptr(),
                context_type.to_glib_none().0,
            ))
        }
    }

    pub fn is_persistent(&self) -> bool {
        unsafe { from_glib(ffi::gst_context_is_persistent(self.as_mut_ptr())) }
    }

    pub fn get_structure(&self) -> &StructureRef {
        unsafe { StructureRef::from_glib_borrow(ffi::gst_context_get_structure(self.as_mut_ptr())) }
    }

    pub fn get_mut_structure(&mut self) -> &mut StructureRef {
        unsafe {
            StructureRef::from_glib_borrow_mut(ffi::gst_context_writable_structure(
                self.as_mut_ptr(),
            ))
        }
    }
}

impl StaticType for ContextRef {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_context_get_type()) }
    }
}

impl fmt::Debug for ContextRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Context")
            .field("type", &self.get_context_type())
            .field("structure", &self.get_structure())
            .finish()
    }
}

impl ToOwned for ContextRef {
    type Owned = GstRc<ContextRef>;

    fn to_owned(&self) -> GstRc<ContextRef> {
        unsafe { from_glib_full(ffi::gst_mini_object_copy(self.as_ptr() as *const _) as *mut _) }
    }
}

unsafe impl Sync for ContextRef {}
unsafe impl Send for ContextRef {}
