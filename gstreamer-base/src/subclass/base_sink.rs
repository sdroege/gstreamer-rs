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

use BaseSink;
use BaseSinkClass;

pub trait BaseSinkImpl: ElementImpl + Send + Sync + 'static {
    fn start(&self, _element: &BaseSink) -> Result<(), gst::ErrorMessage> {
        Ok(())
    }

    fn stop(&self, _element: &BaseSink) -> Result<(), gst::ErrorMessage> {
        Ok(())
    }

    fn render(
        &self,
        element: &BaseSink,
        buffer: &gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn prepare(
        &self,
        _element: &BaseSink,
        _buffer: &gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        Ok(gst::FlowSuccess::Ok)
    }

    fn render_list(
        &self,
        element: &BaseSink,
        list: &gst::BufferListRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        for buffer in list.iter() {
            self.render(element, buffer)?;
        }
        Ok(gst::FlowSuccess::Ok)
    }

    fn prepare_list(
        &self,
        element: &BaseSink,
        list: &gst::BufferListRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        for buffer in list.iter() {
            self.prepare(element, buffer)?;
        }
        Ok(gst::FlowSuccess::Ok)
    }

    fn query(&self, element: &BaseSink, query: &mut gst::QueryRef) -> bool {
        BaseSinkImpl::parent_query(self, element, query)
    }

    fn event(&self, element: &BaseSink, event: gst::Event) -> bool {
        self.parent_event(element, event)
    }

    fn get_caps(&self, element: &BaseSink, filter: Option<&gst::CapsRef>) -> Option<gst::Caps> {
        self.parent_get_caps(element, filter)
    }

    fn set_caps(&self, element: &BaseSink, caps: &gst::CapsRef) -> Result<(), gst::LoggableError> {
        self.parent_set_caps(element, caps)
    }

    fn fixate(&self, element: &BaseSink, caps: gst::Caps) -> gst::Caps {
        self.parent_fixate(element, caps)
    }

    fn unlock(&self, _element: &BaseSink) -> Result<(), gst::ErrorMessage> {
        Ok(())
    }

    fn unlock_stop(&self, _element: &BaseSink) -> Result<(), gst::ErrorMessage> {
        Ok(())
    }

    fn parent_query(&self, element: &BaseSink, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .query
                .map(|f| from_glib(f(element.to_glib_none().0, query.as_mut_ptr())))
                .unwrap_or(false)
        }
    }

    fn parent_event(&self, element: &BaseSink, event: gst::Event) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .event
                .map(|f| from_glib(f(element.to_glib_none().0, event.into_ptr())))
                .unwrap_or(false)
        }
    }

    fn parent_get_caps(
        &self,
        element: &BaseSink,
        filter: Option<&gst::CapsRef>,
    ) -> Option<gst::Caps> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSinkClass;
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

    fn parent_set_caps(
        &self,
        element: &BaseSink,
        caps: &gst::CapsRef,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSinkClass;
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

    fn parent_fixate(&self, element: &BaseSink, caps: gst::Caps) -> gst::Caps {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSinkClass;

            match (*parent_class).fixate {
                Some(fixate) => from_glib_full(fixate(element.to_glib_none().0, caps.into_ptr())),
                None => caps,
            }
        }
    }
}

unsafe impl<T: ObjectSubclass + BaseSinkImpl> IsSubclassable<T> for BaseSinkClass
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(&mut self) {
        <gst::ElementClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut ffi::GstBaseSinkClass);
            klass.start = Some(base_sink_start::<T>);
            klass.stop = Some(base_sink_stop::<T>);
            klass.render = Some(base_sink_render::<T>);
            klass.render_list = Some(base_sink_render_list::<T>);
            klass.prepare = Some(base_sink_prepare::<T>);
            klass.prepare_list = Some(base_sink_prepare_list::<T>);
            klass.query = Some(base_sink_query::<T>);
            klass.event = Some(base_sink_event::<T>);
            klass.get_caps = Some(base_sink_get_caps::<T>);
            klass.set_caps = Some(base_sink_set_caps::<T>);
            klass.fixate = Some(base_sink_fixate::<T>);
            klass.unlock = Some(base_sink_unlock::<T>);
            klass.unlock_stop = Some(base_sink_unlock_stop::<T>);
        }
    }
}

unsafe extern "C" fn base_sink_start<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSink,
) -> glib_ffi::gboolean
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSink = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.start(&wrap) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(&err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_stop<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSink,
) -> glib_ffi::gboolean
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSink = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.stop(&wrap) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(&err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_render<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSink,
    buffer: *mut gst_ffi::GstBuffer,
) -> gst_ffi::GstFlowReturn
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSink = from_glib_borrow(ptr);
    let buffer = gst::BufferRef::from_ptr(buffer);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.render(&wrap, buffer).into()
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_prepare<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSink,
    buffer: *mut gst_ffi::GstBuffer,
) -> gst_ffi::GstFlowReturn
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSink = from_glib_borrow(ptr);
    let buffer = gst::BufferRef::from_ptr(buffer);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.prepare(&wrap, buffer).into()
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_render_list<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSink,
    list: *mut gst_ffi::GstBufferList,
) -> gst_ffi::GstFlowReturn
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSink = from_glib_borrow(ptr);
    let list = gst::BufferListRef::from_ptr(list);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.render_list(&wrap, list).into()
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_prepare_list<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSink,
    list: *mut gst_ffi::GstBufferList,
) -> gst_ffi::GstFlowReturn
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSink = from_glib_borrow(ptr);
    let list = gst::BufferListRef::from_ptr(list);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.prepare_list(&wrap, list).into()
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_query<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSink,
    query_ptr: *mut gst_ffi::GstQuery,
) -> glib_ffi::gboolean
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSink = from_glib_borrow(ptr);
    let query = gst::QueryRef::from_mut_ptr(query_ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        BaseSinkImpl::query(imp, &wrap, query)
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_event<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSink,
    event_ptr: *mut gst_ffi::GstEvent,
) -> glib_ffi::gboolean
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSink = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.event(&wrap, from_glib_full(event_ptr))
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_get_caps<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSink,
    filter: *mut gst_ffi::GstCaps,
) -> *mut gst_ffi::GstCaps
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSink = from_glib_borrow(ptr);
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

unsafe extern "C" fn base_sink_set_caps<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSink,
    caps: *mut gst_ffi::GstCaps,
) -> glib_ffi::gboolean
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSink = from_glib_borrow(ptr);
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

unsafe extern "C" fn base_sink_fixate<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSink,
    caps: *mut gst_ffi::GstCaps,
) -> *mut gst_ffi::GstCaps
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSink = from_glib_borrow(ptr);
    let caps = from_glib_full(caps);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::Caps::new_empty(), {
        imp.fixate(&wrap, caps)
    })
    .into_ptr()
}

unsafe extern "C" fn base_sink_unlock<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSink,
) -> glib_ffi::gboolean
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSink = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.unlock(&wrap) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(&err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_unlock_stop<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseSink,
) -> glib_ffi::gboolean
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseSink = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.unlock_stop(&wrap) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(&err);
                false
            }
        }
    })
    .to_glib()
}
