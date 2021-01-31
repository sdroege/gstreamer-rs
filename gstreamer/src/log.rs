// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use DebugLevel;

use libc::c_char;
use std::borrow::Cow;
use std::ffi::CStr;
use std::fmt;
use std::ptr;

use once_cell::sync::Lazy;

use gobject_sys;
use gst_sys;

use glib::translate::*;
use glib::IsA;
use glib_sys::gpointer;

#[derive(PartialEq, Eq)]
pub struct DebugMessage(ptr::NonNull<gst_sys::GstDebugMessage>);

impl fmt::Debug for DebugMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("DebugMessage").field(&self.get()).finish()
    }
}

impl DebugMessage {
    pub fn get(&self) -> Option<Cow<str>> {
        unsafe {
            let message = gst_sys::gst_debug_message_get(self.0.as_ptr());

            if message.is_null() {
                None
            } else {
                Some(CStr::from_ptr(message).to_string_lossy())
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct DebugCategory(Option<ptr::NonNull<gst_sys::GstDebugCategory>>);

impl DebugCategory {
    pub fn new(name: &str, color: ::DebugColorFlags, description: Option<&str>) -> DebugCategory {
        skip_assert_initialized!();
        extern "C" {
            fn _gst_debug_category_new(
                name: *const c_char,
                color: gst_sys::GstDebugColorFlags,
                description: *const c_char,
            ) -> *mut gst_sys::GstDebugCategory;
        }

        // Gets the category if it exists already
        unsafe {
            let ptr = _gst_debug_category_new(
                name.to_glib_none().0,
                color.to_glib(),
                description.to_glib_none().0,
            );
            // Can be NULL if the debug system is compiled out
            DebugCategory(ptr::NonNull::new(ptr))
        }
    }

    pub fn get(name: &str) -> Option<DebugCategory> {
        skip_assert_initialized!();
        unsafe {
            extern "C" {
                fn _gst_debug_get_category(name: *const c_char) -> *mut gst_sys::GstDebugCategory;
            }

            let cat = _gst_debug_get_category(name.to_glib_none().0);

            if cat.is_null() {
                None
            } else {
                Some(DebugCategory(Some(ptr::NonNull::new_unchecked(cat))))
            }
        }
    }

    pub fn get_threshold(self) -> ::DebugLevel {
        match self.0 {
            Some(cat) => unsafe {
                from_glib(gst_sys::gst_debug_category_get_threshold(cat.as_ptr()))
            },
            None => ::DebugLevel::None,
        }
    }

    pub fn set_threshold(self, threshold: ::DebugLevel) {
        if let Some(cat) = self.0 {
            unsafe { gst_sys::gst_debug_category_set_threshold(cat.as_ptr(), threshold.to_glib()) }
        }
    }

    pub fn reset_threshold(self) {
        if let Some(cat) = self.0 {
            unsafe { gst_sys::gst_debug_category_reset_threshold(cat.as_ptr()) }
        }
    }

    pub fn get_color(self) -> ::DebugColorFlags {
        match self.0 {
            Some(cat) => unsafe { from_glib(gst_sys::gst_debug_category_get_color(cat.as_ptr())) },
            None => ::DebugColorFlags::empty(),
        }
    }

    pub fn get_name<'a>(self) -> &'a str {
        match self.0 {
            Some(cat) => unsafe {
                CStr::from_ptr(gst_sys::gst_debug_category_get_name(cat.as_ptr()))
                    .to_str()
                    .unwrap()
            },
            None => "",
        }
    }

    pub fn get_description<'a>(self) -> Option<&'a str> {
        match self.0 {
            Some(cat) => unsafe {
                let ptr = gst_sys::gst_debug_category_get_description(cat.as_ptr());

                if ptr.is_null() {
                    None
                } else {
                    Some(CStr::from_ptr(ptr).to_str().unwrap())
                }
            },
            None => None,
        }
    }

    #[inline]
    pub fn log<O: IsA<glib::Object>>(
        self,
        obj: Option<&O>,
        level: ::DebugLevel,
        file: &str,
        module: &str,
        line: u32,
        args: fmt::Arguments,
    ) {
        let cat = match self.0 {
            Some(cat) => cat,
            None => return,
        };

        unsafe {
            if level.to_glib() as i32 > cat.as_ref().threshold {
                return;
            }
        }

        let obj_ptr = match obj {
            Some(obj) => obj.to_glib_none().0 as *mut gobject_sys::GObject,
            None => ptr::null_mut(),
        };

        unsafe {
            gst_sys::gst_debug_log(
                cat.as_ptr(),
                level.to_glib(),
                file.to_glib_none().0,
                module.to_glib_none().0,
                line as i32,
                obj_ptr,
                fmt::format(args).replace("%", "%%").to_glib_none().0,
            );
        }
    }
}

unsafe impl Sync for DebugCategory {}
unsafe impl Send for DebugCategory {}

impl fmt::Debug for DebugCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("DebugCategory")
            .field(&self.get_name())
            .finish()
    }
}

pub static CAT_RUST: Lazy<DebugCategory> = Lazy::new(|| {
    DebugCategory::new(
        "GST_RUST",
        ::DebugColorFlags::UNDERLINE,
        Some("GStreamer's Rust binding core"),
    )
});

macro_rules! declare_debug_category_from_name(
    ($cat:ident, $cat_name:expr) => (
        pub static $cat: Lazy<DebugCategory> = Lazy::new(|| DebugCategory::get($cat_name)
            .expect(&format!("Unable to find `DebugCategory` with name {}", $cat_name)));
    );
);

declare_debug_category_from_name!(CAT_DEFAULT, "default");
declare_debug_category_from_name!(CAT_GST_INIT, "GST_INIT");
declare_debug_category_from_name!(CAT_MEMORY, "GST_MEMORY");
declare_debug_category_from_name!(CAT_PARENTAGE, "GST_PARENTAGE");
declare_debug_category_from_name!(CAT_STATES, "GST_STATES");
declare_debug_category_from_name!(CAT_SCHEDULING, "GST_SCHEDULING");
declare_debug_category_from_name!(CAT_BUFFER, "GST_BUFFER");
declare_debug_category_from_name!(CAT_BUFFER_LIST, "GST_BUFFER_LIST");
declare_debug_category_from_name!(CAT_BUS, "GST_BUS");
declare_debug_category_from_name!(CAT_CAPS, "GST_CAPS");
declare_debug_category_from_name!(CAT_CLOCK, "GST_CLOCK");
declare_debug_category_from_name!(CAT_ELEMENT_PADS, "GST_ELEMENT_PADS");
declare_debug_category_from_name!(CAT_PADS, "GST_PADS");
declare_debug_category_from_name!(CAT_PERFORMANCE, "GST_PERFORMANCE");
declare_debug_category_from_name!(CAT_PIPELINE, "GST_PIPELINE");
declare_debug_category_from_name!(CAT_PLUGIN_LOADING, "GST_PLUGIN_LOADING");
declare_debug_category_from_name!(CAT_PLUGIN_INFO, "GST_PLUGIN_INFO");
declare_debug_category_from_name!(CAT_PROPERTIES, "GST_PROPERTIES");
declare_debug_category_from_name!(CAT_NEGOTIATION, "GST_NEGOTIATION");
declare_debug_category_from_name!(CAT_REFCOUNTING, "GST_REFCOUNTING");
declare_debug_category_from_name!(CAT_ERROR_SYSTEM, "GST_ERROR_SYSTEM");
declare_debug_category_from_name!(CAT_EVENT, "GST_EVENT");
declare_debug_category_from_name!(CAT_MESSAGE, "GST_MESSAGE");
declare_debug_category_from_name!(CAT_PARAMS, "GST_PARAMS");
declare_debug_category_from_name!(CAT_CALL_TRACE, "GST_CALL_TRACE");
declare_debug_category_from_name!(CAT_SIGNAL, "GST_SIGNAL");
declare_debug_category_from_name!(CAT_PROBE, "GST_PROBE");
declare_debug_category_from_name!(CAT_REGISTRY, "GST_REGISTRY");
declare_debug_category_from_name!(CAT_QOS, "GST_QOS");
declare_debug_category_from_name!(CAT_META, "GST_META");
declare_debug_category_from_name!(CAT_LOCKING, "GST_LOCKING");
declare_debug_category_from_name!(CAT_CONTEXT, "GST_CONTEXT");

#[macro_export]
macro_rules! gst_error(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        $crate::gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Error, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Error, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_warning(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        $crate::gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Warning, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Warning, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_fixme(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        $crate::gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Fixme, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Fixme, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_info(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        $crate::gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Info, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Info, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_debug(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        $crate::gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Debug, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Debug, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_log(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        $crate::gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Log, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Log, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_trace(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        $crate::gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Trace, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Trace, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_memdump(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        $crate::gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Memdump, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::gst_log_with_level!($cat.clone(), level: $crate::DebugLevel::Memdump, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_log_with_level(
    ($cat:expr, level: $level:expr, obj: $obj:expr, $($args:tt)*) => { {
        $crate::DebugCategory::log($cat.clone(), Some($obj), $level, file!(),
            module_path!(), line!(), format_args!($($args)*))
    }};
    ($cat:expr, level: $level:expr, $($args:tt)*) => { {
        $crate::DebugCategory::log($cat.clone(), None as Option<&$crate::glib::Object>, $level, file!(),
            module_path!(), line!(), format_args!($($args)*))
    }};
);

unsafe extern "C" fn log_handler<T>(
    category: *mut gst_sys::GstDebugCategory,
    level: gst_sys::GstDebugLevel,
    file: *const c_char,
    function: *const c_char,
    line: i32,
    object: *mut gobject_sys::GObject,
    message: *mut gst_sys::GstDebugMessage,
    user_data: gpointer,
) where
    T: Fn(DebugCategory, DebugLevel, &str, &str, u32, Option<&glib::Object>, &DebugMessage)
        + Send
        + Sync
        + 'static,
{
    if category.is_null() {
        return;
    }
    let category = DebugCategory(Some(ptr::NonNull::new_unchecked(category)));
    let level = from_glib(level);
    let file = CStr::from_ptr(file).to_string_lossy();
    let function = CStr::from_ptr(function).to_string_lossy();
    let line = line as u32;
    let object: Borrowed<Option<glib::Object>> = from_glib_borrow(object);
    let message = DebugMessage(ptr::NonNull::new_unchecked(message));
    let handler = &*(user_data as *mut T);
    (handler)(
        category,
        level,
        &file,
        &function,
        line,
        object.as_ref().as_ref(),
        &message,
    );
}

unsafe extern "C" fn log_handler_data_free<T>(data: gpointer) {
    let data = Box::from_raw(data as *mut T);
    drop(data);
}

#[derive(Debug)]
pub struct DebugLogFunction(ptr::NonNull<std::os::raw::c_void>);

// The contained pointer is never dereferenced and has no thread affinity.
// It may be convenient to send it or share it between threads to allow cleaning
// up log functions from other threads than the one that created it.
unsafe impl Send for DebugLogFunction {}
unsafe impl Sync for DebugLogFunction {}

pub fn debug_add_log_function<T>(function: T) -> DebugLogFunction
where
    T: Fn(DebugCategory, DebugLevel, &str, &str, u32, Option<&glib::Object>, &DebugMessage)
        + Send
        + Sync
        + 'static,
{
    skip_assert_initialized!();
    unsafe {
        let user_data = Box::new(function);
        let user_data_ptr = Box::into_raw(user_data) as gpointer;
        gst_sys::gst_debug_add_log_function(
            Some(log_handler::<T>),
            user_data_ptr,
            Some(log_handler_data_free::<T>),
        );
        DebugLogFunction(ptr::NonNull::new_unchecked(user_data_ptr))
    }
}

pub fn debug_remove_default_log_function() {
    skip_assert_initialized!();
    unsafe {
        gst_sys::gst_debug_remove_log_function(None);
    }
}

pub fn debug_remove_log_function(log_fn: DebugLogFunction) {
    skip_assert_initialized!();
    unsafe {
        gst_sys::gst_debug_remove_log_function_by_data(log_fn.0.as_ptr());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::sync::{Arc, Mutex};

    #[test]
    fn get_existing() {
        ::init().unwrap();

        let perf_cat = DebugCategory::get("GST_PERFORMANCE")
            .expect("Unable to find `DebugCategory` with name \"GST_PERFORMANCE\"");
        assert_eq!(perf_cat.get_name(), CAT_PERFORMANCE.get_name());
    }

    #[test]
    fn new_and_log() {
        ::init().unwrap();

        let cat = DebugCategory::new(
            "test-cat",
            ::DebugColorFlags::empty(),
            Some("some debug category"),
        );

        gst_error!(cat, "meh");
        gst_warning!(cat, "meh");
        gst_fixme!(cat, "meh");
        gst_info!(cat, "meh");
        gst_debug!(cat, "meh");
        gst_log!(cat, "meh");
        gst_trace!(cat, "meh");
        gst_memdump!(cat, "meh");

        let obj = ::Bin::new(Some("meh"));
        gst_error!(cat, obj: &obj, "meh");
        gst_warning!(cat, obj: &obj, "meh");
        gst_fixme!(cat, obj: &obj, "meh");
        gst_info!(cat, obj: &obj, "meh");
        gst_debug!(cat, obj: &obj, "meh");
        gst_log!(cat, obj: &obj, "meh");
        gst_trace!(cat, obj: &obj, "meh");
        gst_memdump!(cat, obj: &obj, "meh");
    }

    #[test]
    fn log_handler() {
        ::init().unwrap();

        let cat = DebugCategory::new(
            "test-cat-log",
            ::DebugColorFlags::empty(),
            Some("some debug category"),
        );
        cat.set_threshold(DebugLevel::Info);
        let obj = ::Bin::new(Some("meh"));

        let (sender, receiver) = mpsc::channel();

        let sender = Arc::new(Mutex::new(sender));

        let handler = move |category: DebugCategory,
                            level: DebugLevel,
                            _file: &str,
                            _function: &str,
                            _line: u32,
                            _object: Option<&glib::Object>,
                            message: &DebugMessage| {
            let cat = DebugCategory::get("test-cat-log").unwrap();

            if category != cat {
                // This test can run in parallel with other tests, including new_and_log above.
                // We cannot be certain we only see our own messages.
                return;
            }

            assert_eq!(level, DebugLevel::Info);
            assert_eq!(&message.get().unwrap(), "meh");
            let _ = sender.lock().unwrap().send(());
        };

        debug_remove_default_log_function();
        let log_fn = debug_add_log_function(handler);
        gst_info!(cat, obj: &obj, "meh");

        receiver.recv().unwrap();

        debug_remove_log_function(log_fn);

        gst_info!(cat, obj: &obj, "meh2");
        assert!(receiver.recv().is_err());
    }
}
