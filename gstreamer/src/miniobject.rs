// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{borrow, fmt, ops};
use std::mem;
use std::ptr;
use std::marker::PhantomData;

use ffi;
use glib_ffi::gpointer;
use glib_ffi;
use gobject_ffi;
use glib::translate::{c_ptr_array_len, from_glib, from_glib_full, from_glib_none,
                      FromGlibContainerAsVec, FromGlibPtrArrayContainerAsVec, FromGlibPtrBorrow,
                      FromGlibPtrFull, FromGlibPtrNone, GlibPtrDefault, Stash, StashMut,
                      ToGlibContainerFromSlice, ToGlibPtr, ToGlibPtrMut};
use glib;

pub struct GstRc<T: MiniObject> {
    obj: *mut T,
    borrowed: bool,
    phantom: PhantomData<T>,
}

impl<T: MiniObject> GstRc<T> {
    pub unsafe fn from_glib_none(ptr: *const T::GstType) -> Self {
        assert!(!ptr.is_null());

        ffi::gst_mini_object_ref(ptr as *mut ffi::GstMiniObject);

        GstRc {
            obj: T::from_mut_ptr(ptr as *mut T::GstType) as *mut T,
            borrowed: false,
            phantom: PhantomData,
        }
    }

    pub unsafe fn from_glib_full(ptr: *const T::GstType) -> Self {
        assert!(!ptr.is_null());

        GstRc {
            obj: T::from_mut_ptr(ptr as *mut T::GstType) as *mut T,
            borrowed: false,
            phantom: PhantomData,
        }
    }

    pub unsafe fn from_glib_borrow(ptr: *const T::GstType) -> Self {
        assert!(!ptr.is_null());

        GstRc {
            obj: T::from_mut_ptr(ptr as *mut T::GstType) as *mut T,
            borrowed: true,
            phantom: PhantomData,
        }
    }

    pub fn make_mut(&mut self) -> &mut T {
        unsafe {
            if self.is_writable() {
                return &mut *self.obj;
            }

            self.obj = T::from_mut_ptr(
                ffi::gst_mini_object_make_writable(self.as_mut_ptr() as *mut ffi::GstMiniObject) as
                    *mut T::GstType,
            );
            assert!(self.is_writable());

            &mut *self.obj
        }
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.is_writable() {
            Some(unsafe { &mut *self.obj })
        } else {
            None
        }
    }

    pub fn is_writable(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_mini_object_is_writable(
                self.as_ptr() as *const ffi::GstMiniObject,
            ))
        }
    }

    pub unsafe fn into_ptr(self) -> *mut T::GstType {
        let ptr = self.as_mut_ptr();
        mem::forget(self);

        ptr
    }
}

impl<T: MiniObject> ops::Deref for GstRc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.as_ref()
    }
}

impl<T: MiniObject> AsRef<T> for GstRc<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.obj }
    }
}

impl<T: MiniObject> borrow::Borrow<T> for GstRc<T> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

// FIXME: Not generally possible because neither T nor ToOwned are defined here...
//impl<T: MiniObject> ToOwned for T {
//    type Owned = GstRc<T>;
//
//    fn to_owned(&self) -> GstRc<T> {
//        unsafe { GstRc::from_unowned_ptr(self.as_ptr()) }
//    }
//}

impl<T: MiniObject> Clone for GstRc<T> {
    fn clone(&self) -> GstRc<T> {
        unsafe { GstRc::from_glib_none(self.as_ptr()) }
    }
}

impl<T: MiniObject> Drop for GstRc<T> {
    fn drop(&mut self) {
        if !self.borrowed {
            unsafe {
                ffi::gst_mini_object_unref(self.as_mut_ptr() as *mut ffi::GstMiniObject);
            }
        }
    }
}

unsafe impl<T: MiniObject + Sync + Send> Sync for GstRc<T> {}
unsafe impl<T: MiniObject + Sync + Send> Send for GstRc<T> {}

impl<T: MiniObject + PartialEq> PartialEq for GstRc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl<T: MiniObject + Eq> Eq for GstRc<T> {}

impl<T: MiniObject + fmt::Debug> fmt::Debug for GstRc<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T: MiniObject + fmt::Display> fmt::Display for GstRc<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

pub unsafe trait MiniObject
where
    Self: Sized,
{
    type GstType;

    unsafe fn as_ptr(&self) -> *const Self::GstType {
        self as *const Self as *const Self::GstType
    }

    unsafe fn as_mut_ptr(&self) -> *mut Self::GstType {
        self as *const Self as *mut Self::GstType
    }

    unsafe fn from_ptr<'a>(ptr: *const Self::GstType) -> &'a Self {
        assert!(!ptr.is_null());
        &*(ptr as *const Self)
    }

    unsafe fn from_mut_ptr<'a>(ptr: *mut Self::GstType) -> &'a mut Self {
        assert!(!ptr.is_null());
        &mut *(ptr as *mut Self)
    }

    fn copy(&self) -> GstRc<Self> {
        unsafe {
            GstRc::from_glib_full(
                ffi::gst_mini_object_copy(self.as_ptr() as *const ffi::GstMiniObject) as
                    *const Self::GstType,
            )
        }
    }
}

impl<'a, T: MiniObject + 'static> ToGlibPtr<'a, *const T::GstType> for GstRc<T> {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *const T::GstType, Self> {
        Stash(unsafe { self.as_ptr() }, self)
    }

    fn to_glib_full(&self) -> *const T::GstType {
        unsafe {
            ffi::gst_mini_object_ref(self.as_mut_ptr() as *mut ffi::GstMiniObject);
            self.as_ptr()
        }
    }
}

impl<'a, T: MiniObject + 'static> ToGlibPtr<'a, *mut T::GstType> for GstRc<T> {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *mut T::GstType, Self> {
        Stash(unsafe { self.as_mut_ptr() }, self)
    }

    fn to_glib_full(&self) -> *mut T::GstType {
        unsafe {
            ffi::gst_mini_object_ref(self.as_mut_ptr() as *mut ffi::GstMiniObject);
            self.as_mut_ptr()
        }
    }
}

impl<'a, T: MiniObject + 'static> ToGlibPtrMut<'a, *mut T::GstType> for GstRc<T> {
    type Storage = &'a mut Self;

    fn to_glib_none_mut(&'a mut self) -> StashMut<*mut T::GstType, Self> {
        self.make_mut();
        StashMut(unsafe { self.as_mut_ptr() }, self)
    }
}

impl<'a, T: MiniObject + 'static> ToGlibContainerFromSlice<'a, *mut *mut T::GstType> for GstRc<T> {
    #[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
    type Storage = (
        Vec<Stash<'a, *mut T::GstType, GstRc<T>>>,
        Option<Vec<*mut T::GstType>>,
    );

    fn to_glib_none_from_slice(t: &'a [GstRc<T>]) -> (*mut *mut T::GstType, Self::Storage) {
        skip_assert_initialized!();
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();
        let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
        v_ptr.push(ptr::null_mut() as *mut T::GstType);

        (v_ptr.as_ptr() as *mut *mut T::GstType, (v, Some(v_ptr)))
    }

    fn to_glib_container_from_slice(t: &'a [GstRc<T>]) -> (*mut *mut T::GstType, Self::Storage) {
        skip_assert_initialized!();
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();

        let v_ptr = unsafe {
            let v_ptr = glib_ffi::g_malloc0(mem::size_of::<*mut T::GstType>() * t.len() + 1) as
                *mut *mut T::GstType;

            for (i, s) in v.iter().enumerate() {
                ptr::write(v_ptr.offset(i as isize), s.0);
            }

            v_ptr
        };

        (v_ptr, (v, None))
    }

    fn to_glib_full_from_slice(t: &[GstRc<T>]) -> *mut *mut T::GstType {
        skip_assert_initialized!();
        unsafe {
            let v_ptr = glib_ffi::g_malloc0(mem::size_of::<*mut T::GstType>() * t.len() + 1) as
                *mut *mut T::GstType;

            for (i, s) in t.iter().enumerate() {
                ptr::write(v_ptr.offset(i as isize), s.to_glib_full());
            }

            v_ptr
        }
    }
}

impl<'a, T: MiniObject + 'static> ToGlibContainerFromSlice<'a, *const *mut T::GstType>
    for GstRc<T> {
    #[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
    type Storage = (
        Vec<Stash<'a, *mut T::GstType, GstRc<T>>>,
        Option<Vec<*mut T::GstType>>,
    );

    fn to_glib_none_from_slice(t: &'a [GstRc<T>]) -> (*const *mut T::GstType, Self::Storage) {
        skip_assert_initialized!();
        let (ptr, stash) =
            ToGlibContainerFromSlice::<'a, *mut *mut T::GstType>::to_glib_none_from_slice(t);
        (ptr as *const *mut T::GstType, stash)
    }

    fn to_glib_container_from_slice(_: &'a [GstRc<T>]) -> (*const *mut T::GstType, Self::Storage) {
        skip_assert_initialized!();
        // Can't have consumer free a *const pointer
        unimplemented!()
    }

    fn to_glib_full_from_slice(_: &[GstRc<T>]) -> *const *mut T::GstType {
        skip_assert_initialized!();
        // Can't have consumer free a *const pointer
        unimplemented!()
    }
}

impl<T: MiniObject + 'static> FromGlibPtrNone<*const T::GstType> for GstRc<T> {
    unsafe fn from_glib_none(ptr: *const T::GstType) -> Self {
        Self::from_glib_none(ptr)
    }
}

impl<T: MiniObject + 'static> FromGlibPtrNone<*mut T::GstType> for GstRc<T> {
    unsafe fn from_glib_none(ptr: *mut T::GstType) -> Self {
        Self::from_glib_none(ptr)
    }
}

impl<T: MiniObject + 'static> FromGlibPtrFull<*const T::GstType> for GstRc<T> {
    unsafe fn from_glib_full(ptr: *const T::GstType) -> Self {
        Self::from_glib_full(ptr)
    }
}

impl<T: MiniObject + 'static> FromGlibPtrFull<*mut T::GstType> for GstRc<T> {
    unsafe fn from_glib_full(ptr: *mut T::GstType) -> Self {
        Self::from_glib_full(ptr)
    }
}

impl<T: MiniObject + 'static> FromGlibPtrBorrow<*const T::GstType> for GstRc<T> {
    unsafe fn from_glib_borrow(ptr: *const T::GstType) -> Self {
        Self::from_glib_borrow(ptr)
    }
}

impl<T: MiniObject + 'static> FromGlibPtrBorrow<*mut T::GstType> for GstRc<T> {
    unsafe fn from_glib_borrow(ptr: *mut T::GstType) -> Self {
        Self::from_glib_borrow(ptr)
    }
}

impl<T: MiniObject + 'static> FromGlibContainerAsVec<*mut T::GstType, *mut *mut T::GstType>
    for GstRc<T> {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut *mut T::GstType, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib_none(ptr::read(ptr.offset(i as isize))));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut *mut T::GstType, num: usize) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib_ffi::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut *mut T::GstType, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib_full(ptr::read(ptr.offset(i as isize))));
        }
        glib_ffi::g_free(ptr as *mut _);
        res
    }
}

impl<T: MiniObject + 'static> FromGlibPtrArrayContainerAsVec<*mut T::GstType, *mut *mut T::GstType>
    for GstRc<T> {
    unsafe fn from_glib_none_as_vec(ptr: *mut *mut T::GstType) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, c_ptr_array_len(ptr))
    }

    unsafe fn from_glib_container_as_vec(ptr: *mut *mut T::GstType) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, c_ptr_array_len(ptr))
    }

    unsafe fn from_glib_full_as_vec(ptr: *mut *mut T::GstType) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, c_ptr_array_len(ptr))
    }
}

impl<T: MiniObject + 'static> FromGlibContainerAsVec<*mut T::GstType, *const *mut T::GstType>
    for GstRc<T> {
    unsafe fn from_glib_none_num_as_vec(ptr: *const *mut T::GstType, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *mut *mut _, num)
    }

    unsafe fn from_glib_container_num_as_vec(_: *const *mut T::GstType, _: usize) -> Vec<Self> {
        // Can't free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_num_as_vec(_: *const *mut T::GstType, _: usize) -> Vec<Self> {
        // Can't free a *const
        unimplemented!()
    }
}

impl<
    T: MiniObject + 'static,
> FromGlibPtrArrayContainerAsVec<*mut T::GstType, *const *mut T::GstType> for GstRc<T> {
    unsafe fn from_glib_none_as_vec(ptr: *const *mut T::GstType) -> Vec<Self> {
        FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr as *mut *mut _)
    }

    unsafe fn from_glib_container_as_vec(_: *const *mut T::GstType) -> Vec<Self> {
        // Can't free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_as_vec(_: *const *mut T::GstType) -> Vec<Self> {
        // Can't free a *const
        unimplemented!()
    }
}

impl<T: MiniObject + glib::StaticType> glib::StaticType for GstRc<T> {
    fn static_type() -> glib::types::Type {
        T::static_type()
    }
}

impl<'a, T: MiniObject + glib::StaticType + 'static> glib::value::FromValueOptional<'a>
    for GstRc<T> {
    unsafe fn from_value_optional(v: &'a glib::Value) -> Option<Self> {
        let ptr = gobject_ffi::g_value_get_boxed(v.to_glib_none().0);
        from_glib_none(ptr as *const T::GstType)
    }
}

impl<T: MiniObject + glib::StaticType> glib::value::SetValue for GstRc<T> {
    unsafe fn set_value(v: &mut glib::Value, s: &Self) {
        gobject_ffi::g_value_set_boxed(v.to_glib_none_mut().0, s.as_ptr() as gpointer);
    }
}

impl<T: MiniObject + glib::StaticType> glib::value::SetValueOptional for GstRc<T> {
    unsafe fn set_value_optional(v: &mut glib::Value, s: Option<&Self>) {
        if let Some(s) = s {
            gobject_ffi::g_value_set_boxed(v.to_glib_none_mut().0, s.as_ptr() as gpointer);
        } else {
            gobject_ffi::g_value_set_boxed(v.to_glib_none_mut().0, ptr::null_mut());
        }
    }
}

impl<T: MiniObject + 'static> GlibPtrDefault for GstRc<T> {
    type GlibType = *mut T::GstType;
}
