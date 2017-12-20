// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use PadTemplate;
use Caps;

use ffi;
use glib_ffi;
use gobject_ffi;

use std::ffi::CStr;
use glib;
use glib::translate::{from_glib, from_glib_full, FromGlibPtrNone, ToGlibPtr, ToGlibPtrMut};

#[repr(C)]
pub struct StaticPadTemplate(*mut ffi::GstStaticPadTemplate);

impl StaticPadTemplate {
    pub fn get(&self) -> PadTemplate {
        unsafe { from_glib_full(ffi::gst_static_pad_template_get(self.0)) }
    }

    pub fn get_caps(&self) -> Caps {
        unsafe { from_glib_full(ffi::gst_static_pad_template_get_caps(self.0)) }
    }

    pub fn name_template<'a>(&self) -> &'a str {
        unsafe { CStr::from_ptr((*self.0).name_template).to_str().unwrap() }
    }

    pub fn direction(&self) -> ::PadDirection {
        unsafe { from_glib((*self.0).direction) }
    }

    pub fn presence(&self) -> ::PadPresence {
        unsafe { from_glib((*self.0).presence) }
    }
}

unsafe impl Send for StaticPadTemplate {}
unsafe impl Sync for StaticPadTemplate {}

impl glib::types::StaticType for StaticPadTemplate {
    fn static_type() -> glib::types::Type {
        unsafe { glib::translate::from_glib(ffi::gst_static_pad_template_get_type()) }
    }
}

#[doc(hidden)]
impl<'a> glib::value::FromValueOptional<'a> for StaticPadTemplate {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Option::<StaticPadTemplate>::from_glib_none(gobject_ffi::g_value_get_boxed(
            value.to_glib_none().0,
        ) as *mut ffi::GstStaticPadTemplate)
    }
}

#[doc(hidden)]
impl glib::value::SetValue for StaticPadTemplate {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstStaticPadTemplate>::to_glib_none(this).0
                as glib_ffi::gpointer,
        )
    }
}

#[doc(hidden)]
impl glib::value::SetValueOptional for StaticPadTemplate {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstStaticPadTemplate>::to_glib_none(&this).0
                as glib_ffi::gpointer,
        )
    }
}

#[doc(hidden)]
impl glib::translate::GlibPtrDefault for StaticPadTemplate {
    type GlibType = *mut ffi::GstStaticPadTemplate;
}

#[doc(hidden)]
impl<'a> glib::translate::ToGlibPtr<'a, *const ffi::GstStaticPadTemplate> for StaticPadTemplate {
    type Storage = &'a StaticPadTemplate;

    fn to_glib_none(
        &'a self,
    ) -> glib::translate::Stash<'a, *const ffi::GstStaticPadTemplate, Self> {
        glib::translate::Stash(self.0, self)
    }

    fn to_glib_full(&self) -> *const ffi::GstStaticPadTemplate {
        unimplemented!()
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*const ffi::GstStaticPadTemplate> for StaticPadTemplate {
    #[inline]
    unsafe fn from_glib_none(ptr: *const ffi::GstStaticPadTemplate) -> Self {
        StaticPadTemplate(ptr as *mut _)
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*mut ffi::GstStaticPadTemplate> for StaticPadTemplate {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstStaticPadTemplate) -> Self {
        StaticPadTemplate(ptr)
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrBorrow<*mut ffi::GstStaticPadTemplate> for StaticPadTemplate {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::GstStaticPadTemplate) -> Self {
        StaticPadTemplate(ptr)
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrFull<*mut ffi::GstStaticPadTemplate> for StaticPadTemplate {
    #[inline]
    unsafe fn from_glib_full(_ptr: *mut ffi::GstStaticPadTemplate) -> Self {
        unimplemented!();
    }
}
