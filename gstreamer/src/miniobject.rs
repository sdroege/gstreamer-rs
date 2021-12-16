// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

pub trait IsMiniObject:
    AsRef<Self::RefType> + FromGlibPtrFull<*mut Self::FfiType> + Send + Sync + 'static
{
    type RefType;
    type FfiType;
}

#[macro_export]
macro_rules! mini_object_wrapper (
    ($name:ident, $ref_name:ident, $ffi_name:path) => {
        pub struct $name {
            obj: std::ptr::NonNull<$ffi_name>,
        }

        #[repr(transparent)]
        pub struct $ref_name($ffi_name);

        impl $crate::miniobject::IsMiniObject for $name {
            type RefType = $ref_name;
            type FfiType = $ffi_name;
        }

        impl $name {
            pub unsafe fn from_glib_none(ptr: *const $ffi_name) -> Self {
                skip_assert_initialized!();
                assert!(!ptr.is_null());

                $crate::ffi::gst_mini_object_ref(ptr as *mut $crate::ffi::GstMiniObject);

                $name {
                    obj: std::ptr::NonNull::new_unchecked(ptr as *mut $ffi_name),
                }
            }

            pub unsafe fn from_glib_full(ptr: *const $ffi_name) -> Self {
                skip_assert_initialized!();
                assert!(!ptr.is_null());

                $name {
                    obj: std::ptr::NonNull::new_unchecked(ptr as *mut $ffi_name),
                }
            }

            pub unsafe fn from_glib_borrow(ptr: *const $ffi_name) -> $crate::glib::translate::Borrowed<Self> {
                skip_assert_initialized!();
                assert!(!ptr.is_null());

                $crate::glib::translate::Borrowed::new($name {
                    obj: std::ptr::NonNull::new_unchecked(ptr as *mut $ffi_name),
                })
            }

            pub unsafe fn replace_ptr(&mut self, ptr: *mut $ffi_name) {
                assert!(!ptr.is_null());
                self.obj = std::ptr::NonNull::new_unchecked(ptr);
            }

            pub fn make_mut(&mut self) -> &mut $ref_name {
                unsafe {
                    if self.is_writable() {
                        return &mut *(self.obj.as_mut() as *mut $ffi_name as *mut $ref_name);
                    }

                    let ptr = $crate::ffi::gst_mini_object_make_writable(
                        self.as_mut_ptr() as *mut $crate::ffi::GstMiniObject
                    );
                    self.replace_ptr(ptr as *mut $ffi_name);
                    assert!(self.is_writable());

                    &mut *(self.obj.as_mut() as *mut $ffi_name as *mut $ref_name)
                }
            }

            pub fn get_mut(&mut self) -> Option<&mut $ref_name> {
                if self.is_writable() {
                    Some(unsafe { &mut *(self.obj.as_mut() as *mut $ffi_name as *mut $ref_name) })
                } else {
                    None
                }
            }

            #[doc(alias = "gst_mini_object_is_writable")]
            pub fn is_writable(&self) -> bool {
                unsafe {
                    $crate::glib::translate::from_glib($crate::ffi::gst_mini_object_is_writable(
                        self.as_ptr() as *const $crate::ffi::GstMiniObject
                    ))
                }
            }

            pub fn upcast(self) -> $crate::miniobject::MiniObject {
                unsafe {
                    from_glib_full(self.into_ptr() as *mut $crate::ffi::GstMiniObject)
                }
            }

            pub unsafe fn into_ptr(self) -> *mut $ffi_name {
                let s = std::mem::ManuallyDrop::new(self);
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
                    $crate::ffi::gst_mini_object_unref(self.as_mut_ptr() as *mut $crate::ffi::GstMiniObject);
                }
            }
        }

        impl std::ops::Deref for $name {
            type Target = $ref_name;

            fn deref(&self) -> &Self::Target {
                unsafe { &*(self.obj.as_ref() as *const $ffi_name as *const $ref_name) }
            }
        }

        impl AsRef<$ref_name> for $name {
            fn as_ref(&self) -> &$ref_name {
                &*self
            }
        }

        impl std::borrow::Borrow<$ref_name> for $name {
            fn borrow(&self) -> &$ref_name {
                &*self
            }
        }

        impl<'a> $crate::glib::translate::ToGlibPtr<'a, *const $ffi_name> for $name {
            type Storage = &'a Self;

            fn to_glib_none(&'a self) -> $crate::glib::translate::Stash<'a, *const $ffi_name, Self> {
                $crate::glib::translate::Stash(unsafe { self.as_ptr() }, self)
            }

            fn to_glib_full(&self) -> *const $ffi_name {
                unsafe {
                    $crate::ffi::gst_mini_object_ref(self.as_mut_ptr() as *mut $crate::ffi::GstMiniObject);
                    self.as_ptr()
                }
            }
        }

        impl<'a> $crate::glib::translate::ToGlibPtr<'a, *mut $ffi_name> for $name {
            type Storage = &'a Self;

            fn to_glib_none(&'a self) -> $crate::glib::translate::Stash<'a, *mut $ffi_name, Self> {
                $crate::glib::translate::Stash(unsafe { self.as_mut_ptr() }, self)
            }

            fn to_glib_full(&self) -> *mut $ffi_name {
                unsafe {
                    $crate::ffi::gst_mini_object_ref(self.as_mut_ptr() as *mut $crate::ffi::GstMiniObject);
                    self.as_mut_ptr()
                }
            }
        }

        impl<'a> $crate::glib::translate::ToGlibPtrMut<'a, *mut $ffi_name> for $name {
            type Storage = &'a mut Self;

            fn to_glib_none_mut(&'a mut self) -> $crate::glib::translate::StashMut<*mut $ffi_name, Self> {
                self.make_mut();
                $crate::glib::translate::StashMut(unsafe { self.as_mut_ptr() }, self)
            }
        }

        impl<'a> $crate::glib::translate::ToGlibContainerFromSlice<'a, *mut *mut $ffi_name> for $name {
            #[allow(clippy::type_complexity)]
            type Storage = (
                Vec<$crate::glib::translate::Stash<'a, *mut $ffi_name, Self>>,
                Option<Vec<*mut $ffi_name>>,
            );

            fn to_glib_none_from_slice(t: &'a [$name]) -> (*mut *mut $ffi_name, Self::Storage) {
                skip_assert_initialized!();
                let v: Vec<_> = t.iter().map(|s| $crate::glib::translate::ToGlibPtr::to_glib_none(s)).collect();
                let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
                v_ptr.push(std::ptr::null_mut() as *mut $ffi_name);

                (v_ptr.as_ptr() as *mut *mut $ffi_name, (v, Some(v_ptr)))
            }

            fn to_glib_container_from_slice(t: &'a [$name]) -> (*mut *mut $ffi_name, Self::Storage) {
                skip_assert_initialized!();
                let v: Vec<_> = t.iter().map(|s| $crate::glib::translate::ToGlibPtr::to_glib_none(s)).collect();

                let v_ptr = unsafe {
                    let v_ptr = $crate::glib::ffi::g_malloc0(std::mem::size_of::<*mut $ffi_name>() * t.len() + 1)
                        as *mut *mut $ffi_name;

                    for (i, s) in v.iter().enumerate() {
                        std::ptr::write(v_ptr.add(i), s.0);
                    }

                    v_ptr
                };

                (v_ptr, (v, None))
            }

            fn to_glib_full_from_slice(t: &[$name]) -> *mut *mut $ffi_name {
                skip_assert_initialized!();
                unsafe {
                    let v_ptr = $crate::glib::ffi::g_malloc0(std::mem::size_of::<*mut $ffi_name>() * t.len() + 1)
                        as *mut *mut $ffi_name;

                    for (i, s) in t.iter().enumerate() {
                        std::ptr::write(v_ptr.add(i), $crate::glib::translate::ToGlibPtr::to_glib_full(s));
                    }

                    v_ptr
                }
            }
        }

        impl<'a> $crate::glib::translate::ToGlibContainerFromSlice<'a, *const *mut $ffi_name>
            for $name
        {
            #[allow(clippy::type_complexity)]
            type Storage = (
                Vec<$crate::glib::translate::Stash<'a, *mut $ffi_name, $name>>,
                Option<Vec<*mut $ffi_name>>,
            );

            fn to_glib_none_from_slice(t: &'a [$name]) -> (*const *mut $ffi_name, Self::Storage) {
                skip_assert_initialized!();
                let (ptr, stash) =
                    $crate::glib::translate::ToGlibContainerFromSlice::<'a, *mut *mut $ffi_name>::to_glib_none_from_slice(t);
                (ptr as *const *mut $ffi_name, stash)
            }

            fn to_glib_container_from_slice(_: &'a [$name]) -> (*const *mut $ffi_name, Self::Storage) {
                skip_assert_initialized!();
                // Can't have consumer free a *const pointer
                unimplemented!()
            }

            fn to_glib_full_from_slice(_: &[$name]) -> *const *mut $ffi_name {
                skip_assert_initialized!();
                // Can't have consumer free a *const pointer
                unimplemented!()
            }
        }

        impl $crate::glib::translate::FromGlibPtrNone<*const $ffi_name> for $name {
            unsafe fn from_glib_none(ptr: *const $ffi_name) -> Self {
                Self::from_glib_none(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrNone<*mut $ffi_name> for $name {
            unsafe fn from_glib_none(ptr: *mut $ffi_name) -> Self {
                Self::from_glib_none(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrFull<*const $ffi_name> for $name {
            unsafe fn from_glib_full(ptr: *const $ffi_name) -> Self {
                Self::from_glib_full(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrFull<*mut $ffi_name> for $name {
            unsafe fn from_glib_full(ptr: *mut $ffi_name) -> Self {
                Self::from_glib_full(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrBorrow<*const $ffi_name> for $name {
            unsafe fn from_glib_borrow(ptr: *const $ffi_name) -> $crate::glib::translate::Borrowed<Self> {
                Self::from_glib_borrow(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrBorrow<*mut $ffi_name> for $name {
            unsafe fn from_glib_borrow(ptr: *mut $ffi_name) -> $crate::glib::translate::Borrowed<Self> {
                Self::from_glib_borrow(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibContainerAsVec<*mut $ffi_name, *mut *mut $ffi_name>
            for $name
        {
            unsafe fn from_glib_none_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push($crate::glib::translate::from_glib_none(std::ptr::read(ptr.add(i))));
                }
                res
            }

            unsafe fn from_glib_container_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                let res = $crate::glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
                $crate::glib::ffi::g_free(ptr as *mut _);
                res
            }

            unsafe fn from_glib_full_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push($crate::glib::translate::from_glib_full(std::ptr::read(ptr.add(i))));
                }
                $crate::glib::ffi::g_free(ptr as *mut _);
                res
            }
        }

        impl $crate::glib::translate::FromGlibPtrArrayContainerAsVec<*mut $ffi_name, *mut *mut $ffi_name>
            for $name
        {
            unsafe fn from_glib_none_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, glib::translate::c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_container_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::glib::translate::FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, glib::translate::c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_full_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::glib::translate::FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, glib::translate::c_ptr_array_len(ptr))
            }
        }

        impl $crate::glib::translate::FromGlibContainerAsVec<*mut $ffi_name, *const *mut $ffi_name>
            for $name
        {
            unsafe fn from_glib_none_num_as_vec(ptr: *const *mut $ffi_name, num: usize) -> Vec<Self> {
                $crate::glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *mut *mut _, num)
            }

            unsafe fn from_glib_container_num_as_vec(_: *const *mut $ffi_name, _: usize) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }

            unsafe fn from_glib_full_num_as_vec(_: *const *mut $ffi_name, _: usize) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }
        }

        impl $crate::glib::translate::FromGlibPtrArrayContainerAsVec<*mut $ffi_name, *const *mut $ffi_name> for $name
        {
            unsafe fn from_glib_none_as_vec(ptr: *const *mut $ffi_name) -> Vec<Self> {
                $crate::glib::translate::FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr as *mut *mut _)
            }

            unsafe fn from_glib_container_as_vec(_: *const *mut $ffi_name) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }

            unsafe fn from_glib_full_as_vec(_: *const *mut $ffi_name) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }
        }

        impl $crate::glib::translate::GlibPtrDefault for $name {
            type GlibType = *mut $ffi_name;
        }

        impl $ref_name {
            pub unsafe fn as_ptr(&self) -> *const $ffi_name {
                self as *const Self as *const $ffi_name
            }

            pub unsafe fn as_mut_ptr(&self) -> *mut $ffi_name {
                self as *const Self as *mut $ffi_name
            }

            pub unsafe fn from_ptr<'a>(ptr: *const $ffi_name) -> &'a Self {
                assert!(!ptr.is_null());
                &*(ptr as *const Self)
            }

            pub unsafe fn from_mut_ptr<'a>(ptr: *mut $ffi_name) -> &'a mut Self {
                assert!(!ptr.is_null());
                assert_ne!(
                    $crate::ffi::gst_mini_object_is_writable(ptr as *mut $crate::ffi::GstMiniObject),
                    $crate::glib::ffi::GFALSE
                );
                &mut *(ptr as *mut Self)
            }

            #[doc(alias = "gst_mini_object_copy")]
            pub fn copy(&self) -> $name {
                unsafe {
                    $name::from_glib_full($crate::ffi::gst_mini_object_copy(
                        self.as_ptr() as *const $crate::ffi::GstMiniObject
                    ) as *const $ffi_name)
                }
            }

            pub fn upcast_ref(&self) -> &$crate::miniobject::MiniObjectRef {
                unsafe {
                    &*(self.as_ptr() as *const $crate::miniobject::MiniObjectRef)
                }
            }

            pub fn upcast_mut(&mut self) -> &mut $crate::miniobject::MiniObjectRef {
                unsafe {
                    &mut *(self.as_mut_ptr() as *mut $crate::miniobject::MiniObjectRef)
                }
            }
        }

        impl $crate::glib::translate::GlibPtrDefault for $ref_name {
            type GlibType = *mut $ffi_name;
        }

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
    };
    ($name:ident, $ref_name:ident, $ffi_name:path, $get_type:expr) => {
        $crate::mini_object_wrapper!($name, $ref_name, $ffi_name);

        impl $crate::glib::types::StaticType for $name {
            fn static_type() -> $crate::glib::types::Type {
                $ref_name::static_type()
            }
        }

        impl $crate::glib::types::StaticType for $ref_name {
            fn static_type() -> $crate::glib::types::Type {
                unsafe { $crate::glib::translate::from_glib($get_type()) }
            }
        }

        impl glib::value::ValueType for $name {
            type Type = Self;
        }

        impl glib::value::ValueTypeOptional for $name { }

        unsafe impl<'a> $crate::glib::value::FromValue<'a> for $name {
            type Checker = $crate::glib::value::GenericValueTypeOrNoneChecker<Self>;

            unsafe fn from_value(value: &'a $crate::glib::Value) -> Self {
                skip_assert_initialized!();
                $crate::glib::translate::from_glib_none(
                    $crate::glib::gobject_ffi::g_value_get_boxed($crate::glib::translate::ToGlibPtr::to_glib_none(value).0) as *mut $ffi_name
                )
            }
        }

        impl $crate::glib::value::ToValue for $name {
            fn to_value(&self) -> $crate::glib::Value {
                let mut value = $crate::glib::Value::for_value_type::<Self>();
                unsafe {
                    $crate::glib::gobject_ffi::g_value_set_boxed(
                        $crate::glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        $crate::glib::translate::ToGlibPtr::<*const $ffi_name>::to_glib_none(self).0 as *mut _,
                    )
                }
                value
            }

            fn value_type(&self) -> $crate::glib::Type {
                <Self as $crate::glib::StaticType>::static_type()
            }
        }

        impl $crate::glib::value::ToValueOptional for $name {
            fn to_value_optional(s: Option<&Self>) -> $crate::glib::Value {
                skip_assert_initialized!();
                let mut value = $crate::glib::Value::for_value_type::<Self>();
                unsafe {
                    $crate::glib::gobject_ffi::g_value_set_boxed(
                        $crate::glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        $crate::glib::translate::ToGlibPtr::<*const $ffi_name>::to_glib_none(&s).0 as *mut _,
                    )
                }
                value
            }
        }

        unsafe impl<'a> $crate::glib::value::FromValue<'a> for &'a $ref_name {
            type Checker = $crate::glib::value::GenericValueTypeOrNoneChecker<Self>;

            unsafe fn from_value(value: &'a $crate::glib::Value) -> Self {
                skip_assert_initialized!();
                &*($crate::glib::gobject_ffi::g_value_get_boxed($crate::glib::translate::ToGlibPtr::to_glib_none(value).0) as *const $ref_name)
            }
        }

        // Can't have SetValue/SetValueOptional impls as otherwise one could use it to get
        // immutable references from a mutable reference without borrowing via the value
    };
);

#[cfg(not(any(feature = "v1_20", feature = "dox")))]
mini_object_wrapper!(MiniObject, MiniObjectRef, ffi::GstMiniObject);

#[cfg(any(feature = "v1_20", feature = "dox"))]
mini_object_wrapper!(MiniObject, MiniObjectRef, ffi::GstMiniObject, || {
    ffi::gst_mini_object_get_type()
});

impl MiniObject {
    pub fn downcast<T: IsMiniObject + glib::StaticType>(self) -> Result<T, Self> {
        if self.type_().is_a(T::static_type()) {
            unsafe { Ok(from_glib_full(self.into_ptr() as *mut T::FfiType)) }
        } else {
            Err(self)
        }
    }
}

impl MiniObjectRef {
    pub fn type_(&self) -> glib::Type {
        unsafe { from_glib((*self.as_ptr()).type_) }
    }

    pub fn downcast_ref<T: IsMiniObject + glib::StaticType>(&self) -> Option<&T> {
        if self.type_().is_a(T::static_type()) {
            unsafe { Some(&*(self as *const Self as *const T)) }
        } else {
            None
        }
    }

    pub fn downcast_mut<T: IsMiniObject + glib::StaticType>(&mut self) -> Option<&mut T> {
        if self.type_().is_a(T::static_type()) {
            unsafe { Some(&mut *(self as *mut Self as *mut T)) }
        } else {
            None
        }
    }
}
