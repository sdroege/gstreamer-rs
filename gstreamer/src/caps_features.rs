// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::{Borrow, BorrowMut, ToOwned};
use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::str;

use once_cell::sync::Lazy;

use glib;
use glib::translate::{
    from_glib, from_glib_full, FromGlibPtrFull, FromGlibPtrNone, GlibPtrDefault, Stash, StashMut,
    ToGlibPtr, ToGlibPtrMut,
};
use glib_sys::gpointer;
use gobject_sys;
use gst_sys;

pub struct CapsFeatures(ptr::NonNull<CapsFeaturesRef>);
unsafe impl Send for CapsFeatures {}
unsafe impl Sync for CapsFeatures {}

impl CapsFeatures {
    pub fn new(features: &[&str]) -> Self {
        assert_initialized_main_thread!();
        let mut f = Self::new_empty();

        for feature in features {
            f.add(feature);
        }

        f
    }

    pub fn new_empty() -> Self {
        assert_initialized_main_thread!();
        unsafe {
            CapsFeatures(ptr::NonNull::new_unchecked(
                gst_sys::gst_caps_features_new_empty() as *mut CapsFeaturesRef,
            ))
        }
    }

    pub fn new_any() -> Self {
        assert_initialized_main_thread!();
        unsafe {
            CapsFeatures(ptr::NonNull::new_unchecked(
                gst_sys::gst_caps_features_new_any() as *mut CapsFeaturesRef,
            ))
        }
    }

    pub unsafe fn into_ptr(self) -> *mut gst_sys::GstCapsFeatures {
        let s = mem::ManuallyDrop::new(self);
        s.0.as_ptr() as *mut CapsFeaturesRef as *mut gst_sys::GstCapsFeatures
    }
}

impl Deref for CapsFeatures {
    type Target = CapsFeaturesRef;

    fn deref(&self) -> &CapsFeaturesRef {
        unsafe { self.0.as_ref() }
    }
}

impl DerefMut for CapsFeatures {
    fn deref_mut(&mut self) -> &mut CapsFeaturesRef {
        unsafe { self.0.as_mut() }
    }
}

impl AsRef<CapsFeaturesRef> for CapsFeatures {
    fn as_ref(&self) -> &CapsFeaturesRef {
        self.deref()
    }
}

impl AsMut<CapsFeaturesRef> for CapsFeatures {
    fn as_mut(&mut self) -> &mut CapsFeaturesRef {
        self.deref_mut()
    }
}

impl Clone for CapsFeatures {
    fn clone(&self) -> Self {
        unsafe {
            let ptr = gst_sys::gst_caps_features_copy(&self.0.as_ref().0) as *mut CapsFeaturesRef;
            assert!(!ptr.is_null());
            CapsFeatures(ptr::NonNull::new_unchecked(ptr))
        }
    }
}

impl Drop for CapsFeatures {
    fn drop(&mut self) {
        unsafe { gst_sys::gst_caps_features_free(&mut self.0.as_mut().0) }
    }
}

impl fmt::Debug for CapsFeatures {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("CapsFeatures")
            .field(&self.to_string())
            .finish()
    }
}

impl fmt::Display for CapsFeatures {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Need to make sure to not call ToString::to_string() here, which
        // we have because of the Display impl. We need CapsFeaturesRef::to_string()
        f.write_str(&CapsFeaturesRef::to_string(self.as_ref()))
    }
}

impl str::FromStr for CapsFeatures {
    type Err = glib::BoolError;

    fn from_str(s: &str) -> Result<Self, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            let ptr = gst_sys::gst_caps_features_from_string(s.to_glib_none().0);
            if ptr.is_null() {
                return Err(glib_bool_error!(
                    "Failed to parse caps features from string"
                ));
            }

            Ok(CapsFeatures(ptr::NonNull::new_unchecked(
                ptr as *mut CapsFeaturesRef,
            )))
        }
    }
}

impl Borrow<CapsFeaturesRef> for CapsFeatures {
    fn borrow(&self) -> &CapsFeaturesRef {
        unsafe { self.0.as_ref() }
    }
}

impl BorrowMut<CapsFeaturesRef> for CapsFeatures {
    fn borrow_mut(&mut self) -> &mut CapsFeaturesRef {
        unsafe { self.0.as_mut() }
    }
}

impl glib::types::StaticType for CapsFeatures {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(gst_sys::gst_caps_features_get_type()) }
    }
}

impl<'a> ToGlibPtr<'a, *const gst_sys::GstCapsFeatures> for CapsFeatures {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *const gst_sys::GstCapsFeatures, Self> {
        unsafe { Stash(&self.0.as_ref().0, self) }
    }

    fn to_glib_full(&self) -> *const gst_sys::GstCapsFeatures {
        unsafe { gst_sys::gst_caps_features_copy(&self.0.as_ref().0) }
    }
}

impl<'a> ToGlibPtr<'a, *mut gst_sys::GstCapsFeatures> for CapsFeatures {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *mut gst_sys::GstCapsFeatures, Self> {
        unsafe { Stash(&self.0.as_ref().0 as *const _ as *mut _, self) }
    }

    fn to_glib_full(&self) -> *mut gst_sys::GstCapsFeatures {
        unsafe { gst_sys::gst_caps_features_copy(&self.0.as_ref().0) }
    }
}

impl<'a> ToGlibPtrMut<'a, *mut gst_sys::GstCapsFeatures> for CapsFeatures {
    type Storage = &'a mut Self;

    fn to_glib_none_mut(&'a mut self) -> StashMut<*mut gst_sys::GstCapsFeatures, Self> {
        unsafe { StashMut(&mut self.0.as_mut().0, self) }
    }
}

impl FromGlibPtrNone<*const gst_sys::GstCapsFeatures> for CapsFeatures {
    unsafe fn from_glib_none(ptr: *const gst_sys::GstCapsFeatures) -> Self {
        assert!(!ptr.is_null());
        let ptr = gst_sys::gst_caps_features_copy(ptr);
        assert!(!ptr.is_null());
        CapsFeatures(ptr::NonNull::new_unchecked(ptr as *mut CapsFeaturesRef))
    }
}

impl FromGlibPtrNone<*mut gst_sys::GstCapsFeatures> for CapsFeatures {
    unsafe fn from_glib_none(ptr: *mut gst_sys::GstCapsFeatures) -> Self {
        assert!(!ptr.is_null());
        let ptr = gst_sys::gst_caps_features_copy(ptr);
        assert!(!ptr.is_null());
        CapsFeatures(ptr::NonNull::new_unchecked(ptr as *mut CapsFeaturesRef))
    }
}

impl FromGlibPtrFull<*const gst_sys::GstCapsFeatures> for CapsFeatures {
    unsafe fn from_glib_full(ptr: *const gst_sys::GstCapsFeatures) -> Self {
        assert!(!ptr.is_null());
        CapsFeatures(ptr::NonNull::new_unchecked(ptr as *mut CapsFeaturesRef))
    }
}

impl FromGlibPtrFull<*mut gst_sys::GstCapsFeatures> for CapsFeatures {
    unsafe fn from_glib_full(ptr: *mut gst_sys::GstCapsFeatures) -> Self {
        assert!(!ptr.is_null());
        CapsFeatures(ptr::NonNull::new_unchecked(ptr as *mut CapsFeaturesRef))
    }
}

impl<'a> glib::value::FromValueOptional<'a> for CapsFeatures {
    unsafe fn from_value_optional(v: &'a glib::Value) -> Option<Self> {
        <&'a CapsFeaturesRef as glib::value::FromValueOptional<'a>>::from_value_optional(v)
            .map(ToOwned::to_owned)
    }
}

impl glib::value::SetValue for CapsFeatures {
    unsafe fn set_value(v: &mut glib::Value, s: &Self) {
        <CapsFeaturesRef as glib::value::SetValue>::set_value(v, s.as_ref())
    }
}

impl glib::value::SetValueOptional for CapsFeatures {
    unsafe fn set_value_optional(v: &mut glib::Value, s: Option<&Self>) {
        <CapsFeaturesRef as glib::value::SetValueOptional>::set_value_optional(
            v,
            s.map(|s| s.as_ref()),
        )
    }
}

impl GlibPtrDefault for CapsFeatures {
    type GlibType = *mut gst_sys::GstCapsFeatures;
}

#[repr(transparent)]
pub struct CapsFeaturesRef(gst_sys::GstCapsFeatures);

impl CapsFeaturesRef {
    pub unsafe fn from_glib_borrow<'a>(
        ptr: *const gst_sys::GstCapsFeatures,
    ) -> &'a CapsFeaturesRef {
        assert!(!ptr.is_null());

        &*(ptr as *mut CapsFeaturesRef)
    }

    pub unsafe fn from_glib_borrow_mut<'a>(
        ptr: *mut gst_sys::GstCapsFeatures,
    ) -> &'a mut CapsFeaturesRef {
        assert!(!ptr.is_null());

        &mut *(ptr as *mut CapsFeaturesRef)
    }

    pub unsafe fn as_ptr(&self) -> *const gst_sys::GstCapsFeatures {
        self as *const Self as *const gst_sys::GstCapsFeatures
    }

    pub unsafe fn as_mut_ptr(&self) -> *mut gst_sys::GstCapsFeatures {
        self as *const Self as *mut gst_sys::GstCapsFeatures
    }

    pub fn is_empty(&self) -> bool {
        self.get_size() == 0 && !self.is_any()
    }

    pub fn is_any(&self) -> bool {
        unsafe { from_glib(gst_sys::gst_caps_features_is_any(self.as_ptr())) }
    }

    pub fn contains(&self, feature: &str) -> bool {
        unsafe {
            from_glib(gst_sys::gst_caps_features_contains(
                self.as_ptr(),
                feature.to_glib_none().0,
            ))
        }
    }

    pub fn get_size(&self) -> u32 {
        unsafe { gst_sys::gst_caps_features_get_size(self.as_ptr()) }
    }

    pub fn get_nth(&self, idx: u32) -> Option<&str> {
        if idx >= self.get_size() {
            return None;
        }

        unsafe {
            let feature = gst_sys::gst_caps_features_get_nth(self.as_ptr(), idx);
            if feature.is_null() {
                return None;
            }

            Some(CStr::from_ptr(feature).to_str().unwrap())
        }
    }

    pub fn add(&mut self, feature: &str) {
        unsafe { gst_sys::gst_caps_features_add(self.as_mut_ptr(), feature.to_glib_none().0) }
    }

    pub fn remove(&mut self, feature: &str) {
        unsafe { gst_sys::gst_caps_features_remove(self.as_mut_ptr(), feature.to_glib_none().0) }
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    // This is not an equivalence relation with regards to ANY. Everything is equal to ANY
    pub fn is_equal(&self, other: &CapsFeaturesRef) -> bool {
        unsafe {
            from_glib(gst_sys::gst_caps_features_is_equal(
                self.as_ptr(),
                other.as_ptr(),
            ))
        }
    }
}

impl glib::types::StaticType for CapsFeaturesRef {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(gst_sys::gst_structure_get_type()) }
    }
}

impl<'a> glib::value::FromValueOptional<'a> for &'a CapsFeaturesRef {
    unsafe fn from_value_optional(v: &'a glib::Value) -> Option<Self> {
        let ptr = gobject_sys::g_value_get_boxed(v.to_glib_none().0);
        if ptr.is_null() {
            None
        } else {
            Some(CapsFeaturesRef::from_glib_borrow(
                ptr as *const gst_sys::GstCapsFeatures,
            ))
        }
    }
}

impl glib::value::SetValue for CapsFeaturesRef {
    unsafe fn set_value(v: &mut glib::Value, s: &Self) {
        gobject_sys::g_value_set_boxed(v.to_glib_none_mut().0, s.as_ptr() as gpointer);
    }
}

impl glib::value::SetValueOptional for CapsFeaturesRef {
    unsafe fn set_value_optional(v: &mut glib::Value, s: Option<&Self>) {
        if let Some(s) = s {
            gobject_sys::g_value_set_boxed(v.to_glib_none_mut().0, s.as_ptr() as gpointer);
        } else {
            gobject_sys::g_value_set_boxed(v.to_glib_none_mut().0, ptr::null_mut());
        }
    }
}

#[derive(Debug)]
pub struct Iter<'a> {
    caps_features: &'a CapsFeaturesRef,
    idx: u32,
    n_features: u32,
}

impl<'a> Iter<'a> {
    fn new(caps_features: &'a CapsFeaturesRef) -> Iter<'a> {
        skip_assert_initialized!();
        let n_features = caps_features.get_size();

        Iter {
            caps_features,
            idx: 0,
            n_features,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.n_features {
            return None;
        }

        unsafe {
            let feature = gst_sys::gst_caps_features_get_nth(self.caps_features.as_ptr(), self.idx);
            if feature.is_null() {
                return None;
            }

            self.idx += 1;

            Some(CStr::from_ptr(feature).to_str().unwrap())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.idx == self.n_features {
            return (0, Some(0));
        }

        let remaining = (self.n_features - self.idx) as usize;

        (remaining, Some(remaining))
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.n_features {
            return None;
        }

        self.n_features -= 1;

        unsafe {
            let feature =
                gst_sys::gst_caps_features_get_nth(self.caps_features.as_ptr(), self.n_features);
            if feature.is_null() {
                return None;
            }

            Some(CStr::from_ptr(feature).to_str().unwrap())
        }
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}

impl fmt::Debug for CapsFeaturesRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("CapsFeatures")
            .field(&self.to_string())
            .finish()
    }
}

impl fmt::Display for CapsFeaturesRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = unsafe {
            glib::GString::from_glib_full(gst_sys::gst_caps_features_to_string(self.as_ptr()))
        };
        f.write_str(&s)
    }
}

impl ToOwned for CapsFeaturesRef {
    type Owned = CapsFeatures;

    fn to_owned(&self) -> CapsFeatures {
        unsafe {
            from_glib_full(gst_sys::gst_caps_features_copy(self.as_ptr() as *const _) as *mut _)
        }
    }
}

unsafe impl Sync for CapsFeaturesRef {}
unsafe impl Send for CapsFeaturesRef {}

pub static CAPS_FEATURE_MEMORY_SYSTEM_MEMORY: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(gst_sys::GST_CAPS_FEATURE_MEMORY_SYSTEM_MEMORY)
        .to_str()
        .unwrap()
});
pub static CAPS_FEATURES_MEMORY_SYSTEM_MEMORY: Lazy<CapsFeatures> =
    Lazy::new(|| CapsFeatures::new(&[*CAPS_FEATURE_MEMORY_SYSTEM_MEMORY]));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_value_optional() {
        ::init().unwrap();

        let a = glib::value::Value::from(None::<&CapsFeatures>);
        assert!(a.get::<CapsFeatures>().unwrap().is_none());
        let b = glib::value::Value::from(&CapsFeatures::new_empty());
        assert!(b.get::<CapsFeatures>().unwrap().is_some());
    }
}
