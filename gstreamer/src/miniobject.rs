// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{fmt, ops, borrow};
use std::mem;
use std::marker::PhantomData;

use ffi;
use glib::translate::{from_glib, Stash, StashMut, ToGlibPtr, ToGlibPtrMut, FromGlibPtrNone,
                      FromGlibPtrFull, FromGlibPtrBorrow};

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

            self.obj = T::from_mut_ptr(ffi::gst_mini_object_make_writable(
                self.as_mut_ptr() as *mut ffi::GstMiniObject,
            ) as *mut T::GstType);
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

    pub fn copy(&self) -> Self {
        unsafe {
            GstRc::from_glib_full(ffi::gst_mini_object_copy(
                self.as_ptr() as *const ffi::GstMiniObject,
            ) as *const T::GstType)
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

impl<T: MiniObject + fmt::Display> fmt::Display for GstRc<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (unsafe { &*self.obj }).fmt(f)
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
