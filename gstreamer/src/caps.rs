// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CString;
use std::ffi::CStr;
use std::fmt;
use miniobject::*;
use structure::*;

use glib;
use glib_ffi;
use ffi;
use glib::translate::{from_glib, from_glib_none, from_glib_full, mut_override, ToGlibPtr, ToGlib};

#[repr(C)]
pub struct CapsRef(ffi::GstCaps);

pub type Caps = GstRc<CapsRef>;

unsafe impl MiniObject for CapsRef {
    type GstType = ffi::GstCaps;
}

impl GstRc<CapsRef> {
    pub fn new_empty() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_caps_new_empty()) }
    }

    pub fn new_any() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_caps_new_any()) }
    }

    pub fn new_simple(name: &str, values: &[(&str, &glib::Value)]) -> Self {
        assert_initialized_main_thread!();
        let mut caps = Caps::new_empty();

        let structure = Structure::new(name, values);
        caps.get_mut().unwrap().append_structure(structure);

        caps
    }

    pub fn from_string(value: &str) -> Option<Self> {
        assert_initialized_main_thread!();
        unsafe {
            let caps_ptr = ffi::gst_caps_from_string(value.to_glib_none().0);

            if caps_ptr.is_null() {
                None
            } else {
                Some(from_glib_full(caps_ptr))
            }
        }
    }
}

impl CapsRef {
    pub fn set_simple(&mut self, values: &[(&str, &glib::Value)]) {
        for &(name, ref value) in values {
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

    pub fn get_mut_structure<'a>(&'a mut self, idx: u32) -> Option<&'a mut StructureRef> {
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

    pub fn append_structure(&mut self, structure: Structure) {
        unsafe { ffi::gst_caps_append_structure(self.as_mut_ptr(), structure.into_ptr()) }
    }

    pub fn get_size(&self) -> u32 {
        unsafe { ffi::gst_caps_get_size(self.as_ptr()) }
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    //pub fn iter_mut(&mut self) -> IterMut {
    //    IterMut::new(self)
    //}

    // TODO: All kinds of caps operations
}

impl glib::types::StaticType for GstRc<CapsRef> {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_caps_get_type()) }
    }
}

macro_rules! define_iter(
    ($name:ident, $typ:ty, $styp:ty, $getter:ident) => {

pub struct $name<'a> {
    caps: $typ,
    idx: u32,
    n_structures: u32,
}

impl<'a> $name<'a> {
    pub fn new(caps: $typ) -> $name<'a> {
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

        if let Some(s) = self.caps.$getter(self.idx) {
            self.idx += 1;
            Some(s)
        } else {
            None
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
        if let Some(s) = self.caps.$getter(self.n_structures) {
            Some(s)
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for $name<'a> {}
}
);

define_iter!(Iter, &'a CapsRef, &'a StructureRef, get_structure);
//define_iter!(IterMut, &'a mut CapsRef, &'a mut Structure, get_mut_structure);

impl fmt::Debug for CapsRef {
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
        unsafe { from_glib_none(self.as_ptr()) }
    }
}

unsafe impl Sync for CapsRef {}
unsafe impl Send for CapsRef {}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        gst::init();

        let caps = CapsRef::new_simple(
            "foo/bar",
            &[
                ("int", 12.into()),
                ("bool", true.into()),
                ("string", "bla".into()),
                ("fraction", (1, 2).into()),
                ("array", vec![1.into(), 2.into()].into()),
            ],
        );
        assert_eq!(
            caps.to_string(),
            "foo/bar, int=(int)12, bool=(boolean)true, string=(string)bla, \
                    fraction=(fraction)1/2, array=(int)< 1, 2 >"
        );

        let s = caps.get_structure(0).unwrap();
        assert_eq!(
            s,
            OwnedStructure::new(
                "foo/bar",
                &[
                    ("int", 12.into()),
                    ("bool", true.into()),
                    ("string", "bla".into()),
                    ("fraction", (1, 2).into()),
                    ("array", vec![1.into(), 2.into()].into()),
                ],
            ).as_ref()
        );
    }
}
*/
