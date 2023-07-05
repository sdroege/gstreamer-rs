// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use glib::{prelude::*, translate::*};
use gst::subclass::prelude::*;

use crate::BaseSink;

pub trait BaseSinkImpl: BaseSinkImplExt + ElementImpl {
    fn start(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_start()
    }

    fn stop(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_stop()
    }

    fn render(&self, buffer: &gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_render(buffer)
    }

    fn prepare(&self, buffer: &gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_prepare(buffer)
    }

    fn render_list(&self, list: &gst::BufferList) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_render_list(list)
    }

    fn prepare_list(&self, list: &gst::BufferList) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_prepare_list(list)
    }

    fn query(&self, query: &mut gst::QueryRef) -> bool {
        BaseSinkImplExt::parent_query(self, query)
    }

    fn event(&self, event: gst::Event) -> bool {
        self.parent_event(event)
    }

    fn caps(&self, filter: Option<&gst::Caps>) -> Option<gst::Caps> {
        self.parent_caps(filter)
    }

    fn set_caps(&self, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        self.parent_set_caps(caps)
    }

    fn fixate(&self, caps: gst::Caps) -> gst::Caps {
        self.parent_fixate(caps)
    }

    fn unlock(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_unlock()
    }

    fn unlock_stop(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_unlock_stop()
    }

    fn propose_allocation(
        &self,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError> {
        self.parent_propose_allocation(query)
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::BaseSinkImplExt> Sealed for T {}
}

pub trait BaseSinkImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_start(&self) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .start
                .map(|f| {
                    if from_glib(f(self.obj().unsafe_cast_ref::<BaseSink>().to_glib_none().0)) {
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

    fn parent_stop(&self) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .stop
                .map(|f| {
                    if from_glib(f(self.obj().unsafe_cast_ref::<BaseSink>().to_glib_none().0)) {
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

    fn parent_render(&self, buffer: &gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .render
                .map(|f| {
                    try_from_glib(f(
                        self.obj().unsafe_cast_ref::<BaseSink>().to_glib_none().0,
                        buffer.to_glib_none().0,
                    ))
                })
                .unwrap_or(Ok(gst::FlowSuccess::Ok))
        }
    }

    fn parent_prepare(&self, buffer: &gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .prepare
                .map(|f| {
                    try_from_glib(f(
                        self.obj().unsafe_cast_ref::<BaseSink>().to_glib_none().0,
                        buffer.to_glib_none().0,
                    ))
                })
                .unwrap_or(Ok(gst::FlowSuccess::Ok))
        }
    }

    fn parent_render_list(
        &self,
        list: &gst::BufferList,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_prepare_list(
        &self,
        list: &gst::BufferList,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_query(&self, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .query
                .map(|f| {
                    from_glib(f(
                        self.obj().unsafe_cast_ref::<BaseSink>().to_glib_none().0,
                        query.as_mut_ptr(),
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_event(&self, event: gst::Event) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .event
                .map(|f| {
                    from_glib(f(
                        self.obj().unsafe_cast_ref::<BaseSink>().to_glib_none().0,
                        event.into_glib_ptr(),
                    ))
                })
                .unwrap_or(true)
        }
    }

    fn parent_caps(&self, filter: Option<&gst::Caps>) -> Option<gst::Caps> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSinkClass;

            (*parent_class)
                .get_caps
                .map(|f| {
                    from_glib_full(f(
                        self.obj().unsafe_cast_ref::<BaseSink>().to_glib_none().0,
                        filter.to_glib_none().0,
                    ))
                })
                .unwrap_or(None)
        }
    }

    fn parent_set_caps(&self, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .set_caps
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            self.obj().unsafe_cast_ref::<BaseSink>().to_glib_none().0,
                            caps.to_glib_none().0
                        ),
                        gst::CAT_RUST,
                        "Parent function `set_caps` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_fixate(&self, caps: gst::Caps) -> gst::Caps {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSinkClass;

            match (*parent_class).fixate {
                Some(fixate) => from_glib_full(fixate(
                    self.obj().unsafe_cast_ref::<BaseSink>().to_glib_none().0,
                    caps.into_glib_ptr(),
                )),
                None => caps,
            }
        }
    }

    fn parent_unlock(&self) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .unlock
                .map(|f| {
                    if from_glib(f(self.obj().unsafe_cast_ref::<BaseSink>().to_glib_none().0)) {
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

    fn parent_unlock_stop(&self) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .unlock_stop
                .map(|f| {
                    if from_glib(f(self.obj().unsafe_cast_ref::<BaseSink>().to_glib_none().0)) {
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

    fn parent_propose_allocation(
        &self,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .propose_allocation
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            self.obj().unsafe_cast_ref::<BaseSink>().to_glib_none().0,
                            query.as_mut_ptr(),
                        ),
                        gst::CAT_RUST,
                        "Parent function `propose_allocation` failed",
                    )
                })
                .unwrap_or(Ok(()))
        }
    }
}

impl<T: BaseSinkImpl> BaseSinkImplExt for T {
    fn parent_render_list(
        &self,
        list: &gst::BufferList,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .render_list
                .map(|f| {
                    try_from_glib(f(
                        self.obj().unsafe_cast_ref::<BaseSink>().to_glib_none().0,
                        list.to_glib_none().0,
                    ))
                })
                .unwrap_or_else(|| {
                    for buffer in list.iter() {
                        self.render(&from_glib_borrow(buffer.as_ptr()))?;
                    }
                    Ok(gst::FlowSuccess::Ok)
                })
        }
    }

    fn parent_prepare_list(
        &self,
        list: &gst::BufferList,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSinkClass;
            (*parent_class)
                .prepare_list
                .map(|f| {
                    try_from_glib(f(
                        self.obj().unsafe_cast_ref::<BaseSink>().to_glib_none().0,
                        list.to_glib_none().0,
                    ))
                })
                .unwrap_or_else(|| {
                    for buffer in list.iter() {
                        self.prepare(&from_glib_borrow(buffer.as_ptr()))?;
                    }
                    Ok(gst::FlowSuccess::Ok)
                })
        }
    }
}

unsafe impl<T: BaseSinkImpl> IsSubclassable<T> for BaseSink {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
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
        klass.propose_allocation = Some(base_sink_propose_allocation::<T>);
    }
}

unsafe extern "C" fn base_sink_start<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.start() {
            Ok(()) => true,
            Err(err) => {
                imp.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_sink_stop<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.stop() {
            Ok(()) => true,
            Err(err) => {
                imp.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_sink_render<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
    buffer: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let buffer = from_glib_borrow(buffer);

    gst::panic_to_error!(imp, gst::FlowReturn::Error, { imp.render(&buffer).into() }).into_glib()
}

unsafe extern "C" fn base_sink_prepare<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
    buffer: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let buffer = from_glib_borrow(buffer);

    gst::panic_to_error!(imp, gst::FlowReturn::Error, { imp.prepare(&buffer).into() }).into_glib()
}

unsafe extern "C" fn base_sink_render_list<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
    list: *mut gst::ffi::GstBufferList,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let list = from_glib_borrow(list);

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        imp.render_list(&list).into()
    })
    .into_glib()
}

unsafe extern "C" fn base_sink_prepare_list<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
    list: *mut gst::ffi::GstBufferList,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let list = from_glib_borrow(list);

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        imp.prepare_list(&list).into()
    })
    .into_glib()
}

unsafe extern "C" fn base_sink_query<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
    query_ptr: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let query = gst::QueryRef::from_mut_ptr(query_ptr);

    gst::panic_to_error!(imp, false, { BaseSinkImpl::query(imp, query) }).into_glib()
}

unsafe extern "C" fn base_sink_event<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
    event_ptr: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, { imp.event(from_glib_full(event_ptr)) }).into_glib()
}

unsafe extern "C" fn base_sink_get_caps<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
    filter: *mut gst::ffi::GstCaps,
) -> *mut gst::ffi::GstCaps {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let filter = Option::<gst::Caps>::from_glib_borrow(filter);

    gst::panic_to_error!(imp, None, { imp.caps(filter.as_ref().as_ref()) })
        .map(|caps| caps.into_glib_ptr())
        .unwrap_or(ptr::null_mut())
}

unsafe extern "C" fn base_sink_set_caps<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
    caps: *mut gst::ffi::GstCaps,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let caps = from_glib_borrow(caps);

    gst::panic_to_error!(imp, false, {
        match imp.set_caps(&caps) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_sink_fixate<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
    caps: *mut gst::ffi::GstCaps,
) -> *mut gst::ffi::GstCaps {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let caps = from_glib_full(caps);

    gst::panic_to_error!(imp, gst::Caps::new_empty(), { imp.fixate(caps) }).into_glib_ptr()
}

unsafe extern "C" fn base_sink_unlock<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.unlock() {
            Ok(()) => true,
            Err(err) => {
                imp.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_sink_unlock_stop<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.unlock_stop() {
            Ok(()) => true,
            Err(err) => {
                imp.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_sink_propose_allocation<T: BaseSinkImpl>(
    ptr: *mut ffi::GstBaseSink,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let query = match gst::QueryRef::from_mut_ptr(query).view_mut() {
        gst::QueryViewMut::Allocation(allocation) => allocation,
        _ => unreachable!(),
    };

    gst::panic_to_error!(imp, false, {
        match imp.propose_allocation(query) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}
