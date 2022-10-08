// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;
use gst::prelude::*;

use std::ptr;

use super::base_src::{BaseSrcImpl, CreateSuccess};
use crate::prelude::BaseSrcExtManual;
use crate::PushSrc;

pub trait PushSrcImpl: PushSrcImplExt + BaseSrcImpl {
    fn fill(&self, buffer: &mut gst::BufferRef) -> Result<gst::FlowSuccess, gst::FlowError> {
        PushSrcImplExt::parent_fill(self, buffer)
    }

    fn alloc(&self) -> Result<gst::Buffer, gst::FlowError> {
        PushSrcImplExt::parent_alloc(self)
    }

    fn create(&self, buffer: Option<&mut gst::BufferRef>) -> Result<CreateSuccess, gst::FlowError> {
        PushSrcImplExt::parent_create(self, buffer)
    }
}

pub trait PushSrcImplExt: ObjectSubclass {
    fn parent_fill(&self, buffer: &mut gst::BufferRef) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_alloc(&self) -> Result<gst::Buffer, gst::FlowError>;

    fn parent_create(
        &self,
        buffer: Option<&mut gst::BufferRef>,
    ) -> Result<CreateSuccess, gst::FlowError>;
}

impl<T: PushSrcImpl> PushSrcImplExt for T {
    fn parent_fill(&self, buffer: &mut gst::BufferRef) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstPushSrcClass;
            (*parent_class)
                .fill
                .map(|f| {
                    try_from_glib(f(
                        self.instance()
                            .unsafe_cast_ref::<PushSrc>()
                            .to_glib_none()
                            .0,
                        buffer.as_mut_ptr(),
                    ))
                })
                .unwrap_or(Err(gst::FlowError::NotSupported))
        }
    }

    fn parent_alloc(&self) -> Result<gst::Buffer, gst::FlowError> {
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
                        self.instance()
                            .unsafe_cast_ref::<PushSrc>()
                            .to_glib_none()
                            .0,
                        buffer_ref,
                    ))
                    .map(|_| from_glib_full(buffer_ref))
                })
                .unwrap_or(Err(gst::FlowError::NotSupported))
        }
    }

    fn parent_create(
        &self,
        mut buffer: Option<&mut gst::BufferRef>,
    ) -> Result<CreateSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstPushSrcClass;
            (*parent_class)
                .create
                .map(|f| {
                    let instance = self.instance();
                    let instance = instance.unsafe_cast_ref::<PushSrc>();
                    let orig_buffer_ptr = buffer
                        .as_mut()
                        .map(|b| b.as_mut_ptr())
                        .unwrap_or(ptr::null_mut());
                    let mut buffer_ptr = orig_buffer_ptr;

                    // FIXME: Wrong signature in -sys bindings
                    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
                    let buffer_ref = &mut buffer_ptr as *mut _ as *mut gst::ffi::GstBuffer;
                    let instance_data = self.instance_data::<super::base_src::InstanceData>(crate::BaseSrc::static_type()).unwrap();

                    if let Err(err) = gst::FlowSuccess::try_from_glib(
                        f(
                            instance.to_glib_none().0,
                            buffer_ref,
                        )
                    ) {
                        *instance_data.pending_buffer_list.borrow_mut() = None;
                        return Err(err);
                    }

                    let pending_buffer_list = instance_data.pending_buffer_list.borrow_mut().take();
                    if pending_buffer_list.is_some() &&
                        (buffer.is_some() || instance.src_pad().mode() == gst::PadMode::Pull) {
                        panic!("Buffer lists can only be returned in push mode");
                    }

                    let pending_buffer_list = instance_data.pending_buffer_list.borrow_mut().take();
                    if buffer_ptr.is_null() && pending_buffer_list.is_none() {
                        gst::error!(
                            gst::CAT_RUST,
                            obj: instance,
                            "No buffer and no buffer list returned"
                        );
                        return Err(gst::FlowError::Error);
                    }

                    if !buffer_ptr.is_null() && pending_buffer_list.is_some() {
                        gst::error!(
                            gst::CAT_RUST,
                            obj: instance,
                            "Both buffer and buffer list returned"
                        );
                        return Err(gst::FlowError::Error);
                    }

                    if let Some(passed_buffer) = buffer {
                        if buffer_ptr != orig_buffer_ptr {
                            let new_buffer = gst::Buffer::from_glib_full(buffer_ptr);

                            gst::debug!(
                                gst::CAT_PERFORMANCE,
                                obj: instance,
                                "Returned new buffer from parent create function, copying into passed buffer"
                            );

                            let mut map = match passed_buffer.map_writable() {
                                Ok(map) => map,
                                Err(_) => {
                                    gst::error!(
                                        gst::CAT_RUST,
                                        obj: instance,
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
                                    gst::error!(
                                        gst::CAT_RUST,
                                        obj: instance,
                                        "Failed to copy buffer metadata"
                                    );

                                    Err(gst::FlowError::Error)
                                }
                            }
                        } else {
                            Ok(CreateSuccess::FilledBuffer)
                        }
                    } else if let Some(buffer_list) = pending_buffer_list {
                        Ok(CreateSuccess::NewBufferList(buffer_list))
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
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.fill = Some(push_src_fill::<T>);
        klass.alloc = Some(push_src_alloc::<T>);
        klass.create = Some(push_src_create::<T>);
    }
}

unsafe extern "C" fn push_src_fill<T: PushSrcImpl>(
    ptr: *mut ffi::GstPushSrc,
    buffer: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let buffer = gst::BufferRef::from_mut_ptr(buffer);

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        PushSrcImpl::fill(imp, buffer).into()
    })
    .into_glib()
}

unsafe extern "C" fn push_src_alloc<T: PushSrcImpl>(
    ptr: *mut ffi::GstPushSrc,
    buffer_ptr: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    // FIXME: Wrong signature in -sys bindings
    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
    let buffer_ptr = buffer_ptr as *mut *mut gst::ffi::GstBuffer;

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        match PushSrcImpl::alloc(imp) {
            Ok(buffer) => {
                *buffer_ptr = buffer.into_glib_ptr();
                gst::FlowReturn::Ok
            }
            Err(err) => gst::FlowReturn::from(err),
        }
    })
    .into_glib()
}

#[allow(clippy::needless_option_as_deref)]
unsafe extern "C" fn push_src_create<T: PushSrcImpl>(
    ptr: *mut ffi::GstPushSrc,
    buffer_ptr: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    // FIXME: Wrong signature in -sys bindings
    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
    let buffer_ptr = buffer_ptr as *mut *mut gst::ffi::GstBuffer;

    let mut buffer = if (*buffer_ptr).is_null() {
        None
    } else {
        Some(gst::BufferRef::from_mut_ptr(*buffer_ptr))
    };

    let instance_data = imp
        .instance_data::<super::base_src::InstanceData>(crate::BaseSrc::static_type())
        .unwrap();

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        match PushSrcImpl::create(imp, buffer.as_deref_mut()) {
            Ok(CreateSuccess::NewBuffer(new_buffer)) => {
                // Clear any pending buffer list
                *instance_data.pending_buffer_list.borrow_mut() = None;

                if let Some(passed_buffer) = buffer {
                    if passed_buffer.as_ptr() != new_buffer.as_ptr() {
                        gst::debug!(
                            gst::CAT_PERFORMANCE,
                            imp: imp,
                            "Returned new buffer from create function, copying into passed buffer"
                        );

                        let mut map = match passed_buffer.map_writable() {
                            Ok(map) => map,
                            Err(_) => {
                                gst::error!(
                                    gst::CAT_RUST,
                                    imp: imp,
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
                                gst::error!(
                                    gst::CAT_RUST,
                                    imp: imp,
                                    "Failed to copy buffer metadata"
                                );

                                gst::FlowReturn::Error
                            }
                        }
                    } else {
                        gst::FlowReturn::Ok
                    }
                } else {
                    *buffer_ptr = new_buffer.into_glib_ptr();
                    gst::FlowReturn::Ok
                }
            }
            Ok(CreateSuccess::NewBufferList(new_buffer_list)) => {
                if buffer.is_some()
                    || imp.instance().unsafe_cast_ref::<PushSrc>().src_pad().mode()
                        == gst::PadMode::Pull
                {
                    panic!("Buffer lists can only be returned in push mode");
                }

                *buffer_ptr = ptr::null_mut();

                // Store it in the instance data so that in the end base_src_create() can
                // submit it.
                *instance_data.pending_buffer_list.borrow_mut() = Some(new_buffer_list);

                gst::FlowReturn::Ok
            }
            Ok(CreateSuccess::FilledBuffer) => {
                // Clear any pending buffer list
                *instance_data.pending_buffer_list.borrow_mut() = None;

                gst::FlowReturn::Ok
            }
            Err(err) => gst::FlowReturn::from(err),
        }
    })
    .into_glib()
}
