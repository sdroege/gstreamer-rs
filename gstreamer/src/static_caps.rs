// Take a look at the license at the top of the repository in the LICENSE file.

use std::{ffi::CStr, fmt, marker::PhantomData, ptr};

use glib::{translate::*, StaticType};

use crate::Caps;

#[doc(alias = "GstStaticCaps")]
#[derive(Clone, Copy)]
pub struct StaticCaps(ptr::NonNull<ffi::GstStaticCaps>);

impl StaticCaps {
    #[doc(alias = "gst_static_caps_get")]
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

impl glib::value::ValueType for StaticCaps {
    type Type = Self;
}

unsafe impl glib::translate::TransparentPtrType for StaticCaps {}

#[doc(hidden)]
unsafe impl<'a> glib::value::FromValue<'a> for StaticCaps {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib_none(
            glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *mut ffi::GstStaticCaps
        )
    }
}

#[doc(hidden)]
impl glib::value::ToValue for StaticCaps {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                self.to_glib_none().0 as *mut _,
            )
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<StaticCaps> for glib::Value {
    fn from(v: StaticCaps) -> glib::Value {
        skip_assert_initialized!();
        glib::value::ToValue::to_value(&v)
    }
}

#[doc(hidden)]
impl glib::value::ToValueOptional for StaticCaps {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                s.to_glib_none().0 as *mut _,
            )
        }
        value
    }
}

#[doc(hidden)]
impl glib::translate::GlibPtrDefault for StaticCaps {
    type GlibType = *mut ffi::GstStaticCaps;
}

#[doc(hidden)]
impl<'a> glib::translate::ToGlibPtr<'a, *const ffi::GstStaticCaps> for StaticCaps {
    type Storage = PhantomData<&'a StaticCaps>;

    fn to_glib_none(&'a self) -> glib::translate::Stash<'a, *const ffi::GstStaticCaps, Self> {
        glib::translate::Stash(self.0.as_ptr(), PhantomData)
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
