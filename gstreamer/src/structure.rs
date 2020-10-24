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
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::str;

use thiserror::Error;

use Fraction;

use glib;
use glib::translate::{
    from_glib, from_glib_full, FromGlibPtrFull, FromGlibPtrNone, GlibPtrDefault, Stash, StashMut,
    ToGlib, ToGlibPtr, ToGlibPtrMut,
};
use glib::value::{FromValue, FromValueOptional, SendValue, ToSendValue};
use glib_sys::gpointer;
use gobject_sys;
use gst_sys;

#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum GetError<'name> {
    #[error("GetError: Structure field with name {name} not found")]
    FieldNotFound { name: &'name str },
    #[error("GetError: Structure field with name {name} not retrieved")]
    ValueGetError {
        name: &'name str,
        #[source]
        value_get_error: glib::value::GetError,
    },
}

impl<'name> GetError<'name> {
    fn new_field_not_found(name: &'name str) -> GetError {
        skip_assert_initialized!();
        GetError::FieldNotFound { name }
    }

    fn from_value_get_error(name: &'name str, value_get_error: glib::value::GetError) -> GetError {
        skip_assert_initialized!();
        GetError::ValueGetError {
            name,
            value_get_error,
        }
    }
}

pub struct Structure(ptr::NonNull<StructureRef>);
unsafe impl Send for Structure {}
unsafe impl Sync for Structure {}

impl Structure {
    pub fn builder(name: &str) -> Builder {
        assert_initialized_main_thread!();
        Builder::new(name)
    }

    pub fn new_empty(name: &str) -> Structure {
        assert_initialized_main_thread!();
        unsafe {
            let ptr = gst_sys::gst_structure_new_empty(name.to_glib_none().0) as *mut StructureRef;
            assert!(!ptr.is_null());
            Structure(ptr::NonNull::new_unchecked(ptr))
        }
    }

    pub fn new(name: &str, values: &[(&str, &dyn ToSendValue)]) -> Structure {
        assert_initialized_main_thread!();
        let mut structure = Structure::new_empty(name);

        for &(f, v) in values {
            structure.set_value(f, v.to_send_value());
        }

        structure
    }

    pub unsafe fn into_ptr(self) -> *mut gst_sys::GstStructure {
        let s = mem::ManuallyDrop::new(self);
        s.0.as_ptr() as *mut StructureRef as *mut gst_sys::GstStructure
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_iter<'a, 'b, I>(name: &str, iter: I) -> Structure
    where
        I: IntoIterator<Item = (&'a str, &'b SendValue)>,
    {
        assert_initialized_main_thread!();
        let mut structure = Structure::new_empty(name);

        iter.into_iter().for_each(|(f, v)| unsafe {
            let mut value = v.clone().into_raw();
            gst_sys::gst_structure_take_value(
                &mut structure.0.as_mut().0,
                f.to_glib_none().0,
                &mut value,
            );
        });

        structure
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
            let ptr = gst_sys::gst_structure_copy(&self.0.as_ref().0) as *mut StructureRef;
            assert!(!ptr.is_null());
            Structure(ptr::NonNull::new_unchecked(ptr))
        }
    }
}

impl Drop for Structure {
    fn drop(&mut self) {
        unsafe { gst_sys::gst_structure_free(&mut self.0.as_mut().0) }
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
    type Err = glib::BoolError;

    fn from_str(s: &str) -> Result<Self, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            let structure = gst_sys::gst_structure_from_string(s.to_glib_none().0, ptr::null_mut());
            if structure.is_null() {
                Err(glib_bool_error!("Failed to parse structure from string"))
            } else {
                Ok(Structure(ptr::NonNull::new_unchecked(
                    structure as *mut StructureRef,
                )))
            }
        }
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
            let ptr = gst_sys::gst_structure_copy(&self.0) as *mut StructureRef;
            assert!(!ptr.is_null());
            Structure(ptr::NonNull::new_unchecked(ptr))
        }
    }
}

impl glib::types::StaticType for Structure {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(gst_sys::gst_structure_get_type()) }
    }
}

impl<'a> ToGlibPtr<'a, *const gst_sys::GstStructure> for Structure {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *const gst_sys::GstStructure, Self> {
        unsafe { Stash(&self.0.as_ref().0, self) }
    }

    fn to_glib_full(&self) -> *const gst_sys::GstStructure {
        unsafe { gst_sys::gst_structure_copy(&self.0.as_ref().0) }
    }
}

impl<'a> ToGlibPtr<'a, *mut gst_sys::GstStructure> for Structure {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *mut gst_sys::GstStructure, Self> {
        unsafe { Stash(&self.0.as_ref().0 as *const _ as *mut _, self) }
    }

    fn to_glib_full(&self) -> *mut gst_sys::GstStructure {
        unsafe { gst_sys::gst_structure_copy(&self.0.as_ref().0) }
    }
}

impl<'a> ToGlibPtrMut<'a, *mut gst_sys::GstStructure> for Structure {
    type Storage = &'a mut Self;

    fn to_glib_none_mut(&'a mut self) -> StashMut<*mut gst_sys::GstStructure, Self> {
        unsafe { StashMut(&mut self.0.as_mut().0, self) }
    }
}

impl FromGlibPtrNone<*const gst_sys::GstStructure> for Structure {
    unsafe fn from_glib_none(ptr: *const gst_sys::GstStructure) -> Self {
        assert!(!ptr.is_null());
        let ptr = gst_sys::gst_structure_copy(ptr);
        assert!(!ptr.is_null());
        Structure(ptr::NonNull::new_unchecked(ptr as *mut StructureRef))
    }
}

impl FromGlibPtrNone<*mut gst_sys::GstStructure> for Structure {
    unsafe fn from_glib_none(ptr: *mut gst_sys::GstStructure) -> Self {
        assert!(!ptr.is_null());
        let ptr = gst_sys::gst_structure_copy(ptr);
        assert!(!ptr.is_null());
        Structure(ptr::NonNull::new_unchecked(ptr as *mut StructureRef))
    }
}

impl FromGlibPtrFull<*const gst_sys::GstStructure> for Structure {
    unsafe fn from_glib_full(ptr: *const gst_sys::GstStructure) -> Self {
        assert!(!ptr.is_null());
        Structure(ptr::NonNull::new_unchecked(ptr as *mut StructureRef))
    }
}

impl FromGlibPtrFull<*mut gst_sys::GstStructure> for Structure {
    unsafe fn from_glib_full(ptr: *mut gst_sys::GstStructure) -> Self {
        assert!(!ptr.is_null());
        Structure(ptr::NonNull::new_unchecked(ptr as *mut StructureRef))
    }
}

impl<'a> glib::value::FromValueOptional<'a> for Structure {
    unsafe fn from_value_optional(v: &'a glib::Value) -> Option<Self> {
        <&'a StructureRef as glib::value::FromValueOptional<'a>>::from_value_optional(v)
            .map(ToOwned::to_owned)
    }
}

impl glib::value::SetValue for Structure {
    unsafe fn set_value(v: &mut glib::Value, s: &Self) {
        <StructureRef as glib::value::SetValue>::set_value(v, s.as_ref())
    }
}

impl glib::value::SetValueOptional for Structure {
    unsafe fn set_value_optional(v: &mut glib::Value, s: Option<&Self>) {
        <StructureRef as glib::value::SetValueOptional>::set_value_optional(
            v,
            s.map(|s| s.as_ref()),
        )
    }
}

impl GlibPtrDefault for Structure {
    type GlibType = *mut gst_sys::GstStructure;
}

#[repr(transparent)]
pub struct StructureRef(gst_sys::GstStructure);

unsafe impl Send for StructureRef {}
unsafe impl Sync for StructureRef {}

impl StructureRef {
    pub unsafe fn from_glib_borrow<'a>(ptr: *const gst_sys::GstStructure) -> &'a StructureRef {
        assert!(!ptr.is_null());

        &*(ptr as *mut StructureRef)
    }

    pub unsafe fn from_glib_borrow_mut<'a>(
        ptr: *mut gst_sys::GstStructure,
    ) -> &'a mut StructureRef {
        assert!(!ptr.is_null());

        &mut *(ptr as *mut StructureRef)
    }

    pub unsafe fn as_ptr(&self) -> *const gst_sys::GstStructure {
        self as *const Self as *const gst_sys::GstStructure
    }

    pub unsafe fn as_mut_ptr(&self) -> *mut gst_sys::GstStructure {
        self as *const Self as *mut gst_sys::GstStructure
    }

    pub fn get<'structure, 'name, T: FromValueOptional<'structure>>(
        &'structure self,
        name: &'name str,
    ) -> Result<Option<T>, GetError<'name>> {
        self.get_value(name)?
            .get()
            .map_err(|err| GetError::from_value_get_error(name, err))
    }

    pub fn get_optional<'structure, 'name, T: FromValueOptional<'structure>>(
        &'structure self,
        name: &'name str,
    ) -> Result<Option<T>, GetError<'name>> {
        let value = self.get_value(name);
        if let Ok(value) = value {
            value
                .get()
                .map_err(|err| GetError::from_value_get_error(name, err))
        } else {
            Ok(None)
        }
    }

    pub fn get_some<'structure, 'name, T: FromValue<'structure>>(
        &'structure self,
        name: &'name str,
    ) -> Result<T, GetError<'name>> {
        self.get_value(name)?
            .get_some()
            .map_err(|err| GetError::from_value_get_error(name, err))
    }

    pub fn get_value<'structure, 'name>(
        &'structure self,
        name: &'name str,
    ) -> Result<&SendValue, GetError<'name>> {
        unsafe {
            let value = gst_sys::gst_structure_get_value(&self.0, name.to_glib_none().0);

            if value.is_null() {
                return Err(GetError::new_field_not_found(name));
            }

            Ok(&*(value as *const SendValue))
        }
    }

    pub fn set<T: ToSendValue>(&mut self, name: &str, value: &T) {
        let value = value.to_send_value();
        self.set_value(name, value);
    }

    pub fn set_value(&mut self, name: &str, value: SendValue) {
        unsafe {
            let mut value = value.into_raw();
            gst_sys::gst_structure_take_value(&mut self.0, name.to_glib_none().0, &mut value);
        }
    }

    pub fn get_name<'a>(&self) -> &'a str {
        unsafe {
            CStr::from_ptr(gst_sys::gst_structure_get_name(&self.0))
                .to_str()
                .unwrap()
        }
    }

    pub fn set_name(&mut self, name: &str) {
        unsafe { gst_sys::gst_structure_set_name(&mut self.0, name.to_glib_none().0) }
    }

    pub fn has_field(&self, field: &str) -> bool {
        unsafe {
            from_glib(gst_sys::gst_structure_has_field(
                &self.0,
                field.to_glib_none().0,
            ))
        }
    }

    pub fn has_field_with_type(&self, field: &str, type_: glib::Type) -> bool {
        unsafe {
            from_glib(gst_sys::gst_structure_has_field_typed(
                &self.0,
                field.to_glib_none().0,
                type_.to_glib(),
            ))
        }
    }

    pub fn remove_field(&mut self, field: &str) {
        unsafe {
            gst_sys::gst_structure_remove_field(&mut self.0, field.to_glib_none().0);
        }
    }

    pub fn remove_fields(&mut self, fields: &[&str]) {
        for f in fields {
            self.remove_field(f)
        }
    }

    pub fn remove_all_fields(&mut self) {
        unsafe {
            gst_sys::gst_structure_remove_all_fields(&mut self.0);
        }
    }

    pub fn fields(&self) -> FieldIterator {
        FieldIterator::new(self)
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    pub fn get_nth_field_name<'a>(&self, idx: u32) -> Option<&'a str> {
        unsafe {
            let field_name = gst_sys::gst_structure_nth_field_name(&self.0, idx);
            if field_name.is_null() {
                return None;
            }

            Some(CStr::from_ptr(field_name).to_str().unwrap())
        }
    }

    pub fn n_fields(&self) -> u32 {
        unsafe { gst_sys::gst_structure_n_fields(&self.0) as u32 }
    }

    pub fn can_intersect(&self, other: &StructureRef) -> bool {
        unsafe { from_glib(gst_sys::gst_structure_can_intersect(&self.0, &other.0)) }
    }

    pub fn intersect(&self, other: &StructureRef) -> Option<Structure> {
        unsafe { from_glib_full(gst_sys::gst_structure_intersect(&self.0, &other.0)) }
    }

    pub fn is_subset(&self, superset: &StructureRef) -> bool {
        unsafe { from_glib(gst_sys::gst_structure_is_subset(&self.0, &superset.0)) }
    }

    pub fn fixate(&mut self) {
        unsafe { gst_sys::gst_structure_fixate(&mut self.0) }
    }

    pub fn fixate_field(&mut self, name: &str) -> bool {
        unsafe {
            from_glib(gst_sys::gst_structure_fixate_field(
                &mut self.0,
                name.to_glib_none().0,
            ))
        }
    }

    pub fn fixate_field_bool(&mut self, name: &str, target: bool) -> bool {
        unsafe {
            from_glib(gst_sys::gst_structure_fixate_field_boolean(
                &mut self.0,
                name.to_glib_none().0,
                target.to_glib(),
            ))
        }
    }

    pub fn fixate_field_str(&mut self, name: &str, target: &str) -> bool {
        unsafe {
            from_glib(gst_sys::gst_structure_fixate_field_string(
                &mut self.0,
                name.to_glib_none().0,
                target.to_glib_none().0,
            ))
        }
    }

    pub fn fixate_field_nearest_double(&mut self, name: &str, target: f64) -> bool {
        unsafe {
            from_glib(gst_sys::gst_structure_fixate_field_nearest_double(
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
            from_glib(gst_sys::gst_structure_fixate_field_nearest_fraction(
                &mut self.0,
                name.to_glib_none().0,
                *target.numer(),
                *target.denom(),
            ))
        }
    }

    pub fn fixate_field_nearest_int(&mut self, name: &str, target: i32) -> bool {
        unsafe {
            from_glib(gst_sys::gst_structure_fixate_field_nearest_int(
                &mut self.0,
                name.to_glib_none().0,
                target,
            ))
        }
    }
}

impl fmt::Display for StructureRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = unsafe { glib::GString::from_glib_full(gst_sys::gst_structure_to_string(&self.0)) };
        f.write_str(&s)
    }
}

impl fmt::Debug for StructureRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl PartialEq for StructureRef {
    fn eq(&self, other: &StructureRef) -> bool {
        unsafe { from_glib(gst_sys::gst_structure_is_equal(&self.0, &other.0)) }
    }
}

impl Eq for StructureRef {}

impl glib::types::StaticType for StructureRef {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(gst_sys::gst_structure_get_type()) }
    }
}

impl<'a> glib::value::FromValueOptional<'a> for &'a StructureRef {
    unsafe fn from_value_optional(v: &'a glib::Value) -> Option<Self> {
        let ptr = gobject_sys::g_value_get_boxed(v.to_glib_none().0);
        if ptr.is_null() {
            None
        } else {
            Some(StructureRef::from_glib_borrow(
                ptr as *const gst_sys::GstStructure,
            ))
        }
    }
}

impl glib::value::SetValue for StructureRef {
    unsafe fn set_value(v: &mut glib::Value, s: &Self) {
        gobject_sys::g_value_set_boxed(v.to_glib_none_mut().0, s.as_ptr() as gpointer);
    }
}

impl glib::value::SetValueOptional for StructureRef {
    unsafe fn set_value_optional(v: &mut glib::Value, s: Option<&Self>) {
        if let Some(s) = s {
            gobject_sys::g_value_set_boxed(v.to_glib_none_mut().0, s.as_ptr() as gpointer);
        } else {
            gobject_sys::g_value_set_boxed(v.to_glib_none_mut().0, ptr::null_mut());
        }
    }
}

#[derive(Debug)]
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
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
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

#[derive(Debug)]
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
    type Item = (&'static str, &'a SendValue);

    fn next(&mut self) -> Option<Self::Item> {
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

#[derive(Debug)]
pub struct Builder {
    s: Structure,
}

impl Builder {
    fn new(name: &str) -> Self {
        skip_assert_initialized!();
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
    use std::str::FromStr;

    #[test]
    fn new_set_get() {
        use glib::{value, Type};

        ::init().unwrap();

        let mut s = Structure::new_empty("test");
        assert_eq!(s.get_name(), "test");

        s.set("f1", &"abc");
        s.set("f2", &String::from("bcd"));
        s.set("f3", &123i32);

        assert_eq!(s.get::<&str>("f1"), Ok(Some("abc")));
        assert_eq!(s.get::<&str>("f2"), Ok(Some("bcd")));
        assert_eq!(s.get_some::<i32>("f3"), Ok(123i32));
        assert_eq!(s.get_optional::<&str>("f1"), Ok(Some("abc")));
        assert_eq!(s.get_optional::<&str>("f4"), Ok(None));
        assert_eq!(s.get_optional::<i32>("f3"), Ok(Some(123i32)));
        assert_eq!(s.get_optional::<i32>("f4"), Ok(None));

        assert_eq!(
            s.get::<i32>("f2"),
            Err(GetError::from_value_get_error(
                "f2",
                value::GetError::new_type_mismatch(Type::String, Type::I32),
            ))
        );
        assert_eq!(
            s.get_some::<bool>("f3"),
            Err(GetError::from_value_get_error(
                "f3",
                value::GetError::new_type_mismatch(Type::I32, Type::Bool),
            ))
        );
        assert_eq!(
            s.get::<&str>("f4"),
            Err(GetError::new_field_not_found("f4"))
        );
        assert_eq!(
            s.get_some::<i32>("f4"),
            Err(GetError::new_field_not_found("f4"))
        );

        assert_eq!(s.fields().collect::<Vec<_>>(), vec!["f1", "f2", "f3"]);

        let v = s.iter().map(|(f, v)| (f, v.clone())).collect::<Vec<_>>();
        assert_eq!(v.len(), 3);
        assert_eq!(v[0].0, "f1");
        assert_eq!(v[0].1.get::<&str>(), Ok(Some("abc")));
        assert_eq!(v[1].0, "f2");
        assert_eq!(v[1].1.get::<&str>(), Ok(Some("bcd")));
        assert_eq!(v[2].0, "f3");
        assert_eq!(v[2].1.get_some::<i32>(), Ok(123i32));

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
        assert_eq!(s.get::<&str>("f1"), Ok(Some("abc")));
        assert_eq!(s.get::<&str>("f2"), Ok(Some("bcd")));
        assert_eq!(s.get_some::<i32>("f3"), Ok(123i32));
    }

    #[test]
    fn test_string_conversion() {
        ::init().unwrap();

        let a = "Test, f1=(string)abc, f2=(uint)123;";

        let s = Structure::from_str(&a).unwrap();
        assert_eq!(s.get::<&str>("f1"), Ok(Some("abc")));
        assert_eq!(s.get_some::<u32>("f2"), Ok(123));

        assert_eq!(a, s.to_string());
    }

    #[test]
    fn test_from_value_optional() {
        ::init().unwrap();

        let a = glib::value::Value::from(None::<&Structure>);
        assert!(a.get::<Structure>().unwrap().is_none());
        let b = glib::value::Value::from(&Structure::from_str(&"foo").unwrap());
        assert!(b.get::<Structure>().unwrap().is_some());
    }

    #[test]
    fn test_new_from_iter() {
        ::init().unwrap();

        let s = Structure::builder("test")
            .field("f1", &"abc")
            .field("f2", &String::from("bcd"))
            .field("f3", &123i32)
            .build();

        let s2 = Structure::from_iter(s.get_name(), s.iter().filter(|(f, _)| *f == "f1"));

        assert_eq!(s2.get_name(), "test");
        assert_eq!(s2.get::<&str>("f1"), Ok(Some("abc")));
        assert!(s2.get::<&str>("f2").is_err());
        assert!(s2.get::<&str>("f3").is_err());
    }
}
