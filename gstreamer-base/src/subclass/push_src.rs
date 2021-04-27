// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use std::ptr;

use super::base_src::BaseSrcImpl;
use crate::PushSrc;

pub trait PushSrcImpl: PushSrcImplExt + BaseSrcImpl {
    fn fill(
        &self,
        element: &Self::Type,
        buffer: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        PushSrcImplExt::parent_fill(self, element, buffer)
    }

    fn alloc(&self, element: &Self::Type) -> Result<gst::Buffer, gst::FlowError> {
        PushSrcImplExt::parent_alloc(self, element)
    }

    fn create(&self, element: &Self::Type) -> Result<gst::Buffer, gst::FlowError> {
        PushSrcImplExt::parent_create(self, element)
    }
}

pub trait PushSrcImplExt: ObjectSubclass {
    fn parent_fill(
        &self,
        element: &Self::Type,
        buffer: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_alloc(&self, element: &Self::Type) -> Result<gst::Buffer, gst::FlowError>;

    fn parent_create(&self, element: &Self::Type) -> Result<gst::Buffer, gst::FlowError>;
}

impl<T: PushSrcImpl> PushSrcImplExt for T {
    fn parent_fill(
        &self,
        element: &Self::Type,
        buffer: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstPushSrcClass;
            (*parent_class)
                .fill
                .map(|f| {
                    gst::FlowReturn::from_glib(f(
                        element.unsafe_cast_ref::<PushSrc>().to_glib_none().0,
                        buffer.as_mut_ptr(),
                    ))
                })
                .unwrap_or(gst::FlowReturn::NotSupported)
                .into_result()
        }
    }

    fn parent_alloc(&self, element: &Self::Type) -> Result<gst::Buffer, gst::FlowError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstPushSrcClass;
            (*parent_class)
                .alloc
                .map(|f| {
                    let mut buffer_ptr: *mut gst::ffi::GstBuffer = ptr::null_mut();

                    // FIXME: Wrong signature in -sys bindings
                    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
                    let buffer_ref = &mut buffer_ptr as *mut _ as *mut gst::ffi::GstBuffer;

                    let res = gst::FlowReturn::from_glib(f(
                        element.unsafe_cast_ref::<PushSrc>().to_glib_none().0,
                        buffer_ref,
                    ));
                    res.into_result_value(|| from_glib_full(buffer_ref))
                })
                .unwrap_or(Err(gst::FlowError::NotSupported))
        }
    }

    fn parent_create(&self, element: &Self::Type) -> Result<gst::Buffer, gst::FlowError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstPushSrcClass;
            (*parent_class)
                .create
                .map(|f| {
                    let mut buffer_ptr: *mut gst::ffi::GstBuffer = ptr::null_mut();

                    // FIXME: Wrong signature in -sys bindings
                    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
                    let buffer_ref = &mut buffer_ptr as *mut _ as *mut gst::ffi::GstBuffer;

                    let res = gst::FlowReturn::from_glib(f(
                        element.unsafe_cast_ref::<PushSrc>().to_glib_none().0,
                        buffer_ref,
                    ));
                    res.into_result_value(|| from_glib_full(buffer_ref))
                })
                .unwrap_or(Err(gst::FlowError::NotSupported))
        }
    }
}

unsafe impl<T: PushSrcImpl> IsSubclassable<T> for PushSrc {
    fn class_init(klass: &mut glib::Class<Self>) {
        <crate::BaseSrc as IsSubclassable<T>>::class_init(klass);
        let klass = klass.as_mut();
        klass.fill = Some(push_src_fill::<T>);
        klass.alloc = Some(push_src_alloc::<T>);
        klass.create = Some(push_src_create::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <crate::BaseSrc as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn push_src_fill<T: PushSrcImpl>(
    ptr: *mut ffi::GstPushSrc,
    buffer: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<PushSrc> = from_glib_borrow(ptr);
    let buffer = gst::BufferRef::from_mut_ptr(buffer);

    gst::panic_to_error!(&wrap, &imp.panicked(), gst::FlowReturn::Error, {
        PushSrcImpl::fill(imp, wrap.unsafe_cast_ref(), buffer).into()
    })
    .into_glib()
}

unsafe extern "C" fn push_src_alloc<T: PushSrcImpl>(
    ptr: *mut ffi::GstPushSrc,
    buffer_ptr: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<PushSrc> = from_glib_borrow(ptr);
    // FIXME: Wrong signature in -sys bindings
    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
    let buffer_ptr = buffer_ptr as *mut *mut gst::ffi::GstBuffer;

    gst::panic_to_error!(&wrap, &imp.panicked(), gst::FlowReturn::Error, {
        match PushSrcImpl::alloc(imp, wrap.unsafe_cast_ref()) {
            Ok(buffer) => {
                *buffer_ptr = buffer.into_ptr();
                gst::FlowReturn::Ok
            }
            Err(err) => gst::FlowReturn::from(err),
        }
    })
    .into_glib()
}

unsafe extern "C" fn push_src_create<T: PushSrcImpl>(
    ptr: *mut ffi::GstPushSrc,
    buffer_ptr: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<PushSrc> = from_glib_borrow(ptr);
    // FIXME: Wrong signature in -sys bindings
    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
    let buffer_ptr = buffer_ptr as *mut *mut gst::ffi::GstBuffer;

    gst::panic_to_error!(&wrap, &imp.panicked(), gst::FlowReturn::Error, {
        match PushSrcImpl::create(imp, wrap.unsafe_cast_ref()) {
            Ok(buffer) => {
                *buffer_ptr = buffer.into_ptr();
                gst::FlowReturn::Ok
            }
            Err(err) => gst::FlowReturn::from(err),
        }
    })
    .into_glib()
}
