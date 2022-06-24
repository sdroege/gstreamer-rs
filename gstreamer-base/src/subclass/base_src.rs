// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::translate::*;

use gst::prelude::*;
use gst::subclass::prelude::*;

use std::mem;
use std::ptr;

use atomic_refcell::AtomicRefCell;

use crate::prelude::BaseSrcExtManual;
use crate::BaseSrc;

#[derive(Default)]
pub(super) struct InstanceData {
    pub(super) pending_buffer_list: AtomicRefCell<Option<gst::BufferList>>,
}

#[derive(Debug)]
pub enum CreateSuccess {
    FilledBuffer,
    NewBuffer(gst::Buffer),
    NewBufferList(gst::BufferList),
}

pub trait BaseSrcImpl: BaseSrcImplExt + ElementImpl {
    fn start(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        self.parent_start(element)
    }

    fn stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        self.parent_stop(element)
    }

    fn is_seekable(&self, element: &Self::Type) -> bool {
        self.parent_is_seekable(element)
    }

    fn size(&self, element: &Self::Type) -> Option<u64> {
        self.parent_size(element)
    }

    #[doc(alias = "get_times")]
    fn times(
        &self,
        element: &Self::Type,
        buffer: &gst::BufferRef,
    ) -> (Option<gst::ClockTime>, Option<gst::ClockTime>) {
        self.parent_times(element, buffer)
    }

    fn fill(
        &self,
        element: &Self::Type,
        offset: u64,
        length: u32,
        buffer: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_fill(element, offset, length, buffer)
    }

    fn alloc(
        &self,
        element: &Self::Type,
        offset: u64,
        length: u32,
    ) -> Result<gst::Buffer, gst::FlowError> {
        self.parent_alloc(element, offset, length)
    }

    fn create(
        &self,
        element: &Self::Type,
        offset: u64,
        buffer: Option<&mut gst::BufferRef>,
        length: u32,
    ) -> Result<CreateSuccess, gst::FlowError> {
        self.parent_create(element, offset, buffer, length)
    }

    fn do_seek(&self, element: &Self::Type, segment: &mut gst::Segment) -> bool {
        self.parent_do_seek(element, segment)
    }

    fn query(&self, element: &Self::Type, query: &mut gst::QueryRef) -> bool {
        BaseSrcImplExt::parent_query(self, element, query)
    }

    fn event(&self, element: &Self::Type, event: &gst::Event) -> bool {
        self.parent_event(element, event)
    }

    fn caps(&self, element: &Self::Type, filter: Option<&gst::Caps>) -> Option<gst::Caps> {
        self.parent_caps(element, filter)
    }

    fn negotiate(&self, element: &Self::Type) -> Result<(), gst::LoggableError> {
        self.parent_negotiate(element)
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

    fn decide_allocation(
        &self,
        element: &Self::Type,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError> {
        self.parent_decide_allocation(element, query)
    }
}

pub trait BaseSrcImplExt: ObjectSubclass {
    fn parent_start(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;

    fn parent_stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;

    fn parent_is_seekable(&self, element: &Self::Type) -> bool;

    fn parent_size(&self, element: &Self::Type) -> Option<u64>;

    fn parent_times(
        &self,
        element: &Self::Type,
        buffer: &gst::BufferRef,
    ) -> (Option<gst::ClockTime>, Option<gst::ClockTime>);

    fn parent_fill(
        &self,
        element: &Self::Type,
        offset: u64,
        length: u32,
        buffer: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_alloc(
        &self,
        element: &Self::Type,
        offset: u64,
        length: u32,
    ) -> Result<gst::Buffer, gst::FlowError>;

    fn parent_create(
        &self,
        element: &Self::Type,
        offset: u64,
        buffer: Option<&mut gst::BufferRef>,
        length: u32,
    ) -> Result<CreateSuccess, gst::FlowError>;

    fn parent_do_seek(&self, element: &Self::Type, segment: &mut gst::Segment) -> bool;

    fn parent_query(&self, element: &Self::Type, query: &mut gst::QueryRef) -> bool;

    fn parent_event(&self, element: &Self::Type, event: &gst::Event) -> bool;

    fn parent_caps(&self, element: &Self::Type, filter: Option<&gst::Caps>) -> Option<gst::Caps>;

    fn parent_negotiate(&self, element: &Self::Type) -> Result<(), gst::LoggableError>;

    fn parent_set_caps(
        &self,
        element: &Self::Type,
        caps: &gst::Caps,
    ) -> Result<(), gst::LoggableError>;

    fn parent_fixate(&self, element: &Self::Type, caps: gst::Caps) -> gst::Caps;

    fn parent_unlock(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;

    fn parent_unlock_stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;

    fn parent_decide_allocation(
        &self,
        element: &Self::Type,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError>;
}

impl<T: BaseSrcImpl> BaseSrcImplExt for T {
    fn parent_start(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSrcClass;
            (*parent_class)
                .start
                .map(|f| {
                    if from_glib(f(element.unsafe_cast_ref::<BaseSrc>().to_glib_none().0)) {
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
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSrcClass;
            (*parent_class)
                .stop
                .map(|f| {
                    if from_glib(f(element.unsafe_cast_ref::<BaseSrc>().to_glib_none().0)) {
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

    fn parent_is_seekable(&self, element: &Self::Type) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSrcClass;
            (*parent_class)
                .is_seekable
                .map(|f| from_glib(f(element.unsafe_cast_ref::<BaseSrc>().to_glib_none().0)))
                .unwrap_or(false)
        }
    }

    fn parent_size(&self, element: &Self::Type) -> Option<u64> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSrcClass;
            (*parent_class)
                .get_size
                .map(|f| {
                    let mut size = mem::MaybeUninit::uninit();
                    if from_glib(f(
                        element.unsafe_cast_ref::<BaseSrc>().to_glib_none().0,
                        size.as_mut_ptr(),
                    )) {
                        Some(size.assume_init())
                    } else {
                        None
                    }
                })
                .unwrap_or(None)
        }
    }

    fn parent_times(
        &self,
        element: &Self::Type,
        buffer: &gst::BufferRef,
    ) -> (Option<gst::ClockTime>, Option<gst::ClockTime>) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSrcClass;
            (*parent_class)
                .get_times
                .map(|f| {
                    let mut start = mem::MaybeUninit::uninit();
                    let mut stop = mem::MaybeUninit::uninit();
                    f(
                        element.unsafe_cast_ref::<BaseSrc>().to_glib_none().0,
                        buffer.as_mut_ptr(),
                        start.as_mut_ptr(),
                        stop.as_mut_ptr(),
                    );
                    (
                        from_glib(start.assume_init()),
                        from_glib(stop.assume_init()),
                    )
                })
                .unwrap_or((gst::ClockTime::NONE, gst::ClockTime::NONE))
        }
    }

    fn parent_fill(
        &self,
        element: &Self::Type,
        offset: u64,
        length: u32,
        buffer: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSrcClass;
            (*parent_class)
                .fill
                .map(|f| {
                    try_from_glib(f(
                        element.unsafe_cast_ref::<BaseSrc>().to_glib_none().0,
                        offset,
                        length,
                        buffer.as_mut_ptr(),
                    ))
                })
                .unwrap_or(Err(gst::FlowError::NotSupported))
        }
    }

    fn parent_alloc(
        &self,
        element: &Self::Type,
        offset: u64,
        length: u32,
    ) -> Result<gst::Buffer, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSrcClass;
            (*parent_class)
                .alloc
                .map(|f| {
                    let mut buffer_ptr: *mut gst::ffi::GstBuffer = ptr::null_mut();

                    // FIXME: Wrong signature in -sys bindings
                    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
                    let buffer_ref = &mut buffer_ptr as *mut _ as *mut gst::ffi::GstBuffer;

                    gst::FlowSuccess::try_from_glib(f(
                        element.unsafe_cast_ref::<BaseSrc>().to_glib_none().0,
                        offset,
                        length,
                        buffer_ref,
                    ))
                    .map(|_| from_glib_full(buffer_ptr))
                })
                .unwrap_or(Err(gst::FlowError::NotSupported))
        }
    }

    fn parent_create(
        &self,
        element: &Self::Type,
        offset: u64,
        mut buffer: Option<&mut gst::BufferRef>,
        length: u32,
    ) -> Result<CreateSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSrcClass;
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

                    let instance_data = self.instance_data::<InstanceData>(BaseSrc::static_type()).unwrap();

                    if let Err(err) = gst::FlowSuccess::try_from_glib(
                        f(
                            element.unsafe_cast_ref::<BaseSrc>().to_glib_none().0,
                            offset,
                            length,
                            buffer_ref,
                        )
                    ) {
                        *instance_data.pending_buffer_list.borrow_mut() = None;
                        return Err(err);
                    }

                    let pending_buffer_list = instance_data.pending_buffer_list.borrow_mut().take();
                    if pending_buffer_list.is_some() &&
                        (buffer.is_some() || element.unsafe_cast_ref::<BaseSrc>().src_pad().mode() == gst::PadMode::Pull) {
                        panic!("Buffer lists can only be returned in push mode");
                    }

                    if buffer_ptr.is_null() && pending_buffer_list.is_none() {
                        gst::error!(
                            gst::CAT_RUST,
                            obj: element.unsafe_cast_ref::<BaseSrc>(),
                            "No buffer and no buffer list returned"
                        );
                        return Err(gst::FlowError::Error);
                    }

                    if !buffer_ptr.is_null() && pending_buffer_list.is_some() {
                        gst::error!(
                            gst::CAT_RUST,
                            obj: element.unsafe_cast_ref::<BaseSrc>(),
                            "Both buffer and buffer list returned"
                        );
                        return Err(gst::FlowError::Error);
                    }

                    if let Some(passed_buffer) = buffer {
                        if buffer_ptr != orig_buffer_ptr {
                            let new_buffer = gst::BufferRef::from_ptr(buffer_ptr);

                            gst::debug!(
                                gst::CAT_PERFORMANCE,
                                obj: element.unsafe_cast_ref::<BaseSrc>(),
                                "Returned new buffer from parent create function, copying into passed buffer"
                            );

                            let mut map = match passed_buffer.map_writable() {
                                Ok(map) => map,
                                Err(_) => {
                                    gst::error!(
                                        gst::CAT_RUST,
                                        obj: element.unsafe_cast_ref::<BaseSrc>(),
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
                                        obj: element.unsafe_cast_ref::<BaseSrc>(),
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

    fn parent_do_seek(&self, element: &Self::Type, segment: &mut gst::Segment) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSrcClass;
            (*parent_class)
                .do_seek
                .map(|f| {
                    from_glib(f(
                        element.unsafe_cast_ref::<BaseSrc>().to_glib_none().0,
                        segment.to_glib_none_mut().0,
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_query(&self, element: &Self::Type, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSrcClass;
            (*parent_class)
                .query
                .map(|f| {
                    from_glib(f(
                        element.unsafe_cast_ref::<BaseSrc>().to_glib_none().0,
                        query.as_mut_ptr(),
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_event(&self, element: &Self::Type, event: &gst::Event) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSrcClass;
            (*parent_class)
                .event
                .map(|f| {
                    from_glib(f(
                        element.unsafe_cast_ref::<BaseSrc>().to_glib_none().0,
                        event.to_glib_none().0,
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_caps(&self, element: &Self::Type, filter: Option<&gst::Caps>) -> Option<gst::Caps> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSrcClass;

            (*parent_class)
                .get_caps
                .map(|f| {
                    from_glib_full(f(
                        element.unsafe_cast_ref::<BaseSrc>().to_glib_none().0,
                        filter.to_glib_none().0,
                    ))
                })
                .unwrap_or(None)
        }
    }

    fn parent_negotiate(&self, element: &Self::Type) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSrcClass;
            (*parent_class)
                .negotiate
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(element.unsafe_cast_ref::<BaseSrc>().to_glib_none().0),
                        gst::CAT_RUST,
                        "Parent function `negotiate` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_set_caps(
        &self,
        element: &Self::Type,
        caps: &gst::Caps,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSrcClass;
            (*parent_class)
                .set_caps
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            element.unsafe_cast_ref::<BaseSrc>().to_glib_none().0,
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
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSrcClass;

            match (*parent_class).fixate {
                Some(fixate) => from_glib_full(fixate(
                    element.unsafe_cast_ref::<BaseSrc>().to_glib_none().0,
                    caps.into_glib_ptr(),
                )),
                None => caps,
            }
        }
    }

    fn parent_unlock(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSrcClass;
            (*parent_class)
                .unlock
                .map(|f| {
                    if from_glib(f(element.unsafe_cast_ref::<BaseSrc>().to_glib_none().0)) {
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
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSrcClass;
            (*parent_class)
                .unlock_stop
                .map(|f| {
                    if from_glib(f(element.unsafe_cast_ref::<BaseSrc>().to_glib_none().0)) {
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

    fn parent_decide_allocation(
        &self,
        element: &Self::Type,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseSrcClass;
            (*parent_class)
                .decide_allocation
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            element.unsafe_cast_ref::<BaseSrc>().to_glib_none().0,
                            query.as_mut_ptr(),
                        ),
                        gst::CAT_RUST,
                        "Parent function `decide_allocation` failed",
                    )
                })
                .unwrap_or(Ok(()))
        }
    }
}

unsafe impl<T: BaseSrcImpl> IsSubclassable<T> for BaseSrc {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
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
        klass.decide_allocation = Some(base_src_decide_allocation::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        Self::parent_instance_init(instance);

        instance.set_instance_data(BaseSrc::static_type(), InstanceData::default());
    }
}

unsafe extern "C" fn base_src_start<T: BaseSrcImpl>(
    ptr: *mut ffi::GstBaseSrc,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        match imp.start(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_src_stop<T: BaseSrcImpl>(
    ptr: *mut ffi::GstBaseSrc,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        match imp.stop(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_src_is_seekable<T: BaseSrcImpl>(
    ptr: *mut ffi::GstBaseSrc,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        imp.is_seekable(wrap.unsafe_cast_ref())
    })
    .into_glib()
}

unsafe extern "C" fn base_src_get_size<T: BaseSrcImpl>(
    ptr: *mut ffi::GstBaseSrc,
    size: *mut u64,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        match imp.size(wrap.unsafe_cast_ref()) {
            Some(s) => {
                *size = s;
                true
            }
            None => false,
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_src_get_times<T: BaseSrcImpl>(
    ptr: *mut ffi::GstBaseSrc,
    buffer: *mut gst::ffi::GstBuffer,
    start: *mut gst::ffi::GstClockTime,
    stop: *mut gst::ffi::GstClockTime,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);
    let buffer = gst::BufferRef::from_ptr(buffer);

    *start = gst::ffi::GST_CLOCK_TIME_NONE;
    *stop = gst::ffi::GST_CLOCK_TIME_NONE;

    gst::panic_to_error!(&wrap, imp.panicked(), (), {
        let (start_, stop_) = imp.times(wrap.unsafe_cast_ref(), buffer);
        *start = start_.into_glib();
        *stop = stop_.into_glib();
    });
}

unsafe extern "C" fn base_src_fill<T: BaseSrcImpl>(
    ptr: *mut ffi::GstBaseSrc,
    offset: u64,
    length: u32,
    buffer: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);
    let buffer = gst::BufferRef::from_mut_ptr(buffer);

    gst::panic_to_error!(&wrap, imp.panicked(), gst::FlowReturn::Error, {
        imp.fill(wrap.unsafe_cast_ref(), offset, length, buffer)
            .into()
    })
    .into_glib()
}

unsafe extern "C" fn base_src_alloc<T: BaseSrcImpl>(
    ptr: *mut ffi::GstBaseSrc,
    offset: u64,
    length: u32,
    buffer_ptr: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);
    // FIXME: Wrong signature in -sys bindings
    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
    let buffer_ptr = buffer_ptr as *mut *mut gst::ffi::GstBuffer;

    gst::panic_to_error!(&wrap, imp.panicked(), gst::FlowReturn::Error, {
        match imp.alloc(wrap.unsafe_cast_ref(), offset, length) {
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
unsafe extern "C" fn base_src_create<T: BaseSrcImpl>(
    ptr: *mut ffi::GstBaseSrc,
    offset: u64,
    length: u32,
    buffer_ptr: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);
    // FIXME: Wrong signature in -sys bindings
    // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs-sys/issues/3
    let buffer_ptr = buffer_ptr as *mut *mut gst::ffi::GstBuffer;

    let mut buffer = if (*buffer_ptr).is_null() {
        None
    } else {
        Some(gst::BufferRef::from_mut_ptr(*buffer_ptr))
    };

    let instance_data = imp
        .instance_data::<InstanceData>(BaseSrc::static_type())
        .unwrap();

    // If there is a pending buffer list at this point then unset it.
    if wrap.type_() == T::Type::static_type() {
        *instance_data.pending_buffer_list.borrow_mut() = None;
    }

    let res = gst::panic_to_error!(&wrap, imp.panicked(), gst::FlowReturn::Error, {
        match imp.create(
            wrap.unsafe_cast_ref(),
            offset,
            buffer.as_deref_mut(),
            length,
        ) {
            Ok(CreateSuccess::NewBuffer(new_buffer)) => {
                if let Some(passed_buffer) = buffer {
                    if passed_buffer.as_ptr() != new_buffer.as_ptr() {
                        gst::debug!(
                            gst::CAT_PERFORMANCE,
                            obj: &*wrap,
                            "Returned new buffer from create function, copying into passed buffer"
                        );

                        let mut map = match passed_buffer.map_writable() {
                            Ok(map) => map,
                            Err(_) => {
                                gst::error!(
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
                                gst::error!(
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
                    *buffer_ptr = new_buffer.into_glib_ptr();
                    gst::FlowReturn::Ok
                }
            }
            Ok(CreateSuccess::NewBufferList(new_buffer_list)) => {
                if buffer.is_some()
                    || wrap.unsafe_cast_ref::<BaseSrc>().src_pad().mode() == gst::PadMode::Pull
                {
                    panic!("Buffer lists can only be returned in push mode");
                }

                *buffer_ptr = ptr::null_mut();

                // If this is the final type then submit the buffer list. This can only be done
                // once so can only really be done here.
                // FIXME: This won't work if a non-Rust subclass of a Rust subclass is created.
                if wrap.type_() == T::Type::static_type() {
                    ffi::gst_base_src_submit_buffer_list(
                        wrap.to_glib_none().0,
                        new_buffer_list.into_glib_ptr(),
                    );
                } else {
                    *instance_data.pending_buffer_list.borrow_mut() = Some(new_buffer_list);
                }

                gst::FlowReturn::Ok
            }
            Ok(CreateSuccess::FilledBuffer) => gst::FlowReturn::Ok,
            Err(err) => gst::FlowReturn::from(err),
        }
    })
    .into_glib();

    // If there is a pending buffer list at this point then unset it.
    if wrap.type_() == T::Type::static_type() {
        *instance_data.pending_buffer_list.borrow_mut() = None;
    }

    res
}

unsafe extern "C" fn base_src_do_seek<T: BaseSrcImpl>(
    ptr: *mut ffi::GstBaseSrc,
    segment: *mut gst::ffi::GstSegment,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        let mut s = from_glib_none(segment);
        let res = imp.do_seek(wrap.unsafe_cast_ref(), &mut s);
        ptr::write(segment, *(s.to_glib_none().0));

        res
    })
    .into_glib()
}

unsafe extern "C" fn base_src_query<T: BaseSrcImpl>(
    ptr: *mut ffi::GstBaseSrc,
    query_ptr: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);
    let query = gst::QueryRef::from_mut_ptr(query_ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        BaseSrcImpl::query(imp, wrap.unsafe_cast_ref(), query)
    })
    .into_glib()
}

unsafe extern "C" fn base_src_event<T: BaseSrcImpl>(
    ptr: *mut ffi::GstBaseSrc,
    event_ptr: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        imp.event(wrap.unsafe_cast_ref(), &from_glib_borrow(event_ptr))
    })
    .into_glib()
}

unsafe extern "C" fn base_src_get_caps<T: BaseSrcImpl>(
    ptr: *mut ffi::GstBaseSrc,
    filter: *mut gst::ffi::GstCaps,
) -> *mut gst::ffi::GstCaps {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);
    let filter = Option::<gst::Caps>::from_glib_borrow(filter);

    gst::panic_to_error!(&wrap, imp.panicked(), None, {
        imp.caps(wrap.unsafe_cast_ref(), filter.as_ref().as_ref())
    })
    .map(|caps| caps.into_glib_ptr())
    .unwrap_or(ptr::null_mut())
}

unsafe extern "C" fn base_src_negotiate<T: BaseSrcImpl>(
    ptr: *mut ffi::GstBaseSrc,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        match imp.negotiate(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_src_set_caps<T: BaseSrcImpl>(
    ptr: *mut ffi::GstBaseSrc,
    caps: *mut gst::ffi::GstCaps,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);
    let caps = from_glib_borrow(caps);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        match imp.set_caps(wrap.unsafe_cast_ref(), &caps) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_src_fixate<T: BaseSrcImpl>(
    ptr: *mut ffi::GstBaseSrc,
    caps: *mut gst::ffi::GstCaps,
) -> *mut gst::ffi::GstCaps {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);
    let caps = from_glib_full(caps);

    gst::panic_to_error!(&wrap, imp.panicked(), gst::Caps::new_empty(), {
        imp.fixate(wrap.unsafe_cast_ref(), caps)
    })
    .into_glib_ptr()
}

unsafe extern "C" fn base_src_unlock<T: BaseSrcImpl>(
    ptr: *mut ffi::GstBaseSrc,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        match imp.unlock(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_src_unlock_stop<T: BaseSrcImpl>(
    ptr: *mut ffi::GstBaseSrc,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        match imp.unlock_stop(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_src_decide_allocation<T: BaseSrcImpl>(
    ptr: *mut ffi::GstBaseSrc,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<BaseSrc> = from_glib_borrow(ptr);
    let query = match gst::QueryRef::from_mut_ptr(query).view_mut() {
        gst::QueryViewMut::Allocation(allocation) => allocation,
        _ => unreachable!(),
    };

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        match imp.decide_allocation(wrap.unsafe_cast_ref(), query) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .into_glib()
}
