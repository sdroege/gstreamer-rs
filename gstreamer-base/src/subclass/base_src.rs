// Copyright (C) 2017-2019 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib_sys;
use gst_base_sys;
use gst_sys;

use glib::subclass::prelude::*;
use glib::translate::*;

use gst;
use gst::subclass::prelude::*;

use std::mem;
use std::ptr;

use BaseSrc;
use BaseSrcClass;

#[derive(Debug)]
pub enum CreateSuccess {
    FilledBuffer,
    NewBuffer(gst::Buffer),
}

pub trait BaseSrcImpl: BaseSrcImplExt + ElementImpl + Send + Sync + 'static {
    fn start(&self, element: &BaseSrc) -> Result<(), gst::ErrorMessage> {
        self.parent_start(element)
    }

    fn stop(&self, element: &BaseSrc) -> Result<(), gst::ErrorMessage> {
        self.parent_stop(element)
    }

    fn is_seekable(&self, element: &BaseSrc) -> bool {
        self.parent_is_seekable(element)
    }

    fn get_size(&self, element: &BaseSrc) -> Option<u64> {
        self.parent_get_size(element)
    }

    fn get_times(
        &self,
        element: &BaseSrc,
        buffer: &gst::BufferRef,
    ) -> (gst::ClockTime, gst::ClockTime) {
        self.parent_get_times(element, buffer)
    }

    fn fill(
        &self,
        element: &BaseSrc,
        offset: u64,
        length: u32,
        buffer: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_fill(element, offset, length, buffer)
    }

    fn alloc(
        &self,
        element: &BaseSrc,
        offset: u64,
        length: u32,
    ) -> Result<gst::Buffer, gst::FlowError> {
        self.parent_alloc(element, offset, length)
    }

    fn create(
        &self,
        element: &BaseSrc,
        offset: u64,
        buffer: Option<&mut gst::BufferRef>,
        length: u32,
    ) -> Result<CreateSuccess, gst::FlowError> {
        self.parent_create(element, offset, buffer, length)
    }

    fn do_seek(&self, element: &BaseSrc, segment: &mut gst::Segment) -> bool {
        self.parent_do_seek(element, segment)
    }

    fn query(&self, element: &BaseSrc, query: &mut gst::QueryRef) -> bool {
        BaseSrcImplExt::parent_query(self, element, query)
    }

    fn event(&self, element: &BaseSrc, event: &gst::Event) -> bool {
        self.parent_event(element, event)
    }

    fn get_caps(&self, element: &BaseSrc, filter: Option<&gst::Caps>) -> Option<gst::Caps> {
        self.parent_get_caps(element, filter)
    }

    fn negotiate(&self, element: &BaseSrc) -> Result<(), gst::LoggableError> {
        self.parent_negotiate(element)
    }

    fn set_caps(&self, element: &BaseSrc, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        self.parent_set_caps(element, caps)
    }

    fn fixate(&self, element: &BaseSrc, caps: gst::Caps) -> gst::Caps {
        self.parent_fixate(element, caps)
    }

    fn unlock(&self, element: &BaseSrc) -> Result<(), gst::ErrorMessage> {
        self.parent_unlock(element)
    }

    fn unlock_stop(&self, element: &BaseSrc) -> Result<(), gst::ErrorMessage> {
        self.parent_unlock_stop(element)
    }
}

pub trait BaseSrcImplExt {
    fn parent_start(&self, element: &BaseSrc) -> Result<(), gst::ErrorMessage>;

    fn parent_stop(&self, element: &BaseSrc) -> Result<(), gst::ErrorMessage>;

    fn parent_is_seekable(&self, element: &BaseSrc) -> bool;

    fn parent_get_size(&self, element: &BaseSrc) -> Option<u64>;

    fn parent_get_times(
        &self,
        element: &BaseSrc,
        buffer: &gst::BufferRef,
    ) -> (gst::ClockTime, gst::ClockTime);

    fn parent_fill(
        &self,
        element: &BaseSrc,
        offset: u64,
        length: u32,
        buffer: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_alloc(
        &self,
        element: &BaseSrc,
        offset: u64,
        length: u32,
    ) -> Result<gst::Buffer, gst::FlowError>;

    fn parent_create(
        &self,
        element: &BaseSrc,
        offset: u64,
        buffer: Option<&mut gst::BufferRef>,
        length: u32,
    ) -> Result<CreateSuccess, gst::FlowError>;

    fn parent_do_seek(&self, element: &BaseSrc, segment: &mut gst::Segment) -> bool;

    fn parent_query(&self, element: &BaseSrc, query: &mut gst::QueryRef) -> bool;

    fn parent_event(&self, element: &BaseSrc, event: &gst::Event) -> bool;

    fn parent_get_caps(&self, element: &BaseSrc, filter: Option<&gst::Caps>) -> Option<gst::Caps>;

    fn parent_negotiate(&self, element: &BaseSrc) -> Result<(), gst::LoggableError>;

    fn parent_set_caps(
        &self,
        element: &BaseSrc,
        caps: &gst::Caps,
    ) -> Result<(), gst::LoggableError>;

    fn parent_fixate(&self, element: &BaseSrc, caps: gst::Caps) -> gst::Caps;

    fn parent_unlock(&self, element: &BaseSrc) -> Result<(), gst::ErrorMessage>;

    fn parent_unlock_stop(&self, element: &BaseSrc) -> Result<(), gst::ErrorMessage>;
}

impl<T: BaseSrcImpl + ObjectImpl> BaseSrcImplExt for T {
    fn parent_start(&self, element: &BaseSrc) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSrcClass;
            (*parent_class)
                .start
                .map(|f| {
                    if from_glib(f(element.to_glib_none().0)) {
                        Ok(())
                    } else {
                        Err(gst_error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `start` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_stop(&self, element: &BaseSrc) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSrcClass;
            (*parent_class)
                .stop
                .map(|f| {
                    if from_glib(f(element.to_glib_none().0)) {
                        Ok(())
                    } else {
                        Err(gst_error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `stop` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_is_seekable(&self, element: &BaseSrc) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSrcClass;
            (*parent_class)
                .is_seekable
                .map(|f| from_glib(f(element.to_glib_none().0)))
                .unwrap_or(false)
        }
    }

    fn parent_get_size(&self, element: &BaseSrc) -> Option<u64> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSrcClass;
            (*parent_class)
                .get_size
                .map(|f| {
                    let mut size = mem::MaybeUninit::uninit();
                    if from_glib(f(element.to_glib_none().0, size.as_mut_ptr())) {
                        Some(size.assume_init())
                    } else {
                        None
                    }
                })
                .unwrap_or(None)
        }
    }

    fn parent_get_times(
        &self,
        element: &BaseSrc,
        buffer: &gst::BufferRef,
    ) -> (gst::ClockTime, gst::ClockTime) {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSrcClass;
            (*parent_class)
                .get_times
                .map(|f| {
                    let mut start = mem::MaybeUninit::uninit();
                    let mut stop = mem::MaybeUninit::uninit();
                    f(
                        element.to_glib_none().0,
                        buffer.as_mut_ptr(),
                        start.as_mut_ptr(),
                        stop.as_mut_ptr(),
                    );
                    (
                        from_glib(start.assume_init()),
                        from_glib(stop.assume_init()),
                    )
                })
                .unwrap_or((gst::CLOCK_TIME_NONE, gst::CLOCK_TIME_NONE))
        }
    }

    fn parent_fill(
        &self,
        element: &BaseSrc,
        offset: u64,
        length: u32,
        buffer: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSrcClass;
            (*parent_class)
                .fill
                .map(|f| {
                    gst::FlowReturn::from_glib(f(
                        element.to_glib_none().0,
                        offset,
                        length,
                        buffer.as_mut_ptr(),
                    ))
                })
                .unwrap_or(gst::FlowReturn::NotSupported)
                .into_result()
        }
    }

    fn parent_alloc(
        &self,
        element: &BaseSrc,
        offset: u64,
        length: u32,
    ) -> Result<gst::Buffer, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSrcClass;
            (*parent_class)
                .alloc
                .map(|f| {
                    let mut buffer_ptr: *mut gst_sys::GstBuffer = ptr::null_mut();

                    // FIXME: Wrong signature in -sys bindings
                    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
                    let buffer_ref = &mut buffer_ptr as *mut _ as *mut gst_sys::GstBuffer;

                    let res = gst::FlowReturn::from_glib(f(
                        element.to_glib_none().0,
                        offset,
                        length,
                        buffer_ref,
                    ));
                    res.into_result_value(|| from_glib_full(buffer_ref))
                })
                .unwrap_or(Err(gst::FlowError::NotSupported))
        }
    }

    fn parent_create(
        &self,
        element: &BaseSrc,
        offset: u64,
        mut buffer: Option<&mut gst::BufferRef>,
        length: u32,
    ) -> Result<CreateSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSrcClass;
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
                    let buffer_ref = &mut buffer_ptr as *mut _ as *mut gst_sys::GstBuffer;

                    gst::FlowReturn::from_glib(
                        f(
                            element.to_glib_none().0,
                            offset,
                            length,
                            buffer_ref,
                        )
                    ).into_result()?;

                    if let Some(passed_buffer) = buffer {
                        if buffer_ptr != orig_buffer_ptr {
                            let new_buffer = gst::BufferRef::from_ptr(buffer_ptr);

                            gst_debug!(
                                gst::CAT_PERFORMANCE,
                                obj: element,
                                "Returned new buffer from parent create function, copying into passed buffer"
                            );

                            let mut map = match passed_buffer.map_writable() {
                                Ok(map) => map,
                                Err(_) => {
                                    gst_error!(
                                        gst::CAT_RUST,
                                        obj: element,
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
                                        obj: element,
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

    fn parent_do_seek(&self, element: &BaseSrc, segment: &mut gst::Segment) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSrcClass;
            (*parent_class)
                .do_seek
                .map(|f| from_glib(f(element.to_glib_none().0, segment.to_glib_none_mut().0)))
                .unwrap_or(false)
        }
    }

    fn parent_query(&self, element: &BaseSrc, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSrcClass;
            (*parent_class)
                .query
                .map(|f| from_glib(f(element.to_glib_none().0, query.as_mut_ptr())))
                .unwrap_or(false)
        }
    }

    fn parent_event(&self, element: &BaseSrc, event: &gst::Event) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSrcClass;
            (*parent_class)
                .event
                .map(|f| from_glib(f(element.to_glib_none().0, event.to_glib_none().0)))
                .unwrap_or(false)
        }
    }

    fn parent_get_caps(&self, element: &BaseSrc, filter: Option<&gst::Caps>) -> Option<gst::Caps> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSrcClass;

            (*parent_class)
                .get_caps
                .map(|f| from_glib_full(f(element.to_glib_none().0, filter.to_glib_none().0)))
                .unwrap_or(None)
        }
    }

    fn parent_negotiate(&self, element: &BaseSrc) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSrcClass;
            (*parent_class)
                .negotiate
                .map(|f| {
                    gst_result_from_gboolean!(
                        f(element.to_glib_none().0),
                        gst::CAT_RUST,
                        "Parent function `negotiate` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_set_caps(
        &self,
        element: &BaseSrc,
        caps: &gst::Caps,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSrcClass;
            (*parent_class)
                .set_caps
                .map(|f| {
                    gst_result_from_gboolean!(
                        f(element.to_glib_none().0, caps.to_glib_none().0),
                        gst::CAT_RUST,
                        "Parent function `set_caps` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_fixate(&self, element: &BaseSrc, caps: gst::Caps) -> gst::Caps {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSrcClass;

            match (*parent_class).fixate {
                Some(fixate) => from_glib_full(fixate(element.to_glib_none().0, caps.into_ptr())),
                None => caps,
            }
        }
    }

    fn parent_unlock(&self, element: &BaseSrc) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSrcClass;
            (*parent_class)
                .unlock
                .map(|f| {
                    if from_glib(f(element.to_glib_none().0)) {
                        Ok(())
                    } else {
                        Err(gst_error_msg!(
                            gst::CoreError::Failed,
                            ["Parent function `unlock` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_unlock_stop(&self, element: &BaseSrc) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSrcClass;
            (*parent_class)
                .unlock_stop
                .map(|f| {
                    if from_glib(f(element.to_glib_none().0)) {
                        Ok(())
                    } else {
                        Err(gst_error_msg!(
                            gst::CoreError::Failed,
                            ["Parent function `unlock_stop` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }
}

unsafe impl<T: ObjectSubclass + BaseSrcImpl> IsSubclassable<T> for BaseSrcClass
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(&mut self) {
        <gst::ElementClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gst_base_sys::GstBaseSrcClass);
            klass.start = Some(base_src_start::<T>);
            klass.stop = Some(base_src_stop::<T>);
            klass.is_seekable = Some(base_src_is_seekable::<T>);
            klass.get_size = Some(base_src_get_size::<T>);
            klass.get_times = Some(base_src_get_times::<T>);
            klass.fill = Some(base_src_fill::<T>);
            klass.alloc = Some(base_src_alloc::<T>);
            klass.create = Some(base_src_create::<T>);
            klass.do_seek = Some(base_src_do_seek::<T>);
            klass.query = Some(base_src_query::<T>);
            klass.event = Some(base_src_event::<T>);
            klass.get_caps = Some(base_src_get_caps::<T>);
            klass.negotiate = Some(base_src_negotiate::<T>);
            klass.set_caps = Some(base_src_set_caps::<T>);
            klass.fixate = Some(base_src_fixate::<T>);
            klass.unlock = Some(base_src_unlock::<T>);
            klass.unlock_stop = Some(base_src_unlock_stop::<T>);
        }
    }
}

unsafe extern "C" fn base_src_start<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSrc,
) -> glib_sys::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.start(&wrap) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_src_stop<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSrc,
) -> glib_sys::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.stop(&wrap) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_src_is_seekable<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSrc,
) -> glib_sys::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.is_seekable(&wrap)
    })
    .to_glib()
}

unsafe extern "C" fn base_src_get_size<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSrc,
    size: *mut u64,
) -> glib_sys::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.get_size(&wrap) {
            Some(s) => {
                *size = s;
                true
            }
            None => false,
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_src_get_times<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSrc,
    buffer: *mut gst_sys::GstBuffer,
    start: *mut gst_sys::GstClockTime,
    stop: *mut gst_sys::GstClockTime,
) where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);
    let buffer = gst::BufferRef::from_ptr(buffer);

    *start = gst_sys::GST_CLOCK_TIME_NONE;
    *stop = gst_sys::GST_CLOCK_TIME_NONE;

    gst_panic_to_error!(&wrap, &instance.panicked(), (), {
        let (start_, stop_) = imp.get_times(&wrap, buffer);
        *start = start_.to_glib();
        *stop = stop_.to_glib();
    });
}

unsafe extern "C" fn base_src_fill<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSrc,
    offset: u64,
    length: u32,
    buffer: *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);
    let buffer = gst::BufferRef::from_mut_ptr(buffer);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.fill(&wrap, offset, length, buffer).into()
    })
    .to_glib()
}

unsafe extern "C" fn base_src_alloc<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSrc,
    offset: u64,
    length: u32,
    buffer_ptr: *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);
    // FIXME: Wrong signature in -sys bindings
    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
    let buffer_ptr = buffer_ptr as *mut *mut gst_sys::GstBuffer;

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        match imp.alloc(&wrap, offset, length) {
            Ok(buffer) => {
                *buffer_ptr = buffer.into_ptr();
                gst::FlowReturn::Ok
            }
            Err(err) => gst::FlowReturn::from(err),
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_src_create<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSrc,
    offset: u64,
    length: u32,
    buffer_ptr: *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);
    // FIXME: Wrong signature in -sys bindings
    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
    let buffer_ptr = buffer_ptr as *mut *mut gst_sys::GstBuffer;

    let mut buffer = if (*buffer_ptr).is_null() {
        None
    } else {
        Some(gst::BufferRef::from_mut_ptr(*buffer_ptr))
    };

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        match imp.create(&wrap, offset, buffer.as_deref_mut(), length) {
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
    .to_glib()
}

unsafe extern "C" fn base_src_do_seek<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSrc,
    segment: *mut gst_sys::GstSegment,
) -> glib_sys::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        let mut s = from_glib_none(segment);
        let res = imp.do_seek(&wrap, &mut s);
        ptr::write(segment, *(s.to_glib_none().0));

        res
    })
    .to_glib()
}

unsafe extern "C" fn base_src_query<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSrc,
    query_ptr: *mut gst_sys::GstQuery,
) -> glib_sys::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);
    let query = gst::QueryRef::from_mut_ptr(query_ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        BaseSrcImpl::query(imp, &wrap, query)
    })
    .to_glib()
}

unsafe extern "C" fn base_src_event<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSrc,
    event_ptr: *mut gst_sys::GstEvent,
) -> glib_sys::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.event(&wrap, &from_glib_borrow(event_ptr))
    })
    .to_glib()
}

unsafe extern "C" fn base_src_get_caps<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSrc,
    filter: *mut gst_sys::GstCaps,
) -> *mut gst_sys::GstCaps
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);
    let filter = Option::<gst::Caps>::from_glib_borrow(filter);

    gst_panic_to_error!(&wrap, &instance.panicked(), None, {
        imp.get_caps(&wrap, filter.as_ref().as_ref())
    })
    .map(|caps| caps.into_ptr())
    .unwrap_or(ptr::null_mut())
}

unsafe extern "C" fn base_src_negotiate<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSrc,
) -> glib_sys::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.negotiate(&wrap) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_src_set_caps<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSrc,
    caps: *mut gst_sys::GstCaps,
) -> glib_sys::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);
    let caps = from_glib_borrow(caps);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.set_caps(&wrap, &caps) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_src_fixate<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSrc,
    caps: *mut gst_sys::GstCaps,
) -> *mut gst_sys::GstCaps
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);
    let caps = from_glib_full(caps);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::Caps::new_empty(), {
        imp.fixate(&wrap, caps)
    })
    .into_ptr()
}

unsafe extern "C" fn base_src_unlock<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSrc,
) -> glib_sys::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.unlock(&wrap) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_src_unlock_stop<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSrc,
) -> glib_sys::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.unlock_stop(&wrap) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}
