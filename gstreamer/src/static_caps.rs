// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Caps;

use ffi;
use glib_ffi;
use gobject_ffi;

use glib;
use glib::translate::{from_glib_full, FromGlibPtrNone, ToGlibPtr, ToGlibPtrMut};

#[repr(C)]
pub struct StaticCaps(*mut ffi::GstStaticCaps);

impl StaticCaps {
    pub fn get(&self) -> Caps {
        unsafe { from_glib_full(ffi::gst_static_caps_get(self.0)) }
    }
}

unsafe impl Send for StaticCaps {}
unsafe impl Sync for StaticCaps {}

impl glib::types::StaticType for StaticCaps {
    fn static_type() -> glib::types::Type {
        unsafe { glib::translate::from_glib(ffi::gst_static_caps_get_type()) }
    }
}

#[doc(hidden)]
impl<'a> glib::value::FromValueOptional<'a> for StaticCaps {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Option::<StaticCaps>::from_glib_none(
            gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *mut ffi::GstStaticCaps,
        )
    }
}

#[doc(hidden)]
impl glib::value::SetValue for StaticCaps {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstStaticCaps>::to_glib_none(this).0 as
                glib_ffi::gpointer,
        )
    }
}

#[doc(hidden)]
impl glib::value::SetValueOptional for StaticCaps {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstStaticCaps>::to_glib_none(&this).0 as
                glib_ffi::gpointer,
        )
    }
}

#[doc(hidden)]
impl glib::translate::GlibPtrDefault for StaticCaps {
    type GlibType = *mut ffi::GstStaticCaps;
}

#[doc(hidden)]
impl<'a> glib::translate::ToGlibPtr<'a, *const ffi::GstStaticCaps> for StaticCaps {
    type Storage = &'a StaticCaps;

    fn to_glib_none(&'a self) -> glib::translate::Stash<'a, *const ffi::GstStaticCaps, Self> {
        glib::translate::Stash(self.0, self)
    }

    fn to_glib_full(&self) -> *const ffi::GstStaticCaps {
        unimplemented!()
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*const ffi::GstStaticCaps> for StaticCaps {
    #[inline]
    unsafe fn from_glib_none(ptr: *const ffi::GstStaticCaps) -> Self {
        StaticCaps(ptr as *mut _)
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*mut ffi::GstStaticCaps> for StaticCaps {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstStaticCaps) -> Self {
        StaticCaps(ptr)
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrBorrow<*mut ffi::GstStaticCaps> for StaticCaps {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::GstStaticCaps) -> Self {
        StaticCaps(ptr)
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrFull<*mut ffi::GstStaticCaps> for StaticCaps {
    #[inline]
    unsafe fn from_glib_full(_ptr: *mut ffi::GstStaticCaps) -> Self {
        unimplemented!();
    }
}
