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

use std::ptr;

use BaseSink;
use BaseSinkClass;

pub trait BaseSinkImpl: BaseSinkImplExt + ElementImpl + Send + Sync + 'static {
    fn start(&self, element: &BaseSink) -> Result<(), gst::ErrorMessage> {
        self.parent_start(element)
    }

    fn stop(&self, element: &BaseSink) -> Result<(), gst::ErrorMessage> {
        self.parent_stop(element)
    }

    fn render(
        &self,
        element: &BaseSink,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_render(element, buffer)
    }

    fn prepare(
        &self,
        element: &BaseSink,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_prepare(element, buffer)
    }

    fn render_list(
        &self,
        element: &BaseSink,
        list: &gst::BufferList,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_render_list(element, list)
    }

    fn prepare_list(
        &self,
        element: &BaseSink,
        list: &gst::BufferList,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_prepare_list(element, list)
    }

    fn query(&self, element: &BaseSink, query: &mut gst::QueryRef) -> bool {
        BaseSinkImplExt::parent_query(self, element, query)
    }

    fn event(&self, element: &BaseSink, event: gst::Event) -> bool {
        self.parent_event(element, event)
    }

    fn get_caps(&self, element: &BaseSink, filter: Option<&gst::Caps>) -> Option<gst::Caps> {
        self.parent_get_caps(element, filter)
    }

    fn set_caps(&self, element: &BaseSink, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        self.parent_set_caps(element, caps)
    }

    fn fixate(&self, element: &BaseSink, caps: gst::Caps) -> gst::Caps {
        self.parent_fixate(element, caps)
    }

    fn unlock(&self, element: &BaseSink) -> Result<(), gst::ErrorMessage> {
        self.parent_unlock(element)
    }

    fn unlock_stop(&self, element: &BaseSink) -> Result<(), gst::ErrorMessage> {
        self.parent_unlock_stop(element)
    }
}

pub trait BaseSinkImplExt {
    fn parent_start(&self, element: &BaseSink) -> Result<(), gst::ErrorMessage>;

    fn parent_stop(&self, element: &BaseSink) -> Result<(), gst::ErrorMessage>;

    fn parent_render(
        &self,
        element: &BaseSink,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_prepare(
        &self,
        element: &BaseSink,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_render_list(
        &self,
        element: &BaseSink,
        list: &gst::BufferList,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_prepare_list(
        &self,
        element: &BaseSink,
        list: &gst::BufferList,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_query(&self, element: &BaseSink, query: &mut gst::QueryRef) -> bool;

    fn parent_event(&self, element: &BaseSink, event: gst::Event) -> bool;

    fn parent_get_caps(&self, element: &BaseSink, filter: Option<&gst::Caps>) -> Option<gst::Caps>;

    fn parent_set_caps(
        &self,
        element: &BaseSink,
        caps: &gst::Caps,
    ) -> Result<(), gst::LoggableError>;

    fn parent_fixate(&self, element: &BaseSink, caps: gst::Caps) -> gst::Caps;

    fn parent_unlock(&self, element: &BaseSink) -> Result<(), gst::ErrorMessage>;

    fn parent_unlock_stop(&self, element: &BaseSink) -> Result<(), gst::ErrorMessage>;
}

impl<T: BaseSinkImpl + ObjectImpl> BaseSinkImplExt for T {
    fn parent_start(&self, element: &BaseSink) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSinkClass;
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

    fn parent_stop(&self, element: &BaseSink) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSinkClass;
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

    fn parent_render(
        &self,
        element: &BaseSink,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSinkClass;
            (*parent_class)
                .render
                .map(|f| {
                    gst::FlowReturn::from_glib(f(element.to_glib_none().0, buffer.to_glib_none().0))
                })
                .unwrap_or(gst::FlowReturn::Ok)
                .into_result()
        }
    }

    fn parent_prepare(
        &self,
        element: &BaseSink,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSinkClass;
            (*parent_class)
                .prepare
                .map(|f| from_glib(f(element.to_glib_none().0, buffer.to_glib_none().0)))
                .unwrap_or(gst::FlowReturn::Ok)
                .into_result()
        }
    }

    fn parent_render_list(
        &self,
        element: &BaseSink,
        list: &gst::BufferList,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSinkClass;
            (*parent_class)
                .render_list
                .map(|f| {
                    gst::FlowReturn::from_glib(f(element.to_glib_none().0, list.to_glib_none().0))
                        .into_result()
                })
                .unwrap_or_else(|| {
                    for buffer in list.iter() {
                        self.render(element, &from_glib_borrow(buffer.as_ptr()))?;
                    }
                    Ok(gst::FlowSuccess::Ok)
                })
        }
    }

    fn parent_prepare_list(
        &self,
        element: &BaseSink,
        list: &gst::BufferList,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSinkClass;
            (*parent_class)
                .prepare_list
                .map(|f| {
                    gst::FlowReturn::from_glib(f(element.to_glib_none().0, list.to_glib_none().0))
                        .into_result()
                })
                .unwrap_or_else(|| {
                    for buffer in list.iter() {
                        self.prepare(element, &from_glib_borrow(buffer.as_ptr()))?;
                    }
                    Ok(gst::FlowSuccess::Ok)
                })
        }
    }

    fn parent_query(&self, element: &BaseSink, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSinkClass;
            (*parent_class)
                .query
                .map(|f| from_glib(f(element.to_glib_none().0, query.as_mut_ptr())))
                .unwrap_or(false)
        }
    }

    fn parent_event(&self, element: &BaseSink, event: gst::Event) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSinkClass;
            (*parent_class)
                .event
                .map(|f| from_glib(f(element.to_glib_none().0, event.into_ptr())))
                .unwrap_or(true)
        }
    }

    fn parent_get_caps(&self, element: &BaseSink, filter: Option<&gst::Caps>) -> Option<gst::Caps> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSinkClass;

            (*parent_class)
                .get_caps
                .map(|f| from_glib_full(f(element.to_glib_none().0, filter.to_glib_none().0)))
                .unwrap_or(None)
        }
    }

    fn parent_set_caps(
        &self,
        element: &BaseSink,
        caps: &gst::Caps,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSinkClass;
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

    fn parent_fixate(&self, element: &BaseSink, caps: gst::Caps) -> gst::Caps {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSinkClass;

            match (*parent_class).fixate {
                Some(fixate) => from_glib_full(fixate(element.to_glib_none().0, caps.into_ptr())),
                None => caps,
            }
        }
    }

    fn parent_unlock(&self, element: &BaseSink) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSinkClass;
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

    fn parent_unlock_stop(&self, element: &BaseSink) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseSinkClass;
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

unsafe impl<T: ObjectSubclass + BaseSinkImpl> IsSubclassable<T> for BaseSinkClass
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(&mut self) {
        <gst::ElementClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gst_base_sys::GstBaseSinkClass);
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
    ptr: *mut gst_base_sys::GstBaseSink,
) -> glib_sys::gboolean
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);

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

unsafe extern "C" fn base_sink_stop<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSink,
) -> glib_sys::gboolean
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);

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

unsafe extern "C" fn base_sink_render<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSink,
    buffer: *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);
    let buffer = from_glib_borrow(buffer);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.render(&wrap, &buffer).into()
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_prepare<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSink,
    buffer: *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);
    let buffer = from_glib_borrow(buffer);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.prepare(&wrap, &buffer).into()
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_render_list<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSink,
    list: *mut gst_sys::GstBufferList,
) -> gst_sys::GstFlowReturn
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);
    let list = from_glib_borrow(list);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.render_list(&wrap, &list).into()
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_prepare_list<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSink,
    list: *mut gst_sys::GstBufferList,
) -> gst_sys::GstFlowReturn
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);
    let list = from_glib_borrow(list);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.prepare_list(&wrap, &list).into()
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_query<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSink,
    query_ptr: *mut gst_sys::GstQuery,
) -> glib_sys::gboolean
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);
    let query = gst::QueryRef::from_mut_ptr(query_ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        BaseSinkImpl::query(imp, &wrap, query)
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_event<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSink,
    event_ptr: *mut gst_sys::GstEvent,
) -> glib_sys::gboolean
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.event(&wrap, from_glib_full(event_ptr))
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_get_caps<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSink,
    filter: *mut gst_sys::GstCaps,
) -> *mut gst_sys::GstCaps
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);
    let filter = Option::<gst::Caps>::from_glib_borrow(filter);

    gst_panic_to_error!(&wrap, &instance.panicked(), None, {
        imp.get_caps(&wrap, filter.as_ref().as_ref())
    })
    .map(|caps| caps.into_ptr())
    .unwrap_or(ptr::null_mut())
}

unsafe extern "C" fn base_sink_set_caps<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSink,
    caps: *mut gst_sys::GstCaps,
) -> glib_sys::gboolean
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);
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

unsafe extern "C" fn base_sink_fixate<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSink,
    caps: *mut gst_sys::GstCaps,
) -> *mut gst_sys::GstCaps
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);
    let caps = from_glib_full(caps);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::Caps::new_empty(), {
        imp.fixate(&wrap, caps)
    })
    .into_ptr()
}

unsafe extern "C" fn base_sink_unlock<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSink,
) -> glib_sys::gboolean
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);

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

unsafe extern "C" fn base_sink_unlock_stop<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseSink,
) -> glib_sys::gboolean
where
    T: BaseSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);

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
