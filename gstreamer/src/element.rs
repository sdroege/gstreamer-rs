// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Element;
use ElementClass;

use glib;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib::translate::FromGlibPtrBorrow;
use glib::translate::{
    from_glib, from_glib_full, from_glib_none, FromGlib, FromGlibPtrContainer, ToGlib, ToGlibPtr,
};
use miniobject::MiniObject;
use ClockTime;
use ElementFlags;
use Event;
use Format;
use FormattedValue;
use GenericFormattedValue;
use Pad;
use PadTemplate;
use QueryRef;
use SpecificFormattedValue;
use State;
use StateChange;
use StateChangeError;
use StateChangeReturn;
use StateChangeSuccess;

use std::ffi::CStr;
use std::mem;

use libc;

#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib_sys;
use gobject_sys;
use gst_sys;

impl Element {
    pub fn link_many<E: IsA<Element>>(elements: &[&E]) -> Result<(), glib::BoolError> {
        skip_assert_initialized!();
        for e in elements.windows(2) {
            unsafe {
                glib_result_from_gboolean!(
                    gst_sys::gst_element_link(
                        e[0].as_ref().to_glib_none().0,
                        e[1].as_ref().to_glib_none().0,
                    ),
                    "Failed to link elements"
                )?;
            }
        }

        Ok(())
    }

    pub fn unlink_many<E: IsA<Element>>(elements: &[&E]) {
        skip_assert_initialized!();
        for e in elements.windows(2) {
            unsafe {
                gst_sys::gst_element_unlink(
                    e[0].as_ref().to_glib_none().0,
                    e[1].as_ref().to_glib_none().0,
                );
            }
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
pub struct NotifyWatchId(libc::c_ulong);

impl ToGlib for NotifyWatchId {
    type GlibType = libc::c_ulong;

    fn to_glib(&self) -> libc::c_ulong {
        self.0
    }
}

impl FromGlib<libc::c_ulong> for NotifyWatchId {
    fn from_glib(val: libc::c_ulong) -> NotifyWatchId {
        skip_assert_initialized!();
        assert_ne!(val, 0);
        NotifyWatchId(val)
    }
}

pub trait ElementExtManual: 'static {
    fn get_element_class(&self) -> &ElementClass;

    fn change_state(&self, transition: StateChange)
        -> Result<StateChangeSuccess, StateChangeError>;
    fn continue_state(
        &self,
        ret: StateChangeReturn,
    ) -> Result<StateChangeSuccess, StateChangeError>;

    fn get_state(
        &self,
        timeout: ClockTime,
    ) -> (Result<StateChangeSuccess, StateChangeError>, State, State);
    fn set_state(&self, state: State) -> Result<StateChangeSuccess, StateChangeError>;

    fn query(&self, query: &mut QueryRef) -> bool;

    fn send_event(&self, event: Event) -> bool;

    fn get_metadata<'a>(&self, key: &str) -> Option<&'a str>;

    fn get_pad_template(&self, name: &str) -> Option<PadTemplate>;
    fn get_pad_template_list(&self) -> Vec<PadTemplate>;

    #[allow(clippy::too_many_arguments)]
    fn message_full<T: ::MessageErrorDomain>(
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

    fn get_element_flags(&self) -> ElementFlags;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[allow(clippy::too_many_arguments)]
    fn message_full_with_details<T: ::MessageErrorDomain>(
        &self,
        type_: ElementMessageType,
        code: T,
        message: Option<&str>,
        debug: Option<&str>,
        file: &str,
        function: &str,
        line: u32,
        structure: ::Structure,
    );

    fn post_error_message(&self, msg: &::ErrorMessage);

    fn iterate_pads(&self) -> ::Iterator<Pad>;
    fn iterate_sink_pads(&self) -> ::Iterator<Pad>;
    fn iterate_src_pads(&self) -> ::Iterator<Pad>;

    fn get_pads(&self) -> Vec<Pad>;
    fn get_sink_pads(&self) -> Vec<Pad>;
    fn get_src_pads(&self) -> Vec<Pad>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn add_property_deep_notify_watch<'a, P: Into<Option<&'a str>>>(
        &self,
        property_name: P,
        include_value: bool,
    ) -> NotifyWatchId;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn add_property_notify_watch<'a, P: Into<Option<&'a str>>>(
        &self,
        property_name: P,
        include_value: bool,
    ) -> NotifyWatchId;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn remove_property_notify_watch(&self, watch_id: NotifyWatchId);

    fn query_convert<V: Into<GenericFormattedValue>, U: SpecificFormattedValue>(
        &self,
        src_val: V,
    ) -> Option<U>;
    fn query_convert_generic<V: Into<GenericFormattedValue>>(
        &self,
        src_val: V,
        dest_format: Format,
    ) -> Option<GenericFormattedValue>;

    fn query_duration<T: SpecificFormattedValue>(&self) -> Option<T>;
    fn query_duration_generic(&self, format: Format) -> Option<GenericFormattedValue>;

    fn query_position<T: SpecificFormattedValue>(&self) -> Option<T>;
    fn query_position_generic(&self, format: Format) -> Option<GenericFormattedValue>;

    fn seek<V: Into<GenericFormattedValue>>(
        &self,
        rate: f64,
        flags: ::SeekFlags,
        start_type: ::SeekType,
        start: V,
        stop_type: ::SeekType,
        stop: V,
    ) -> Result<(), glib::error::BoolError>;
    fn seek_simple<V: Into<GenericFormattedValue>>(
        &self,
        seek_flags: ::SeekFlags,
        seek_pos: V,
    ) -> Result<(), glib::error::BoolError>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn call_async<F>(&self, func: F)
    where
        F: FnOnce(&Self) + Send + 'static;
}

impl<O: IsA<Element>> ElementExtManual for O {
    fn get_element_class(&self) -> &ElementClass {
        unsafe {
            let klass = (*(self.as_ptr() as *mut gobject_sys::GTypeInstance)).g_class
                as *const ElementClass;
            &*klass
        }
    }

    fn change_state(
        &self,
        transition: StateChange,
    ) -> Result<StateChangeSuccess, StateChangeError> {
        let ret: StateChangeReturn = unsafe {
            from_glib(gst_sys::gst_element_change_state(
                self.as_ref().to_glib_none().0,
                transition.to_glib(),
            ))
        };
        ret.into_result()
    }

    fn continue_state(
        &self,
        ret: StateChangeReturn,
    ) -> Result<StateChangeSuccess, StateChangeError> {
        let ret: StateChangeReturn = unsafe {
            from_glib(gst_sys::gst_element_continue_state(
                self.as_ref().to_glib_none().0,
                ret.to_glib(),
            ))
        };
        ret.into_result()
    }

    fn get_state(
        &self,
        timeout: ClockTime,
    ) -> (Result<StateChangeSuccess, StateChangeError>, State, State) {
        unsafe {
            let mut state = mem::uninitialized();
            let mut pending = mem::uninitialized();
            let ret: StateChangeReturn = from_glib(gst_sys::gst_element_get_state(
                self.as_ref().to_glib_none().0,
                &mut state,
                &mut pending,
                timeout.to_glib(),
            ));
            (ret.into_result(), from_glib(state), from_glib(pending))
        }
    }

    fn set_state(&self, state: State) -> Result<StateChangeSuccess, StateChangeError> {
        let ret: StateChangeReturn = unsafe {
            from_glib(gst_sys::gst_element_set_state(
                self.as_ref().to_glib_none().0,
                state.to_glib(),
            ))
        };
        ret.into_result()
    }

    fn query(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(gst_sys::gst_element_query(
                self.as_ref().to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn send_event(&self, event: Event) -> bool {
        unsafe {
            from_glib(gst_sys::gst_element_send_event(
                self.as_ref().to_glib_none().0,
                event.into_ptr(),
            ))
        }
    }

    fn get_metadata<'a>(&self, key: &str) -> Option<&'a str> {
        self.get_element_class().get_metadata(key)
    }

    fn get_pad_template(&self, name: &str) -> Option<PadTemplate> {
        self.get_element_class().get_pad_template(name)
    }

    fn get_pad_template_list(&self) -> Vec<PadTemplate> {
        self.get_element_class().get_pad_template_list()
    }

    fn set_element_flags(&self, flags: ElementFlags) {
        unsafe {
            let ptr: *mut gst_sys::GstObject = self.as_ptr() as *mut _;
            let _guard = ::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags |= flags.to_glib();
        }
    }

    fn get_element_flags(&self) -> ElementFlags {
        unsafe {
            let ptr: *mut gst_sys::GstObject = self.as_ptr() as *mut _;
            let _guard = ::utils::MutexGuard::lock(&(*ptr).lock);
            from_glib((*ptr).flags)
        }
    }

    fn message_full<T: ::MessageErrorDomain>(
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
                ElementMessageType::Error => gst_sys::GST_MESSAGE_ERROR,
                ElementMessageType::Warning => gst_sys::GST_MESSAGE_WARNING,
                ElementMessageType::Info => gst_sys::GST_MESSAGE_INFO,
            };

            gst_sys::gst_element_message_full(
                self.as_ref().to_glib_none().0,
                type_,
                T::domain().to_glib(),
                code.code(),
                message.to_glib_full(),
                debug.to_glib_full(),
                file.to_glib_none().0,
                function.to_glib_none().0,
                line as i32,
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn message_full_with_details<T: ::MessageErrorDomain>(
        &self,
        type_: ElementMessageType,
        code: T,
        message: Option<&str>,
        debug: Option<&str>,
        file: &str,
        function: &str,
        line: u32,
        structure: ::Structure,
    ) {
        unsafe {
            let type_ = match type_ {
                ElementMessageType::Error => gst_sys::GST_MESSAGE_ERROR,
                ElementMessageType::Warning => gst_sys::GST_MESSAGE_WARNING,
                ElementMessageType::Info => gst_sys::GST_MESSAGE_INFO,
            };

            gst_sys::gst_element_message_full_with_details(
                self.as_ref().to_glib_none().0,
                type_,
                T::domain().to_glib(),
                code.code(),
                message.to_glib_full(),
                debug.to_glib_full(),
                file.to_glib_none().0,
                function.to_glib_none().0,
                line as i32,
                structure.into_ptr(),
            );
        }
    }

    fn post_error_message(&self, msg: &::ErrorMessage) {
        let ::ErrorMessage {
            error_domain,
            error_code,
            ref message,
            ref debug,
            filename,
            function,
            line,
        } = *msg;

        unsafe {
            gst_sys::gst_element_message_full(
                self.as_ref().to_glib_none().0,
                gst_sys::GST_MESSAGE_ERROR,
                error_domain.to_glib(),
                error_code,
                message.to_glib_full(),
                debug.to_glib_full(),
                filename.to_glib_none().0,
                function.to_glib_none().0,
                line as i32,
            );
        }
    }

    fn iterate_pads(&self) -> ::Iterator<Pad> {
        unsafe {
            from_glib_full(gst_sys::gst_element_iterate_pads(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn iterate_sink_pads(&self) -> ::Iterator<Pad> {
        unsafe {
            from_glib_full(gst_sys::gst_element_iterate_sink_pads(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn iterate_src_pads(&self) -> ::Iterator<Pad> {
        unsafe {
            from_glib_full(gst_sys::gst_element_iterate_src_pads(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_pads(&self) -> Vec<Pad> {
        unsafe {
            let elt: &gst_sys::GstElement = &*(self.as_ptr() as *const _);
            ::utils::MutexGuard::lock(&elt.object.lock);
            FromGlibPtrContainer::from_glib_none(elt.pads)
        }
    }

    fn get_sink_pads(&self) -> Vec<Pad> {
        unsafe {
            let elt: &gst_sys::GstElement = &*(self.as_ptr() as *const _);
            ::utils::MutexGuard::lock(&elt.object.lock);
            FromGlibPtrContainer::from_glib_none(elt.sinkpads)
        }
    }

    fn get_src_pads(&self) -> Vec<Pad> {
        unsafe {
            let elt: &gst_sys::GstElement = &*(self.as_ptr() as *const _);
            ::utils::MutexGuard::lock(&elt.object.lock);
            FromGlibPtrContainer::from_glib_none(elt.srcpads)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn add_property_deep_notify_watch<'a, P: Into<Option<&'a str>>>(
        &self,
        property_name: P,
        include_value: bool,
    ) -> NotifyWatchId {
        let property_name = property_name.into();
        let property_name = property_name.to_glib_none();
        unsafe {
            from_glib(gst_sys::gst_element_add_property_deep_notify_watch(
                self.as_ref().to_glib_none().0,
                property_name.0,
                include_value.to_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn add_property_notify_watch<'a, P: Into<Option<&'a str>>>(
        &self,
        property_name: P,
        include_value: bool,
    ) -> NotifyWatchId {
        let property_name = property_name.into();
        let property_name = property_name.to_glib_none();
        unsafe {
            from_glib(gst_sys::gst_element_add_property_notify_watch(
                self.as_ref().to_glib_none().0,
                property_name.0,
                include_value.to_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn remove_property_notify_watch(&self, watch_id: NotifyWatchId) {
        unsafe {
            gst_sys::gst_element_remove_property_notify_watch(
                self.as_ref().to_glib_none().0,
                watch_id.0,
            );
        }
    }

    fn query_convert<V: Into<GenericFormattedValue>, U: SpecificFormattedValue>(
        &self,
        src_val: V,
    ) -> Option<U> {
        let src_val = src_val.into();
        unsafe {
            let mut dest_val = mem::uninitialized();
            let ret = from_glib(gst_sys::gst_element_query_convert(
                self.as_ref().to_glib_none().0,
                src_val.get_format().to_glib(),
                src_val.to_raw_value(),
                U::get_default_format().to_glib(),
                &mut dest_val,
            ));
            if ret {
                Some(U::from_raw(U::get_default_format(), dest_val))
            } else {
                None
            }
        }
    }

    fn query_convert_generic<V: Into<GenericFormattedValue>>(
        &self,
        src_val: V,
        dest_format: Format,
    ) -> Option<GenericFormattedValue> {
        let src_val = src_val.into();
        unsafe {
            let mut dest_val = mem::uninitialized();
            let ret = from_glib(gst_sys::gst_element_query_convert(
                self.as_ref().to_glib_none().0,
                src_val.get_format().to_glib(),
                src_val.get_value(),
                dest_format.to_glib(),
                &mut dest_val,
            ));
            if ret {
                Some(GenericFormattedValue::new(dest_format, dest_val))
            } else {
                None
            }
        }
    }

    fn query_duration<T: SpecificFormattedValue>(&self) -> Option<T> {
        unsafe {
            let mut duration = mem::uninitialized();
            let ret = from_glib(gst_sys::gst_element_query_duration(
                self.as_ref().to_glib_none().0,
                T::get_default_format().to_glib(),
                &mut duration,
            ));
            if ret {
                Some(T::from_raw(T::get_default_format(), duration))
            } else {
                None
            }
        }
    }

    fn query_duration_generic(&self, format: Format) -> Option<GenericFormattedValue> {
        unsafe {
            let mut duration = mem::uninitialized();
            let ret = from_glib(gst_sys::gst_element_query_duration(
                self.as_ref().to_glib_none().0,
                format.to_glib(),
                &mut duration,
            ));
            if ret {
                Some(GenericFormattedValue::new(format, duration))
            } else {
                None
            }
        }
    }

    fn query_position<T: SpecificFormattedValue>(&self) -> Option<T> {
        unsafe {
            let mut cur = mem::uninitialized();
            let ret = from_glib(gst_sys::gst_element_query_position(
                self.as_ref().to_glib_none().0,
                T::get_default_format().to_glib(),
                &mut cur,
            ));
            if ret {
                Some(T::from_raw(T::get_default_format(), cur))
            } else {
                None
            }
        }
    }

    fn query_position_generic(&self, format: Format) -> Option<GenericFormattedValue> {
        unsafe {
            let mut cur = mem::uninitialized();
            let ret = from_glib(gst_sys::gst_element_query_position(
                self.as_ref().to_glib_none().0,
                format.to_glib(),
                &mut cur,
            ));
            if ret {
                Some(GenericFormattedValue::new(format, cur))
            } else {
                None
            }
        }
    }

    fn seek<V: Into<GenericFormattedValue>>(
        &self,
        rate: f64,
        flags: ::SeekFlags,
        start_type: ::SeekType,
        start: V,
        stop_type: ::SeekType,
        stop: V,
    ) -> Result<(), glib::error::BoolError> {
        let start = start.into();
        let stop = stop.into();

        assert_eq!(stop.get_format(), start.get_format());

        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_element_seek(
                    self.as_ref().to_glib_none().0,
                    rate,
                    start.get_format().to_glib(),
                    flags.to_glib(),
                    start_type.to_glib(),
                    start.get_value(),
                    stop_type.to_glib(),
                    stop.get_value(),
                ),
                "Failed to seek",
            )
        }
    }

    fn seek_simple<V: Into<GenericFormattedValue>>(
        &self,
        seek_flags: ::SeekFlags,
        seek_pos: V,
    ) -> Result<(), glib::error::BoolError> {
        let seek_pos = seek_pos.into();
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_element_seek_simple(
                    self.as_ref().to_glib_none().0,
                    seek_pos.get_format().to_glib(),
                    seek_flags.to_glib(),
                    seek_pos.get_value(),
                ),
                "Failed to seek",
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn call_async<F>(&self, func: F)
    where
        F: FnOnce(&Self) + Send + 'static,
    {
        let user_data: Box<Option<F>> = Box::new(Some(func));

        unsafe extern "C" fn trampoline<O: IsA<Element>, F: FnOnce(&O) + Send + 'static>(
            element: *mut gst_sys::GstElement,
            user_data: glib_sys::gpointer,
        ) {
            let user_data: &mut Option<F> = &mut *(user_data as *mut _);
            let callback = user_data.take().unwrap();

            callback(&Element::from_glib_borrow(element).unsafe_cast());
        }

        unsafe extern "C" fn free_user_data<O: IsA<Element>, F: FnOnce(&O) + Send + 'static>(
            user_data: glib_sys::gpointer,
        ) {
            let _: Box<Option<F>> = Box::from_raw(user_data as *mut _);
        }

        unsafe {
            gst_sys::gst_element_call_async(
                self.as_ref().to_glib_none().0,
                Some(trampoline::<Self, F>),
                Box::into_raw(user_data) as *mut _,
                Some(free_user_data::<Self, F>),
            );
        }
    }
}

impl ElementClass {
    pub fn get_metadata<'a>(&self, key: &str) -> Option<&'a str> {
        unsafe {
            let klass = self as *const _ as *const gst_sys::GstElementClass;

            let ptr =
                gst_sys::gst_element_class_get_metadata(klass as *mut _, key.to_glib_none().0);

            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    pub fn get_pad_template(&self, name: &str) -> Option<PadTemplate> {
        unsafe {
            let klass = self as *const _ as *const gst_sys::GstElementClass;

            from_glib_none(gst_sys::gst_element_class_get_pad_template(
                klass as *mut _,
                name.to_glib_none().0,
            ))
        }
    }

    pub fn get_pad_template_list(&self) -> Vec<PadTemplate> {
        unsafe {
            let klass = self as *const _ as *const gst_sys::GstElementClass;

            FromGlibPtrContainer::from_glib_none(gst_sys::gst_element_class_get_pad_template_list(
                klass as *mut _,
            ))
        }
    }
}

lazy_static! {
    pub static ref ELEMENT_METADATA_AUTHOR: &'static str = unsafe {
        CStr::from_ptr(gst_sys::GST_ELEMENT_METADATA_AUTHOR)
            .to_str()
            .unwrap()
    };
    pub static ref ELEMENT_METADATA_DESCRIPTION: &'static str = unsafe {
        CStr::from_ptr(gst_sys::GST_ELEMENT_METADATA_DESCRIPTION)
            .to_str()
            .unwrap()
    };
    pub static ref ELEMENT_METADATA_DOC_URI: &'static str = unsafe {
        CStr::from_ptr(gst_sys::GST_ELEMENT_METADATA_DOC_URI)
            .to_str()
            .unwrap()
    };
    pub static ref ELEMENT_METADATA_ICON_NAME: &'static str = unsafe {
        CStr::from_ptr(gst_sys::GST_ELEMENT_METADATA_ICON_NAME)
            .to_str()
            .unwrap()
    };
    pub static ref ELEMENT_METADATA_KLASS: &'static str = unsafe {
        CStr::from_ptr(gst_sys::GST_ELEMENT_METADATA_KLASS)
            .to_str()
            .unwrap()
    };
    pub static ref ELEMENT_METADATA_LONGNAME: &'static str = unsafe {
        CStr::from_ptr(gst_sys::GST_ELEMENT_METADATA_LONGNAME)
            .to_str()
            .unwrap()
    };
}

#[macro_export]
macro_rules! gst_element_error(
    ($obj:expr, $err:expr, ($msg:expr), [$debug:expr]) => { {
        use $crate::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Error,
            $err,
            Some($msg),
            Some($debug),
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($msg:expr)) => { {
        use $crate::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Error,
            $err,
            Some($msg),
            None,
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, [$debug:expr]) => { {
        use $crate::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Error,
            $err,
            None,
            Some($debug),
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*]) => { {
        use $crate::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Error,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*)) => { {
        use $crate::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Error,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, [$($debug:tt)*]) => { {
        use $crate::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Error,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
        );
    }};

    ($obj:expr, $err:expr, ($msg:expr), [$debug:expr], details: $details:expr) => { {
        use $crate::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Error,
            $err,
            Some($msg),
            Some($debug),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($msg:expr), details: $details:expr) => { {
        use $crate::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Error,
            $err,
            Some($msg),
            None,
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, [$debug:expr], details: $details:expr) => { {
        use $crate::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Error,
            $err,
            None,
            Some($debug),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*], details: $details:expr) => { {
        use $crate::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Error,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), details: $details:expr) => { {
        use $crate::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Error,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, [$($debug:tt)*], details: $details:expr) => { {
        use $crate::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Error,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
);

#[macro_export]
macro_rules! gst_element_warning(
    ($obj:expr, $err:expr, ($msg:expr), [$debug:expr]) => { {
        use $crate::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Warning,
            $err,
            Some($msg),
            Some($debug),
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($msg:expr)) => { {
        use $crate::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Warning,
            $err,
            Some($msg),
            None,
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, [$debug:expr]) => { {
        use $crate::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Warning,
            $err,
            None,
            Some($debug),
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*]) => { {
        use $crate::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Warning,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*)) => { {
        use $crate::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Warning,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, [$($debug:tt)*]) => { {
        use $crate::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Warning,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
        );
    }};

    ($obj:expr, $err:expr, ($msg:expr), [$debug:expr], details: $details:expr) => { {
        use $crate::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Warning,
            $err,
            Some($msg),
            Some($debug),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($msg:expr), details: $details:expr) => { {
        use $crate::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Warning,
            $err,
            Some($msg),
            None,
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, [$debug:expr], details: $details:expr) => { {
        use $crate::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Warning,
            $err,
            None,
            Some($debug),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*], details: $details:expr) => { {
        use $crate::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Warning,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), details: $details:expr) => { {
        use $crate::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Warning,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, [$($debug:tt)*], details: $details:expr) => { {
        use $crate::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Warning,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
);

#[macro_export]
macro_rules! gst_element_info(
    ($obj:expr, $err:expr, ($msg:expr), [$debug:expr]) => { {
        use $crate::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Info,
            $err,
            Some($msg),
            Some($debug),
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($msg:expr)) => { {
        use $crate::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Info,
            $err,
            Some($msg),
            None,
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, [$debug:expr]) => { {
        use $crate::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Info,
            $err,
            None,
            Some($debug),
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*]) => { {
        use $crate::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Info,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*)) => { {
        use $crate::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Info,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            module_path!(),
            line!(),
        );
    }};
    ($obj:expr, $err:expr, [$($debug:tt)*]) => { {
        use $crate::ElementExtManual;
        $obj.message_full(
            $crate::ElementMessageType::Info,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
        );
    }};

    ($obj:expr, $err:expr, ($msg:expr), [$debug:expr], details: $details:expr) => { {
        use $crate::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Info,
            $err,
            Some($msg),
            Some($debug),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($msg:expr), details: $details:expr) => { {
        use $crate::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Info,
            $err,
            Some($msg),
            None,
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, [$debug:expr], details: $details:expr) => { {
        use $crate::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Info,
            $err,
            None,
            Some($debug),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*], details: $details:expr) => { {
        use $crate::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Info,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, ($($msg:tt)*), details: $details:expr) => { {
        use $crate::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Info,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
    ($obj:expr, $err:expr, [$($debug:tt)*], details: $details:expr) => { {
        use $crate::ElementExtManual;
        $obj.message_full_with_details(
            $crate::ElementMessageType::Info,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
            $details,
        );
    }};
);

#[cfg(test)]
mod tests {
    use super::*;
    use glib::GString;
    use prelude::*;
    #[cfg(feature = "v1_10")]
    use std::sync::mpsc::channel;

    #[test]
    fn test_get_pads() {
        ::init().unwrap();

        let identity = ::ElementFactory::make("identity", None).unwrap();

        let mut pad_names = identity
            .get_pads()
            .iter()
            .map(|p| p.get_name())
            .collect::<Vec<GString>>();
        pad_names.sort();
        assert_eq!(pad_names, vec![String::from("sink"), String::from("src")]);

        let mut pad_names = identity
            .get_sink_pads()
            .iter()
            .map(|p| p.get_name())
            .collect::<Vec<GString>>();
        pad_names.sort();
        assert_eq!(pad_names, vec![String::from("sink")]);

        let mut pad_names = identity
            .get_src_pads()
            .iter()
            .map(|p| p.get_name())
            .collect::<Vec<GString>>();
        pad_names.sort();
        assert_eq!(pad_names, vec![String::from("src")]);
    }

    #[test]
    #[cfg(feature = "v1_14")]
    fn test_foreach_pad() {
        ::init().unwrap();

        let identity = ::ElementFactory::make("identity", None).unwrap();

        let mut pad_names = Vec::new();
        identity.foreach_pad(|_element, pad| {
            pad_names.push(pad.get_name());

            true
        });
        pad_names.sort();
        assert_eq!(pad_names, vec![String::from("sink"), String::from("src")]);
    }

    #[cfg(feature = "v1_10")]
    #[test]
    fn test_call_async() {
        ::init().unwrap();

        let identity = ::ElementFactory::make("identity", None).unwrap();
        let (sender, receiver) = channel();

        identity.call_async(move |_| {
            sender.send(()).unwrap();
        });

        assert_eq!(receiver.recv(), Ok(()));
    }
}
