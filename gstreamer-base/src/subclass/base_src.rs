// Copyright (C) 2017,2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib_ffi;
use gst_ffi;

use glib::translate::*;
use prelude::*;

use glib::subclass::prelude::*;
use gst;
use gst::subclass::prelude::*;

use std::ptr;

use BaseSrc;
use BaseSrcClass;

pub trait BaseSrcImpl: ElementImpl + Send + Sync + 'static {
    fn start(&self, _element: &BaseSrc) -> bool {
        true
    }

    fn stop(&self, _element: &BaseSrc) -> bool {
        true
    }

    fn is_seekable(&self, _element: &BaseSrc) -> bool {
        false
    }

    fn get_size(&self, _element: &BaseSrc) -> Option<u64> {
        None
    }

    fn fill(
        &self,
        _element: &BaseSrc,
        _offset: u64,
        _length: u32,
        _buffer: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unimplemented!()
    }

    fn create(
        &self,
        element: &BaseSrc,
        offset: u64,
        length: u32,
    ) -> Result<gst::Buffer, gst::FlowError> {
        self.parent_create(element, offset, length)
    }

    fn do_seek(&self, element: &BaseSrc, segment: &mut gst::Segment) -> bool {
        self.parent_do_seek(element, segment)
    }

    fn query(&self, element: &BaseSrc, query: &mut gst::QueryRef) -> bool {
        BaseSrcImpl::parent_query(self, element, query)
    }

    fn event(&self, element: &BaseSrc, event: &gst::Event) -> bool {
        self.parent_event(element, event)
    }

    fn get_caps(&self, element: &BaseSrc, filter: Option<&gst::CapsRef>) -> Option<gst::Caps> {
        self.parent_get_caps(element, filter)
    }

    fn negotiate(&self, element: &BaseSrc) -> Result<(), gst::LoggableError> {
        self.parent_negotiate(element)
    }

    fn set_caps(&self, element: &BaseSrc, caps: &gst::CapsRef) -> Result<(), gst::LoggableError> {
        self.parent_set_caps(element, caps)
    }

    fn fixate(&self, element: &BaseSrc, caps: gst::Caps) -> gst::Caps {
        self.parent_fixate(element, caps)
    }

    fn unlock(&self, _element: &BaseSrc) -> bool {
        true
    }

    fn unlock_stop(&self, _element: &BaseSrc) -> bool {
        true
    }

    fn parent_create(
        &self,
        element: &BaseSrc,
        offset: u64,
        length: u32,
    ) -> Result<gst::Buffer, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSrcClass;
            (*parent_class)
                .create
                .map(|f| {
                    let mut buffer: *mut gst_ffi::GstBuffer = ptr::null_mut();
                    // FIXME: Wrong signature in -sys bindings
                    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
                    let buffer_ref = &mut buffer as *mut _ as *mut gst_ffi::GstBuffer;
                    let ret: gst::FlowReturn =
                        from_glib(f(element.to_glib_none().0, offset, length, buffer_ref));

                    ret.into_result_value(|| from_glib_full(buffer))
                })
                .unwrap_or(Err(gst::FlowError::Error))
        }
    }

    fn parent_do_seek(&self, element: &BaseSrc, segment: &mut gst::Segment) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSrcClass;
            (*parent_class)
                .do_seek
                .map(|f| from_glib(f(element.to_glib_none().0, segment.to_glib_none_mut().0)))
                .unwrap_or(false)
        }
    }

    fn parent_query(&self, element: &BaseSrc, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSrcClass;
            (*parent_class)
                .query
                .map(|f| from_glib(f(element.to_glib_none().0, query.as_mut_ptr())))
                .unwrap_or(false)
        }
    }

    fn parent_event(&self, element: &BaseSrc, event: &gst::Event) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSrcClass;
            (*parent_class)
                .event
                .map(|f| from_glib(f(element.to_glib_none().0, event.to_glib_none().0)))
                .unwrap_or(false)
        }
    }

    fn parent_get_caps(
        &self,
        element: &BaseSrc,
        filter: Option<&gst::CapsRef>,
    ) -> Option<gst::Caps> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSrcClass;
            let filter_ptr = if let Some(filter) = filter {
                filter.as_mut_ptr()
            } else {
                ptr::null_mut()
            };

            (*parent_class)
                .get_caps
                .map(|f| from_glib_full(f(element.to_glib_none().0, filter_ptr)))
                .unwrap_or(None)
        }
    }

    fn parent_negotiate(&self, element: &BaseSrc) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSrcClass;
            let f = (*parent_class).negotiate.ok_or_else(|| {
                gst_loggable_error!(gst::CAT_RUST, "Parent function `negotiate` is not defined")
            })?;
            gst_result_from_gboolean!(
                f(element.to_glib_none().0),
                gst::CAT_RUST,
                "Parent function `negotiate` failed"
            )
        }
    }

    fn parent_set_caps(
        &self,
        element: &BaseSrc,
        caps: &gst::CapsRef,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSrcClass;
            let f = (*parent_class).set_caps.ok_or_else(|| {
                gst_loggable_error!(gst::CAT_RUST, "Parent function `set_caps` is not defined")
            })?;
            gst_result_from_gboolean!(
                f(element.to_glib_none().0, caps.as_mut_ptr()),
                gst::CAT_RUST,
                "Parent function `set_caps` failed"
            )
        }
    }

    fn parent_fixate(&self, element: &BaseSrc, caps: gst::Caps) -> gst::Caps {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSrcClass;

            match (*parent_class).fixate {
                Some(fixate) => from_glib_full(fixate(element.to_glib_none().0, caps.into_ptr())),
                None => caps,
            }
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
            let klass = &mut *(self as *mut Self as *mut ffi::GstBaseSrcClass);
            klass.start = Some(base_src_start::<T>);
            klass.stop = Some(base_src_stop::<T>);
            klass.is_seekable = Some(base_src_is_seekable::<T>);
            klass.get_size = Some(base_src_get_size::<T>);
            klass.fill = Some(base_src_fill::<T>);
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
    ptr: *mut ffi::GstBaseSrc,
) -> glib_ffi::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSrc = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, { imp.start(&wrap) }).to_glib()
}

unsafe extern "C" fn base_src_stop<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSrc,
) -> glib_ffi::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSrc = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, { imp.stop(&wrap) }).to_glib()
}

unsafe extern "C" fn base_src_is_seekable<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSrc,
) -> glib_ffi::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSrc = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.is_seekable(&wrap)
    })
    .to_glib()
}

unsafe extern "C" fn base_src_get_size<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSrc,
    size: *mut u64,
) -> glib_ffi::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSrc = from_glib_borrow(ptr);

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

unsafe extern "C" fn base_src_fill<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSrc,
    offset: u64,
    length: u32,
    buffer: *mut gst_ffi::GstBuffer,
) -> gst_ffi::GstFlowReturn
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSrc = from_glib_borrow(ptr);
    let buffer = gst::BufferRef::from_mut_ptr(buffer);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.fill(&wrap, offset, length, buffer).into()
    })
    .to_glib()
}

unsafe extern "C" fn base_src_create<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSrc,
    offset: u64,
    length: u32,
    buffer_ptr: *mut gst_ffi::GstBuffer,
) -> gst_ffi::GstFlowReturn
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSrc = from_glib_borrow(ptr);
    // FIXME: Wrong signature in -sys bindings
    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
    let buffer_ptr = buffer_ptr as *mut *mut gst_ffi::GstBuffer;

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        match imp.create(&wrap, offset, length) {
            Ok(buffer) => {
                *buffer_ptr = buffer.into_ptr();
                gst::FlowReturn::Ok
            }
            Err(err) => gst::FlowReturn::from(err),
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_src_do_seek<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSrc,
    segment: *mut gst_ffi::GstSegment,
) -> glib_ffi::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSrc = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.do_seek(&wrap, &mut from_glib_borrow(segment))
    })
    .to_glib()
}

unsafe extern "C" fn base_src_query<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSrc,
    query_ptr: *mut gst_ffi::GstQuery,
) -> glib_ffi::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSrc = from_glib_borrow(ptr);
    let query = gst::QueryRef::from_mut_ptr(query_ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        BaseSrcImpl::query(imp, &wrap, query)
    })
    .to_glib()
}

unsafe extern "C" fn base_src_event<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSrc,
    event_ptr: *mut gst_ffi::GstEvent,
) -> glib_ffi::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSrc = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.event(&wrap, &from_glib_borrow(event_ptr))
    })
    .to_glib()
}

unsafe extern "C" fn base_src_get_caps<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSrc,
    filter: *mut gst_ffi::GstCaps,
) -> *mut gst_ffi::GstCaps
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSrc = from_glib_borrow(ptr);
    let filter = if filter.is_null() {
        None
    } else {
        Some(gst::CapsRef::from_ptr(filter))
    };

    gst_panic_to_error!(&wrap, &instance.panicked(), None, {
        imp.get_caps(&wrap, filter)
    })
    .map(|caps| caps.into_ptr())
    .unwrap_or(ptr::null_mut())
}

unsafe extern "C" fn base_src_negotiate<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSrc,
) -> glib_ffi::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSrc = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.negotiate(&wrap) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_src_set_caps<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSrc,
    caps: *mut gst_ffi::GstCaps,
) -> glib_ffi::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSrc = from_glib_borrow(ptr);
    let caps = gst::CapsRef::from_ptr(caps);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.set_caps(&wrap, caps) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_src_fixate<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSrc,
    caps: *mut gst_ffi::GstCaps,
) -> *mut gst_ffi::GstCaps
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSrc = from_glib_borrow(ptr);
    let caps = from_glib_full(caps);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::Caps::new_empty(), {
        imp.fixate(&wrap, caps)
    })
    .into_ptr()
}

unsafe extern "C" fn base_src_unlock<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSrc,
) -> glib_ffi::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSrc = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, { imp.unlock(&wrap) }).to_glib()
}

unsafe extern "C" fn base_src_unlock_stop<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSrc,
) -> glib_ffi::gboolean
where
    T: BaseSrcImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSrc = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.unlock_stop(&wrap)
    })
    .to_glib()
}
