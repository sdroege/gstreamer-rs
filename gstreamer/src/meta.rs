// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::marker::PhantomData;
use std::ops;

use Buffer;
use BufferRef;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use Caps;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use CapsRef;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use ClockTime;

use glib;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib::translate::ToGlib;
use glib::translate::{from_glib, from_glib_none, FromGlib, ToGlibPtr};
use glib_sys;
use gst_sys;

pub unsafe trait MetaAPI: Sync + Send + Sized {
    type GstType;

    fn get_meta_api() -> glib::Type;

    unsafe fn from_ptr(buffer: &BufferRef, ptr: *const Self::GstType) -> MetaRef<Self> {
        assert!(!ptr.is_null());

        let meta_api = Self::get_meta_api();
        if meta_api != glib::Type::Invalid {
            assert_eq!(
                meta_api,
                from_glib((*(*(ptr as *const gst_sys::GstMeta)).info).api)
            )
        }

        MetaRef {
            meta: &*(ptr as *const Self),
            buffer,
        }
    }

    unsafe fn from_mut_ptr<T>(
        buffer: &mut BufferRef,
        ptr: *mut Self::GstType,
    ) -> MetaRefMut<Self, T> {
        assert!(!ptr.is_null());

        let meta_api = Self::get_meta_api();
        if meta_api != glib::Type::Invalid {
            assert_eq!(
                meta_api,
                from_glib((*(*(ptr as *const gst_sys::GstMeta)).info).api)
            )
        }

        MetaRefMut {
            meta: &mut *(ptr as *mut Self),
            buffer,
            mode: PhantomData,
        }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct MetaSeqnum(u64);

pub struct MetaRef<'a, T: MetaAPI + 'a> {
    meta: &'a T,
    buffer: &'a BufferRef,
}

pub enum Standalone {}
pub enum Iterated {}

pub struct MetaRefMut<'a, T: MetaAPI + 'a, U> {
    meta: &'a mut T,
    buffer: &'a mut BufferRef,
    mode: PhantomData<U>,
}

impl<'a, T: MetaAPI + fmt::Debug + 'a> fmt::Debug for MetaRef<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MetaRef")
            .field("meta", &self.meta)
            .field("buffer", &self.buffer)
            .finish()
    }
}

impl<'a, T: MetaAPI + fmt::Debug + 'a, U> fmt::Debug for MetaRefMut<'a, T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MetaRef")
            .field("meta", &self.meta)
            .field("buffer", &self.buffer)
            .field("mode", &self.mode)
            .finish()
    }
}

impl<'a, T: MetaAPI> ops::Deref for MetaRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.meta
    }
}

impl<'a, T: MetaAPI> AsRef<MetaRef<'a, T>> for MetaRef<'a, T> {
    fn as_ref(&self) -> &MetaRef<'a, T> {
        self
    }
}

impl<'a, T: MetaAPI, U> ops::Deref for MetaRefMut<'a, T, U> {
    type Target = T;

    fn deref(&self) -> &T {
        self.meta
    }
}

impl<'a, T: MetaAPI, U> ops::DerefMut for MetaRefMut<'a, T, U> {
    fn deref_mut(&mut self) -> &mut T {
        self.meta
    }
}

impl<'a, T: MetaAPI, U> AsRef<MetaRef<'a, T>> for MetaRefMut<'a, T, U> {
    fn as_ref(&self) -> &MetaRef<'a, T> {
        unsafe { &*(self as *const MetaRefMut<'a, T, U> as *const MetaRef<'a, T>) }
    }
}

impl<'a, T: MetaAPI> MetaRef<'a, T> {
    pub fn get_api(&self) -> glib::Type {
        unsafe {
            let meta = self.meta as *const _ as *const gst_sys::GstMeta;
            let info = (*meta).info;
            glib::Type::from_glib((*info).api)
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_seqnum(&self) -> MetaSeqnum {
        unsafe {
            let meta = self.meta as *const _ as *const gst_sys::GstMeta;
            MetaSeqnum(gst_sys::gst_meta_get_seqnum(meta))
        }
    }

    pub fn as_ptr(&self) -> *const T::GstType {
        self.meta as *const _ as *const <T as MetaAPI>::GstType
    }
}

impl<'a> MetaRef<'a, Meta> {
    pub fn downcast_ref<T: MetaAPI>(&self) -> Option<&MetaRef<'a, T>> {
        let target_type = T::get_meta_api();
        let type_ = self.get_api();

        if type_ == glib::Type::Invalid || target_type == type_ {
            Some(unsafe { &*(self as *const MetaRef<'a, Meta> as *const MetaRef<'a, T>) })
        } else {
            None
        }
    }
}

impl<'a, T: MetaAPI, U> MetaRefMut<'a, T, U> {
    pub fn get_api(&self) -> glib::Type {
        unsafe {
            let meta = self.meta as *const _ as *const gst_sys::GstMeta;
            let info = (*meta).info;
            glib::Type::from_glib((*info).api)
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_seqnum(&self) -> u64 {
        unsafe {
            let meta = self.meta as *const _ as *const gst_sys::GstMeta;
            gst_sys::gst_meta_get_seqnum(meta)
        }
    }

    pub fn as_ptr(&self) -> *const T::GstType {
        self.meta as *const _ as *const <T as MetaAPI>::GstType
    }

    pub fn as_mut_ptr(&mut self) -> *mut T::GstType {
        self.meta as *mut _ as *mut <T as MetaAPI>::GstType
    }
}

impl<'a, T: MetaAPI> MetaRefMut<'a, T, Standalone> {
    pub fn remove(mut self) {
        unsafe {
            let res = gst_sys::gst_buffer_remove_meta(
                self.buffer.as_mut_ptr(),
                self.as_mut_ptr() as *mut gst_sys::GstMeta,
            );
            assert_ne!(res, glib_sys::GFALSE);
        }
    }
}

impl<'a, U> MetaRefMut<'a, Meta, U> {
    pub fn downcast_ref<T: MetaAPI>(&mut self) -> Option<&MetaRefMut<'a, T, U>> {
        let target_type = T::get_meta_api();
        let type_ = self.get_api();

        if type_ == glib::Type::Invalid || target_type == type_ {
            Some(unsafe { &*(self as *mut MetaRefMut<'a, Meta, U> as *const MetaRefMut<'a, T, U>) })
        } else {
            None
        }
    }
}

#[repr(transparent)]
pub struct Meta(gst_sys::GstMeta);

unsafe impl Send for Meta {}
unsafe impl Sync for Meta {}

impl Meta {
    fn get_api(&self) -> glib::Type {
        unsafe { glib::Type::from_glib((*self.0.info).api) }
    }
}

unsafe impl MetaAPI for Meta {
    type GstType = gst_sys::GstMeta;

    fn get_meta_api() -> glib::Type {
        glib::Type::Invalid
    }
}

impl fmt::Debug for Meta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Meta")
            .field("api", &self.get_api())
            .finish()
    }
}

#[repr(transparent)]
pub struct ParentBufferMeta(gst_sys::GstParentBufferMeta);

unsafe impl Send for ParentBufferMeta {}
unsafe impl Sync for ParentBufferMeta {}

impl ParentBufferMeta {
    pub fn add<'a>(buffer: &'a mut BufferRef, parent: &Buffer) -> MetaRefMut<'a, Self, Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = gst_sys::gst_buffer_add_parent_buffer_meta(
                buffer.as_mut_ptr(),
                parent.to_glib_none().0,
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn get_parent(&self) -> &BufferRef {
        unsafe { BufferRef::from_ptr(self.0.buffer) }
    }

    pub fn get_parent_owned(&self) -> Buffer {
        unsafe { from_glib_none(self.0.buffer) }
    }
}

unsafe impl MetaAPI for ParentBufferMeta {
    type GstType = gst_sys::GstParentBufferMeta;

    fn get_meta_api() -> glib::Type {
        unsafe { from_glib(gst_sys::gst_parent_buffer_meta_api_get_type()) }
    }
}

impl fmt::Debug for ParentBufferMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ParentBufferMeta")
            .field("parent", &self.get_parent())
            .finish()
    }
}

#[repr(transparent)]
pub struct ProtectionMeta(gst_sys::GstProtectionMeta);

unsafe impl Send for ProtectionMeta {}
unsafe impl Sync for ProtectionMeta {}

impl ProtectionMeta {
    pub fn add(buffer: &mut BufferRef, info: ::Structure) -> MetaRefMut<Self, Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta =
                gst_sys::gst_buffer_add_protection_meta(buffer.as_mut_ptr(), info.into_ptr());

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn get_info(&self) -> &::StructureRef {
        unsafe { ::StructureRef::from_glib_borrow(self.0.info) }
    }

    pub fn get_info_mut(&mut self) -> &mut ::StructureRef {
        unsafe { ::StructureRef::from_glib_borrow_mut(self.0.info) }
    }
}

unsafe impl MetaAPI for ProtectionMeta {
    type GstType = gst_sys::GstProtectionMeta;

    fn get_meta_api() -> glib::Type {
        unsafe { from_glib(gst_sys::gst_protection_meta_api_get_type()) }
    }
}

impl fmt::Debug for ProtectionMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ProtectionMeta")
            .field("info", &self.get_info())
            .finish()
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[repr(transparent)]
pub struct ReferenceTimestampMeta(gst_sys::GstReferenceTimestampMeta);

#[cfg(any(feature = "v1_14", feature = "dox"))]
unsafe impl Send for ReferenceTimestampMeta {}
#[cfg(any(feature = "v1_14", feature = "dox"))]
unsafe impl Sync for ReferenceTimestampMeta {}

#[cfg(any(feature = "v1_14", feature = "dox"))]
impl ReferenceTimestampMeta {
    pub fn add<'a>(
        buffer: &'a mut BufferRef,
        reference: &Caps,
        timestamp: ClockTime,
        duration: ClockTime,
    ) -> MetaRefMut<'a, Self, Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = gst_sys::gst_buffer_add_reference_timestamp_meta(
                buffer.as_mut_ptr(),
                reference.to_glib_none().0,
                timestamp.to_glib(),
                duration.to_glib(),
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn get_reference(&self) -> &CapsRef {
        unsafe { CapsRef::from_ptr(self.0.reference) }
    }

    pub fn get_parent_owned(&self) -> Caps {
        unsafe { from_glib_none(self.0.reference) }
    }

    pub fn get_timestamp(&self) -> ClockTime {
        from_glib(self.0.timestamp)
    }

    pub fn get_duration(&self) -> ClockTime {
        from_glib(self.0.duration)
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
unsafe impl MetaAPI for ReferenceTimestampMeta {
    type GstType = gst_sys::GstReferenceTimestampMeta;

    fn get_meta_api() -> glib::Type {
        unsafe { from_glib(gst_sys::gst_reference_timestamp_meta_api_get_type()) }
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
impl fmt::Debug for ReferenceTimestampMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ReferenceTimestampMeta")
            .field("reference", &self.get_reference())
            .field("timestamp", &self.get_timestamp())
            .field("duration", &self.get_duration())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_get_iterate_meta() {
        ::init().unwrap();

        let mut buffer = ::Buffer::new();
        let parent = ::Buffer::new();
        {
            let meta = ParentBufferMeta::add(buffer.get_mut().unwrap(), &parent);
            unsafe {
                assert_eq!(meta.get_parent().as_ptr(), parent.as_ptr());
            }
        }

        {
            let metas = buffer.iter_meta::<Meta>();
            assert_eq!(metas.count(), 1);
        }
        {
            let metas = buffer.get_mut().unwrap().iter_meta_mut::<Meta>();
            assert_eq!(metas.count(), 1);
        }
        {
            let metas = buffer.iter_meta::<ParentBufferMeta>().collect::<Vec<_>>();
            assert_eq!(metas.len(), 1);
            unsafe {
                assert_eq!(metas[0].get_parent().as_ptr(), parent.as_ptr());
            }
        }
        {
            let metas = buffer
                .get_mut()
                .unwrap()
                .iter_meta_mut::<ParentBufferMeta>()
                .collect::<Vec<_>>();
            assert_eq!(metas.len(), 1);
            unsafe {
                assert_eq!(metas[0].get_parent().as_ptr(), parent.as_ptr());
            }
        }

        {
            let meta = buffer
                .get_mut()
                .unwrap()
                .get_meta_mut::<ParentBufferMeta>()
                .unwrap();
            unsafe {
                assert_eq!(meta.get_parent().as_ptr(), parent.as_ptr());
            }
            meta.remove();
        }

        {
            let metas = buffer.iter_meta::<Meta>();
            assert_eq!(metas.count(), 0);
        }
        {
            let metas = buffer.get_mut().unwrap().iter_meta_mut::<Meta>();
            assert_eq!(metas.count(), 0);
        }
        {
            let metas = buffer.iter_meta::<ParentBufferMeta>();
            assert_eq!(metas.count(), 0);
        }
        {
            let metas = buffer
                .get_mut()
                .unwrap()
                .iter_meta_mut::<ParentBufferMeta>();
            assert_eq!(metas.count(), 0);
        }

        assert!(buffer.get_meta::<ParentBufferMeta>().is_none());
    }
}
