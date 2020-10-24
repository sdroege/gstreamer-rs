// Copyright (C) 2016-2020 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_export]
macro_rules! gst_define_mini_object_wrapper(
    ($name:ident, $ref_name:ident, $gst_sys_name:path, $get_type:expr) => {
        pub struct $name {
            obj: ::std::ptr::NonNull<$ref_name>,
        }

        #[repr(transparent)]
        pub struct $ref_name($gst_sys_name);

        impl $name {
            pub unsafe fn from_glib_none(ptr: *const $gst_sys_name) -> Self {
                skip_assert_initialized!();
                assert!(!ptr.is_null());

                $crate::gst_sys::gst_mini_object_ref(ptr as *mut $crate::gst_sys::GstMiniObject);

                $name {
                    obj: ::std::ptr::NonNull::new_unchecked(ptr as *mut $gst_sys_name as *mut $ref_name),
                }
            }

            pub unsafe fn from_glib_full(ptr: *const $gst_sys_name) -> Self {
                skip_assert_initialized!();
                assert!(!ptr.is_null());

                $name {
                    obj: ::std::ptr::NonNull::new_unchecked(ptr as *mut $gst_sys_name as *mut $ref_name),
                }
            }

            pub unsafe fn from_glib_borrow(ptr: *const $gst_sys_name) -> $crate::glib::translate::Borrowed<Self> {
                skip_assert_initialized!();
                assert!(!ptr.is_null());

                $crate::glib::translate::Borrowed::new($name {
                    obj: ::std::ptr::NonNull::new_unchecked(ptr as *mut $gst_sys_name as *mut $ref_name),
                })
            }

            pub unsafe fn replace_ptr(&mut self, ptr: *mut $gst_sys_name) {
                assert!(!ptr.is_null());
                self.obj = ::std::ptr::NonNull::new_unchecked(ptr as *mut $ref_name);
            }

            pub fn make_mut(&mut self) -> &mut $ref_name {
                unsafe {
                    if self.is_writable() {
                        return self.obj.as_mut();
                    }

                    let ptr = $crate::gst_sys::gst_mini_object_make_writable(
                        self.as_mut_ptr() as *mut $crate::gst_sys::GstMiniObject
                    );
                    self.replace_ptr(ptr as *mut $gst_sys_name);
                    assert!(self.is_writable());

                    self.obj.as_mut()
                }
            }

            pub fn get_mut(&mut self) -> Option<&mut $ref_name> {
                if self.is_writable() {
                    Some(unsafe { self.obj.as_mut() })
                } else {
                    None
                }
            }

            pub fn is_writable(&self) -> bool {
                unsafe {
                    $crate::glib::translate::from_glib($crate::gst_sys::gst_mini_object_is_writable(
                        self.as_ptr() as *const $crate::gst_sys::GstMiniObject
                    ))
                }
            }

            pub unsafe fn into_ptr(self) -> *mut $gst_sys_name {
                let s = ::std::mem::ManuallyDrop::new(self);
                s.as_mut_ptr()
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                unsafe { $name::from_glib_none(self.as_ptr()) }
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    $crate::gst_sys::gst_mini_object_unref(self.as_mut_ptr() as *mut $crate::gst_sys::GstMiniObject);
                }
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = $ref_name;

            fn deref(&self) -> &Self::Target {
                unsafe { &*(self.obj.as_ptr() as *const Self::Target) }
            }
        }

        impl AsRef<$ref_name> for $name {
            fn as_ref(&self) -> &$ref_name {
                &*self
            }
        }

        impl ::std::borrow::Borrow<$ref_name> for $name {
            fn borrow(&self) -> &$ref_name {
                &*self
            }
        }

        impl $crate::glib::types::StaticType for $name {
            fn static_type() -> $crate::glib::types::Type {
                $ref_name::static_type()
            }
        }

        impl<'a> $crate::glib::translate::ToGlibPtr<'a, *const $gst_sys_name> for $name {
            type Storage = &'a Self;

            fn to_glib_none(&'a self) -> $crate::glib::translate::Stash<'a, *const $gst_sys_name, Self> {
                $crate::glib::translate::Stash(unsafe { self.as_ptr() }, self)
            }

            fn to_glib_full(&self) -> *const $gst_sys_name {
                unsafe {
                    $crate::gst_sys::gst_mini_object_ref(self.as_mut_ptr() as *mut $crate::gst_sys::GstMiniObject);
                    self.as_ptr()
                }
            }
        }

        impl<'a> $crate::glib::translate::ToGlibPtr<'a, *mut $gst_sys_name> for $name {
            type Storage = &'a Self;

            fn to_glib_none(&'a self) -> $crate::glib::translate::Stash<'a, *mut $gst_sys_name, Self> {
                $crate::glib::translate::Stash(unsafe { self.as_mut_ptr() }, self)
            }

            fn to_glib_full(&self) -> *mut $gst_sys_name {
                unsafe {
                    $crate::gst_sys::gst_mini_object_ref(self.as_mut_ptr() as *mut $crate::gst_sys::GstMiniObject);
                    self.as_mut_ptr()
                }
            }
        }

        impl<'a> $crate::glib::translate::ToGlibPtrMut<'a, *mut $gst_sys_name> for $name {
            type Storage = &'a mut Self;

            fn to_glib_none_mut(&'a mut self) -> $crate::glib::translate::StashMut<*mut $gst_sys_name, Self> {
                self.make_mut();
                $crate::glib::translate::StashMut(unsafe { self.as_mut_ptr() }, self)
            }
        }

        impl<'a> $crate::glib::translate::ToGlibContainerFromSlice<'a, *mut *mut $gst_sys_name> for $name {
            #[allow(clippy::type_complexity)]
            type Storage = (
                Vec<$crate::glib::translate::Stash<'a, *mut $gst_sys_name, Self>>,
                Option<Vec<*mut $gst_sys_name>>,
            );

            fn to_glib_none_from_slice(t: &'a [$name]) -> (*mut *mut $gst_sys_name, Self::Storage) {
                skip_assert_initialized!();
                let v: Vec<_> = t.iter().map(|s| $crate::glib::translate::ToGlibPtr::to_glib_none(s)).collect();
                let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
                v_ptr.push(::std::ptr::null_mut() as *mut $gst_sys_name);

                (v_ptr.as_ptr() as *mut *mut $gst_sys_name, (v, Some(v_ptr)))
            }

            fn to_glib_container_from_slice(t: &'a [$name]) -> (*mut *mut $gst_sys_name, Self::Storage) {
                skip_assert_initialized!();
                let v: Vec<_> = t.iter().map(|s| $crate::glib::translate::ToGlibPtr::to_glib_none(s)).collect();

                let v_ptr = unsafe {
                    let v_ptr = $crate::glib_sys::g_malloc0(::std::mem::size_of::<*mut $gst_sys_name>() * t.len() + 1)
                        as *mut *mut $gst_sys_name;

                    for (i, s) in v.iter().enumerate() {
                        ::std::ptr::write(v_ptr.add(i), s.0);
                    }

                    v_ptr
                };

                (v_ptr, (v, None))
            }

            fn to_glib_full_from_slice(t: &[$name]) -> *mut *mut $gst_sys_name {
                skip_assert_initialized!();
                unsafe {
                    let v_ptr = $crate::glib_sys::g_malloc0(::std::mem::size_of::<*mut $gst_sys_name>() * t.len() + 1)
                        as *mut *mut $gst_sys_name;

                    for (i, s) in t.iter().enumerate() {
                        ::std::ptr::write(v_ptr.add(i), $crate::glib::translate::ToGlibPtr::to_glib_full(s));
                    }

                    v_ptr
                }
            }
        }

        impl<'a> $crate::glib::translate::ToGlibContainerFromSlice<'a, *const *mut $gst_sys_name>
            for $name
        {
            #[allow(clippy::type_complexity)]
            type Storage = (
                Vec<$crate::glib::translate::Stash<'a, *mut $gst_sys_name, $name>>,
                Option<Vec<*mut $gst_sys_name>>,
            );

            fn to_glib_none_from_slice(t: &'a [$name]) -> (*const *mut $gst_sys_name, Self::Storage) {
                skip_assert_initialized!();
                let (ptr, stash) =
                    $crate::glib::translate::ToGlibContainerFromSlice::<'a, *mut *mut $gst_sys_name>::to_glib_none_from_slice(t);
                (ptr as *const *mut $gst_sys_name, stash)
            }

            fn to_glib_container_from_slice(_: &'a [$name]) -> (*const *mut $gst_sys_name, Self::Storage) {
                skip_assert_initialized!();
                // Can't have consumer free a *const pointer
                unimplemented!()
            }

            fn to_glib_full_from_slice(_: &[$name]) -> *const *mut $gst_sys_name {
                skip_assert_initialized!();
                // Can't have consumer free a *const pointer
                unimplemented!()
            }
        }

        impl $crate::glib::translate::FromGlibPtrNone<*const $gst_sys_name> for $name {
            unsafe fn from_glib_none(ptr: *const $gst_sys_name) -> Self {
                Self::from_glib_none(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrNone<*mut $gst_sys_name> for $name {
            unsafe fn from_glib_none(ptr: *mut $gst_sys_name) -> Self {
                Self::from_glib_none(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrFull<*const $gst_sys_name> for $name {
            unsafe fn from_glib_full(ptr: *const $gst_sys_name) -> Self {
                Self::from_glib_full(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrFull<*mut $gst_sys_name> for $name {
            unsafe fn from_glib_full(ptr: *mut $gst_sys_name) -> Self {
                Self::from_glib_full(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrBorrow<*const $gst_sys_name> for $name {
            unsafe fn from_glib_borrow(ptr: *const $gst_sys_name) -> $crate::glib::translate::Borrowed<Self> {
                Self::from_glib_borrow(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrBorrow<*mut $gst_sys_name> for $name {
            unsafe fn from_glib_borrow(ptr: *mut $gst_sys_name) -> $crate::glib::translate::Borrowed<Self> {
                Self::from_glib_borrow(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibContainerAsVec<*mut $gst_sys_name, *mut *mut $gst_sys_name>
            for $name
        {
            unsafe fn from_glib_none_num_as_vec(ptr: *mut *mut $gst_sys_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push($crate::glib::translate::from_glib_none(::std::ptr::read(ptr.add(i))));
                }
                res
            }

            unsafe fn from_glib_container_num_as_vec(ptr: *mut *mut $gst_sys_name, num: usize) -> Vec<Self> {
                let res = $crate::glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
                $crate::glib_sys::g_free(ptr as *mut _);
                res
            }

            unsafe fn from_glib_full_num_as_vec(ptr: *mut *mut $gst_sys_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push($crate::glib::translate::from_glib_full(::std::ptr::read(ptr.add(i))));
                }
                $crate::glib_sys::g_free(ptr as *mut _);
                res
            }
        }

        impl $crate::glib::translate::FromGlibPtrArrayContainerAsVec<*mut $gst_sys_name, *mut *mut $gst_sys_name>
            for $name
        {
            unsafe fn from_glib_none_as_vec(ptr: *mut *mut $gst_sys_name) -> Vec<Self> {
                $crate::glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, glib::translate::c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_container_as_vec(ptr: *mut *mut $gst_sys_name) -> Vec<Self> {
                $crate::glib::translate::FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, glib::translate::c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_full_as_vec(ptr: *mut *mut $gst_sys_name) -> Vec<Self> {
                $crate::glib::translate::FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, glib::translate::c_ptr_array_len(ptr))
            }
        }

        impl $crate::glib::translate::FromGlibContainerAsVec<*mut $gst_sys_name, *const *mut $gst_sys_name>
            for $name
        {
            unsafe fn from_glib_none_num_as_vec(ptr: *const *mut $gst_sys_name, num: usize) -> Vec<Self> {
                $crate::glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *mut *mut _, num)
            }

            unsafe fn from_glib_container_num_as_vec(_: *const *mut $gst_sys_name, _: usize) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }

            unsafe fn from_glib_full_num_as_vec(_: *const *mut $gst_sys_name, _: usize) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }
        }

        impl $crate::glib::translate::FromGlibPtrArrayContainerAsVec<*mut $gst_sys_name, *const *mut $gst_sys_name> for $name
        {
            unsafe fn from_glib_none_as_vec(ptr: *const *mut $gst_sys_name) -> Vec<Self> {
                $crate::glib::translate::FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr as *mut *mut _)
            }

            unsafe fn from_glib_container_as_vec(_: *const *mut $gst_sys_name) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }

            unsafe fn from_glib_full_as_vec(_: *const *mut $gst_sys_name) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }
        }

        impl<'a> $crate::glib::value::FromValueOptional<'a>
            for $name
        {
            unsafe fn from_value_optional(v: &'a glib::Value) -> Option<Self> {
                let ptr = $crate::gobject_sys::g_value_get_boxed($crate::glib::translate::ToGlibPtr::to_glib_none(v).0);
                $crate::glib::translate::from_glib_none(ptr as *const $gst_sys_name)
            }
        }

        impl $crate::glib::value::SetValue for $name {
            unsafe fn set_value(v: &mut glib::Value, s: &Self) {
                $crate::gobject_sys::g_value_set_boxed($crate::glib::translate::ToGlibPtrMut::to_glib_none_mut(v).0, s.as_ptr() as $crate::glib_sys::gpointer);
            }
        }

        impl $crate::glib::value::SetValueOptional for $name {
            unsafe fn set_value_optional(v: &mut glib::Value, s: Option<&Self>) {
                if let Some(s) = s {
                    $crate::gobject_sys::g_value_set_boxed($crate::glib::translate::ToGlibPtrMut::to_glib_none_mut(v).0, s.as_ptr() as $crate::glib_sys::gpointer);
                } else {
                    $crate::gobject_sys::g_value_set_boxed($crate::glib::translate::ToGlibPtrMut::to_glib_none_mut(v).0, ::std::ptr::null_mut());
                }
            }
        }

        impl $crate::glib::translate::GlibPtrDefault for $name {
            type GlibType = *mut $gst_sys_name;
        }

        impl $ref_name {
            pub unsafe fn as_ptr(&self) -> *const $gst_sys_name {
                self as *const Self as *const $gst_sys_name
            }

            pub unsafe fn as_mut_ptr(&self) -> *mut $gst_sys_name {
                self as *const Self as *mut $gst_sys_name
            }

            pub unsafe fn from_ptr<'a>(ptr: *const $gst_sys_name) -> &'a Self {
                assert!(!ptr.is_null());
                &*(ptr as *const Self)
            }

            pub unsafe fn from_mut_ptr<'a>(ptr: *mut $gst_sys_name) -> &'a mut Self {
                assert!(!ptr.is_null());
                assert_ne!(
                    $crate::gst_sys::gst_mini_object_is_writable(ptr as *mut $crate::gst_sys::GstMiniObject),
                    $crate::glib_sys::GFALSE
                );
                &mut *(ptr as *mut Self)
            }

            pub fn copy(&self) -> $name {
                unsafe {
                    $name::from_glib_full($crate::gst_sys::gst_mini_object_copy(
                        self.as_ptr() as *const $crate::gst_sys::GstMiniObject
                    ) as *const $gst_sys_name)
                }
            }
        }

        impl $crate::glib::types::StaticType for $ref_name {
            fn static_type() -> $crate::glib::types::Type {
                unsafe { $crate::glib::translate::from_glib($get_type()) }
            }
        }

        impl $crate::glib::translate::GlibPtrDefault for $ref_name {
            type GlibType = *mut $gst_sys_name;
        }

        impl<'a> $crate::glib::value::FromValueOptional<'a>
            for &'a $ref_name
        {
            unsafe fn from_value_optional(v: &'a glib::Value) -> Option<Self> {
                let ptr = gobject_sys::g_value_get_boxed($crate::glib::translate::ToGlibPtr::to_glib_none(v).0);
                if ptr.is_null() {
                    None
                } else {
                    Some(&*(ptr as *const $ref_name))
                }
            }
        }

        // Can't have SetValue/SetValueOptional impls as otherwise one could use it to get
        // immutable references from a mutable reference without borrowing via the value

        impl ToOwned for $ref_name {
            type Owned = $name;

            fn to_owned(&self) -> $name {
                self.copy()
            }
        }

        unsafe impl Sync for $ref_name {}
        unsafe impl Send for $ref_name {}
        unsafe impl Sync for $name {}
        unsafe impl Send for $name {}
    }
);
