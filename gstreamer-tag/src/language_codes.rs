// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ffi;
use glib::translate::*;

#[doc(alias = "gst_tag_get_language_codes")]
pub fn language_codes() -> glib::collections::StrV {
    skip_assert_initialized!();

    unsafe { glib::collections::StrV::from_glib_full(ffi::gst_tag_get_language_codes()) }
}

#[doc(alias = "gst_tag_get_language_name")]
pub fn language_name<'a>(language_code: &str) -> Option<&'a glib::GStr> {
    skip_assert_initialized!();

    unsafe {
        let ptr = language_code
            .run_with_gstr(|language_code| ffi::gst_tag_get_language_name(language_code.as_ptr()));

        if ptr.is_null() {
            None
        } else {
            Some(glib::GStr::from_ptr(ptr))
        }
    }
}

#[doc(alias = "gst_tag_get_language_code_iso_639_1")]
pub fn language_code_iso_639_1<'a>(language_code: &str) -> Option<&'a glib::GStr> {
    skip_assert_initialized!();

    unsafe {
        let ptr = language_code.run_with_gstr(|language_code| {
            ffi::gst_tag_get_language_code_iso_639_1(language_code.as_ptr())
        });

        if ptr.is_null() {
            None
        } else {
            Some(glib::GStr::from_ptr(ptr))
        }
    }
}

#[doc(alias = "gst_tag_get_language_code_iso_639_2T")]
pub fn language_code_iso_639_2t<'a>(language_code: &str) -> Option<&'a glib::GStr> {
    skip_assert_initialized!();

    unsafe {
        let ptr = language_code.run_with_gstr(|language_code| {
            ffi::gst_tag_get_language_code_iso_639_2T(language_code.as_ptr())
        });

        if ptr.is_null() {
            None
        } else {
            Some(glib::GStr::from_ptr(ptr))
        }
    }
}

#[doc(alias = "gst_tag_get_language_code_iso_639_2B")]
pub fn language_code_iso_639_2b<'a>(language_code: &str) -> Option<&'a glib::GStr> {
    skip_assert_initialized!();

    unsafe {
        let ptr = language_code.run_with_gstr(|language_code| {
            ffi::gst_tag_get_language_code_iso_639_2B(language_code.as_ptr())
        });

        if ptr.is_null() {
            None
        } else {
            Some(glib::GStr::from_ptr(ptr))
        }
    }
}

#[doc(alias = "gst_tag_check_language_code")]
pub fn check_language_code(language_code: &str) -> bool {
    skip_assert_initialized!();

    unsafe {
        from_glib(language_code.run_with_gstr(|language_code| {
            ffi::gst_tag_check_language_code(language_code.as_ptr())
        }))
    }
}
