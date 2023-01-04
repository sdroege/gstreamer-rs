// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    fmt,
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut},
    ptr, slice,
};

use glib::translate::*;

use crate::{AllocationParams, Allocator, MemoryFlags};

mini_object_wrapper!(Memory, MemoryRef, ffi::GstMemory, || {
    ffi::gst_memory_get_type()
});

pub struct MemoryMap<'a, T> {
    memory: &'a MemoryRef,
    map_info: ffi::GstMapInfo,
    phantom: PhantomData<T>,
}

pub struct MappedMemory<T> {
    memory: Memory,
    map_info: ffi::GstMapInfo,
    phantom: PhantomData<T>,
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        MemoryRef::fmt(self, f)
    }
}

impl fmt::Debug for MemoryRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Memory")
            .field("ptr", &self.as_ptr())
            .field("allocator", &self.allocator())
            .field("parent", &self.parent())
            .field("maxsize", &self.maxsize())
            .field("align", &self.align())
            .field("offset", &self.offset())
            .field("size", &self.size())
            .field("flags", &self.flags())
            .finish()
    }
}

pub enum Readable {}
pub enum Writable {}

impl Memory {
    unsafe extern "C" fn drop_box<T>(vec: glib::ffi::gpointer) {
        let slice: Box<T> = Box::from_raw(vec as *mut T);
        drop(slice);
    }

    pub fn with_size(size: usize) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_allocator_alloc(
                ptr::null_mut(),
                size,
                ptr::null_mut(),
            ))
        }
    }

    pub fn with_size_and_params(size: usize, params: &AllocationParams) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_allocator_alloc(
                ptr::null_mut(),
                size,
                params.as_ptr() as *mut _,
            ))
        }
    }

    pub fn from_slice<T: AsRef<[u8]> + Send + 'static>(slice: T) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let b = Box::new(slice);
            let (size, data) = {
                let slice = (*b).as_ref();
                (slice.len(), slice.as_ptr())
            };
            let user_data = Box::into_raw(b);
            from_glib_full(ffi::gst_memory_new_wrapped(
                ffi::GST_MEMORY_FLAG_READONLY,
                data as glib::ffi::gpointer,
                size,
                0,
                size,
                user_data as glib::ffi::gpointer,
                Some(Self::drop_box::<T>),
            ))
        }
    }

    pub fn from_mut_slice<T: AsMut<[u8]> + Send + 'static>(slice: T) -> Self {
        assert_initialized_main_thread!();

        unsafe {
            let mut b = Box::new(slice);
            let (size, data) = {
                let slice = (*b).as_mut();
                (slice.len(), slice.as_mut_ptr())
            };
            let user_data = Box::into_raw(b);
            from_glib_full(ffi::gst_memory_new_wrapped(
                0,
                data as glib::ffi::gpointer,
                size,
                0,
                size,
                user_data as glib::ffi::gpointer,
                Some(Self::drop_box::<T>),
            ))
        }
    }

    pub fn into_mapped_memory_readable(self) -> Result<MappedMemory<Readable>, Self> {
        unsafe {
            let mut map_info = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(ffi::gst_memory_map(
                self.as_mut_ptr(),
                map_info.as_mut_ptr(),
                ffi::GST_MAP_READ,
            ));
            if res {
                Ok(MappedMemory {
                    memory: self,
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(self)
            }
        }
    }

    pub fn into_mapped_memory_writable(self) -> Result<MappedMemory<Writable>, Self> {
        unsafe {
            let mut map_info = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(ffi::gst_memory_map(
                self.as_mut_ptr(),
                map_info.as_mut_ptr(),
                ffi::GST_MAP_READWRITE,
            ));
            if res {
                Ok(MappedMemory {
                    memory: self,
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(self)
            }
        }
    }
}

impl MemoryRef {
    #[doc(alias = "get_allocator")]
    pub fn allocator(&self) -> Option<&Allocator> {
        unsafe {
            if self.0.allocator.is_null() {
                None
            } else {
                Some(&*(&self.0.allocator as *const *mut ffi::GstAllocator as *const Allocator))
            }
        }
    }

    #[doc(alias = "get_parent")]
    pub fn parent(&self) -> Option<&MemoryRef> {
        unsafe {
            if self.0.parent.is_null() {
                None
            } else {
                Some(MemoryRef::from_ptr(self.0.parent))
            }
        }
    }

    #[doc(alias = "get_maxsize")]
    pub fn maxsize(&self) -> usize {
        self.0.maxsize
    }

    #[doc(alias = "get_align")]
    pub fn align(&self) -> usize {
        self.0.align
    }

    #[doc(alias = "get_offset")]
    pub fn offset(&self) -> usize {
        self.0.offset
    }

    #[doc(alias = "get_size")]
    pub fn size(&self) -> usize {
        self.0.size
    }

    #[doc(alias = "get_flags")]
    pub fn flags(&self) -> MemoryFlags {
        unsafe { from_glib(self.0.mini_object.flags) }
    }

    pub fn copy_part(&self, offset: isize, size: Option<usize>) -> Memory {
        let pos_sz = match size {
            Some(val) => val as isize,
            None => 0,
        };
        assert!(offset + pos_sz < (self.maxsize() as isize));
        unsafe {
            from_glib_full(ffi::gst_memory_copy(
                self.as_mut_ptr(),
                offset,
                match size {
                    Some(val) => val as isize,
                    None => -1,
                },
            ))
        }
    }

    #[doc(alias = "gst_memory_is_span")]
    pub fn is_span(&self, mem2: &MemoryRef) -> Option<usize> {
        unsafe {
            let mut offset = mem::MaybeUninit::uninit();
            let res = from_glib(ffi::gst_memory_is_span(
                self.as_mut_ptr(),
                mem2.as_mut_ptr(),
                offset.as_mut_ptr(),
            ));
            if res {
                Some(offset.assume_init())
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_memory_is_type")]
    pub fn is_type(&self, mem_type: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_memory_is_type(
                self.as_mut_ptr(),
                mem_type.to_glib_none().0,
            ))
        }
    }

    pub fn map_readable(&self) -> Result<MemoryMap<Readable>, glib::BoolError> {
        unsafe {
            let mut map_info = mem::MaybeUninit::zeroed();
            let res =
                ffi::gst_memory_map(self.as_mut_ptr(), map_info.as_mut_ptr(), ffi::GST_MAP_READ);
            if res == glib::ffi::GTRUE {
                Ok(MemoryMap {
                    memory: self,
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(glib::bool_error!("Failed to map memory readable"))
            }
        }
    }

    pub fn map_writable(&mut self) -> Result<MemoryMap<Writable>, glib::BoolError> {
        unsafe {
            let mut map_info = mem::MaybeUninit::zeroed();
            let res = ffi::gst_memory_map(
                self.as_mut_ptr(),
                map_info.as_mut_ptr(),
                ffi::GST_MAP_READWRITE,
            );
            if res == glib::ffi::GTRUE {
                Ok(MemoryMap {
                    memory: self,
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(glib::bool_error!("Failed to map memory writable"))
            }
        }
    }

    #[doc(alias = "gst_memory_share")]
    pub fn share(&self, offset: isize, size: Option<usize>) -> Memory {
        let pos_sz = match size {
            Some(val) => val as isize,
            None => 0,
        };
        assert!(offset + pos_sz < (self.maxsize() as isize));
        unsafe {
            from_glib_full(ffi::gst_memory_share(
                self.as_ptr() as *mut _,
                offset,
                match size {
                    Some(val) => val as isize,
                    None => -1,
                },
            ))
        }
    }

    #[doc(alias = "gst_memory_resize")]
    pub fn resize(&mut self, offset: isize, size: usize) {
        assert!(offset + (size as isize) < (self.maxsize() as isize));
        unsafe { ffi::gst_memory_resize(self.as_mut_ptr(), offset, size) }
    }

    pub fn dump(&self, size: Option<usize>) -> Dump {
        Dump { memory: self, size }
    }
}

impl<'a, T> MemoryMap<'a, T> {
    #[doc(alias = "get_size")]
    pub fn size(&self) -> usize {
        self.map_info.size
    }

    #[doc(alias = "get_memory")]
    pub fn memory(&self) -> &MemoryRef {
        self.memory
    }

    pub fn as_slice(&self) -> &[u8] {
        if self.map_info.size == 0 {
            return &[];
        }
        unsafe { slice::from_raw_parts(self.map_info.data as *const u8, self.map_info.size) }
    }
}

impl<'a> MemoryMap<'a, Writable> {
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        if self.map_info.size == 0 {
            return &mut [];
        }
        unsafe { slice::from_raw_parts_mut(self.map_info.data as *mut u8, self.map_info.size) }
    }
}

impl<'a, T> AsRef<[u8]> for MemoryMap<'a, T> {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<'a> AsMut<[u8]> for MemoryMap<'a, Writable> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<'a, T> Deref for MemoryMap<'a, T> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<'a> DerefMut for MemoryMap<'a, Writable> {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<'a, T> fmt::Debug for MemoryMap<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("MemoryMap").field(&self.memory()).finish()
    }
}

impl<'a, T> PartialEq for MemoryMap<'a, T> {
    fn eq(&self, other: &MemoryMap<'a, T>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<'a, T> Eq for MemoryMap<'a, T> {}

impl<'a, T> Drop for MemoryMap<'a, T> {
    fn drop(&mut self) {
        unsafe {
            ffi::gst_memory_unmap(self.memory.as_mut_ptr(), &mut self.map_info);
        }
    }
}

unsafe impl<'a, T> Send for MemoryMap<'a, T> {}
unsafe impl<'a, T> Sync for MemoryMap<'a, T> {}

impl<T> MappedMemory<T> {
    pub fn as_slice(&self) -> &[u8] {
        if self.map_info.size == 0 {
            return &[];
        }
        unsafe { slice::from_raw_parts(self.map_info.data as *const u8, self.map_info.size) }
    }

    #[doc(alias = "get_size")]
    pub fn size(&self) -> usize {
        self.map_info.size
    }

    #[doc(alias = "get_memory")]
    pub fn memory(&self) -> &MemoryRef {
        self.memory.as_ref()
    }

    pub fn into_memory(self) -> Memory {
        let mut s = mem::ManuallyDrop::new(self);
        let memory = unsafe { ptr::read(&s.memory) };
        unsafe {
            ffi::gst_memory_unmap(memory.as_mut_ptr(), &mut s.map_info);
        }

        memory
    }
}

impl MappedMemory<Writable> {
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        if self.map_info.size == 0 {
            return &mut [];
        }
        unsafe { slice::from_raw_parts_mut(self.map_info.data as *mut u8, self.map_info.size) }
    }
}

impl<T> AsRef<[u8]> for MappedMemory<T> {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsMut<[u8]> for MappedMemory<Writable> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<T> Deref for MappedMemory<T> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl DerefMut for MappedMemory<Writable> {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<T> Drop for MappedMemory<T> {
    fn drop(&mut self) {
        unsafe {
            ffi::gst_memory_unmap(self.memory.as_mut_ptr(), &mut self.map_info);
        }
    }
}

impl<T> fmt::Debug for MappedMemory<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("MappedMemory").field(&self.memory()).finish()
    }
}

impl<T> PartialEq for MappedMemory<T> {
    fn eq(&self, other: &MappedMemory<T>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<T> Eq for MappedMemory<T> {}

unsafe impl<T> Send for MappedMemory<T> {}
unsafe impl<T> Sync for MappedMemory<T> {}

pub struct Dump<'a> {
    memory: &'a MemoryRef,
    size: Option<usize>,
}

impl<'a> Dump<'a> {
    fn fmt(&self, f: &mut fmt::Formatter, debug: bool) -> fmt::Result {
        use pretty_hex::*;

        let map = self.memory.map_readable().expect("Failed to map memory");
        let data = map.as_slice();
        let size = self.size.unwrap_or_else(|| self.memory.size());
        let data = &data[0..size];

        if debug {
            write!(f, "{:?}", data.hex_dump())
        } else {
            write!(f, "{}", data.hex_dump())
        }
    }
}

impl<'a> fmt::Display for Dump<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f, false)
    }
}

impl<'a> fmt::Debug for Dump<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f, true)
    }
}

pub unsafe trait MemoryType: crate::prelude::IsMiniObject + AsRef<Memory>
where
    <Self as crate::prelude::IsMiniObject>::RefType: AsRef<MemoryRef> + AsMut<MemoryRef>,
{
    fn check_memory_type(mem: &MemoryRef) -> bool;
}

#[derive(Debug, thiserror::Error)]
pub enum MemoryTypeMismatchError {
    #[error(transparent)]
    ValueTypeMismatch(#[from] glib::value::ValueTypeMismatchError),
    #[error("the memory is not of the requested type {requested}")]
    MemoryTypeMismatch { requested: &'static str },
}

pub struct MemoryTypeValueTypeChecker<M>(PhantomData<M>);

unsafe impl<M> glib::value::ValueTypeChecker for MemoryTypeValueTypeChecker<M>
where
    M: MemoryType + glib::StaticType,
    <M as crate::prelude::IsMiniObject>::RefType: AsRef<MemoryRef> + AsMut<MemoryRef>,
{
    type Error = glib::value::ValueTypeMismatchOrNoneError<MemoryTypeMismatchError>;

    fn check(value: &glib::Value) -> Result<(), Self::Error> {
        skip_assert_initialized!();
        let mem = value.get::<&Memory>().map_err(|err| match err {
            glib::value::ValueTypeMismatchOrNoneError::UnexpectedNone => {
                glib::value::ValueTypeMismatchOrNoneError::UnexpectedNone
            }
            glib::value::ValueTypeMismatchOrNoneError::WrongValueType(err) => {
                glib::value::ValueTypeMismatchOrNoneError::WrongValueType(
                    MemoryTypeMismatchError::ValueTypeMismatch(err),
                )
            }
        })?;

        if mem.is_memory_type::<M>() {
            Ok(())
        } else {
            Err(glib::value::ValueTypeMismatchOrNoneError::WrongValueType(
                MemoryTypeMismatchError::MemoryTypeMismatch {
                    requested: std::any::type_name::<M>(),
                },
            ))
        }
    }
}

impl AsRef<MemoryRef> for MemoryRef {
    fn as_ref(&self) -> &MemoryRef {
        self
    }
}

impl AsMut<MemoryRef> for MemoryRef {
    fn as_mut(&mut self) -> &mut MemoryRef {
        self
    }
}

impl AsRef<Memory> for Memory {
    fn as_ref(&self) -> &Memory {
        self
    }
}

unsafe impl MemoryType for Memory {
    fn check_memory_type(_mem: &MemoryRef) -> bool {
        skip_assert_initialized!();
        true
    }
}

impl Memory {
    pub fn downcast_memory<M: MemoryType>(self) -> Result<M, Self>
    where
        <M as crate::prelude::IsMiniObject>::RefType: AsRef<MemoryRef> + AsMut<MemoryRef>,
    {
        if M::check_memory_type(&self) {
            unsafe { Ok(from_glib_full(self.into_glib_ptr() as *mut M::FfiType)) }
        } else {
            Err(self)
        }
    }
}

impl MemoryRef {
    pub fn is_memory_type<M: MemoryType>(&self) -> bool
    where
        <M as crate::prelude::IsMiniObject>::RefType: AsRef<MemoryRef> + AsMut<MemoryRef>,
    {
        M::check_memory_type(self)
    }

    pub fn downcast_memory_ref<M: MemoryType>(&self) -> Option<&M::RefType>
    where
        <M as crate::prelude::IsMiniObject>::RefType: AsRef<MemoryRef> + AsMut<MemoryRef>,
    {
        if M::check_memory_type(self) {
            unsafe { Some(&*(self as *const Self as *const M::RefType)) }
        } else {
            None
        }
    }

    pub fn downcast_memory_mut<M: MemoryType>(&mut self) -> Option<&mut M::RefType>
    where
        <M as crate::prelude::IsMiniObject>::RefType: AsRef<MemoryRef> + AsMut<MemoryRef>,
    {
        if M::check_memory_type(self) {
            unsafe { Some(&mut *(self as *mut Self as *mut M::RefType)) }
        } else {
            None
        }
    }
}

#[macro_export]
macro_rules! memory_object_wrapper {
    ($name:ident, $ref_name:ident, $ffi_name:path, $mem_type_check:expr, $parent_memory_type:path, $parent_memory_ref_type:path) => {
        $crate::mini_object_wrapper!($name, $ref_name, $ffi_name);

        unsafe impl $crate::memory::MemoryType for $name {
            fn check_memory_type(mem: &$crate::MemoryRef) -> bool {
                skip_assert_initialized!();
                $mem_type_check(mem)
            }
        }

        impl $name {
            pub fn downcast_memory<M: $crate::memory::MemoryType>(self) -> Result<M, Self>
            where
                <M as $crate::miniobject::IsMiniObject>::RefType: AsRef<$crate::MemoryRef>
                    + AsMut<$crate::MemoryRef>
                    + AsRef<$ref_name>
                    + AsMut<$ref_name>,
            {
                if M::check_memory_type(&self) {
                    unsafe {
                        Ok($crate::glib::translate::from_glib_full(
                            self.into_glib_ptr() as *mut M::FfiType
                        ))
                    }
                } else {
                    Err(self)
                }
            }

            pub fn upcast_memory<M>(self) -> M
            where
                M: $crate::memory::MemoryType
                    + $crate::glib::translate::FromGlibPtrFull<
                        *const <M as $crate::miniobject::IsMiniObject>::FfiType,
                    >,
                <M as $crate::miniobject::IsMiniObject>::RefType:
                    AsRef<$crate::MemoryRef> + AsMut<$crate::MemoryRef>,
                Self: AsRef<M>,
            {
                unsafe {
                    $crate::glib::translate::from_glib_full(
                        self.into_glib_ptr() as *const <M as $crate::miniobject::IsMiniObject>::FfiType
                    )
                }
            }
        }

        impl $ref_name {
            pub fn upcast_memory_ref<M>(&self) -> &M::RefType
            where
                M: $crate::memory::MemoryType,
                <M as $crate::miniobject::IsMiniObject>::RefType:
                    AsRef<$crate::MemoryRef> + AsMut<$crate::MemoryRef>,
                Self: AsRef<M::RefType> + AsMut<M::RefType>
            {
                self.as_ref()
            }

            pub fn upcast_memory_mut<M>(&mut self) -> &mut M::RefType
            where
                M: $crate::memory::MemoryType,
                <M as $crate::miniobject::IsMiniObject>::RefType:
                    AsRef<$crate::MemoryRef> + AsMut<$crate::MemoryRef>,
                Self: AsRef<M::RefType> + AsMut<M::RefType>
            {
                self.as_mut()
            }
        }

        impl std::ops::Deref for $ref_name {
            type Target = $parent_memory_ref_type;

            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const _ as *const Self::Target) }
            }
        }

        impl std::ops::DerefMut for $ref_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *(self as *mut _ as *mut Self::Target) }
            }
        }

        impl AsRef<$parent_memory_type> for $name {
            fn as_ref(&self) -> &$parent_memory_type {
                unsafe { &*(self as *const _ as *const $parent_memory_type) }
            }
        }

        impl AsRef<$parent_memory_ref_type> for $ref_name {
            fn as_ref(&self) -> &$parent_memory_ref_type {
                self
            }
        }

        impl AsMut<$parent_memory_ref_type> for $ref_name {
            fn as_mut(&mut self) -> &mut $parent_memory_ref_type {
                &mut *self
            }
        }

        impl $crate::glib::types::StaticType for $name {
            fn static_type() -> glib::types::Type {
                $ref_name::static_type()
            }
        }

        impl $crate::glib::types::StaticType for $ref_name {
            fn static_type() -> $crate::glib::types::Type {
                unsafe { $crate::glib::translate::from_glib($crate::ffi::gst_memory_get_type()) }
            }
        }

        impl $crate::glib::value::ValueType for $name {
            type Type = Self;
        }

        unsafe impl<'a> $crate::glib::value::FromValue<'a> for $name {
            type Checker = $crate::memory::MemoryTypeValueTypeChecker<Self>;

            unsafe fn from_value(value: &'a $crate::glib::Value) -> Self {
                skip_assert_initialized!();
                $crate::glib::translate::from_glib_none($crate::glib::gobject_ffi::g_value_get_boxed(
                    $crate::glib::translate::ToGlibPtr::to_glib_none(value).0,
                ) as *mut $ffi_name)
            }
        }

        unsafe impl<'a> $crate::glib::value::FromValue<'a> for &'a $name {
            type Checker = $crate::memory::MemoryTypeValueTypeChecker<$name>;

            unsafe fn from_value(value: &'a $crate::glib::Value) -> Self {
                skip_assert_initialized!();
                assert_eq!(
                    std::mem::size_of::<$name>(),
                    std::mem::size_of::<$crate::glib::ffi::gpointer>()
                );
                let value = &*(value as *const $crate::glib::Value as *const $crate::glib::gobject_ffi::GValue);
                let ptr = &value.data[0].v_pointer as *const $crate::glib::ffi::gpointer
                    as *const *const $ffi_name;
                assert!(!(*ptr).is_null());
                &*(ptr as *const $name)
            }
        }

        impl $crate::glib::value::ToValue for $name {
            fn to_value(&self) -> $crate::glib::Value {
                let mut value = $crate::glib::Value::for_value_type::<Self>();
                unsafe {
                    $crate::glib::gobject_ffi::g_value_set_boxed(
                        $crate::glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        $crate::glib::translate::ToGlibPtr::<*const $ffi_name>::to_glib_none(self).0
                            as *mut _,
                    )
                }
                value
            }

            fn value_type(&self) -> glib::Type {
                <Self as glib::StaticType>::static_type()
            }
        }

        impl $crate::glib::value::ToValueOptional for $name {
            fn to_value_optional(s: Option<&Self>) -> $crate::glib::Value {
                skip_assert_initialized!();
                let mut value = $crate::glib::Value::for_value_type::<Self>();
                unsafe {
                    $crate::glib::gobject_ffi::g_value_set_boxed(
                        $crate::glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        $crate::glib::translate::ToGlibPtr::<*const $ffi_name>::to_glib_none(&s).0
                            as *mut _,
                    )
                }
                value
            }
        }

        impl From<$name> for $crate::glib::Value {
            fn from(v: $name) -> $crate::glib::Value {
                skip_assert_initialized!();
                let mut value = $crate::glib::Value::for_value_type::<$name>();
                unsafe {
                    $crate::glib::gobject_ffi::g_value_take_boxed(
                        $crate::glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        $crate::glib::translate::IntoGlibPtr::<*mut $ffi_name>::into_glib_ptr(v) as *mut _,
                    )
                }
                value
            }
        }

        unsafe impl<'a> $crate::glib::value::FromValue<'a> for &'a $ref_name {
            type Checker = $crate::memory::MemoryTypeValueTypeChecker<$name>;

            unsafe fn from_value(value: &'a glib::Value) -> Self {
                skip_assert_initialized!();
                &*($crate::glib::gobject_ffi::g_value_get_boxed($crate::glib::translate::ToGlibPtr::to_glib_none(value).0)
                    as *const $ref_name)
            }
        }

        // Can't have SetValue/SetValueOptional impls as otherwise one could use it to get
        // immutable references from a mutable reference without borrowing via the value
    };
    ($name:ident, $ref_name:ident, $ffi_name:path, $mem_type_check:expr, $parent_memory_type:path, $parent_memory_ref_type:path, $($parent_parent_memory_type:path, $parent_parent_memory_ref_type:path),*) => {
        $crate::memory_object_wrapper!($name, $ref_name, $ffi_name, $mem_type_check, $parent_memory_type, $parent_memory_ref_type);

        $(
            impl AsRef<$parent_parent_memory_type> for $name {
                fn as_ref(&self) -> &$parent_parent_memory_type {
                    unsafe { &*(self as *const _ as *const $parent_parent_memory_type) }
                }
            }

            impl AsRef<$parent_parent_memory_ref_type> for $ref_name {
                fn as_ref(&self) -> &$parent_parent_memory_ref_type {
                    self
                }
            }

            impl AsMut<$parent_parent_memory_ref_type> for $ref_name {
                fn as_mut(&mut self) -> &mut $parent_parent_memory_ref_type {
                    &mut *self
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_map() {
        crate::init().unwrap();

        let mem = crate::Memory::from_slice(vec![1, 2, 3, 4]);
        let map = mem.map_readable().unwrap();
        assert_eq!(map.as_slice(), &[1, 2, 3, 4]);
        drop(map);

        let mem = mem.into_mapped_memory_readable().unwrap();
        assert_eq!(mem.as_slice(), &[1, 2, 3, 4]);

        let mem = mem.into_memory();
        let map = mem.map_readable().unwrap();
        assert_eq!(map.as_slice(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_dump() {
        crate::init().unwrap();

        let mem = crate::Memory::from_slice(vec![1, 2, 3, 4]);
        println!("{}", mem.dump(Some(mem.size())));

        let mem = crate::Memory::from_slice(vec![1, 2, 3, 4]);
        println!("{:?}", mem.dump(Some(2)));

        let mem = crate::Memory::from_slice(vec![0; 64]);
        dbg!(mem.dump(None));
    }

    #[test]
    fn test_value() {
        use glib::prelude::*;

        crate::init().unwrap();

        let v = None::<&crate::Memory>.to_value();
        assert!(matches!(v.get::<Option<crate::Memory>>(), Ok(None)));

        let mem = crate::Memory::from_slice(vec![1, 2, 3, 4]);
        let v = mem.to_value();
        assert!(matches!(v.get::<Option<crate::Memory>>(), Ok(Some(_))));
        assert!(matches!(v.get::<crate::Memory>(), Ok(_)));
    }
}
