// Take a look at the license at the top of the repository in the LICENSE file.

use std::{ffi::CStr, future::Future, mem, num::NonZeroU64, pin::Pin};

use glib::translate::*;

use crate::{
    format::{
        CompatibleFormattedValue, FormattedValue, SpecificFormattedValueFullRange,
        SpecificFormattedValueIntrinsic,
    },
    prelude::*,
    ClockTime, Element, ElementFlags, Event, Format, GenericFormattedValue, Pad, PadTemplate,
    Plugin, QueryRef, Rank, State,
};

impl Element {
    #[doc(alias = "gst_element_link_many")]
    pub fn link_many<E: IsA<Element>>(elements: &[&E]) -> Result<(), glib::BoolError> {
        skip_assert_initialized!();
        for e in elements.windows(2) {
            unsafe {
                glib::result_from_gboolean!(
                    ffi::gst_element_link(
                        e[0].as_ref().to_glib_none().0,
                        e[1].as_ref().to_glib_none().0,
                    ),
                    "Failed to link elements '{}' and '{}'",
                    e[0].as_ref().name(),
                    e[1].as_ref().name(),
                )?;
            }
        }

        Ok(())
    }

    #[doc(alias = "gst_element_unlink_many")]
    pub fn unlink_many<E: IsA<Element>>(elements: &[&E]) {
        skip_assert_initialized!();
        for e in elements.windows(2) {
            unsafe {
                ffi::gst_element_unlink(
                    e[0].as_ref().to_glib_none().0,
                    e[1].as_ref().to_glib_none().0,
                );
            }
        }
    }

    #[doc(alias = "gst_element_register")]
    pub fn register(
        plugin: Option<&Plugin>,
        name: &str,
        rank: Rank,
        type_: glib::types::Type,
    ) -> Result<(), glib::error::BoolError> {
        skip_assert_initialized!();
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_register(
                    plugin.to_glib_none().0,
                    name.to_glib_none().0,
                    rank.into_glib() as u32,
                    type_.into_glib()
                ),
                "Failed to register element factory"
            )
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum ElementMessageType {
    Error,
    Warning,
    Info,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotifyWatchId(NonZeroU64);

impl IntoGlib for NotifyWatchId {
    type GlibType = libc::c_ulong;

    #[inline]
    fn into_glib(self) -> libc::c_ulong {
        self.0.get() as libc::c_ulong
    }
}

impl FromGlib<libc::c_ulong> for NotifyWatchId {
    #[inline]
    unsafe fn from_glib(val: libc::c_ulong) -> NotifyWatchId {
        skip_assert_initialized!();
        debug_assert_ne!(val, 0);
        NotifyWatchId(NonZeroU64::new_unchecked(val as _))
    }
}

pub trait ElementExtManual: 'static {
    #[doc(alias = "get_element_class")]
    fn element_class(&self) -> &glib::Class<Element>;

    #[doc(alias = "get_current_state")]
    fn current_state(&self) -> State;

    #[doc(alias = "get_pending_state")]
    fn pending_state(&self) -> State;

    #[doc(alias = "gst_element_query")]
    fn query(&self, query: &mut QueryRef) -> bool;

    #[doc(alias = "gst_element_send_event")]
    fn send_event(&self, event: impl Into<Event>) -> bool;

    #[doc(alias = "get_metadata")]
    fn metadata<'a>(&self, key: &str) -> Option<&'a str>;

    #[doc(alias = "get_pad_template")]
    fn pad_template(&self, name: &str) -> Option<PadTemplate>;
    #[doc(alias = "get_pad_template_list")]
    fn pad_template_list(&self) -> glib::List<PadTemplate>;

    #[allow(clippy::too_many_arguments)]
    #[doc(alias = "gst_element_message_full")]
    fn message_full<T: crate::MessageErrorDomain>(
        &self,
        type_: ElementMessageType,
        code: T,
        message: Option<&str>,
        debug: Option<&str>,
        file: &str,
        function: &str,
        line: u32,
    );

    fn set_element_flags(&self, flags: ElementFlags);

    fn unset_element_flags(&self, flags: ElementFlags);

    #[doc(alias = "get_element_flags")]
    fn element_flags(&self) -> ElementFlags;

    #[allow(clippy::too_many_arguments)]
    #[doc(alias = "gst_element_message_full_with_details")]
    fn message_full_with_details<T: crate::MessageErrorDomain>(
        &self,
        type_: ElementMessageType,
        code: T,
        message: Option<&str>,
        debug: Option<&str>,
        file: &str,
        function: &str,
        line: u32,
        structure: crate::Structure,
    );

    fn post_error_message(&self, msg: crate::ErrorMessage);

    #[doc(alias = "gst_element_iterate_pads")]
    fn iterate_pads(&self) -> crate::Iterator<Pad>;
    #[doc(alias = "gst_element_iterate_sink_pads")]
    fn iterate_sink_pads(&self) -> crate::Iterator<Pad>;
    #[doc(alias = "gst_element_iterate_src_pads")]
    fn iterate_src_pads(&self) -> crate::Iterator<Pad>;

    #[doc(alias = "get_pads")]
    #[doc(alias = "gst_element_foreach_pad")]
    fn pads(&self) -> Vec<Pad>;
    #[doc(alias = "get_sink_pads")]
    #[doc(alias = "gst_element_foreach_sink_pad")]
    fn sink_pads(&self) -> Vec<Pad>;
    #[doc(alias = "get_src_pads")]
    #[doc(alias = "gst_element_foreach_src_pad")]
    fn src_pads(&self) -> Vec<Pad>;

    fn num_pads(&self) -> u16;
    fn num_sink_pads(&self) -> u16;
    fn num_src_pads(&self) -> u16;

    #[doc(alias = "gst_element_add_property_deep_notify_watch")]
    fn add_property_deep_notify_watch(
        &self,
        property_name: Option<&str>,
        include_value: bool,
    ) -> NotifyWatchId;

    #[doc(alias = "gst_element_add_property_notify_watch")]
    fn add_property_notify_watch(
        &self,
        property_name: Option<&str>,
        include_value: bool,
    ) -> NotifyWatchId;

    #[doc(alias = "gst_element_remove_property_notify_watch")]
    fn remove_property_notify_watch(&self, watch_id: NotifyWatchId);

    #[doc(alias = "gst_element_query_convert")]
    fn query_convert<U: SpecificFormattedValueFullRange>(
        &self,
        src_val: impl FormattedValue,
    ) -> Option<U>;
    fn query_convert_generic(
        &self,
        src_val: impl FormattedValue,
        dest_format: Format,
    ) -> Option<GenericFormattedValue>;

    #[doc(alias = "gst_element_query_duration")]
    fn query_duration<T: SpecificFormattedValueIntrinsic>(&self) -> Option<T>;
    #[doc(alias = "gst_element_query_duration")]
    fn query_duration_generic(&self, format: Format) -> Option<GenericFormattedValue>;

    #[doc(alias = "gst_element_query_position")]
    fn query_position<T: SpecificFormattedValueIntrinsic>(&self) -> Option<T>;
    #[doc(alias = "gst_element_query_position")]
    fn query_position_generic(&self, format: Format) -> Option<GenericFormattedValue>;

    #[doc(alias = "gst_element_seek")]
    fn seek<V: FormattedValue>(
        &self,
        rate: f64,
        flags: crate::SeekFlags,
        start_type: crate::SeekType,
        start: V,
        stop_type: crate::SeekType,
        stop: impl CompatibleFormattedValue<V>,
    ) -> Result<(), glib::error::BoolError>;
    #[doc(alias = "gst_element_seek_simple")]
    fn seek_simple(
        &self,
        seek_flags: crate::SeekFlags,
        seek_pos: impl FormattedValue,
    ) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_element_call_async")]
    fn call_async<F>(&self, func: F)
    where
        F: FnOnce(&Self) + Send + 'static;

    fn call_async_future<F, T>(&self, func: F) -> Pin<Box<dyn Future<Output = T> + Send + 'static>>
    where
        F: FnOnce(&Self) -> T + Send + 'static,
        T: Send + 'static;

    #[doc(alias = "get_current_running_time")]
    #[doc(alias = "gst_element_get_current_running_time")]
    fn current_running_time(&self) -> Option<crate::ClockTime>;

    #[doc(alias = "get_current_clock_time")]
    #[doc(alias = "gst_element_get_current_clock_time")]
    fn current_clock_time(&self) -> Option<crate::ClockTime>;

    #[doc(alias = "gst_element_get_request_pad")]
    #[doc(alias = "get_request_pad")]
    #[doc(alias = "gst_element_request_pad_simple")]
    fn request_pad_simple(&self, name: &str) -> Option<Pad>;

    #[doc(alias = "gst_element_link")]
    fn link(&self, dest: &impl IsA<Element>) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_element_link_filtered")]
    fn link_filtered(
        &self,
        dest: &impl IsA<Element>,
        filter: &crate::Caps,
    ) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_element_link_pads")]
    fn link_pads(
        &self,
        srcpadname: Option<&str>,
        dest: &impl IsA<Element>,
        destpadname: Option<&str>,
    ) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_element_link_pads_filtered")]
    fn link_pads_filtered(
        &self,
        srcpadname: Option<&str>,
        dest: &impl IsA<Element>,
        destpadname: Option<&str>,
        filter: &crate::Caps,
    ) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_element_link_pads_full")]
    fn link_pads_full(
        &self,
        srcpadname: Option<&str>,
        dest: &impl IsA<Element>,
        destpadname: Option<&str>,
        flags: crate::PadLinkCheck,
    ) -> Result<(), glib::error::BoolError>;
}

impl<O: IsA<Element>> ElementExtManual for O {
    #[inline]
    fn element_class(&self) -> &glib::Class<Element> {
        unsafe {
            let klass = (*(self.as_ptr() as *mut glib::gobject_ffi::GTypeInstance)).g_class
                as *const glib::Class<Element>;
            &*klass
        }
    }

    fn current_state(&self) -> State {
        self.state(Some(ClockTime::ZERO)).1
    }

    fn pending_state(&self) -> State {
        self.state(Some(ClockTime::ZERO)).2
    }

    fn query(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(ffi::gst_element_query(
                self.as_ref().to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn send_event(&self, event: impl Into<Event>) -> bool {
        unsafe {
            from_glib(ffi::gst_element_send_event(
                self.as_ref().to_glib_none().0,
                event.into().into_glib_ptr(),
            ))
        }
    }

    fn metadata<'a>(&self, key: &str) -> Option<&'a str> {
        self.element_class().metadata(key)
    }

    fn pad_template(&self, name: &str) -> Option<PadTemplate> {
        self.element_class().pad_template(name)
    }

    fn pad_template_list(&self) -> glib::List<PadTemplate> {
        self.element_class().pad_template_list()
    }

    fn set_element_flags(&self, flags: ElementFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = self.as_ref().object_lock();
            (*ptr).flags |= flags.into_glib();
        }
    }

    fn unset_element_flags(&self, flags: ElementFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = self.as_ref().object_lock();
            (*ptr).flags &= !flags.into_glib();
        }
    }

    fn element_flags(&self) -> ElementFlags {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = self.as_ref().object_lock();
            from_glib((*ptr).flags)
        }
    }

    fn message_full<T: crate::MessageErrorDomain>(
        &self,
        type_: ElementMessageType,
        code: T,
        message: Option<&str>,
        debug: Option<&str>,
        file: &str,
        function: &str,
        line: u32,
    ) {
        unsafe {
            let type_ = match type_ {
                ElementMessageType::Error => ffi::GST_MESSAGE_ERROR,
                ElementMessageType::Warning => ffi::GST_MESSAGE_WARNING,
                ElementMessageType::Info => ffi::GST_MESSAGE_INFO,
            };

            ffi::gst_element_message_full(
                self.as_ref().to_glib_none().0,
                type_,
                T::domain().into_glib(),
                code.code(),
                message.to_glib_full(),
                debug.to_glib_full(),
                file.to_glib_none().0,
                function.to_glib_none().0,
                line as i32,
            );
        }
    }

    fn message_full_with_details<T: crate::MessageErrorDomain>(
        &self,
        type_: ElementMessageType,
        code: T,
        message: Option<&str>,
        debug: Option<&str>,
        file: &str,
        function: &str,
        line: u32,
        structure: crate::Structure,
    ) {
        unsafe {
            let type_ = match type_ {
                ElementMessageType::Error => ffi::GST_MESSAGE_ERROR,
                ElementMessageType::Warning => ffi::GST_MESSAGE_WARNING,
                ElementMessageType::Info => ffi::GST_MESSAGE_INFO,
            };

            ffi::gst_element_message_full_with_details(
                self.as_ref().to_glib_none().0,
                type_,
                T::domain().into_glib(),
                code.code(),
                message.to_glib_full(),
                debug.to_glib_full(),
                file.to_glib_none().0,
                function.to_glib_none().0,
                line as i32,
                structure.into_glib_ptr(),
            );
        }
    }

    fn post_error_message(&self, msg: crate::ErrorMessage) {
        let crate::ErrorMessage {
            error_domain,
            error_code,
            ref message,
            ref debug,
            filename,
            function,
            line,
        } = msg;

        unsafe {
            ffi::gst_element_message_full(
                self.as_ref().to_glib_none().0,
                ffi::GST_MESSAGE_ERROR,
                error_domain.into_glib(),
                error_code,
                message.to_glib_full(),
                debug.to_glib_full(),
                filename.to_glib_none().0,
                function.to_glib_none().0,
                line as i32,
            );
        }
    }

    fn iterate_pads(&self) -> crate::Iterator<Pad> {
        unsafe {
            from_glib_full(ffi::gst_element_iterate_pads(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn iterate_sink_pads(&self) -> crate::Iterator<Pad> {
        unsafe {
            from_glib_full(ffi::gst_element_iterate_sink_pads(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn iterate_src_pads(&self) -> crate::Iterator<Pad> {
        unsafe {
            from_glib_full(ffi::gst_element_iterate_src_pads(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pads(&self) -> Vec<Pad> {
        unsafe {
            let elt: &ffi::GstElement = &*(self.as_ptr() as *const _);
            let _guard = self.as_ref().object_lock();
            FromGlibPtrContainer::from_glib_none(elt.pads)
        }
    }

    fn sink_pads(&self) -> Vec<Pad> {
        unsafe {
            let elt: &ffi::GstElement = &*(self.as_ptr() as *const _);
            let _guard = self.as_ref().object_lock();
            FromGlibPtrContainer::from_glib_none(elt.sinkpads)
        }
    }

    fn src_pads(&self) -> Vec<Pad> {
        unsafe {
            let elt: &ffi::GstElement = &*(self.as_ptr() as *const _);
            let _guard = self.as_ref().object_lock();
            FromGlibPtrContainer::from_glib_none(elt.srcpads)
        }
    }

    fn num_pads(&self) -> u16 {
        unsafe {
            let elt: &ffi::GstElement = &*(self.as_ptr() as *const _);
            let _guard = self.as_ref().object_lock();
            elt.numpads
        }
    }

    fn num_sink_pads(&self) -> u16 {
        unsafe {
            let elt: &ffi::GstElement = &*(self.as_ptr() as *const _);
            let _guard = self.as_ref().object_lock();
            elt.numsinkpads
        }
    }

    fn num_src_pads(&self) -> u16 {
        unsafe {
            let elt: &ffi::GstElement = &*(self.as_ptr() as *const _);
            let _guard = self.as_ref().object_lock();
            elt.numsrcpads
        }
    }

    fn add_property_deep_notify_watch(
        &self,
        property_name: Option<&str>,
        include_value: bool,
    ) -> NotifyWatchId {
        let property_name = property_name.to_glib_none();
        unsafe {
            from_glib(ffi::gst_element_add_property_deep_notify_watch(
                self.as_ref().to_glib_none().0,
                property_name.0,
                include_value.into_glib(),
            ))
        }
    }

    fn add_property_notify_watch(
        &self,
        property_name: Option<&str>,
        include_value: bool,
    ) -> NotifyWatchId {
        let property_name = property_name.to_glib_none();
        unsafe {
            from_glib(ffi::gst_element_add_property_notify_watch(
                self.as_ref().to_glib_none().0,
                property_name.0,
                include_value.into_glib(),
            ))
        }
    }

    fn remove_property_notify_watch(&self, watch_id: NotifyWatchId) {
        unsafe {
            ffi::gst_element_remove_property_notify_watch(
                self.as_ref().to_glib_none().0,
                watch_id.into_glib(),
            );
        }
    }

    fn query_convert<U: SpecificFormattedValueFullRange>(
        &self,
        src_val: impl FormattedValue,
    ) -> Option<U> {
        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_element_query_convert(
                self.as_ref().to_glib_none().0,
                src_val.format().into_glib(),
                src_val.into_raw_value(),
                U::default_format().into_glib(),
                dest_val.as_mut_ptr(),
            ));
            if ret {
                Some(U::from_raw(U::default_format(), dest_val.assume_init()))
            } else {
                None
            }
        }
    }

    fn query_convert_generic(
        &self,
        src_val: impl FormattedValue,
        dest_format: Format,
    ) -> Option<GenericFormattedValue> {
        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_element_query_convert(
                self.as_ref().to_glib_none().0,
                src_val.format().into_glib(),
                src_val.into_raw_value(),
                dest_format.into_glib(),
                dest_val.as_mut_ptr(),
            ));
            if ret {
                Some(GenericFormattedValue::new(
                    dest_format,
                    dest_val.assume_init(),
                ))
            } else {
                None
            }
        }
    }

    fn query_duration<T: SpecificFormattedValueIntrinsic>(&self) -> Option<T> {
        unsafe {
            let mut duration = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_element_query_duration(
                self.as_ref().to_glib_none().0,
                T::default_format().into_glib(),
                duration.as_mut_ptr(),
            ));
            if ret {
                try_from_glib(duration.assume_init()).ok()
            } else {
                None
            }
        }
    }

    fn query_duration_generic(&self, format: Format) -> Option<GenericFormattedValue> {
        unsafe {
            let mut duration = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_element_query_duration(
                self.as_ref().to_glib_none().0,
                format.into_glib(),
                duration.as_mut_ptr(),
            ));
            if ret {
                Some(GenericFormattedValue::new(format, duration.assume_init()))
            } else {
                None
            }
        }
    }

    fn query_position<T: SpecificFormattedValueIntrinsic>(&self) -> Option<T> {
        unsafe {
            let mut cur = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_element_query_position(
                self.as_ref().to_glib_none().0,
                T::default_format().into_glib(),
                cur.as_mut_ptr(),
            ));
            if ret {
                try_from_glib(cur.assume_init()).ok()
            } else {
                None
            }
        }
    }

    fn query_position_generic(&self, format: Format) -> Option<GenericFormattedValue> {
        unsafe {
            let mut cur = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_element_query_position(
                self.as_ref().to_glib_none().0,
                format.into_glib(),
                cur.as_mut_ptr(),
            ));
            if ret {
                Some(GenericFormattedValue::new(format, cur.assume_init()))
            } else {
                None
            }
        }
    }

    fn seek<V: FormattedValue>(
        &self,
        rate: f64,
        flags: crate::SeekFlags,
        start_type: crate::SeekType,
        start: V,
        stop_type: crate::SeekType,
        stop: impl CompatibleFormattedValue<V>,
    ) -> Result<(), glib::error::BoolError> {
        let stop = stop.try_into_checked(start).unwrap();

        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_seek(
                    self.as_ref().to_glib_none().0,
                    rate,
                    start.format().into_glib(),
                    flags.into_glib(),
                    start_type.into_glib(),
                    start.into_raw_value(),
                    stop_type.into_glib(),
                    stop.into_raw_value(),
                ),
                "Failed to seek",
            )
        }
    }

    fn seek_simple(
        &self,
        seek_flags: crate::SeekFlags,
        seek_pos: impl FormattedValue,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_seek_simple(
                    self.as_ref().to_glib_none().0,
                    seek_pos.format().into_glib(),
                    seek_flags.into_glib(),
                    seek_pos.into_raw_value(),
                ),
                "Failed to seek",
            )
        }
    }

    fn call_async<F>(&self, func: F)
    where
        F: FnOnce(&Self) + Send + 'static,
    {
        let user_data: Box<Option<F>> = Box::new(Some(func));

        unsafe extern "C" fn trampoline<O: IsA<Element>, F: FnOnce(&O) + Send + 'static>(
            element: *mut ffi::GstElement,
            user_data: glib::ffi::gpointer,
        ) {
            let user_data: &mut Option<F> = &mut *(user_data as *mut _);
            let callback = user_data.take().unwrap();

            callback(Element::from_glib_borrow(element).unsafe_cast_ref());
        }

        unsafe extern "C" fn free_user_data<O: IsA<Element>, F: FnOnce(&O) + Send + 'static>(
            user_data: glib::ffi::gpointer,
        ) {
            let _: Box<Option<F>> = Box::from_raw(user_data as *mut _);
        }

        unsafe {
            ffi::gst_element_call_async(
                self.as_ref().to_glib_none().0,
                Some(trampoline::<Self, F>),
                Box::into_raw(user_data) as *mut _,
                Some(free_user_data::<Self, F>),
            );
        }
    }

    fn call_async_future<F, T>(&self, func: F) -> Pin<Box<dyn Future<Output = T> + Send + 'static>>
    where
        F: FnOnce(&Self) -> T + Send + 'static,
        T: Send + 'static,
    {
        use futures_channel::oneshot;

        let (sender, receiver) = oneshot::channel();

        self.call_async(move |element| {
            let _ = sender.send(func(element));
        });

        Box::pin(async move { receiver.await.expect("sender dropped") })
    }

    fn current_running_time(&self) -> Option<crate::ClockTime> {
        let base_time = self.base_time();
        let clock_time = self.current_clock_time();

        clock_time
            .zip(base_time)
            .and_then(|(ct, bt)| ct.checked_sub(bt))
    }

    fn current_clock_time(&self) -> Option<crate::ClockTime> {
        if let Some(clock) = self.clock() {
            clock.time()
        } else {
            crate::ClockTime::NONE
        }
    }

    fn request_pad_simple(&self, name: &str) -> Option<Pad> {
        unsafe {
            #[cfg(feature = "v1_20")]
            {
                from_glib_full(ffi::gst_element_request_pad_simple(
                    self.as_ref().to_glib_none().0,
                    name.to_glib_none().0,
                ))
            }
            #[cfg(not(feature = "v1_20"))]
            {
                from_glib_full(ffi::gst_element_get_request_pad(
                    self.as_ref().to_glib_none().0,
                    name.to_glib_none().0,
                ))
            }
        }
    }

    fn link(&self, dest: &impl IsA<Element>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_link(
                    self.as_ref().to_glib_none().0,
                    dest.as_ref().to_glib_none().0
                ),
                "Failed to link elements '{}' and '{}'",
                self.as_ref().name(),
                dest.as_ref().name(),
            )
        }
    }

    fn link_filtered(
        &self,
        dest: &impl IsA<Element>,
        filter: &crate::Caps,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_link_filtered(
                    self.as_ref().to_glib_none().0,
                    dest.as_ref().to_glib_none().0,
                    filter.to_glib_none().0
                ),
                "Failed to link elements '{}' and '{}' with filter '{:?}'",
                self.as_ref().name(),
                dest.as_ref().name(),
                filter,
            )
        }
    }

    fn link_pads(
        &self,
        srcpadname: Option<&str>,
        dest: &impl IsA<Element>,
        destpadname: Option<&str>,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_link_pads(
                    self.as_ref().to_glib_none().0,
                    srcpadname.to_glib_none().0,
                    dest.as_ref().to_glib_none().0,
                    destpadname.to_glib_none().0
                ),
                "Failed to link pads '{}' and '{}'",
                if let Some(srcpadname) = srcpadname {
                    format!("{}:{}", self.as_ref().name(), srcpadname)
                } else {
                    format!("{}:*", self.as_ref().name())
                },
                if let Some(destpadname) = destpadname {
                    format!("{}:{}", dest.as_ref().name(), destpadname)
                } else {
                    format!("{}:*", dest.as_ref().name())
                },
            )
        }
    }

    fn link_pads_filtered(
        &self,
        srcpadname: Option<&str>,
        dest: &impl IsA<Element>,
        destpadname: Option<&str>,
        filter: &crate::Caps,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_link_pads_filtered(
                    self.as_ref().to_glib_none().0,
                    srcpadname.to_glib_none().0,
                    dest.as_ref().to_glib_none().0,
                    destpadname.to_glib_none().0,
                    filter.to_glib_none().0
                ),
                "Failed to link pads '{}' and '{}' with filter '{:?}'",
                if let Some(srcpadname) = srcpadname {
                    format!("{}:{}", self.as_ref().name(), srcpadname)
                } else {
                    format!("{}:*", self.as_ref().name())
                },
                if let Some(destpadname) = destpadname {
                    format!("{}:{}", dest.as_ref().name(), destpadname)
                } else {
                    format!("{}:*", dest.as_ref().name())
                },
                filter,
            )
        }
    }

    fn link_pads_full(
        &self,
        srcpadname: Option<&str>,
        dest: &impl IsA<Element>,
        destpadname: Option<&str>,
        flags: crate::PadLinkCheck,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_element_link_pads_full(
                    self.as_ref().to_glib_none().0,
                    srcpadname.to_glib_none().0,
                    dest.as_ref().to_glib_none().0,
                    destpadname.to_glib_none().0,
                    flags.into_glib()
                ),
                "Failed to link pads '{}' and '{}' with flags '{:?}'",
                if let Some(srcpadname) = srcpadname {
                    format!("{}:{}", self.as_ref().name(), srcpadname)
                } else {
                    format!("{}:*", self.as_ref().name())
                },
                if let Some(destpadname) = destpadname {
                    format!("{}:{}", dest.as_ref().name(), destpadname)
                } else {
                    format!("{}:*", dest.as_ref().name())
                },
                flags,
            )
        }
    }
}

pub unsafe trait ElementClassExt {
    #[doc(alias = "get_metadata")]
    #[doc(alias = "gst_element_class_get_metadata")]
    fn metadata<'a>(&self, key: &str) -> Option<&'a str> {
        unsafe {
            let klass = self as *const _ as *const ffi::GstElementClass;

            let ptr = ffi::gst_element_class_get_metadata(klass as *mut _, key.to_glib_none().0);

            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    #[doc(alias = "get_pad_template")]
    #[doc(alias = "gst_element_class_get_pad_template")]
    fn pad_template(&self, name: &str) -> Option<PadTemplate> {
        unsafe {
            let klass = self as *const _ as *const ffi::GstElementClass;

            from_glib_none(ffi::gst_element_class_get_pad_template(
                klass as *mut _,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "get_pad_template_list")]
    #[doc(alias = "gst_element_class_get_pad_template_list")]
    fn pad_template_list(&self) -> glib::List<PadTemplate> {
        unsafe {
            let klass = self as *const _ as *const ffi::GstElementClass;

            glib::List::from_glib_none(ffi::gst_element_class_get_pad_template_list(
                klass as *mut _,
            ))
        }
    }
}

unsafe impl<T: IsA<Element> + glib::object::IsClass> ElementClassExt for glib::object::Class<T> {}

#[doc(alias = "GST_ELEMENT_METADATA_AUTHOR")]
pub static ELEMENT_METADATA_AUTHOR: &glib::GStr =
    unsafe { glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_ELEMENT_METADATA_AUTHOR) };
#[doc(alias = "GST_ELEMENT_METADATA_DESCRIPTION")]
pub static ELEMENT_METADATA_DESCRIPTION: &glib::GStr =
    unsafe { glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_ELEMENT_METADATA_DESCRIPTION) };
#[doc(alias = "GST_ELEMENT_METADATA_DOC_URI")]
pub static ELEMENT_METADATA_DOC_URI: &glib::GStr =
    unsafe { glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_ELEMENT_METADATA_DOC_URI) };
#[doc(alias = "GST_ELEMENT_METADATA_ICON_NAME")]
pub static ELEMENT_METADATA_ICON_NAME: &glib::GStr =
    unsafe { glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_ELEMENT_METADATA_ICON_NAME) };
#[doc(alias = "GST_ELEMENT_METADATA_KLASS")]
pub static ELEMENT_METADATA_KLASS: &glib::GStr =
    unsafe { glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_ELEMENT_METADATA_KLASS) };
#[doc(alias = "GST_ELEMENT_METADATA_LONGNAME")]
pub static ELEMENT_METADATA_LONGNAME: &glib::GStr =
    unsafe { glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_ELEMENT_METADATA_LONGNAME) };

#[doc(alias = "GST_ELEMENT_ERROR")]
#[doc(alias = "GST_ELEMENT_ERROR_WITH_DETAILS")]
#[macro_export]
macro_rules! element_error(
    ($obj:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*]) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Error,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            $crate::glib::function_name!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*)) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Error,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            $crate::glib::function_name!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, [$($debug:tt)*]) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Error,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            $crate::glib::function_name!(),
            line!(),
        );
    }};

    ($obj:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*], details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Error,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            $crate::glib::function_name!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Error,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            $crate::glib::function_name!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, [$($debug:tt)*], details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Error,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            $crate::glib::function_name!(),
            line!(),
            $details,
        );
    }};
);

#[doc(alias = "GST_ELEMENT_WARNING")]
#[doc(alias = "GST_ELEMENT_WARNING_WITH_DETAILS")]
#[macro_export]
macro_rules! element_warning(
    ($obj:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*]) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Warning,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            $crate::glib::function_name!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*)) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Warning,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            $crate::glib::function_name!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, [$($debug:tt)*]) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Warning,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            $crate::glib::function_name!(),
            line!(),
        );
    }};

    ($obj:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*], details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Warning,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            $crate::glib::function_name!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Warning,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            $crate::glib::function_name!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, [$($debug:tt)*], details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Warning,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            $crate::glib::function_name!(),
            line!(),
            $details,
        );
    }};
);

#[doc(alias = "GST_ELEMENT_INFO")]
#[doc(alias = "GST_ELEMENT_INFO_WITH_DETAILS")]
#[macro_export]
macro_rules! element_info(
    ($obj:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*]) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Info,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            $crate::glib::function_name!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*)) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Info,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            $crate::glib::function_name!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, [$($debug:tt)*]) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Info,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            $crate::glib::function_name!(),
            line!(),
        );
    }};

    ($obj:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*], details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Info,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            $crate::glib::function_name!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Info,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            $crate::glib::function_name!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, [$($debug:tt)*], details: $details:expr) => { {
        use $crate::prelude::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Info,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            $crate::glib::function_name!(),
            line!(),
            $details,
        );
    }};
);

#[doc(alias = "GST_ELEMENT_ERROR")]
#[doc(alias = "GST_ELEMENT_ERROR_WITH_DETAILS")]
#[macro_export]
macro_rules! element_imp_error(
    ($imp:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*]) => { {
        let obj = $imp.obj();
        $crate::element_error!(obj, $err, ($($msg)*), [$($debug)*]);
    }};
    ($imp:expr, $err:expr, ($($msg:tt)*)) => { {
        let obj = $imp.obj();
        $crate::element_error!(obj, $err, ($($msg)*));
    }};
    ($imp:expr, $err:expr, [$($debug:tt)*]) => { {
        let obj = $imp.obj();
        $crate::element_error!(obj, $err, [$($debug)*]);
    }};

    ($imp:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*], details: $details:expr) => { {
        let obj = $imp.obj();
        $crate::element_error!(obj, $err, ($($msg)*), [$($debug)*], details: $details);
    }};
    ($imp:expr, $err:expr, ($($msg:tt)*), details: $details:expr) => { {
        let obj = $imp.obj();
        $crate::element_error!(obj, $err, ($($msg)*), details: $details);
    }};
    ($imp:expr, $err:expr, [$($debug:tt)*], details: $details:expr) => { {
        let obj = $imp.obj();
        $crate::element_error!(obj, $err, [$($debug)*], details: $details);
    }};
);

#[doc(alias = "GST_ELEMENT_WARNING")]
#[doc(alias = "GST_ELEMENT_WARNING_WITH_DETAILS")]
#[macro_export]
macro_rules! element_imp_warning(
    ($imp:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*]) => { {
        let obj = $imp.obj();
        $crate::element_warning!(obj, $err, ($($msg)*), [$($debug)*]);
    }};
    ($imp:expr, $err:expr, ($($msg:tt)*)) => { {
        let obj = $imp.obj();
        $crate::element_warning!(obj, $err, ($($msg)*));
    }};
    ($imp:expr, $err:expr, [$($debug:tt)*]) => { {
        let obj = $imp.obj();
        $crate::element_warning!(obj, $err, [$($debug)*]);
    }};

    ($imp:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*], details: $details:expr) => { {
        let obj = $imp.obj();
        $crate::element_warning!(obj, $err, ($($msg)*), [$($debug)*], details: $details);
    }};
    ($imp:expr, $err:expr, ($($msg:tt)*), details: $details:expr) => { {
        let obj = $imp.obj();
        $crate::element_warning!(obj, $err, ($($msg)*), details: $details);
    }};
    ($imp:expr, $err:expr, [$($debug:tt)*], details: $details:expr) => { {
        let obj = $imp.obj();
        $crate::element_warning!(obj, $err, [$($debug)*], details: $details);
    }};
);

#[doc(alias = "GST_ELEMENT_INFO")]
#[doc(alias = "GST_ELEMENT_INFO_WITH_DETAILS")]
#[macro_export]
macro_rules! element_imp_info(
    ($imp:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*]) => { {
        let obj = $imp.obj();
        $crate::element_info!(obj, $err, ($($msg)*), [$($debug)*]);
    }};
    ($imp:expr, $err:expr, ($($msg:tt)*)) => { {
        let obj = $imp.obj();
        $crate::element_info!(obj, $err, ($($msg)*));
    }};
    ($imp:expr, $err:expr, [$($debug:tt)*]) => { {
        let obj = $imp.obj();
        $crate::element_info!(obj, $err, [$($debug)*]);
    }};

    ($imp:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*], details: $details:expr) => { {
        let obj = $imp.obj();
        $crate::element_info!(obj, $err, ($($msg)*), [$($debug)*], details: $details);
    }};
    ($imp:expr, $err:expr, ($($msg:tt)*), details: $details:expr) => { {
        let obj = $imp.obj();
        $crate::element_info!(obj, $err, ($($msg)*), details: $details);
    }};
    ($imp:expr, $err:expr, [$($debug:tt)*], details: $details:expr) => { {
        let obj = $imp.obj();
        $crate::element_info!(obj, $err, [$($debug)*], details: $details);
    }};
);

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    use glib::GString;

    use super::*;

    #[test]
    fn test_get_pads() {
        crate::init().unwrap();

        let identity = crate::ElementFactory::make("identity").build().unwrap();

        let mut pad_names = identity
            .pads()
            .iter()
            .map(|p| p.name())
            .collect::<Vec<GString>>();
        pad_names.sort();
        assert_eq!(pad_names, vec![String::from("sink"), String::from("src")]);

        let mut pad_names = identity
            .sink_pads()
            .iter()
            .map(|p| p.name())
            .collect::<Vec<GString>>();
        pad_names.sort();
        assert_eq!(pad_names, vec![String::from("sink")]);

        let mut pad_names = identity
            .src_pads()
            .iter()
            .map(|p| p.name())
            .collect::<Vec<GString>>();
        pad_names.sort();
        assert_eq!(pad_names, vec![String::from("src")]);
    }

    #[test]
    fn test_foreach_pad() {
        crate::init().unwrap();

        let identity = crate::ElementFactory::make("identity").build().unwrap();

        let mut pad_names = Vec::new();
        identity.foreach_pad(|_element, pad| {
            pad_names.push(pad.name());

            true
        });
        pad_names.sort();
        assert_eq!(pad_names, vec![String::from("sink"), String::from("src")]);
    }

    #[test]
    fn test_call_async() {
        crate::init().unwrap();

        let identity = crate::ElementFactory::make("identity").build().unwrap();
        let (sender, receiver) = channel();

        identity.call_async(move |_| {
            sender.send(()).unwrap();
        });

        assert_eq!(receiver.recv(), Ok(()));
    }

    #[test]
    fn test_element_error() {
        crate::init().unwrap();

        let identity = crate::ElementFactory::make("identity").build().unwrap();

        crate::element_error!(identity, crate::CoreError::Failed, ("msg"), ["debug"]);
        crate::element_error!(identity, crate::CoreError::Failed, ["debug"]);
        crate::element_error!(identity, crate::CoreError::Failed, ("msg"));

        // We define a new variable for each call so there would be a compiler warning if the
        // string formatting did not actually use it.
        let x = 123i32;
        crate::element_error!(identity, crate::CoreError::Failed, ("msg {x}"), ["debug"]);
        let x = 123i32;
        crate::element_error!(identity, crate::CoreError::Failed, ["debug {x}"]);
        let x = 123i32;
        crate::element_error!(identity, crate::CoreError::Failed, ("msg {}", x));
    }
}
