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
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::str;

use ffi;
use glib;
use glib::translate::{
    from_glib, from_glib_full, from_glib_none, FromGlibPtrFull, FromGlibPtrNone, GlibPtrDefault,
    Stash, StashMut, ToGlibPtr, ToGlibPtrMut,
};
use glib_ffi::gpointer;
use gobject_ffi;

pub struct CapsFeatures(ptr::NonNull<CapsFeaturesRef>, PhantomData<CapsFeaturesRef>);
unsafe impl Send for CapsFeatures {}
unsafe impl Sync for CapsFeatures {}

impl CapsFeatures {
    pub fn new(features: &[&str]) -> Self {
        let mut f = Self::new_empty();

        for feature in features {
            f.add(feature);
        }

        f
    }

    pub fn new_empty() -> Self {
        assert_initialized_main_thread!();
        unsafe {
            CapsFeatures(
                ptr::NonNull::new_unchecked(
                    ffi::gst_caps_features_new_empty() as *mut CapsFeaturesRef
                ),
                PhantomData,
            )
        }
    }

    pub fn new_any() -> Self {
        assert_initialized_main_thread!();
        unsafe {
            CapsFeatures(
                ptr::NonNull::new_unchecked(
                    ffi::gst_caps_features_new_any() as *mut CapsFeaturesRef
                ),
                PhantomData,
            )
        }
    }

    pub fn from_string(value: &str) -> Option<Self> {
        assert_initialized_main_thread!();
        unsafe {
            let ptr = ffi::gst_caps_features_from_string(value.to_glib_none().0);
            if ptr.is_null() {
                return None;
            }

            Some(CapsFeatures(
                ptr::NonNull::new_unchecked(ptr as *mut CapsFeaturesRef),
                PhantomData,
            ))
        }
    }

    pub unsafe fn into_ptr(self) -> *mut ffi::GstCapsFeatures {
        let ptr = self.0.as_ptr() as *mut CapsFeaturesRef as *mut ffi::GstCapsFeatures;
        mem::forget(self);

        ptr
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
            let ptr = ffi::gst_caps_features_copy(&self.0.as_ref().0) as *mut CapsFeaturesRef;
            assert!(!ptr.is_null());
            CapsFeatures(ptr::NonNull::new_unchecked(ptr), PhantomData)
        }
    }
}

impl Drop for CapsFeatures {
    fn drop(&mut self) {
        unsafe { ffi::gst_caps_features_free(&mut self.0.as_mut().0) }
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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        skip_assert_initialized!();
        CapsFeatures::from_string(s).ok_or(())
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
        unsafe { from_glib(ffi::gst_caps_features_get_type()) }
    }
}

impl<'a> ToGlibPtr<'a, *const ffi::GstCapsFeatures> for CapsFeatures {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GstCapsFeatures, Self> {
        unsafe { Stash(&self.0.as_ref().0, self) }
    }

    fn to_glib_full(&self) -> *const ffi::GstCapsFeatures {
        unsafe { ffi::gst_caps_features_copy(&self.0.as_ref().0) }
    }
}

impl<'a> ToGlibPtr<'a, *mut ffi::GstCapsFeatures> for CapsFeatures {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GstCapsFeatures, Self> {
        unsafe { Stash(&self.0.as_ref().0 as *const _ as *mut _, self) }
    }

    fn to_glib_full(&self) -> *mut ffi::GstCapsFeatures {
        unsafe { ffi::gst_caps_features_copy(&self.0.as_ref().0) }
    }
}

impl<'a> ToGlibPtrMut<'a, *mut ffi::GstCapsFeatures> for CapsFeatures {
    type Storage = &'a mut Self;

    fn to_glib_none_mut(&'a mut self) -> StashMut<*mut ffi::GstCapsFeatures, Self> {
        unsafe { StashMut(&mut self.0.as_mut().0, self) }
    }
}

impl FromGlibPtrNone<*const ffi::GstCapsFeatures> for CapsFeatures {
    unsafe fn from_glib_none(ptr: *const ffi::GstCapsFeatures) -> Self {
        assert!(!ptr.is_null());
        let ptr = ffi::gst_caps_features_copy(ptr);
        assert!(!ptr.is_null());
        CapsFeatures(
            ptr::NonNull::new_unchecked(ptr as *mut CapsFeaturesRef),
            PhantomData,
        )
    }
}

impl FromGlibPtrNone<*mut ffi::GstCapsFeatures> for CapsFeatures {
    unsafe fn from_glib_none(ptr: *mut ffi::GstCapsFeatures) -> Self {
        assert!(!ptr.is_null());
        let ptr = ffi::gst_caps_features_copy(ptr);
        assert!(!ptr.is_null());
        CapsFeatures(
            ptr::NonNull::new_unchecked(ptr as *mut CapsFeaturesRef),
            PhantomData,
        )
    }
}

impl FromGlibPtrFull<*const ffi::GstCapsFeatures> for CapsFeatures {
    unsafe fn from_glib_full(ptr: *const ffi::GstCapsFeatures) -> Self {
        assert!(!ptr.is_null());
        CapsFeatures(
            ptr::NonNull::new_unchecked(ptr as *mut CapsFeaturesRef),
            PhantomData,
        )
    }
}

impl FromGlibPtrFull<*mut ffi::GstCapsFeatures> for CapsFeatures {
    unsafe fn from_glib_full(ptr: *mut ffi::GstCapsFeatures) -> Self {
        assert!(!ptr.is_null());
        CapsFeatures(
            ptr::NonNull::new_unchecked(ptr as *mut CapsFeaturesRef),
            PhantomData,
        )
    }
}

impl<'a> glib::value::FromValueOptional<'a> for CapsFeatures {
    unsafe fn from_value_optional(v: &'a glib::Value) -> Option<Self> {
        let ptr = gobject_ffi::g_value_get_boxed(v.to_glib_none().0);
        assert!(!ptr.is_null());
        from_glib_none(ptr as *const ffi::GstCapsFeatures)
    }
}

impl glib::value::SetValue for CapsFeatures {
    unsafe fn set_value(v: &mut glib::Value, s: &Self) {
        gobject_ffi::g_value_set_boxed(v.to_glib_none_mut().0, s.0.as_ptr() as gpointer);
    }
}

impl glib::value::SetValueOptional for CapsFeatures {
    unsafe fn set_value_optional(v: &mut glib::Value, s: Option<&Self>) {
        if let Some(s) = s {
            gobject_ffi::g_value_set_boxed(v.to_glib_none_mut().0, s.as_ptr() as gpointer);
        } else {
            gobject_ffi::g_value_set_boxed(v.to_glib_none_mut().0, ptr::null_mut());
        }
    }
}

impl GlibPtrDefault for CapsFeatures {
    type GlibType = *mut ffi::GstCapsFeatures;
}

#[repr(C)]
pub struct CapsFeaturesRef(ffi::GstCapsFeatures);

impl CapsFeaturesRef {
    pub unsafe fn from_glib_borrow<'a>(ptr: *const ffi::GstCapsFeatures) -> &'a CapsFeaturesRef {
        assert!(!ptr.is_null());

        &*(ptr as *mut CapsFeaturesRef)
    }

    pub unsafe fn from_glib_borrow_mut<'a>(
        ptr: *mut ffi::GstCapsFeatures,
    ) -> &'a mut CapsFeaturesRef {
        assert!(!ptr.is_null());

        &mut *(ptr as *mut CapsFeaturesRef)
    }

    pub unsafe fn as_ptr(&self) -> *const ffi::GstCapsFeatures {
        self as *const Self as *const ffi::GstCapsFeatures
    }

    pub unsafe fn as_mut_ptr(&self) -> *mut ffi::GstCapsFeatures {
        self as *const Self as *mut ffi::GstCapsFeatures
    }

    pub fn to_string(&self) -> String {
        unsafe { from_glib_full(ffi::gst_caps_features_to_string(self.as_ptr())) }
    }

    pub fn is_empty(&self) -> bool {
        self.get_size() == 0 && !self.is_any()
    }

    pub fn is_any(&self) -> bool {
        unsafe { from_glib(ffi::gst_caps_features_is_any(self.as_ptr())) }
    }

    pub fn contains(&self, feature: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_caps_features_contains(
                self.as_ptr(),
                feature.to_glib_none().0,
            ))
        }
    }

    pub fn get_size(&self) -> u32 {
        unsafe { ffi::gst_caps_features_get_size(self.as_ptr()) }
    }

    pub fn get_nth(&self, idx: u32) -> Option<&str> {
        if idx >= self.get_size() {
            return None;
        }

        unsafe {
            let feature = ffi::gst_caps_features_get_nth(self.as_ptr(), idx);
            if feature.is_null() {
                return None;
            }

            Some(CStr::from_ptr(feature).to_str().unwrap())
        }
    }

    pub fn add(&mut self, feature: &str) {
        unsafe { ffi::gst_caps_features_add(self.as_mut_ptr(), feature.to_glib_none().0) }
    }

    pub fn remove(&mut self, feature: &str) {
        unsafe { ffi::gst_caps_features_remove(self.as_mut_ptr(), feature.to_glib_none().0) }
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    // This is not an equivalence relation with regards to ANY. Everything is equal to ANY
    pub fn is_equal(&self, other: &CapsFeaturesRef) -> bool {
        unsafe {
            from_glib(ffi::gst_caps_features_is_equal(
                self.as_ptr(),
                other.as_ptr(),
            ))
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
            let feature = ffi::gst_caps_features_get_nth(self.caps_features.as_ptr(), self.idx);
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
                ffi::gst_caps_features_get_nth(self.caps_features.as_ptr(), self.n_features);
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
        f.write_str(&self.to_string())
    }
}

impl ToOwned for CapsFeaturesRef {
    type Owned = CapsFeatures;

    fn to_owned(&self) -> CapsFeatures {
        unsafe { from_glib_full(ffi::gst_caps_features_copy(self.as_ptr() as *const _) as *mut _) }
    }
}

unsafe impl Sync for CapsFeaturesRef {}
unsafe impl Send for CapsFeaturesRef {}

lazy_static! {
    pub static ref CAPS_FEATURE_MEMORY_SYSTEM_MEMORY: &'static str = unsafe {
        CStr::from_ptr(ffi::GST_CAPS_FEATURE_MEMORY_SYSTEM_MEMORY)
            .to_str()
            .unwrap()
    };
    pub static ref CAPS_FEATURES_MEMORY_SYSTEM_MEMORY: CapsFeatures =
        CapsFeatures::new(&[*CAPS_FEATURE_MEMORY_SYSTEM_MEMORY]);
}
