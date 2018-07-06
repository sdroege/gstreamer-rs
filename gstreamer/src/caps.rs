// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use miniobject::*;
use std::fmt;
use std::str;
use structure::*;

use CapsIntersectMode;

use ffi;
use glib;
use glib::translate::{from_glib, from_glib_full, ToGlib, ToGlibPtr};
use glib::value::ToSendValue;

#[repr(C)]
pub struct CapsRef(ffi::GstCaps);

pub type Caps = GstRc<CapsRef>;

unsafe impl MiniObject for CapsRef {
    type GstType = ffi::GstCaps;
}

impl GstRc<CapsRef> {
    pub fn builder(name: &str) -> Builder {
        assert_initialized_main_thread!();
        Builder::new(name)
    }

    pub fn new_empty() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_caps_new_empty()) }
    }

    pub fn new_any() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_caps_new_any()) }
    }

    pub fn new_simple(name: &str, values: &[(&str, &ToSendValue)]) -> Self {
        assert_initialized_main_thread!();
        let mut caps = Caps::new_empty();

        let structure = Structure::new(name, values);
        caps.get_mut().unwrap().append_structure(structure);

        caps
    }

    pub fn from_string(value: &str) -> Option<Self> {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_caps_from_string(value.to_glib_none().0)) }
    }

    pub fn fixate(caps: Self) -> Self {
        skip_assert_initialized!();
        unsafe { from_glib_full(ffi::gst_caps_fixate(caps.into_ptr())) }
    }

    pub fn merge(caps: Self, other: Self) -> Self {
        skip_assert_initialized!();
        unsafe { from_glib_full(ffi::gst_caps_merge(caps.into_ptr(), other.into_ptr())) }
    }

    pub fn merge_structure(caps: Self, other: Structure) -> Self {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_caps_merge_structure(
                caps.into_ptr(),
                other.into_ptr(),
            ))
        }
    }

    pub fn normalize(caps: Self) -> Self {
        skip_assert_initialized!();
        unsafe { from_glib_full(ffi::gst_caps_normalize(caps.into_ptr())) }
    }

    pub fn simplify(caps: Self) -> Self {
        skip_assert_initialized!();
        unsafe { from_glib_full(ffi::gst_caps_simplify(caps.into_ptr())) }
    }

    pub fn truncate(caps: Self) -> Self {
        skip_assert_initialized!();
        unsafe { from_glib_full(ffi::gst_caps_truncate(caps.into_ptr())) }
    }
}

impl str::FromStr for Caps {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        skip_assert_initialized!();
        Caps::from_string(s).ok_or(())
    }
}

impl CapsRef {
    pub fn set_simple(&mut self, values: &[(&str, &ToSendValue)]) {
        for &(name, value) in values {
            let value = value.to_value();

            unsafe {
                ffi::gst_caps_set_value(
                    self.as_mut_ptr(),
                    name.to_glib_none().0,
                    value.to_glib_none().0,
                );
            }
        }
    }

    pub fn to_string(&self) -> String {
        unsafe { from_glib_full(ffi::gst_caps_to_string(self.as_ptr())) }
    }

    pub fn get_structure(&self, idx: u32) -> Option<&StructureRef> {
        if idx >= self.get_size() {
            return None;
        }

        unsafe {
            let structure = ffi::gst_caps_get_structure(self.as_ptr(), idx);
            if structure.is_null() {
                return None;
            }

            Some(StructureRef::from_glib_borrow(
                structure as *const ffi::GstStructure,
            ))
        }
    }

    pub fn get_mut_structure(&mut self, idx: u32) -> Option<&mut StructureRef> {
        if idx >= self.get_size() {
            return None;
        }

        unsafe {
            let structure = ffi::gst_caps_get_structure(self.as_ptr(), idx);
            if structure.is_null() {
                return None;
            }

            Some(StructureRef::from_glib_borrow_mut(
                structure as *mut ffi::GstStructure,
            ))
        }
    }

    pub fn get_size(&self) -> u32 {
        unsafe { ffi::gst_caps_get_size(self.as_ptr()) }
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    pub fn iter_mut(&mut self) -> IterMut {
        IterMut::new(self)
    }

    pub fn append_structure(&mut self, structure: Structure) {
        unsafe { ffi::gst_caps_append_structure(self.as_mut_ptr(), structure.into_ptr()) }
    }

    pub fn remove_structure(&mut self, idx: u32) {
        unsafe { ffi::gst_caps_remove_structure(self.as_mut_ptr(), idx) }
    }

    pub fn append(&mut self, other: Caps) {
        unsafe { ffi::gst_caps_append(self.as_mut_ptr(), other.into_ptr()) }
    }

    pub fn can_intersect(&self, other: &Self) -> bool {
        unsafe { from_glib(ffi::gst_caps_can_intersect(self.as_ptr(), other.as_ptr())) }
    }

    pub fn intersect(&self, other: &Self) -> Caps {
        unsafe {
            from_glib_full(ffi::gst_caps_intersect(
                self.as_mut_ptr(),
                other.as_mut_ptr(),
            ))
        }
    }

    pub fn intersect_with_mode(&self, other: &Self, mode: CapsIntersectMode) -> Caps {
        unsafe {
            from_glib_full(ffi::gst_caps_intersect_full(
                self.as_mut_ptr(),
                other.as_mut_ptr(),
                mode.to_glib(),
            ))
        }
    }

    pub fn is_always_compatible(&self, other: &Self) -> bool {
        unsafe {
            from_glib(ffi::gst_caps_is_always_compatible(
                self.as_ptr(),
                other.as_ptr(),
            ))
        }
    }

    pub fn is_any(&self) -> bool {
        unsafe { from_glib(ffi::gst_caps_is_any(self.as_ptr())) }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { from_glib(ffi::gst_caps_is_empty(self.as_ptr())) }
    }

    pub fn is_fixed(&self) -> bool {
        unsafe { from_glib(ffi::gst_caps_is_fixed(self.as_ptr())) }
    }

    pub fn is_equal_fixed(&self, other: &Self) -> bool {
        unsafe { from_glib(ffi::gst_caps_is_equal_fixed(self.as_ptr(), other.as_ptr())) }
    }

    pub fn is_strictly_equal(&self, other: &Self) -> bool {
        unsafe {
            from_glib(ffi::gst_caps_is_strictly_equal(
                self.as_ptr(),
                other.as_ptr(),
            ))
        }
    }

    pub fn is_subset(&self, superset: &Self) -> bool {
        unsafe { from_glib(ffi::gst_caps_is_subset(self.as_ptr(), superset.as_ptr())) }
    }

    pub fn is_subset_structure(&self, structure: &StructureRef) -> bool {
        unsafe {
            from_glib(ffi::gst_caps_is_subset_structure(
                self.as_ptr(),
                structure.as_ptr(),
            ))
        }
    }

    pub fn subtract(&self, other: &Self) -> Caps {
        skip_assert_initialized!();
        unsafe { from_glib_full(ffi::gst_caps_subtract(self.as_mut_ptr(), other.as_mut_ptr())) }
    }
}

impl glib::types::StaticType for CapsRef {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_caps_get_type()) }
    }
}

macro_rules! define_iter(
    ($name:ident, $typ:ty, $styp:ty) => {
    pub struct $name<'a> {
        caps: $typ,
        idx: u32,
        n_structures: u32,
    }

    impl<'a> $name<'a> {
        fn new(caps: $typ) -> $name<'a> {
            skip_assert_initialized!();
            let n_structures = caps.get_size();

            $name {
                caps: caps,
                idx: 0,
                n_structures: n_structures,
            }
        }
    }

    impl<'a> Iterator for $name<'a> {
        type Item = $styp;

        fn next(&mut self) -> Option<Self::Item> {
            if self.idx >= self.n_structures {
                return None;
            }

            unsafe {
                let structure = ffi::gst_caps_get_structure(self.caps.as_ptr(), self.idx);
                if structure.is_null() {
                    return None;
                }

                self.idx += 1;
                Some(StructureRef::from_glib_borrow_mut(
                    structure as *mut ffi::GstStructure,
                ))
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            if self.idx == self.n_structures {
                return (0, Some(0));
            }

            let remaining = (self.n_structures - self.idx) as usize;

            (remaining, Some(remaining))
        }
    }

    impl<'a> DoubleEndedIterator for $name<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.idx == self.n_structures {
                return None;
            }

            self.n_structures -= 1;

            unsafe {
                let structure = ffi::gst_caps_get_structure(self.caps.as_ptr(), self.n_structures);
                if structure.is_null() {
                    return None;
                }

                Some(StructureRef::from_glib_borrow_mut(
                    structure as *mut ffi::GstStructure,
                ))
            }
        }
    }

    impl<'a> ExactSizeIterator for $name<'a> {}
    }
);

define_iter!(Iter, &'a CapsRef, &'a StructureRef);
define_iter!(IterMut, &'a mut CapsRef, &'a mut StructureRef);

impl fmt::Debug for CapsRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Caps").field(&self.to_string()).finish()
    }
}

impl fmt::Display for CapsRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl PartialEq for CapsRef {
    fn eq(&self, other: &CapsRef) -> bool {
        unsafe { from_glib(ffi::gst_caps_is_equal(self.as_ptr(), other.as_ptr())) }
    }
}

impl Eq for CapsRef {}

impl ToOwned for CapsRef {
    type Owned = GstRc<CapsRef>;

    fn to_owned(&self) -> GstRc<CapsRef> {
        unsafe { from_glib_full(ffi::gst_mini_object_copy(self.as_ptr() as *const _) as *mut _) }
    }
}

unsafe impl Sync for CapsRef {}
unsafe impl Send for CapsRef {}

pub struct Builder {
    s: ::Structure,
}

impl Builder {
    fn new(name: &str) -> Self {
        Builder {
            s: ::Structure::new_empty(name),
        }
    }

    pub fn field<V: ToSendValue>(mut self, name: &str, value: &V) -> Self {
        self.s.set(name, value);
        self
    }

    pub fn build(self) -> Caps {
        let mut caps = Caps::new_empty();
        caps.get_mut().unwrap().append_structure(self.s);
        caps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Array;
    use Fraction;

    #[test]
    fn test_simple() {
        ::init().unwrap();

        let caps = Caps::new_simple(
            "foo/bar",
            &[
                ("int", &12),
                ("bool", &true),
                ("string", &"bla"),
                ("fraction", &Fraction::new(1, 2)),
                ("array", &Array::new(&[&1, &2])),
            ],
        );
        assert_eq!(
            caps.to_string(),
            "foo/bar, int=(int)12, bool=(boolean)true, string=(string)bla, fraction=(fraction)1/2, array=(int)< 1, 2 >"
        );

        let s = caps.get_structure(0).unwrap();
        assert_eq!(
            s,
            Structure::new(
                "foo/bar",
                &[
                    ("int", &12),
                    ("bool", &true),
                    ("string", &"bla"),
                    ("fraction", &Fraction::new(1, 2)),
                    ("array", &Array::new(&[&1, &2])),
                ],
            ).as_ref()
        );
    }

    #[test]
    fn test_builder() {
        ::init().unwrap();

        let caps = Caps::builder("foo/bar")
            .field("int", &12)
            .field("bool", &true)
            .field("string", &"bla")
            .field("fraction", &Fraction::new(1, 2))
            .field("array", &Array::new(&[&1, &2]))
            .build();
        assert_eq!(
            caps.to_string(),
            "foo/bar, int=(int)12, bool=(boolean)true, string=(string)bla, fraction=(fraction)1/2, array=(int)< 1, 2 >"
        );
    }
}
