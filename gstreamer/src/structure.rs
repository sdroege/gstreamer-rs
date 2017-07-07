// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::ptr;
use std::mem;
use std::ffi::{CStr, CString};
use std::ops::{Deref, DerefMut};
use std::borrow::{Borrow, ToOwned, BorrowMut};
use std::marker::PhantomData;

use glib;
use glib::translate::{from_glib, from_glib_full, from_glib_none, Stash, StashMut, ToGlibPtr, ToGlibPtrMut, FromGlibPtrNone, FromGlibPtrBorrow, FromGlibPtrFull};
use glib::value::{Value, ToValue, FromValueOptional};
use ffi;
use glib_ffi;

pub struct Structure(*mut StructureRef, PhantomData<StructureRef>, bool);

impl Structure {
    pub fn new_empty(name: &str) -> Structure {
        Structure(
            unsafe { ffi::gst_structure_new_empty(name.to_glib_none().0) as *mut StructureRef },
            PhantomData,
            false,
        )
    }

    pub fn new(name: &str, values: &[(&str, &Value)]) -> Structure {
        let mut structure = Structure::new_empty(name);

        for &(f, v) in values {
            structure.set_value(f, v.clone());
        }

        structure
    }

    pub fn from_string(s: &str) -> Option<Structure> {
        unsafe {
            let structure = ffi::gst_structure_from_string(s.to_glib_none().0, ptr::null_mut());
            if structure.is_null() {
                None
            } else {
                Some(Structure(structure as *mut StructureRef, PhantomData, false))
            }
        }
    }

    pub unsafe fn into_ptr(self) -> *mut ffi::GstStructure {
        let ptr = self.0 as *mut StructureRef as *mut ffi::GstStructure;
        mem::forget(self);

        ptr
    }
}

impl Deref for Structure {
    type Target = StructureRef;

    fn deref(&self) -> &StructureRef {
        unsafe { &*self.0 }
    }
}

impl DerefMut for Structure {
    fn deref_mut(&mut self) -> &mut StructureRef {
        unsafe { &mut *self.0 }
    }
}

impl AsRef<StructureRef> for Structure {
    fn as_ref(&self) -> &StructureRef {
        self.deref()
    }
}

impl AsMut<StructureRef> for Structure {
    fn as_mut(&mut self) -> &mut StructureRef {
        self.deref_mut()
    }
}

impl Clone for Structure {
    fn clone(&self) -> Self {
        Structure(
            unsafe { ffi::gst_structure_copy(&(*self.0).0) as *mut StructureRef },
            PhantomData,
            false,
        )
    }
}

impl Drop for Structure {
    fn drop(&mut self) {
        if !self.2 {
            unsafe { ffi::gst_structure_free(&mut (*self.0).0) }
        }
    }
}

impl fmt::Debug for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl PartialEq for Structure {
    fn eq(&self, other: &Structure) -> bool {
        self.as_ref().eq(other)
    }
}

impl PartialEq<StructureRef> for Structure {
    fn eq(&self, other: &StructureRef) -> bool {
        self.as_ref().eq(other)
    }
}

impl Eq for Structure {}

impl Borrow<StructureRef> for Structure {
    fn borrow(&self) -> &StructureRef {
        unsafe { &*self.0 }
    }
}

impl BorrowMut<StructureRef> for Structure {
    fn borrow_mut(&mut self) -> &mut StructureRef {
        unsafe { &mut *self.0 }
    }
}

impl ToOwned for StructureRef {
    type Owned = Structure;

    fn to_owned(&self) -> Structure {
        Structure(
            unsafe { ffi::gst_structure_copy(&self.0) as *mut StructureRef },
            PhantomData,
            false,
        )
    }
}

impl<'a> ToGlibPtr<'a, *const ffi::GstStructure> for Structure {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GstStructure, Self> {
        Stash(unsafe { &(*self.0).0 }, self)
    }

    fn to_glib_full(&self) -> *const ffi::GstStructure {
        unsafe {
            ffi::gst_structure_copy(&(*self.0).0)
        }
    }
}

impl<'a> ToGlibPtr<'a, *mut ffi::GstStructure> for Structure {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GstStructure, Self> {
        Stash(unsafe { &mut (*self.0).0 }, self)
    }

    fn to_glib_full(&self) -> *mut ffi::GstStructure {
        unsafe {
            ffi::gst_structure_copy(&(*self.0).0)
        }
    }
}

impl<'a> ToGlibPtrMut<'a, *mut ffi::GstStructure> for Structure {
    type Storage = &'a mut Self;

    fn to_glib_none_mut(&'a mut self) -> StashMut<*mut ffi::GstStructure, Self> {
        StashMut(unsafe { &mut (*self.0).0 }, self)
    }
}

impl FromGlibPtrNone<*const ffi::GstStructure> for Structure {
    unsafe fn from_glib_none(ptr: *const ffi::GstStructure) -> Self {
        Structure(
            ffi::gst_structure_copy(ptr) as *mut StructureRef,
            PhantomData,
            false,
        )
    }
}

impl FromGlibPtrNone<*mut ffi::GstStructure> for Structure {
    unsafe fn from_glib_none(ptr: *mut ffi::GstStructure) -> Self {
        Structure(
            ffi::gst_structure_copy(ptr) as *mut StructureRef,
            PhantomData,
            false,
        )
    }
}

impl FromGlibPtrFull<*const ffi::GstStructure> for Structure {
    unsafe fn from_glib_full(ptr: *const ffi::GstStructure) -> Self {
        Structure(
            ptr as *mut StructureRef,
            PhantomData,
            false,
        )
    }
}

impl FromGlibPtrFull<*mut ffi::GstStructure> for Structure {
    unsafe fn from_glib_full(ptr: *mut ffi::GstStructure) -> Self {
        Structure(
            ptr as *mut StructureRef,
            PhantomData,
            false,
        )
    }
}

impl FromGlibPtrBorrow<*const ffi::GstStructure> for Structure {
    unsafe fn from_glib_borrow(ptr: *const ffi::GstStructure) -> Self {
        Structure(
            ptr as *mut StructureRef,
            PhantomData,
            true,
        )
    }
}

impl FromGlibPtrBorrow<*mut ffi::GstStructure> for Structure {
    unsafe fn from_glib_borrow(ptr: *mut ffi::GstStructure) -> Self {
        Structure(
            ptr as *mut StructureRef,
            PhantomData,
            true,
        )
    }
}

#[repr(C)]
pub struct StructureRef(ffi::GstStructure);

impl StructureRef {
    pub unsafe fn from_glib_borrow<'a>(ptr: *const ffi::GstStructure) -> &'a StructureRef {
        assert!(!ptr.is_null());

        &*(ptr as *mut StructureRef)
    }

    pub unsafe fn from_glib_borrow_mut<'a>(ptr: *mut ffi::GstStructure) -> &'a mut StructureRef {
        assert!(!ptr.is_null());

        &mut *(ptr as *mut StructureRef)
    }

    pub fn to_string(&self) -> String {
        unsafe {
            from_glib_full(ffi::gst_structure_to_string(&self.0))
        }
    }

    pub fn get<'a, T: FromValueOptional<'a>>(&'a self, name: &str) -> Option<T> {
        self.get_value(name).and_then(|v| v.get())
    }

    pub fn get_value<'a>(&'a self, name: &str) -> Option<&Value> {
        unsafe {
            let name_cstr = CString::new(name).unwrap();

            let value = ffi::gst_structure_get_value(&self.0, name_cstr.as_ptr());

            if value.is_null() {
                return None;
            }

            Some(&*(value as *const Value))
        }
    }

    pub fn set<T: ToValue>(&mut self, name: &str, value: T) {
        let value = value.to_value();
        self.set_value(name, value);
    }

    pub fn set_value(&mut self, name: &str, mut value: Value) {
        unsafe {
            let name_cstr = CString::new(name).unwrap();
            ffi::gst_structure_take_value(&mut self.0, name_cstr.as_ptr(), value.to_glib_none_mut().0);
            mem::forget(value);
        }
    }

    pub fn get_name(&self) -> &str {
        unsafe {
            CStr::from_ptr(ffi::gst_structure_get_name(&self.0)).to_str().unwrap()
        }
    }

    pub fn has_field(&self, field: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_structure_has_field(&self.0, field.to_glib_none().0))
        }
    }

    pub fn remove_field(&mut self, field: &str) {
        unsafe {
            ffi::gst_structure_remove_field(&mut self.0, field.to_glib_none().0);
        }
    }

    pub fn remove_all_fields(&mut self) {
        unsafe {
            ffi::gst_structure_remove_all_fields(&mut self.0);
        }
    }

    pub fn fields(&self) -> FieldIterator {
        FieldIterator::new(self)
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    fn get_nth_field_name(&self, idx: u32) -> Option<&str> {
        unsafe {
            let field_name = ffi::gst_structure_nth_field_name(&self.0, idx);
            if field_name.is_null() {
                return None;
            }

            Some(CStr::from_ptr(field_name).to_str().unwrap())
        }
    }

    fn n_fields(&self) -> u32 {
        unsafe { ffi::gst_structure_n_fields(&self.0) as u32 }
    }

    // TODO: Various operations
}

impl fmt::Debug for StructureRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl PartialEq for StructureRef {
    fn eq(&self, other: &StructureRef) -> bool {
        unsafe { from_glib(ffi::gst_structure_is_equal(&self.0, &other.0)) }
    }
}

impl Eq for StructureRef {}

pub struct FieldIterator<'a> {
    structure: &'a StructureRef,
    idx: u32,
    n_fields: u32,
}

impl<'a> FieldIterator<'a> {
    pub fn new(structure: &'a StructureRef) -> FieldIterator<'a> {
        let n_fields = structure.n_fields();

        FieldIterator {
            structure: structure,
            idx: 0,
            n_fields: n_fields,
        }
    }
}

impl<'a> Iterator for FieldIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.idx >= self.n_fields {
            return None;
        }

        if let Some(field_name) = self.structure.get_nth_field_name(self.idx) {
            self.idx += 1;
            Some(field_name)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.idx == self.n_fields {
            return (0, Some(0));
        }

        let remaining = (self.n_fields - self.idx) as usize;

        (remaining, Some(remaining))
    }
}

impl<'a> DoubleEndedIterator for FieldIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.n_fields {
            return None;
        }

        self.n_fields -= 1;
        if let Some(field_name) = self.structure.get_nth_field_name(self.n_fields) {
            Some(field_name)
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for FieldIterator<'a> {}

pub struct Iter<'a> {
    iter: FieldIterator<'a>,
}

impl<'a> Iter<'a> {
    pub fn new(structure: &'a StructureRef) -> Iter<'a> {
        Iter { iter: FieldIterator::new(structure) }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, &'a Value);

    fn next(&mut self) -> Option<(&'a str, &'a Value)> {
        if let Some(f) = self.iter.next() {
            let v = self.iter.structure.get_value(f);
            Some((f, v.unwrap()))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(f) = self.iter.next_back() {
            let v = self.iter.structure.get_value(f);
            Some((f, v.unwrap()))
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_set_get() {
        ::init().unwrap();

        let mut s = Structure::new_empty("test");
        assert_eq!(s.get_name(), "test");

        s.set("f1", "abc");
        s.set("f2", String::from("bcd"));
        s.set("f3", 123i32);

        assert_eq!(s.get::<&str>("f1").unwrap(), "abc");
        assert_eq!(s.get::<&str>("f2").unwrap(), "bcd");
        assert_eq!(s.get::<i32>("f3").unwrap(), 123i32);

        assert_eq!(s.fields().collect::<Vec<_>>(), vec!["f1", "f2", "f3"]);
        /*
        assert_eq!(
            s.iter()
                .map(|(f, v)| (f, v.clone()))
                .collect::<Vec<_>>(),
            vec![
                ("f1", "abc".into()),
                ("f2","bcd".into()),
                ("f3", 123i32.into()),
            ]
        );

        let s2 = Structure::new(
            "test",
            &[
                ("f1", "abc".into()),
                ("f2", "bcd".into()),
                ("f3", 123i32.into()),
            ],
        );
        assert_eq!(s, s2);
*/
    }
}
