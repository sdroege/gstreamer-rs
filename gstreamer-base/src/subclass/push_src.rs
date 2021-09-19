// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use gst::{gst_debug, gst_error};

use std::ptr;

use super::base_src::{BaseSrcImpl, CreateSuccess};
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

    fn create(
        &self,
        element: &Self::Type,
        buffer: Option<&mut gst::BufferRef>,
    ) -> Result<CreateSuccess, gst::FlowError> {
        PushSrcImplExt::parent_create(self, element, buffer)
    }
}

pub trait PushSrcImplExt: ObjectSubclass {
    fn parent_fill(
        &self,
        element: &Self::Type,
        buffer: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_alloc(&self, element: &Self::Type) -> Result<gst::Buffer, gst::FlowError>;

    fn parent_create(
        &self,
        element: &Self::Type,
        buffer: Option<&mut gst::BufferRef>,
    ) -> Result<CreateSuccess, gst::FlowError>;
}

impl<T: PushSrcImpl> PushSrcImplExt for T {
    fn parent_fill(
        &self,
        element: &Self::Type,
        buffer: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstPushSrcClass;
            (*parent_class)
                .fill
                .map(|f| {
                    try_from_glib(f(
                        element.unsafe_cast_ref::<PushSrc>().to_glib_none().0,
                        buffer.as_mut_ptr(),
                    ))
                })
                .unwrap_or(Err(gst::FlowError::NotSupported))
        }
    }

    fn parent_alloc(&self, element: &Self::Type) -> Result<gst::Buffer, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstPushSrcClass;
            (*parent_class)
                .alloc
                .map(|f| {
                    let mut buffer_ptr: *mut gst::ffi::GstBuffer = ptr::null_mut();

                    // FIXME: Wrong signature in -sys bindings
                    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
                    let buffer_ref = &mut buffer_ptr as *mut _ as *mut gst::ffi::GstBuffer;

                    gst::FlowSuccess::try_from_glib(f(
                        element.unsafe_cast_ref::<PushSrc>().to_glib_none().0,
                        buffer_ref,
                    ))
                    .map(|_| from_glib_full(buffer_ref))
                })
                .unwrap_or(Err(gst::FlowError::NotSupported))
        }
    }

    fn parent_create(
        &self,
        element: &Self::Type,
        mut buffer: Option<&mut gst::BufferRef>,
    ) -> Result<CreateSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstPushSrcClass;
            (*parent_class)
                .create
                .map(|f| {
                    let orig_buffer_ptr = buffer
                        .as_mut()
                        .map(|b| b.as_mut_ptr())
                        .unwrap_or(ptr::null_mut());
                    let mut buffer_ptr = orig_buffer_ptr;

                    // FIXME: Wrong signature in -sys bindings
                    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
                    let buffer_ref = &mut buffer_ptr as *mut _ as *mut gst::ffi::GstBuffer;

                    gst::FlowSuccess::try_from_glib(
                        f(
                            element.unsafe_cast_ref::<PushSrc>().to_glib_none().0,
                            buffer_ref,
                        )
                    )?;

                    if let Some(passed_buffer) = buffer {
                        if buffer_ptr != orig_buffer_ptr {
                            let new_buffer = gst::BufferRef::from_ptr(buffer_ptr);

                            gst_debug!(
                                gst::CAT_PERFORMANCE,
                                obj: element.unsafe_cast_ref::<PushSrc>(),
                                "Returned new buffer from parent create function, copying into passed buffer"
                            );

                            let mut map = match passed_buffer.map_writable() {
                                Ok(map) => map,
                                Err(_) => {
                                    gst_error!(
                                        gst::CAT_RUST,
                                        obj: element.unsafe_cast_ref::<PushSrc>(),
                                        "Failed to map passed buffer writable"
                                    );
                                    return Err(gst::FlowError::Error);
                                }
                            };

                            let copied_size = new_buffer.copy_to_slice(0, &mut *map);
                            drop(map);

                            if let Err(copied_size) = copied_size {
                                passed_buffer.set_size(copied_size);
                            }

                            match new_buffer.copy_into(passed_buffer, gst::BUFFER_COPY_METADATA, 0, None) {
                                Ok(_) => Ok(CreateSuccess::FilledBuffer),
                                Err(_) => {
                                    gst_error!(
                                        gst::CAT_RUST,
                                        obj: element.unsafe_cast_ref::<PushSrc>(),
                                        "Failed to copy buffer metadata"
                                    );

                                    Err(gst::FlowError::Error)
                                }
                            }
                        } else {
                            Ok(CreateSuccess::FilledBuffer)
                        }
                    } else {
                        Ok(CreateSuccess::NewBuffer(from_glib_full(buffer_ptr)))
                    }
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

    gst::panic_to_error!(&wrap, imp.panicked(), gst::FlowReturn::Error, {
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

    gst::panic_to_error!(&wrap, imp.panicked(), gst::FlowReturn::Error, {
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

    let mut buffer = if (*buffer_ptr).is_null() {
        None
    } else {
        Some(gst::BufferRef::from_mut_ptr(*buffer_ptr))
    };

    gst::panic_to_error!(&wrap, imp.panicked(), gst::FlowReturn::Error, {
        match PushSrcImpl::create(imp, wrap.unsafe_cast_ref(), buffer.as_deref_mut()) {
            Ok(CreateSuccess::NewBuffer(new_buffer)) => {
                if let Some(passed_buffer) = buffer {
                    if passed_buffer.as_ptr() != new_buffer.as_ptr() {
                        gst_debug!(
                            gst::CAT_PERFORMANCE,
                            obj: &*wrap,
                            "Returned new buffer from create function, copying into passed buffer"
                        );

                        let mut map = match passed_buffer.map_writable() {
                            Ok(map) => map,
                            Err(_) => {
                                gst_error!(
                                    gst::CAT_RUST,
                                    obj: &*wrap,
                                    "Failed to map passed buffer writable"
                                );
                                return gst::FlowReturn::Error;
                            }
                        };

                        let copied_size = new_buffer.copy_to_slice(0, &mut *map);
                        drop(map);

                        if let Err(copied_size) = copied_size {
                            passed_buffer.set_size(copied_size);
                        }

                        match new_buffer.copy_into(
                            passed_buffer,
                            gst::BUFFER_COPY_METADATA,
                            0,
                            None,
                        ) {
                            Ok(_) => gst::FlowReturn::Ok,
                            Err(_) => {
                                gst_error!(
                                    gst::CAT_RUST,
                                    obj: &*wrap,
                                    "Failed to copy buffer metadata"
                                );

                                gst::FlowReturn::Error
                            }
                        }
                    } else {
                        gst::FlowReturn::Ok
                    }
                } else {
                    *buffer_ptr = new_buffer.into_ptr();
                    gst::FlowReturn::Ok
                }
            }
            Ok(CreateSuccess::FilledBuffer) => gst::FlowReturn::Ok,
            Err(err) => gst::FlowReturn::from(err),
        }
    })
    .into_glib()
}
