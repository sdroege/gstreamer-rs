// Take a look at the license at the top of the repository in the LICENSE file.

use std::borrow::{Borrow, BorrowMut, ToOwned};
use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::str;

use once_cell::sync::Lazy;

use glib::translate::*;
use glib::StaticType;

#[doc(alias = "GstCapsFeatures")]
pub struct CapsFeatures(ptr::NonNull<ffi::GstCapsFeatures>);
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

    pub fn from_quarks(features: &[glib::Quark]) -> Self {
        assert_initialized_main_thread!();
        let mut f = Self::new_empty();

        for feature in features {
            f.add_from_quark(*feature);
        }

        f
    }

    #[doc(alias = "gst_caps_features_new_empty")]
    pub fn new_empty() -> Self {
        assert_initialized_main_thread!();
        unsafe {
            CapsFeatures(ptr::NonNull::new_unchecked(
                ffi::gst_caps_features_new_empty(),
            ))
        }
    }

    #[doc(alias = "gst_caps_features_new_any")]
    pub fn new_any() -> Self {
        assert_initialized_main_thread!();
        unsafe { CapsFeatures(ptr::NonNull::new_unchecked(ffi::gst_caps_features_new_any())) }
    }

    pub unsafe fn into_ptr(self) -> *mut ffi::GstCapsFeatures {
        let s = mem::ManuallyDrop::new(self);
        s.0.as_ptr()
    }
}

impl Deref for CapsFeatures {
    type Target = CapsFeaturesRef;

    fn deref(&self) -> &CapsFeaturesRef {
        unsafe { &*(self.0.as_ref() as *const ffi::GstCapsFeatures as *const CapsFeaturesRef) }
    }
}

impl DerefMut for CapsFeatures {
    fn deref_mut(&mut self) -> &mut CapsFeaturesRef {
        unsafe { &mut *(self.0.as_mut() as *mut ffi::GstCapsFeatures as *mut CapsFeaturesRef) }
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
            let ptr = ffi::gst_caps_features_copy(self.0.as_ref());
            assert!(!ptr.is_null());
            CapsFeatures(ptr::NonNull::new_unchecked(ptr))
        }
    }
}

impl Drop for CapsFeatures {
    fn drop(&mut self) {
        unsafe { ffi::gst_caps_features_free(self.0.as_mut()) }
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

    #[doc(alias = "gst_caps_features_from_string")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_initialized_main_thread!();
        unsafe {
            let ptr = ffi::gst_caps_features_from_string(s.to_glib_none().0);
            if ptr.is_null() {
                return Err(glib::bool_error!(
                    "Failed to parse caps features from string"
                ));
            }

            Ok(Self(ptr::NonNull::new_unchecked(ptr)))
        }
    }
}

impl Borrow<CapsFeaturesRef> for CapsFeatures {
    fn borrow(&self) -> &CapsFeaturesRef {
        self.as_ref()
    }
}

impl BorrowMut<CapsFeaturesRef> for CapsFeatures {
    fn borrow_mut(&mut self) -> &mut CapsFeaturesRef {
        self.as_mut()
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
        unsafe { Stash(self.0.as_ref(), self) }
    }

    fn to_glib_full(&self) -> *const ffi::GstCapsFeatures {
        unsafe { ffi::gst_caps_features_copy(self.0.as_ref()) }
    }
}

impl<'a> ToGlibPtr<'a, *mut ffi::GstCapsFeatures> for CapsFeatures {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GstCapsFeatures, Self> {
        unsafe {
            Stash(
                self.0.as_ref() as *const ffi::GstCapsFeatures as *mut ffi::GstCapsFeatures,
                self,
            )
        }
    }

    fn to_glib_full(&self) -> *mut ffi::GstCapsFeatures {
        unsafe { ffi::gst_caps_features_copy(self.0.as_ref()) }
    }
}

impl<'a> ToGlibPtrMut<'a, *mut ffi::GstCapsFeatures> for CapsFeatures {
    type Storage = &'a mut Self;

    fn to_glib_none_mut(&'a mut self) -> StashMut<*mut ffi::GstCapsFeatures, Self> {
        unsafe { StashMut(self.0.as_mut(), self) }
    }
}

impl FromGlibPtrNone<*const ffi::GstCapsFeatures> for CapsFeatures {
    unsafe fn from_glib_none(ptr: *const ffi::GstCapsFeatures) -> Self {
        assert!(!ptr.is_null());
        let ptr = ffi::gst_caps_features_copy(ptr);
        assert!(!ptr.is_null());
        CapsFeatures(ptr::NonNull::new_unchecked(ptr))
    }
}

impl FromGlibPtrNone<*mut ffi::GstCapsFeatures> for CapsFeatures {
    unsafe fn from_glib_none(ptr: *mut ffi::GstCapsFeatures) -> Self {
        assert!(!ptr.is_null());
        let ptr = ffi::gst_caps_features_copy(ptr);
        assert!(!ptr.is_null());
        CapsFeatures(ptr::NonNull::new_unchecked(ptr))
    }
}

impl FromGlibPtrFull<*const ffi::GstCapsFeatures> for CapsFeatures {
    unsafe fn from_glib_full(ptr: *const ffi::GstCapsFeatures) -> Self {
        assert!(!ptr.is_null());
        CapsFeatures(ptr::NonNull::new_unchecked(
            ptr as *mut ffi::GstCapsFeatures,
        ))
    }
}

impl FromGlibPtrFull<*mut ffi::GstCapsFeatures> for CapsFeatures {
    unsafe fn from_glib_full(ptr: *mut ffi::GstCapsFeatures) -> Self {
        assert!(!ptr.is_null());
        CapsFeatures(ptr::NonNull::new_unchecked(ptr))
    }
}

impl glib::value::ValueType for CapsFeatures {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for CapsFeatures {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib_none(glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0)
            as *mut ffi::GstCapsFeatures)
    }
}

impl glib::value::ToValue for CapsFeatures {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                ToGlibPtr::<*mut ffi::GstCapsFeatures>::to_glib_none(self).0 as *mut _,
            )
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl glib::value::ToValueOptional for CapsFeatures {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                ToGlibPtr::<*mut ffi::GstCapsFeatures>::to_glib_none(&s).0 as *mut _,
            )
        }
        value
    }
}

impl GlibPtrDefault for CapsFeatures {
    type GlibType = *mut ffi::GstCapsFeatures;
}

#[repr(transparent)]
#[doc(alias = "GstCapsFeatures")]
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

    pub fn is_empty(&self) -> bool {
        self.size() == 0 && !self.is_any()
    }

    #[doc(alias = "gst_caps_features_is_any")]
    pub fn is_any(&self) -> bool {
        unsafe { from_glib(ffi::gst_caps_features_is_any(self.as_ptr())) }
    }

    #[doc(alias = "gst_caps_features_contains")]
    pub fn contains(&self, feature: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_caps_features_contains(
                self.as_ptr(),
                feature.to_glib_none().0,
            ))
        }
    }

    pub fn contains_quark(&self, feature: glib::Quark) -> bool {
        unsafe {
            from_glib(ffi::gst_caps_features_contains_id(
                self.as_ptr(),
                feature.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_size")]
    #[doc(alias = "gst_caps_features_get_size")]
    pub fn size(&self) -> u32 {
        unsafe { ffi::gst_caps_features_get_size(self.as_ptr()) }
    }

    #[doc(alias = "get_nth")]
    #[doc(alias = "gst_caps_features_get_nth")]
    pub fn nth(&self, idx: u32) -> Option<&str> {
        if idx >= self.size() {
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

    pub fn nth_quark(&self, idx: u32) -> Option<glib::Quark> {
        if idx >= self.size() {
            return None;
        }

        unsafe {
            let feature = ffi::gst_caps_features_get_nth_id(self.as_ptr(), idx);
            Some(from_glib(feature))
        }
    }

    #[doc(alias = "gst_caps_features_add")]
    pub fn add(&mut self, feature: &str) {
        unsafe { ffi::gst_caps_features_add(self.as_mut_ptr(), feature.to_glib_none().0) }
    }

    #[doc(alias = "gst_caps_features_remove")]
    pub fn remove(&mut self, feature: &str) {
        unsafe { ffi::gst_caps_features_remove(self.as_mut_ptr(), feature.to_glib_none().0) }
    }

    pub fn add_from_quark(&mut self, feature: glib::Quark) {
        unsafe { ffi::gst_caps_features_add_id(self.as_mut_ptr(), feature.into_glib()) }
    }

    pub fn remove_by_quark(&mut self, feature: glib::Quark) {
        unsafe { ffi::gst_caps_features_remove_id(self.as_mut_ptr(), feature.into_glib()) }
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    // This is not an equivalence relation with regards to ANY. Everything is equal to ANY
    #[doc(alias = "gst_caps_features_is_equal")]
    pub fn is_equal(&self, other: &CapsFeaturesRef) -> bool {
        unsafe {
            from_glib(ffi::gst_caps_features_is_equal(
                self.as_ptr(),
                other.as_ptr(),
            ))
        }
    }
}

impl glib::types::StaticType for CapsFeaturesRef {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_structure_get_type()) }
    }
}

unsafe impl<'a> glib::value::FromValue<'a> for &'a CapsFeaturesRef {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        &*(glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *const CapsFeaturesRef)
    }
}

impl glib::value::ToValue for CapsFeaturesRef {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<CapsFeatures>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                self.as_mut_ptr() as *mut _,
            )
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl glib::value::ToValueOptional for CapsFeaturesRef {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<CapsFeatures>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                s.map(|s| s.as_mut_ptr()).unwrap_or(ptr::null_mut()) as *mut _,
            )
        }
        value
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
        let n_features = caps_features.size();

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
        let s = unsafe {
            glib::GString::from_glib_full(ffi::gst_caps_features_to_string(self.as_ptr()))
        };
        f.write_str(&s)
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

pub static CAPS_FEATURE_MEMORY_SYSTEM_MEMORY: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_CAPS_FEATURE_MEMORY_SYSTEM_MEMORY)
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
        use glib::ToValue;

        crate::init().unwrap();

        let a = None::<CapsFeatures>.to_value();
        assert!(a.get::<Option<CapsFeatures>>().unwrap().is_none());
        let b = glib::value::Value::from(&CapsFeatures::new_empty());
        assert!(b.get::<Option<CapsFeatures>>().unwrap().is_some());
    }
}
