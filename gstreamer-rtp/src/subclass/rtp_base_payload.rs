// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use gst::subclass::prelude::*;

use crate::prelude::*;
use crate::RTPBasePayload;

pub trait RTPBasePayloadImpl: RTPBasePayloadImplExt + ElementImpl {
    fn caps(&self, element: &Self::Type, pad: &gst::Pad, filter: Option<&gst::Caps>) -> gst::Caps {
        self.parent_caps(element, pad, filter)
    }

    fn set_caps(&self, element: &Self::Type, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        self.parent_set_caps(element, caps)
    }

    fn handle_buffer(
        &self,
        element: &Self::Type,
        buffer: gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_handle_buffer(element, buffer)
    }

    fn query(&self, element: &Self::Type, pad: &gst::Pad, query: &mut gst::QueryRef) -> bool {
        RTPBasePayloadImplExt::parent_query(self, element, pad, query)
    }

    fn sink_event(&self, element: &Self::Type, event: gst::Event) -> bool {
        self.parent_sink_event(element, event)
    }

    fn src_event(&self, element: &Self::Type, event: gst::Event) -> bool {
        self.parent_src_event(element, event)
    }
}

pub trait RTPBasePayloadImplExt: ObjectSubclass {
    fn parent_caps(
        &self,
        element: &Self::Type,
        pad: &gst::Pad,
        filter: Option<&gst::Caps>,
    ) -> gst::Caps;

    fn parent_set_caps(
        &self,
        element: &Self::Type,
        caps: &gst::Caps,
    ) -> Result<(), gst::LoggableError>;

    fn parent_handle_buffer(
        &self,
        element: &Self::Type,
        buffer: gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_query(&self, element: &Self::Type, pad: &gst::Pad, query: &mut gst::QueryRef)
        -> bool;

    fn parent_sink_event(&self, element: &Self::Type, event: gst::Event) -> bool;

    fn parent_src_event(&self, element: &Self::Type, event: gst::Event) -> bool;
}

impl<T: RTPBasePayloadImpl> RTPBasePayloadImplExt for T {
    fn parent_caps(
        &self,
        element: &Self::Type,
        pad: &gst::Pad,
        filter: Option<&gst::Caps>,
    ) -> gst::Caps {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTPBasePayloadClass;
            let f = (*parent_class)
                .get_caps
                .expect("Missing parent function `get_caps`");
            from_glib_full(f(
                element.unsafe_cast_ref::<RTPBasePayload>().to_glib_none().0,
                pad.to_glib_none().0,
                filter.to_glib_none().0,
            ))
        }
    }

    fn parent_set_caps(
        &self,
        element: &Self::Type,
        caps: &gst::Caps,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTPBasePayloadClass;
            (*parent_class)
                .set_caps
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            element.unsafe_cast_ref::<RTPBasePayload>().to_glib_none().0,
                            caps.to_glib_none().0
                        ),
                        gst::CAT_RUST,
                        "Parent function `set_caps` failed"
                    )
                })
                .unwrap_or_else(|| {
                    // Trigger negotiation as the base class does
                    element
                        .unsafe_cast_ref::<RTPBasePayload>()
                        .set_outcaps(None)
                        .map_err(|_| gst::loggable_error!(gst::CAT_RUST, "Failed to negotiate"))
                })
        }
    }

    fn parent_handle_buffer(
        &self,
        element: &Self::Type,
        buffer: gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTPBasePayloadClass;
            (*parent_class)
                .handle_buffer
                .map(|f| {
                    try_from_glib(f(
                        element.unsafe_cast_ref::<RTPBasePayload>().to_glib_none().0,
                        buffer.into_glib_ptr(),
                    ))
                })
                .unwrap_or(Err(gst::FlowError::Error))
        }
    }

    fn parent_query(
        &self,
        element: &Self::Type,
        pad: &gst::Pad,
        query: &mut gst::QueryRef,
    ) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTPBasePayloadClass;
            (*parent_class)
                .query
                .map(|f| {
                    from_glib(f(
                        element.unsafe_cast_ref::<RTPBasePayload>().to_glib_none().0,
                        pad.to_glib_none().0,
                        query.as_mut_ptr(),
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_sink_event(&self, element: &Self::Type, event: gst::Event) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTPBasePayloadClass;
            (*parent_class)
                .sink_event
                .map(|f| {
                    from_glib(f(
                        element.unsafe_cast_ref::<RTPBasePayload>().to_glib_none().0,
                        event.into_glib_ptr(),
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_src_event(&self, element: &Self::Type, event: gst::Event) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTPBasePayloadClass;
            (*parent_class)
                .src_event
                .map(|f| {
                    from_glib(f(
                        element.unsafe_cast_ref::<RTPBasePayload>().to_glib_none().0,
                        event.into_glib_ptr(),
                    ))
                })
                .unwrap_or(false)
        }
    }
}

unsafe impl<T: RTPBasePayloadImpl> IsSubclassable<T> for RTPBasePayload {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.get_caps = Some(rtp_base_payload_get_caps::<T>);
        klass.set_caps = Some(rtp_base_payload_set_caps::<T>);
        klass.handle_buffer = Some(rtp_base_payload_handle_buffer::<T>);
        klass.query = Some(rtp_base_payload_query::<T>);
        klass.sink_event = Some(rtp_base_payload_sink_event::<T>);
        klass.src_event = Some(rtp_base_payload_src_event::<T>);
    }
}

unsafe extern "C" fn rtp_base_payload_get_caps<T: RTPBasePayloadImpl>(
    ptr: *mut ffi::GstRTPBasePayload,
    pad: *mut gst::ffi::GstPad,
    filter: *mut gst::ffi::GstCaps,
) -> *mut gst::ffi::GstCaps {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<RTPBasePayload> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), gst::Caps::new_empty(), {
        RTPBasePayloadImpl::caps(
            imp,
            wrap.unsafe_cast_ref(),
            &from_glib_borrow(pad),
            Option::<gst::Caps>::from_glib_borrow(filter)
                .as_ref()
                .as_ref(),
        )
    })
    .to_glib_full()
}

unsafe extern "C" fn rtp_base_payload_set_caps<T: RTPBasePayloadImpl>(
    ptr: *mut ffi::GstRTPBasePayload,
    caps: *mut gst::ffi::GstCaps,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<RTPBasePayload> = from_glib_borrow(ptr);
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

unsafe extern "C" fn rtp_base_payload_handle_buffer<T: RTPBasePayloadImpl>(
    ptr: *mut ffi::GstRTPBasePayload,
    buffer: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<RTPBasePayload> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), gst::FlowReturn::Error, {
        imp.handle_buffer(wrap.unsafe_cast_ref(), from_glib_full(buffer))
            .into()
    })
    .into_glib()
}

unsafe extern "C" fn rtp_base_payload_query<T: RTPBasePayloadImpl>(
    ptr: *mut ffi::GstRTPBasePayload,
    pad: *mut gst::ffi::GstPad,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<RTPBasePayload> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        RTPBasePayloadImpl::query(
            imp,
            wrap.unsafe_cast_ref(),
            &from_glib_borrow(pad),
            gst::QueryRef::from_mut_ptr(query),
        )
    })
    .into_glib()
}

unsafe extern "C" fn rtp_base_payload_sink_event<T: RTPBasePayloadImpl>(
    ptr: *mut ffi::GstRTPBasePayload,
    event: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<RTPBasePayload> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        imp.sink_event(wrap.unsafe_cast_ref(), from_glib_full(event))
    })
    .into_glib()
}

unsafe extern "C" fn rtp_base_payload_src_event<T: RTPBasePayloadImpl>(
    ptr: *mut ffi::GstRTPBasePayload,
    event: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<RTPBasePayload> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        imp.src_event(wrap.unsafe_cast_ref(), from_glib_full(event))
    })
    .into_glib()
}
