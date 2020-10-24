// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Caps;

use glib_sys;
use gobject_sys;
use gst_sys;

use glib;
use glib::translate::*;

use std::ffi::CStr;
use std::fmt;
use std::ptr;

pub struct StaticCaps(ptr::NonNull<gst_sys::GstStaticCaps>);

impl StaticCaps {
    pub fn get(&self) -> Caps {
        unsafe { from_glib_full(gst_sys::gst_static_caps_get(self.0.as_ptr())) }
    }
}

unsafe impl Send for StaticCaps {}
unsafe impl Sync for StaticCaps {}

impl fmt::Debug for StaticCaps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StaticCaps")
            .field("str", &unsafe {
                CStr::from_ptr(self.0.as_ref().string).to_str()
            })
            .finish()
    }
}

impl glib::types::StaticType for StaticCaps {
    fn static_type() -> glib::types::Type {
        unsafe { glib::translate::from_glib(gst_sys::gst_static_caps_get_type()) }
    }
}

#[doc(hidden)]
impl<'a> glib::value::FromValueOptional<'a> for StaticCaps {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Option::<StaticCaps>::from_glib_none(
            gobject_sys::g_value_get_boxed(value.to_glib_none().0) as *mut gst_sys::GstStaticCaps
        )
    }
}

#[doc(hidden)]
impl glib::value::SetValue for StaticCaps {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        gobject_sys::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const gst_sys::GstStaticCaps>::to_glib_none(this).0
                as glib_sys::gpointer,
        )
    }
}

#[doc(hidden)]
impl glib::value::SetValueOptional for StaticCaps {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        gobject_sys::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const gst_sys::GstStaticCaps>::to_glib_none(&this).0
                as glib_sys::gpointer,
        )
    }
}

#[doc(hidden)]
impl glib::translate::GlibPtrDefault for StaticCaps {
    type GlibType = *mut gst_sys::GstStaticCaps;
}

#[doc(hidden)]
impl<'a> glib::translate::ToGlibPtr<'a, *const gst_sys::GstStaticCaps> for StaticCaps {
    type Storage = &'a StaticCaps;

    fn to_glib_none(&'a self) -> glib::translate::Stash<'a, *const gst_sys::GstStaticCaps, Self> {
        glib::translate::Stash(self.0.as_ptr(), self)
    }

    fn to_glib_full(&self) -> *const gst_sys::GstStaticCaps {
        unimplemented!()
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*const gst_sys::GstStaticCaps> for StaticCaps {
    #[inline]
    unsafe fn from_glib_none(ptr: *const gst_sys::GstStaticCaps) -> Self {
        assert!(!ptr.is_null());
        StaticCaps(ptr::NonNull::new_unchecked(ptr as *mut _))
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*mut gst_sys::GstStaticCaps> for StaticCaps {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut gst_sys::GstStaticCaps) -> Self {
        assert!(!ptr.is_null());
        StaticCaps(ptr::NonNull::new_unchecked(ptr))
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrBorrow<*mut gst_sys::GstStaticCaps> for StaticCaps {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut gst_sys::GstStaticCaps) -> Borrowed<Self> {
        assert!(!ptr.is_null());
        Borrowed::new(StaticCaps(ptr::NonNull::new_unchecked(ptr)))
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrFull<*mut gst_sys::GstStaticCaps> for StaticCaps {
    #[inline]
    unsafe fn from_glib_full(_ptr: *mut gst_sys::GstStaticCaps) -> Self {
        unimplemented!();
    }
}
