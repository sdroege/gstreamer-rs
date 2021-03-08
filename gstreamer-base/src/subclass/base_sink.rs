// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use gst::subclass::prelude::*;

use std::ptr;

use crate::BaseSink;

pub trait BaseSinkImpl: BaseSinkImplExt + ElementImpl {
    fn start(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        self.parent_start(element)
    }

    fn stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        self.parent_stop(element)
    }

    fn render(
        &self,
        element: &Self::Type,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_render(element, buffer)
    }

    fn prepare(
        &self,
        element: &Self::Type,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_prepare(element, buffer)
    }

    fn render_list(
        &self,
        element: &Self::Type,
        list: &gst::BufferList,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_render_list(element, list)
    }

    fn prepare_list(
        &self,
        element: &Self::Type,
        list: &gst::BufferList,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_prepare_list(element, list)
    }

    fn query(&self, element: &Self::Type, query: &mut gst::QueryRef) -> bool {
        BaseSinkImplExt::parent_query(self, element, query)
    }

    fn event(&self, element: &Self::Type, event: gst::Event) -> bool {
        self.parent_event(element, event)
    }

    fn get_caps(&self, element: &Self::Type, filter: Option<&gst::Caps>) -> Option<gst::Caps> {
        self.parent_get_caps(element, filter)
    }

    fn set_caps(&self, element: &Self::Type, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        self.parent_set_caps(element, caps)
    }

    fn fixate(&self, element: &Self::Type, caps: gst::Caps) -> gst::Caps {
        self.parent_fixate(element, caps)
    }

    fn unlock(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        self.parent_unlock(element)
    }

    fn unlock_stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        self.parent_unlock_stop(element)
    }
}

pub trait BaseSinkImplExt: ObjectSubclass {
    fn parent_start(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;

    fn parent_stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;

    fn parent_render(
        &self,
        element: &Self::Type,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_prepare(
        &self,
        element: &Self::Type,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_render_list(
        &self,
        element: &Self::Type,
        list: &gst::BufferList,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_prepare_list(
        &self,
        element: &Self::Type,
        list: &gst::BufferList,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_query(&self, element: &Self::Type, query: &mut gst::QueryRef) -> bool;

    fn parent_event(&self, element: &Self::Type, event: gst::Event) -> bool;

    fn parent_get_caps(
        &self,
        element: &Self::Type,
        filter: Option<&gst::Caps>,
    ) -> Option<gst::Caps>;

    fn parent_set_caps(
        &self,
        element: &Self::Type,
        caps: &gst::Caps,
    ) -> Result<(), gst::LoggableError>;

    fn parent_fixate(&self, element: &Self::Type, caps: gst::Caps) -> gst::Caps;

    fn parent_unlock(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;

    fn parent_unlock_stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;
}

impl<T: BaseSinkImpl> BaseSinkImplExt for T {
    fn parent_start(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .start
                .map(|f| {
                    if from_glib(f(element.unsafe_cast_ref::<BaseSink>().to_glib_none().0)) {
                        Ok(())
                    } else {
                        Err(gst::error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `start` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .stop
                .map(|f| {
                    if from_glib(f(element.unsafe_cast_ref::<BaseSink>().to_glib_none().0)) {
                        Ok(())
                    } else {
                        Err(gst::error_msg!(
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
        element: &Self::Type,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .render
                .map(|f| {
                    gst::FlowReturn::from_glib(f(
                        element.unsafe_cast_ref::<BaseSink>().to_glib_none().0,
                        buffer.to_glib_none().0,
                    ))
                })
                .unwrap_or(gst::FlowReturn::Ok)
                .into_result()
        }
    }

    fn parent_prepare(
        &self,
        element: &Self::Type,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .prepare
                .map(|f| {
                    from_glib(f(
                        element.unsafe_cast_ref::<BaseSink>().to_glib_none().0,
                        buffer.to_glib_none().0,
                    ))
                })
                .unwrap_or(gst::FlowReturn::Ok)
                .into_result()
        }
    }

    fn parent_render_list(
        &self,
        element: &Self::Type,
        list: &gst::BufferList,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .render_list
                .map(|f| {
                    gst::FlowReturn::from_glib(f(
                        element.unsafe_cast_ref::<BaseSink>().to_glib_none().0,
                        list.to_glib_none().0,
                    ))
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
        element: &Self::Type,
        list: &gst::BufferList,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .prepare_list
                .map(|f| {
                    gst::FlowReturn::from_glib(f(
                        element.unsafe_cast_ref::<BaseSink>().to_glib_none().0,
                        list.to_glib_none().0,
                    ))
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

    fn parent_query(&self, element: &Self::Type, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .query
                .map(|f| {
                    from_glib(f(
                        element.unsafe_cast_ref::<BaseSink>().to_glib_none().0,
                        query.as_mut_ptr(),
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_event(&self, element: &Self::Type, event: gst::Event) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .event
                .map(|f| {
                    from_glib(f(
                        element.unsafe_cast_ref::<BaseSink>().to_glib_none().0,
                        event.into_ptr(),
                    ))
                })
                .unwrap_or(true)
        }
    }

    fn parent_get_caps(
        &self,
        element: &Self::Type,
        filter: Option<&gst::Caps>,
    ) -> Option<gst::Caps> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSinkClass;

            (*parent_class)
                .get_caps
                .map(|f| {
                    from_glib_full(f(
                        element.unsafe_cast_ref::<BaseSink>().to_glib_none().0,
                        filter.to_glib_none().0,
                    ))
                })
                .unwrap_or(None)
        }
    }

    fn parent_set_caps(
        &self,
        element: &Self::Type,
        caps: &gst::Caps,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .set_caps
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            element.unsafe_cast_ref::<BaseSink>().to_glib_none().0,
                            caps.to_glib_none().0
                        ),
                        gst::CAT_RUST,
                        "Parent function `set_caps` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_fixate(&self, element: &Self::Type, caps: gst::Caps) -> gst::Caps {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSinkClass;

            match (*parent_class).fixate {
                Some(fixate) => from_glib_full(fixate(
                    element.unsafe_cast_ref::<BaseSink>().to_glib_none().0,
                    caps.into_ptr(),
                )),
                None => caps,
            }
        }
    }

    fn parent_unlock(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .unlock
                .map(|f| {
                    if from_glib(f(element.unsafe_cast_ref::<BaseSink>().to_glib_none().0)) {
                        Ok(())
                    } else {
                        Err(gst::error_msg!(
                            gst::CoreError::Failed,
                            ["Parent function `unlock` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_unlock_stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .unlock_stop
                .map(|f| {
                    if from_glib(f(element.unsafe_cast_ref::<BaseSink>().to_glib_none().0)) {
                        Ok(())
                    } else {
                        Err(gst::error_msg!(
                            gst::CoreError::Failed,
                            ["Parent function `unlock_stop` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }
}

unsafe impl<T: BaseSinkImpl> IsSubclassable<T> for BaseSink
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn class_init(klass: &mut glib::Class<Self>) {
        <gst::Element as IsSubclassable<T>>::class_init(klass);
        let klass = klass.as_mut();
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

unsafe extern "C" fn base_sink_start<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
) -> glib::ffi::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.start(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_stop<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
) -> glib::ffi::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.stop(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_render<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
    buffer: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);
    let buffer = from_glib_borrow(buffer);

    gst::panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.render(wrap.unsafe_cast_ref(), &buffer).into()
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_prepare<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
    buffer: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);
    let buffer = from_glib_borrow(buffer);

    gst::panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.prepare(wrap.unsafe_cast_ref(), &buffer).into()
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_render_list<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
    list: *mut gst::ffi::GstBufferList,
) -> gst::ffi::GstFlowReturn
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);
    let list = from_glib_borrow(list);

    gst::panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.render_list(wrap.unsafe_cast_ref(), &list).into()
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_prepare_list<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
    list: *mut gst::ffi::GstBufferList,
) -> gst::ffi::GstFlowReturn
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);
    let list = from_glib_borrow(list);

    gst::panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.prepare_list(wrap.unsafe_cast_ref(), &list).into()
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_query<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
    query_ptr: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);
    let query = gst::QueryRef::from_mut_ptr(query_ptr);

    gst::panic_to_error!(&wrap, &instance.panicked(), false, {
        BaseSinkImpl::query(imp, wrap.unsafe_cast_ref(), query)
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_event<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
    event_ptr: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.event(wrap.unsafe_cast_ref(), from_glib_full(event_ptr))
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_get_caps<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
    filter: *mut gst::ffi::GstCaps,
) -> *mut gst::ffi::GstCaps
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);
    let filter = Option::<gst::Caps>::from_glib_borrow(filter);

    gst::panic_to_error!(&wrap, &instance.panicked(), None, {
        imp.get_caps(wrap.unsafe_cast_ref(), filter.as_ref().as_ref())
    })
    .map(|caps| caps.into_ptr())
    .unwrap_or(ptr::null_mut())
}

unsafe extern "C" fn base_sink_set_caps<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
    caps: *mut gst::ffi::GstCaps,
) -> glib::ffi::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);
    let caps = from_glib_borrow(caps);

    gst::panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.set_caps(wrap.unsafe_cast_ref(), &caps) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_fixate<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
    caps: *mut gst::ffi::GstCaps,
) -> *mut gst::ffi::GstCaps
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);
    let caps = from_glib_full(caps);

    gst::panic_to_error!(&wrap, &instance.panicked(), gst::Caps::new_empty(), {
        imp.fixate(wrap.unsafe_cast_ref(), caps)
    })
    .into_ptr()
}

unsafe extern "C" fn base_sink_unlock<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
) -> glib::ffi::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.unlock(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_sink_unlock_stop<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
) -> glib::ffi::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseSink> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.unlock_stop(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}
