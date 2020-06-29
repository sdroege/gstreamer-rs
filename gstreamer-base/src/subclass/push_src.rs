// Copyright (C) 2020 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_base_sys;
use gst_sys;

use glib::subclass::prelude::*;
use glib::translate::*;

use gst;
use gst::subclass::prelude::*;

use std::ptr;

use super::base_src::BaseSrcImpl;
use PushSrc;
use PushSrcClass;

pub trait PushSrcImpl: PushSrcImplExt + BaseSrcImpl + Send + Sync + 'static {
    fn fill(
        &self,
        element: &PushSrc,
        buffer: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        PushSrcImplExt::parent_fill(self, element, buffer)
    }

    fn alloc(&self, element: &PushSrc) -> Result<gst::Buffer, gst::FlowError> {
        PushSrcImplExt::parent_alloc(self, element)
    }

    fn create(&self, element: &PushSrc) -> Result<gst::Buffer, gst::FlowError> {
        PushSrcImplExt::parent_create(self, element)
    }
}

pub trait PushSrcImplExt {
    fn parent_fill(
        &self,
        element: &PushSrc,
        buffer: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_alloc(&self, element: &PushSrc) -> Result<gst::Buffer, gst::FlowError>;

    fn parent_create(&self, element: &PushSrc) -> Result<gst::Buffer, gst::FlowError>;
}

impl<T: PushSrcImpl + ObjectImpl> PushSrcImplExt for T {
    fn parent_fill(
        &self,
        element: &PushSrc,
        buffer: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstPushSrcClass;
            (*parent_class)
                .fill
                .map(|f| {
                    gst::FlowReturn::from_glib(f(element.to_glib_none().0, buffer.as_mut_ptr()))
                })
                .unwrap_or(gst::FlowReturn::NotSupported)
                .into_result()
        }
    }

    fn parent_alloc(&self, element: &PushSrc) -> Result<gst::Buffer, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstPushSrcClass;
            (*parent_class)
                .alloc
                .map(|f| {
                    let mut buffer_ptr: *mut gst_sys::GstBuffer = ptr::null_mut();

                    // FIXME: Wrong signature in -sys bindings
                    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
                    let buffer_ref = &mut buffer_ptr as *mut _ as *mut gst_sys::GstBuffer;

                    let res = gst::FlowReturn::from_glib(f(element.to_glib_none().0, buffer_ref));
                    res.into_result_value(|| from_glib_full(buffer_ref))
                })
                .unwrap_or(Err(gst::FlowError::NotSupported))
        }
    }

    fn parent_create(&self, element: &PushSrc) -> Result<gst::Buffer, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstPushSrcClass;
            (*parent_class)
                .create
                .map(|f| {
                    let mut buffer_ptr: *mut gst_sys::GstBuffer = ptr::null_mut();

                    // FIXME: Wrong signature in -sys bindings
                    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
                    let buffer_ref = &mut buffer_ptr as *mut _ as *mut gst_sys::GstBuffer;

                    let res = gst::FlowReturn::from_glib(f(element.to_glib_none().0, buffer_ref));
                    res.into_result_value(|| from_glib_full(buffer_ref))
                })
                .unwrap_or(Err(gst::FlowError::NotSupported))
        }
    }
}

unsafe impl<T: ObjectSubclass + PushSrcImpl> IsSubclassable<T> for PushSrcClass
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(&mut self) {
        <::BaseSrcClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gst_base_sys::GstPushSrcClass);
            klass.fill = Some(push_src_fill::<T>);
            klass.alloc = Some(push_src_alloc::<T>);
            klass.create = Some(push_src_create::<T>);
        }
    }
}

unsafe extern "C" fn push_src_fill<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstPushSrc,
    buffer: *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
where
    T: PushSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<PushSrc> = from_glib_borrow(ptr);
    let buffer = gst::BufferRef::from_mut_ptr(buffer);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        PushSrcImpl::fill(imp, &wrap, buffer).into()
    })
    .to_glib()
}

unsafe extern "C" fn push_src_alloc<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstPushSrc,
    buffer_ptr: *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
where
    T: PushSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<PushSrc> = from_glib_borrow(ptr);
    // FIXME: Wrong signature in -sys bindings
    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
    let buffer_ptr = buffer_ptr as *mut *mut gst_sys::GstBuffer;

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        match PushSrcImpl::alloc(imp, &wrap) {
            Ok(buffer) => {
                *buffer_ptr = buffer.into_ptr();
                gst::FlowReturn::Ok
            }
            Err(err) => gst::FlowReturn::from(err),
        }
    })
    .to_glib()
}

unsafe extern "C" fn push_src_create<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstPushSrc,
    buffer_ptr: *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
where
    T: PushSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<PushSrc> = from_glib_borrow(ptr);
    // FIXME: Wrong signature in -sys bindings
    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
    let buffer_ptr = buffer_ptr as *mut *mut gst_sys::GstBuffer;

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        match PushSrcImpl::create(imp, &wrap) {
            Ok(buffer) => {
                *buffer_ptr = buffer.into_ptr();
                gst::FlowReturn::Ok
            }
            Err(err) => gst::FlowReturn::from(err),
        }
    })
    .to_glib()
}
