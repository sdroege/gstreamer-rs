// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
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

use Fraction;

use ffi;
use glib;
use glib::translate::{from_glib, from_glib_full, from_glib_none, FromGlibPtrFull, FromGlibPtrNone,
                      GlibPtrDefault, Stash, StashMut, ToGlib, ToGlibPtr, ToGlibPtrMut};
use glib::value::{FromValueOptional, SendValue, ToSendValue};
use glib_ffi::gpointer;
use gobject_ffi;

pub struct Structure(ptr::NonNull<StructureRef>, PhantomData<StructureRef>);
unsafe impl Send for Structure {}

impl Structure {
    pub fn builder(name: &str) -> Builder {
        assert_initialized_main_thread!();
        Builder::new(name)
    }

    pub fn new_empty(name: &str) -> Structure {
        assert_initialized_main_thread!();
        unsafe {
            let ptr = ffi::gst_structure_new_empty(name.to_glib_none().0) as *mut StructureRef;
            assert!(!ptr.is_null());
            Structure(ptr::NonNull::new_unchecked(ptr), PhantomData)
        }
    }

    pub fn new(name: &str, values: &[(&str, &ToSendValue)]) -> Structure {
        assert_initialized_main_thread!();
        let mut structure = Structure::new_empty(name);

        for &(f, v) in values {
            structure.set_value(f, v.to_send_value());
        }

        structure
    }

    pub fn from_string(s: &str) -> Option<Structure> {
        assert_initialized_main_thread!();
        unsafe {
            let structure = ffi::gst_structure_from_string(s.to_glib_none().0, ptr::null_mut());
            if structure.is_null() {
                None
            } else {
                Some(Structure(
                    ptr::NonNull::new_unchecked(structure as *mut StructureRef),
                    PhantomData,
                ))
            }
        }
    }

    pub unsafe fn into_ptr(self) -> *mut ffi::GstStructure {
        let ptr = self.0.as_ptr() as *mut StructureRef as *mut ffi::GstStructure;
        mem::forget(self);

        ptr
    }
}

impl Deref for Structure {
    type Target = StructureRef;

    fn deref(&self) -> &StructureRef {
        unsafe { self.0.as_ref() }
    }
}

impl DerefMut for Structure {
    fn deref_mut(&mut self) -> &mut StructureRef {
        unsafe { self.0.as_mut() }
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
        unsafe {
            let ptr = ffi::gst_structure_copy(&self.0.as_ref().0) as *mut StructureRef;
            assert!(!ptr.is_null());
            Structure(ptr::NonNull::new_unchecked(ptr), PhantomData)
        }
    }
}

impl Drop for Structure {
    fn drop(&mut self) {
        unsafe { ffi::gst_structure_free(&mut self.0.as_mut().0) }
    }
}

impl fmt::Debug for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Structure").field(&self.to_string()).finish()
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Need to make sure to not call ToString::to_string() here, which
        // we have because of the Display impl. We need StructureRef::to_string()
        f.write_str(&StructureRef::to_string(self.as_ref()))
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

impl str::FromStr for Structure {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        skip_assert_initialized!();
        Structure::from_string(s).ok_or(())
    }
}

impl Borrow<StructureRef> for Structure {
    fn borrow(&self) -> &StructureRef {
        unsafe { self.0.as_ref() }
    }
}

impl BorrowMut<StructureRef> for Structure {
    fn borrow_mut(&mut self) -> &mut StructureRef {
        unsafe { self.0.as_mut() }
    }
}

impl ToOwned for StructureRef {
    type Owned = Structure;

    fn to_owned(&self) -> Structure {
        unsafe {
            let ptr = ffi::gst_structure_copy(&self.0) as *mut StructureRef;
            assert!(!ptr.is_null());
            Structure(ptr::NonNull::new_unchecked(ptr), PhantomData)
        }
    }
}

impl glib::types::StaticType for Structure {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_structure_get_type()) }
    }
}

impl<'a> ToGlibPtr<'a, *const ffi::GstStructure> for Structure {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GstStructure, Self> {
        unsafe { Stash(&self.0.as_ref().0, self) }
    }

    fn to_glib_full(&self) -> *const ffi::GstStructure {
        unsafe { ffi::gst_structure_copy(&self.0.as_ref().0) }
    }
}

impl<'a> ToGlibPtr<'a, *mut ffi::GstStructure> for Structure {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GstStructure, Self> {
        unsafe { Stash(&self.0.as_ref().0 as *const _ as *mut _, self) }
    }

    fn to_glib_full(&self) -> *mut ffi::GstStructure {
        unsafe { ffi::gst_structure_copy(&self.0.as_ref().0) }
    }
}

impl<'a> ToGlibPtrMut<'a, *mut ffi::GstStructure> for Structure {
    type Storage = &'a mut Self;

    fn to_glib_none_mut(&'a mut self) -> StashMut<*mut ffi::GstStructure, Self> {
        unsafe { StashMut(&mut self.0.as_mut().0, self) }
    }
}

impl FromGlibPtrNone<*const ffi::GstStructure> for Structure {
    unsafe fn from_glib_none(ptr: *const ffi::GstStructure) -> Self {
        assert!(!ptr.is_null());
        let ptr = ffi::gst_structure_copy(ptr);
        assert!(!ptr.is_null());
        Structure(
            ptr::NonNull::new_unchecked(ptr as *mut StructureRef),
            PhantomData,
        )
    }
}

impl FromGlibPtrNone<*mut ffi::GstStructure> for Structure {
    unsafe fn from_glib_none(ptr: *mut ffi::GstStructure) -> Self {
        assert!(!ptr.is_null());
        let ptr = ffi::gst_structure_copy(ptr);
        assert!(!ptr.is_null());
        Structure(
            ptr::NonNull::new_unchecked(ptr as *mut StructureRef),
            PhantomData,
        )
    }
}

impl FromGlibPtrFull<*const ffi::GstStructure> for Structure {
    unsafe fn from_glib_full(ptr: *const ffi::GstStructure) -> Self {
        assert!(!ptr.is_null());
        Structure(
            ptr::NonNull::new_unchecked(ptr as *mut StructureRef),
            PhantomData,
        )
    }
}

impl FromGlibPtrFull<*mut ffi::GstStructure> for Structure {
    unsafe fn from_glib_full(ptr: *mut ffi::GstStructure) -> Self {
        assert!(!ptr.is_null());
        Structure(
            ptr::NonNull::new_unchecked(ptr as *mut StructureRef),
            PhantomData,
        )
    }
}

impl<'a> glib::value::FromValueOptional<'a> for Structure {
    unsafe fn from_value_optional(v: &'a glib::Value) -> Option<Self> {
        let ptr = gobject_ffi::g_value_get_boxed(v.to_glib_none().0);
        assert!(!ptr.is_null());
        from_glib_none(ptr as *const ffi::GstStructure)
    }
}

impl glib::value::SetValue for Structure {
    unsafe fn set_value(v: &mut glib::Value, s: &Self) {
        gobject_ffi::g_value_set_boxed(v.to_glib_none_mut().0, s.0.as_ptr() as gpointer);
    }
}

impl glib::value::SetValueOptional for Structure {
    unsafe fn set_value_optional(v: &mut glib::Value, s: Option<&Self>) {
        if let Some(s) = s {
            gobject_ffi::g_value_set_boxed(v.to_glib_none_mut().0, s.as_ptr() as gpointer);
        } else {
            gobject_ffi::g_value_set_boxed(v.to_glib_none_mut().0, ptr::null_mut());
        }
    }
}

impl GlibPtrDefault for Structure {
    type GlibType = *mut ffi::GstStructure;
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

    pub unsafe fn as_ptr(&self) -> *const ffi::GstStructure {
        self as *const Self as *const ffi::GstStructure
    }

    pub unsafe fn as_mut_ptr(&self) -> *mut ffi::GstStructure {
        self as *const Self as *mut ffi::GstStructure
    }

    pub fn to_string(&self) -> String {
        unsafe { from_glib_full(ffi::gst_structure_to_string(&self.0)) }
    }

    pub fn get<'a, T: FromValueOptional<'a>>(&'a self, name: &str) -> Option<T> {
        self.get_value(name).and_then(|v| v.get())
    }

    pub fn get_value<'a>(&'a self, name: &str) -> Option<&SendValue> {
        unsafe {
            let value = ffi::gst_structure_get_value(&self.0, name.to_glib_none().0);

            if value.is_null() {
                return None;
            }

            Some(&*(value as *const SendValue))
        }
    }

    pub fn set<T: ToSendValue>(&mut self, name: &str, value: &T) {
        let value = value.to_send_value();
        self.set_value(name, value);
    }

    pub fn set_value(&mut self, name: &str, mut value: SendValue) {
        unsafe {
            ffi::gst_structure_take_value(
                &mut self.0,
                name.to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            mem::forget(value);
        }
    }

    pub fn get_name(&self) -> &str {
        unsafe {
            CStr::from_ptr(ffi::gst_structure_get_name(&self.0))
                .to_str()
                .unwrap()
        }
    }

    pub fn set_name(&mut self, name: &str) {
        unsafe { ffi::gst_structure_set_name(&mut self.0, name.to_glib_none().0) }
    }

    pub fn has_field(&self, field: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_structure_has_field(
                &self.0,
                field.to_glib_none().0,
            ))
        }
    }

    pub fn has_field_with_type(&self, field: &str, type_: glib::Type) -> bool {
        unsafe {
            from_glib(ffi::gst_structure_has_field_typed(
                &self.0,
                field.to_glib_none().0,
                type_.to_glib(),
            ))
        }
    }

    pub fn remove_field(&mut self, field: &str) {
        unsafe {
            ffi::gst_structure_remove_field(&mut self.0, field.to_glib_none().0);
        }
    }

    pub fn remove_fields(&mut self, fields: &[&str]) {
        for f in fields {
            self.remove_field(f)
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

    pub fn get_nth_field_name(&self, idx: u32) -> Option<&str> {
        unsafe {
            let field_name = ffi::gst_structure_nth_field_name(&self.0, idx);
            if field_name.is_null() {
                return None;
            }

            Some(CStr::from_ptr(field_name).to_str().unwrap())
        }
    }

    pub fn n_fields(&self) -> u32 {
        unsafe { ffi::gst_structure_n_fields(&self.0) as u32 }
    }

    pub fn can_intersect(&self, other: &StructureRef) -> bool {
        unsafe { from_glib(ffi::gst_structure_can_intersect(&self.0, &other.0)) }
    }

    pub fn intersect(&self, other: &StructureRef) -> Option<Structure> {
        unsafe { from_glib_full(ffi::gst_structure_intersect(&self.0, &other.0)) }
    }

    pub fn is_subset(&self, superset: &StructureRef) -> bool {
        unsafe { from_glib(ffi::gst_structure_is_subset(&self.0, &superset.0)) }
    }

    pub fn fixate(&mut self) {
        unsafe { ffi::gst_structure_fixate(&mut self.0) }
    }

    pub fn fixate_field(&mut self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_structure_fixate_field(
                &mut self.0,
                name.to_glib_none().0,
            ))
        }
    }

    pub fn fixate_field_bool(&mut self, name: &str, target: bool) -> bool {
        unsafe {
            from_glib(ffi::gst_structure_fixate_field_boolean(
                &mut self.0,
                name.to_glib_none().0,
                target.to_glib(),
            ))
        }
    }

    pub fn fixate_field_str(&mut self, name: &str, target: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_structure_fixate_field_string(
                &mut self.0,
                name.to_glib_none().0,
                target.to_glib_none().0,
            ))
        }
    }

    pub fn fixate_field_nearest_double(&mut self, name: &str, target: f64) -> bool {
        unsafe {
            from_glib(ffi::gst_structure_fixate_field_nearest_double(
                &mut self.0,
                name.to_glib_none().0,
                target,
            ))
        }
    }

    pub fn fixate_field_nearest_fraction<T: Into<Fraction>>(
        &mut self,
        name: &str,
        target: T,
    ) -> bool {
        skip_assert_initialized!();

        let target = target.into();
        unsafe {
            from_glib(ffi::gst_structure_fixate_field_nearest_fraction(
                &mut self.0,
                name.to_glib_none().0,
                *target.numer(),
                *target.denom(),
            ))
        }
    }

    pub fn fixate_field_nearest_int(&mut self, name: &str, target: i32) -> bool {
        unsafe {
            from_glib(ffi::gst_structure_fixate_field_nearest_int(
                &mut self.0,
                name.to_glib_none().0,
                target,
            ))
        }
    }
}

impl fmt::Display for StructureRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string())
    }
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
    fn new(structure: &'a StructureRef) -> FieldIterator<'a> {
        skip_assert_initialized!();
        let n_fields = structure.n_fields();

        FieldIterator {
            structure,
            idx: 0,
            n_fields,
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
    fn new(structure: &'a StructureRef) -> Iter<'a> {
        skip_assert_initialized!();
        Iter {
            iter: FieldIterator::new(structure),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, &'a SendValue);

    fn next(&mut self) -> Option<(&'a str, &'a SendValue)> {
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

pub struct Builder {
    s: Structure,
}

impl Builder {
    fn new(name: &str) -> Self {
        Builder {
            s: Structure::new_empty(name),
        }
    }

    pub fn field<V: ToSendValue>(mut self, name: &str, value: &V) -> Self {
        self.s.set(name, value);
        self
    }

    pub fn build(self) -> Structure {
        self.s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_set_get() {
        ::init().unwrap();

        let mut s = Structure::new_empty("test");
        assert_eq!(s.get_name(), "test");

        s.set("f1", &"abc");
        s.set("f2", &String::from("bcd"));
        s.set("f3", &123i32);

        assert_eq!(s.get::<&str>("f1").unwrap(), "abc");
        assert_eq!(s.get::<&str>("f2").unwrap(), "bcd");
        assert_eq!(s.get::<i32>("f3").unwrap(), 123i32);

        assert_eq!(s.fields().collect::<Vec<_>>(), vec!["f1", "f2", "f3"]);

        let v = s.iter().map(|(f, v)| (f, v.clone())).collect::<Vec<_>>();
        assert_eq!(v.len(), 3);
        assert_eq!(v[0].0, "f1");
        assert_eq!(v[0].1.get::<&str>().unwrap(), "abc");
        assert_eq!(v[1].0, "f2");
        assert_eq!(v[1].1.get::<&str>().unwrap(), "bcd");
        assert_eq!(v[2].0, "f3");
        assert_eq!(v[2].1.get::<i32>().unwrap(), 123i32);

        let s2 = Structure::new("test", &[("f1", &"abc"), ("f2", &"bcd"), ("f3", &123i32)]);
        assert_eq!(s, s2);
    }

    #[test]
    fn test_builder() {
        ::init().unwrap();

        let s = Structure::builder("test")
            .field("f1", &"abc")
            .field("f2", &String::from("bcd"))
            .field("f3", &123i32)
            .build();

        assert_eq!(s.get_name(), "test");
        assert_eq!(s.get::<&str>("f1").unwrap(), "abc");
        assert_eq!(s.get::<&str>("f2").unwrap(), "bcd");
        assert_eq!(s.get::<i32>("f3").unwrap(), 123i32);
    }

    #[test]
    fn test_string_conversion() {
        let a = "Test, f1=(string)abc, f2=(uint)123;";

        let s = Structure::from_string(&a).unwrap();
        assert_eq!(s.get::<&str>("f1").unwrap(), "abc");
        assert_eq!(s.get::<u32>("f2").unwrap(), 123);

        assert_eq!(a, s.to_string());
    }
}
