// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::Caps;
use crate::PadTemplate;

use glib::translate::*;
use std::ffi::CStr;

use std::fmt;
use std::ptr;

pub struct StaticPadTemplate(ptr::NonNull<ffi::GstStaticPadTemplate>);

impl StaticPadTemplate {
    pub fn get(&self) -> PadTemplate {
        unsafe { from_glib_full(ffi::gst_static_pad_template_get(self.0.as_ptr())) }
    }

    pub fn get_caps(&self) -> Caps {
        unsafe { from_glib_full(ffi::gst_static_pad_template_get_caps(self.0.as_ptr())) }
    }

    pub fn name_template<'a>(&self) -> &'a str {
        unsafe {
            CStr::from_ptr(self.0.as_ref().name_template)
                .to_str()
                .unwrap()
        }
    }

    pub fn direction(&self) -> crate::PadDirection {
        unsafe { from_glib(self.0.as_ref().direction) }
    }

    pub fn presence(&self) -> crate::PadPresence {
        unsafe { from_glib(self.0.as_ref().presence) }
    }
}

unsafe impl Send for StaticPadTemplate {}
unsafe impl Sync for StaticPadTemplate {}

impl fmt::Debug for StaticPadTemplate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StaticPadTemplate")
            .field("name_template", &unsafe {
                CStr::from_ptr(self.0.as_ref().name_template).to_str()
            })
            .field("direction", &unsafe {
                from_glib::<_, crate::PadDirection>(self.0.as_ref().direction)
            })
            .field("presence", &unsafe {
                from_glib::<_, crate::PadPresence>(self.0.as_ref().presence)
            })
            .field("static_caps", &unsafe {
                from_glib_none::<_, crate::StaticCaps>(&self.0.as_ref().static_caps as *const _)
            })
            .finish()
    }
}

impl glib::types::StaticType for StaticPadTemplate {
    fn static_type() -> glib::types::Type {
        unsafe { glib::translate::from_glib(ffi::gst_static_pad_template_get_type()) }
    }
}

#[doc(hidden)]
impl<'a> glib::value::FromValueOptional<'a> for StaticPadTemplate {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Option::<StaticPadTemplate>::from_glib_none(glib::gobject_ffi::g_value_get_boxed(
            value.to_glib_none().0,
        ) as *mut ffi::GstStaticPadTemplate)
    }
}

#[doc(hidden)]
impl glib::value::SetValue for StaticPadTemplate {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstStaticPadTemplate>::to_glib_none(this).0
                as glib::ffi::gpointer,
        )
    }
}

#[doc(hidden)]
impl glib::value::SetValueOptional for StaticPadTemplate {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        glib::gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstStaticPadTemplate>::to_glib_none(&this).0
                as glib::ffi::gpointer,
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
        glib::translate::Stash(self.0.as_ptr(), self)
    }

    fn to_glib_full(&self) -> *const ffi::GstStaticPadTemplate {
        unimplemented!()
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*const ffi::GstStaticPadTemplate> for StaticPadTemplate {
    #[inline]
    unsafe fn from_glib_none(ptr: *const ffi::GstStaticPadTemplate) -> Self {
        assert!(!ptr.is_null());
        StaticPadTemplate(ptr::NonNull::new_unchecked(ptr as *mut _))
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*mut ffi::GstStaticPadTemplate> for StaticPadTemplate {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstStaticPadTemplate) -> Self {
        assert!(!ptr.is_null());
        StaticPadTemplate(ptr::NonNull::new_unchecked(ptr))
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrBorrow<*mut ffi::GstStaticPadTemplate> for StaticPadTemplate {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::GstStaticPadTemplate) -> Borrowed<Self> {
        assert!(!ptr.is_null());
        Borrowed::new(StaticPadTemplate(ptr::NonNull::new_unchecked(ptr)))
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrFull<*mut ffi::GstStaticPadTemplate> for StaticPadTemplate {
    #[inline]
    unsafe fn from_glib_full(_ptr: *mut ffi::GstStaticPadTemplate) -> Self {
        unimplemented!();
    }
}
