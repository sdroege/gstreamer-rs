// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc::c_char;
use std::ffi::CStr;
use std::fmt;
use std::ptr;

use ffi;
use gobject_ffi;

use glib::translate::{from_glib, ToGlib, ToGlibPtr};
use glib::IsA;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct DebugCategory(ptr::NonNull<ffi::GstDebugCategory>);

#[cfg_attr(feature = "cargo-clippy", allow(trivially_copy_pass_by_ref))]
impl DebugCategory {
    pub fn new<'a, P: Into<Option<&'a str>>>(
        name: &str,
        color: ::DebugColorFlags,
        description: P,
    ) -> DebugCategory {
        extern "C" {
            fn _gst_debug_category_new(
                name: *const c_char,
                color: ffi::GstDebugColorFlags,
                description: *const c_char,
            ) -> *mut ffi::GstDebugCategory;
        }
        let description = description.into();

        // Gets the category if it exists already
        unsafe {
            let ptr = _gst_debug_category_new(
                name.to_glib_none().0,
                color.to_glib(),
                description.to_glib_none().0,
            );
            assert!(!ptr.is_null());
            DebugCategory(ptr::NonNull::new_unchecked(ptr))
        }
    }

    pub fn get(name: &str) -> Option<DebugCategory> {
        unsafe {
            extern "C" {
                fn _gst_debug_get_category(name: *const c_char) -> *mut ffi::GstDebugCategory;
            }

            let cat = _gst_debug_get_category(name.to_glib_none().0);

            if cat.is_null() {
                None
            } else {
                Some(DebugCategory(ptr::NonNull::new_unchecked(cat)))
            }
        }
    }

    pub fn get_threshold(&self) -> ::DebugLevel {
        from_glib(unsafe { ffi::gst_debug_category_get_threshold(self.0.as_ptr()) })
    }

    pub fn set_threshold(&self, threshold: ::DebugLevel) {
        unsafe { ffi::gst_debug_category_set_threshold(self.0.as_ptr(), threshold.to_glib()) }
    }

    pub fn reset_threshold(&self) {
        unsafe { ffi::gst_debug_category_reset_threshold(self.0.as_ptr()) }
    }

    pub fn get_color(&self) -> ::DebugColorFlags {
        unsafe { from_glib(ffi::gst_debug_category_get_color(self.0.as_ptr())) }
    }

    pub fn get_name(&self) -> &str {
        unsafe {
            CStr::from_ptr(ffi::gst_debug_category_get_name(self.0.as_ptr()))
                .to_str()
                .unwrap()
        }
    }

    pub fn get_description(&self) -> Option<&str> {
        unsafe {
            let ptr = ffi::gst_debug_category_get_name(self.0.as_ptr());

            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    #[inline]
    pub fn log<O: IsA<::Object>>(
        &self,
        obj: Option<&O>,
        level: ::DebugLevel,
        file: &str,
        module: &str,
        line: u32,
        args: fmt::Arguments,
    ) {
        unsafe {
            if level.to_glib() as i32 > self.0.as_ref().threshold {
                return;
            }
        }

        let obj_ptr = match obj {
            Some(obj) => obj.to_glib_none().0 as *mut gobject_ffi::GObject,
            None => ptr::null_mut(),
        };

        unsafe {
            ffi::gst_debug_log(
                self.0.as_ptr(),
                level.to_glib(),
                file.to_glib_none().0,
                module.to_glib_none().0,
                line as i32,
                obj_ptr,
                fmt::format(args).to_glib_none().0,
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

#[macro_export]
macro_rules! gst_error(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat, level: $crate::DebugLevel::Error, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat, level: $crate::DebugLevel::Error, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_warning(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat, level: $crate::DebugLevel::Warning, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat, level: $crate::DebugLevel::Warning, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_fixme(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat, level: $crate::DebugLevel::Fixme, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat, level: $crate::DebugLevel::Fixme, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_info(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat, level: $crate::DebugLevel::Info, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat, level: $crate::DebugLevel::Info, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_debug(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat, level: $crate::DebugLevel::Debug, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat, level: $crate::DebugLevel::Debug, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_log(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat, level: $crate::DebugLevel::Log, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat, level: $crate::DebugLevel::Log, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_trace(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat, level: $crate::DebugLevel::Trace, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat, level: $crate::DebugLevel::Trace, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_memdump(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat, level: $crate::DebugLevel::Memdump, obj: $obj, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        gst_log_with_level!($cat, level: $crate::DebugLevel::Memdump, $($args)*)
    }};
);

#[macro_export]
macro_rules! gst_log_with_level(
    ($cat:expr, level: $level:expr, obj: $obj:expr, $($args:tt)*) => { {
        $crate::DebugCategory::log(&$cat, Some($obj), $level, file!(),
            module_path!(), line!(), format_args!($($args)*))
    }};
    ($cat:expr, level: $level:expr, $($args:tt)*) => { {
        $crate::DebugCategory::log(&$cat, None as Option<&$crate::Object>, $level, file!(),
            module_path!(), line!(), format_args!($($args)*))
    }};
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_existing() {
        ::init().unwrap();

        assert_ne!(DebugCategory::get("GST_PERFORMANCE"), None);
    }

    #[test]
    fn new_and_log() {
        ::init().unwrap();

        let cat = DebugCategory::new(
            "test-cat",
            ::DebugColorFlags::empty(),
            "some debug category",
        );

        gst_error!(cat, "meh");
        gst_warning!(cat, "meh");
        gst_fixme!(cat, "meh");
        gst_info!(cat, "meh");
        gst_debug!(cat, "meh");
        gst_log!(cat, "meh");
        gst_trace!(cat, "meh");
        gst_memdump!(cat, "meh");

        let obj = ::Bin::new("meh");
        gst_error!(cat, obj: &obj, "meh");
        gst_warning!(cat, obj: &obj, "meh");
        gst_fixme!(cat, obj: &obj, "meh");
        gst_info!(cat, obj: &obj, "meh");
        gst_debug!(cat, obj: &obj, "meh");
        gst_log!(cat, obj: &obj, "meh");
        gst_trace!(cat, obj: &obj, "meh");
        gst_memdump!(cat, obj: &obj, "meh");
    }
}
