// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Caps;

use glib::translate::*;

use std::ffi::CStr;
use std::fmt;
use std::ptr;

pub struct StaticCaps(ptr::NonNull<ffi::GstStaticCaps>);

impl StaticCaps {
    pub fn get(&self) -> Caps {
        unsafe { from_glib_full(ffi::gst_static_caps_get(self.0.as_ptr())) }
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
        unsafe { glib::translate::from_glib(ffi::gst_static_caps_get_type()) }
    }
}

#[doc(hidden)]
impl<'a> glib::value::FromValueOptional<'a> for StaticCaps {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Option::<StaticCaps>::from_glib_none(glib::gobject_ffi::g_value_get_boxed(
            value.to_glib_none().0,
        ) as *mut ffi::GstStaticCaps)
    }
}

#[doc(hidden)]
impl glib::value::SetValue for StaticCaps {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstStaticCaps>::to_glib_none(this).0
                as glib::ffi::gpointer,
        )
    }
}

#[doc(hidden)]
impl glib::value::SetValueOptional for StaticCaps {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        glib::gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstStaticCaps>::to_glib_none(&this).0
                as glib::ffi::gpointer,
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
        glib::translate::Stash(self.0.as_ptr(), self)
    }

    fn to_glib_full(&self) -> *const ffi::GstStaticCaps {
        unimplemented!()
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*const ffi::GstStaticCaps> for StaticCaps {
    #[inline]
    unsafe fn from_glib_none(ptr: *const ffi::GstStaticCaps) -> Self {
        assert!(!ptr.is_null());
        StaticCaps(ptr::NonNull::new_unchecked(ptr as *mut _))
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*mut ffi::GstStaticCaps> for StaticCaps {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstStaticCaps) -> Self {
        assert!(!ptr.is_null());
        StaticCaps(ptr::NonNull::new_unchecked(ptr))
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrBorrow<*mut ffi::GstStaticCaps> for StaticCaps {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::GstStaticCaps) -> Borrowed<Self> {
        assert!(!ptr.is_null());
        Borrowed::new(StaticCaps(ptr::NonNull::new_unchecked(ptr)))
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrFull<*mut ffi::GstStaticCaps> for StaticCaps {
    #[inline]
    unsafe fn from_glib_full(_ptr: *mut ffi::GstStaticCaps) -> Self {
        unimplemented!();
    }
}
