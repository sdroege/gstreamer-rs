// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ops;

use miniobject::MiniObject;
use BufferRef;

use ffi;
use glib;
use glib::translate::{from_glib, FromGlib};
use glib_ffi;

pub unsafe trait MetaAPI: Sized {
    type GstType;

    fn get_meta_api() -> glib::Type;

    unsafe fn from_ptr<'a>(
        buffer: *const ffi::GstBuffer,
        ptr: *const Self::GstType,
    ) -> MetaRef<'a, Self> {
        assert!(!ptr.is_null());

        let meta_api = Self::get_meta_api();
        if meta_api != glib::Type::Invalid {
            assert_eq!(
                meta_api,
                from_glib((*(*(ptr as *const ffi::GstMeta)).info).api)
            )
        }

        MetaRef {
            meta: mem::transmute(ptr),
            buffer,
        }
    }

    unsafe fn from_mut_ptr<'a, T>(
        buffer: *mut ffi::GstBuffer,
        ptr: *mut Self::GstType,
    ) -> MetaRefMut<'a, Self, T> {
        assert!(!ptr.is_null());

        let meta_api = Self::get_meta_api();
        if meta_api != glib::Type::Invalid {
            assert_eq!(
                meta_api,
                from_glib((*(*(ptr as *const ffi::GstMeta)).info).api)
            )
        }

        MetaRefMut {
            meta: mem::transmute(ptr),
            buffer,
            mode: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct MetaRef<'a, T: MetaAPI + 'a> {
    meta: &'a T,
    buffer: *const ffi::GstBuffer,
}

pub enum Standalone {}
pub enum Iterated {}

#[derive(Debug)]
pub struct MetaRefMut<'a, T: MetaAPI + 'a, U> {
    meta: &'a mut T,
    buffer: *mut ffi::GstBuffer,
    mode: PhantomData<U>,
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
        unsafe { mem::transmute(self) }
    }
}

impl<'a, T: MetaAPI> MetaRef<'a, T> {
    pub fn get_api(&self) -> glib::Type {
        unsafe {
            let meta = self.meta as *const _ as *const ffi::GstMeta;
            let info = (*meta).info;
            glib::Type::from_glib((*info).api)
        }
    }

    pub fn as_ptr(&self) -> *const T::GstType {
        self.meta as *const _ as *const <T as MetaAPI>::GstType
    }
}

impl<'a> MetaRef<'a, Meta<'a>> {
    pub fn downcast_ref<T: MetaAPI>(&self) -> Option<&MetaRef<'a, T>> {
        let target_type = T::get_meta_api();
        let type_ = self.get_api();

        if type_ == glib::Type::Invalid || target_type == type_ {
            Some(unsafe { mem::transmute(self) })
        } else {
            None
        }
    }
}

impl<'a, T: MetaAPI, U> MetaRefMut<'a, T, U> {
    pub fn get_api(&self) -> glib::Type {
        unsafe {
            let meta = self.meta as *const _ as *const ffi::GstMeta;
            let info = (*meta).info;
            glib::Type::from_glib((*info).api)
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
            let res =
                ffi::gst_buffer_remove_meta(self.buffer, self.as_mut_ptr() as *mut ffi::GstMeta);
            assert_ne!(res, glib_ffi::GFALSE);
        }
    }
}

impl<'a, U> MetaRefMut<'a, Meta<'a>, U> {
    pub fn downcast_ref<T: MetaAPI>(&mut self) -> Option<&MetaRefMut<'a, T, U>> {
        let target_type = T::get_meta_api();
        let type_ = self.get_api();

        if type_ == glib::Type::Invalid || target_type == type_ {
            Some(unsafe { mem::transmute(self) })
        } else {
            None
        }
    }
}

#[repr(C)]
pub struct Meta<'a>(ffi::GstMeta, PhantomData<&'a ()>);

impl<'a> Meta<'a> {
    fn get_api(&self) -> glib::Type {
        unsafe { glib::Type::from_glib((*self.0.info).api) }
    }
}

unsafe impl<'a> MetaAPI for Meta<'a> {
    type GstType = ffi::GstMeta;

    fn get_meta_api() -> glib::Type {
        glib::Type::Invalid
    }
}

impl<'a> fmt::Debug for Meta<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Meta")
            .field("api", &self.get_api())
            .finish()
    }
}

#[repr(C)]
pub struct ParentBufferMeta<'a>(ffi::GstParentBufferMeta, PhantomData<&'a ()>);

impl<'a> ParentBufferMeta<'a> {
    pub fn add(buffer: &'a mut BufferRef, parent: &BufferRef) -> MetaRefMut<'a, Self, Standalone> {
        unsafe {
            let meta =
                ffi::gst_buffer_add_parent_buffer_meta(buffer.as_mut_ptr(), parent.as_mut_ptr());

            Self::from_mut_ptr(buffer.as_mut_ptr(), meta)
        }
    }

    pub fn get_parent(&self) -> &BufferRef {
        unsafe { BufferRef::from_ptr(self.0.buffer) }
    }
}

unsafe impl<'a> MetaAPI for ParentBufferMeta<'a> {
    type GstType = ffi::GstParentBufferMeta;

    fn get_meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_parent_buffer_meta_api_get_type()) }
    }
}

impl<'a> fmt::Debug for ParentBufferMeta<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ParentBufferMeta")
            .field("parent", &self.get_parent())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_get_iterate_meta() {
        gst::init().unwrap();

        let mut buffer = ::Buffer::new();
        let parent = ::Buffer::new();
        {
            let meta = ParentBufferMeta::add(buffer.get_mut().unwrap(), &*parent);
            unsafe {
                assert_eq!(meta.get_parent().as_ptr(), parent.as_ptr());
            }
        }

        {
            let metas = buffer.iter_meta::<Meta>().collect::<Vec<_>>();
            assert_eq!(metas.len(), 1);
        }
        {
            let metas = buffer
                .get_mut()
                .unwrap()
                .iter_meta_mut::<Meta>()
                .collect::<Vec<_>>();
            assert_eq!(metas.len(), 1);
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
            let metas = buffer.iter_meta::<Meta>().collect::<Vec<_>>();
            assert_eq!(metas.len(), 0);
        }
        {
            let metas = buffer
                .get_mut()
                .unwrap()
                .iter_meta_mut::<Meta>()
                .collect::<Vec<_>>();
            assert_eq!(metas.len(), 0);
        }
        {
            let metas = buffer.iter_meta::<ParentBufferMeta>().collect::<Vec<_>>();
            assert_eq!(metas.len(), 0);
        }
        {
            let metas = buffer
                .get_mut()
                .unwrap()
                .iter_meta_mut::<ParentBufferMeta>()
                .collect::<Vec<_>>();
            assert_eq!(metas.len(), 0);
        }

        assert!(buffer.get_meta::<ParentBufferMeta>().is_none());
    }
}
