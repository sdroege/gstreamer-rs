// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Element;

use glib;
use glib::IsA;
use glib::translate::{from_glib, from_glib_full, from_glib_none, FromGlibPtrContainer, ToGlibPtr};
use QueryRef;
use Event;
use Pad;
use PadTemplate;
use miniobject::MiniObject;

use std::ffi::CStr;

use ffi;
use gobject_ffi;

impl Element {
    pub fn link_many<E: IsA<Element>>(elements: &[&E]) -> Result<(), glib::BoolError> {
        skip_assert_initialized!();
        for (e1, e2) in elements.iter().zip(elements.iter().skip(1)) {
            unsafe {
                let ret: bool = from_glib(ffi::gst_element_link(
                    e1.to_glib_none().0,
                    e2.to_glib_none().0,
                ));
                if !ret {
                    return Err(glib::BoolError("Failed to link elements"));
                }
            }
        }

        Ok(())
    }

    pub fn unlink_many<E: IsA<Element>>(elements: &[&E]) {
        skip_assert_initialized!();
        for (e1, e2) in elements.iter().zip(elements.iter().skip(1)) {
            unsafe {
                ffi::gst_element_unlink(e1.to_glib_none().0, e2.to_glib_none().0);
            }
        }
    }
}

pub enum ElementMessageType {
    Error,
    Warning,
    Info,
}

pub trait ElementExtManual {
    fn query(&self, query: &mut QueryRef) -> bool;

    fn send_event(&self, event: Event) -> bool;

    fn get_metadata<'a>(&self, key: &str) -> Option<&'a str>;

    fn get_pad_template(&self, name: &str) -> Option<PadTemplate>;
    fn get_pad_template_list(&self) -> Vec<PadTemplate>;

    fn message_full<T: ::MessageErrorDomain>(
        &self,
        type_: ElementMessageType,
        code: T,
        message: Option<&str>,
        debug: Option<&str>,
        file: &str,
        function: &str,
        line: i32,
    );
    #[cfg(feature = "v1_10")]
    fn message_full_with_details<T: ::MessageErrorDomain>(
        &self,
        type_: ElementMessageType,
        code: T,
        message: Option<&str>,
        debug: Option<&str>,
        file: &str,
        function: &str,
        line: i32,
        structure: ::Structure,
    );

    fn iterate_pads(&self) -> ::Iterator<Pad>;
    fn iterate_sink_pads(&self) -> ::Iterator<Pad>;
    fn iterate_src_pads(&self) -> ::Iterator<Pad>;

    fn get_pads(&self) -> Vec<Pad>;
    fn get_sink_pads(&self) -> Vec<Pad>;
    fn get_src_pads(&self) -> Vec<Pad>;
}

impl<O: IsA<Element>> ElementExtManual for O {
    fn query(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(ffi::gst_element_query(
                self.to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn send_event(&self, event: Event) -> bool {
        unsafe {
            from_glib(ffi::gst_element_send_event(
                self.to_glib_none().0,
                event.into_ptr(),
            ))
        }
    }

    fn get_metadata<'a>(&self, key: &str) -> Option<&'a str> {
        unsafe {
            let klass = (*(self.to_glib_none().0 as *mut gobject_ffi::GTypeInstance)).g_class
                as *mut ffi::GstElementClass;

            let ptr = ffi::gst_element_class_get_metadata(klass, key.to_glib_none().0);

            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    fn get_pad_template(&self, name: &str) -> Option<PadTemplate> {
        unsafe {
            let klass = (*(self.to_glib_none().0 as *mut gobject_ffi::GTypeInstance)).g_class
                as *mut ffi::GstElementClass;

            from_glib_none(ffi::gst_element_class_get_pad_template(
                klass,
                name.to_glib_none().0,
            ))
        }
    }

    fn get_pad_template_list(&self) -> Vec<PadTemplate> {
        unsafe {
            let klass = (*(self.to_glib_none().0 as *mut gobject_ffi::GTypeInstance)).g_class
                as *mut ffi::GstElementClass;

            FromGlibPtrContainer::from_glib_none(
                ffi::gst_element_class_get_pad_template_list(klass),
            )
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
        line: i32,
    ) {
        unsafe {
            let type_ = match type_ {
                ElementMessageType::Error => ffi::GST_MESSAGE_ERROR,
                ElementMessageType::Warning => ffi::GST_MESSAGE_WARNING,
                ElementMessageType::Info => ffi::GST_MESSAGE_INFO,
            };

            ffi::gst_element_message_full(
                self.to_glib_none().0,
                type_,
                T::domain(),
                code.code(),
                message.to_glib_full(),
                debug.to_glib_full(),
                file.to_glib_none().0,
                function.to_glib_none().0,
                line,
            );
        }
    }

    #[cfg(feature = "v1_10")]
    fn message_full_with_details<T: ::MessageErrorDomain>(
        &self,
        type_: ElementMessageType,
        code: T,
        message: Option<&str>,
        debug: Option<&str>,
        file: &str,
        function: &str,
        line: i32,
        structure: ::Structure,
    ) {
        unsafe {
            let type_ = match type_ {
                ElementMessageType::Error => ffi::GST_MESSAGE_ERROR,
                ElementMessageType::Warning => ffi::GST_MESSAGE_WARNING,
                ElementMessageType::Info => ffi::GST_MESSAGE_INFO,
            };

            ffi::gst_element_message_full_with_details(
                self.to_glib_none().0,
                type_,
                T::domain(),
                code.code(),
                message.to_glib_full(),
                debug.to_glib_full(),
                file.to_glib_none().0,
                function.to_glib_none().0,
                line,
                structure.into_ptr(),
            );
        }
    }

    fn iterate_pads(&self) -> ::Iterator<Pad> {
        unsafe { from_glib_full(ffi::gst_element_iterate_pads(self.to_glib_none().0)) }
    }

    fn iterate_sink_pads(&self) -> ::Iterator<Pad> {
        unsafe { from_glib_full(ffi::gst_element_iterate_sink_pads(self.to_glib_none().0)) }
    }

    fn iterate_src_pads(&self) -> ::Iterator<Pad> {
        unsafe { from_glib_full(ffi::gst_element_iterate_src_pads(self.to_glib_none().0)) }
    }

    fn get_pads(&self) -> Vec<Pad> {
        unsafe {
            let stash = self.to_glib_none();
            let elt: &ffi::GstElement = &*stash.0;
            ::utils::MutexGuard::lock(&elt.object.lock);
            FromGlibPtrContainer::from_glib_none(elt.pads)
        }
    }

    fn get_sink_pads(&self) -> Vec<Pad> {
        unsafe {
            let stash = self.to_glib_none();
            let elt: &ffi::GstElement = &*stash.0;
            ::utils::MutexGuard::lock(&elt.object.lock);
            FromGlibPtrContainer::from_glib_none(elt.sinkpads)
        }
    }

    fn get_src_pads(&self) -> Vec<Pad> {
        unsafe {
            let stash = self.to_glib_none();
            let elt: &ffi::GstElement = &*stash.0;
            ::utils::MutexGuard::lock(&elt.object.lock);
            FromGlibPtrContainer::from_glib_none(elt.srcpads)
        }
    }
}

lazy_static!{
    pub static ref ELEMENT_METADATA_AUTHOR: &'static str = unsafe { CStr::from_ptr(ffi::GST_ELEMENT_METADATA_AUTHOR).to_str().unwrap() };
    pub static ref ELEMENT_METADATA_DESCRIPTION: &'static str = unsafe { CStr::from_ptr(ffi::GST_ELEMENT_METADATA_DESCRIPTION).to_str().unwrap() };
    pub static ref ELEMENT_METADATA_DOC_URI: &'static str = unsafe { CStr::from_ptr(ffi::GST_ELEMENT_METADATA_DOC_URI).to_str().unwrap() };
    pub static ref ELEMENT_METADATA_ICON_NAME: &'static str = unsafe { CStr::from_ptr(ffi::GST_ELEMENT_METADATA_ICON_NAME).to_str().unwrap() };
    pub static ref ELEMENT_METADATA_KLASS: &'static str = unsafe { CStr::from_ptr(ffi::GST_ELEMENT_METADATA_KLASS).to_str().unwrap() };
    pub static ref ELEMENT_METADATA_LONGNAME: &'static str = unsafe { CStr::from_ptr(ffi::GST_ELEMENT_METADATA_LONGNAME).to_str().unwrap() };
}
