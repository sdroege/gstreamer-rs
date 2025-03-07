// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{ffi, Asset, Extractable, Timeline};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GESFormatter")]
    pub struct Formatter(Object<ffi::GESFormatter, ffi::GESFormatterClass>) @implements Extractable;

    match fn {
        type_ => || ffi::ges_formatter_get_type(),
    }
}

impl Formatter {
    pub const NONE: Option<&'static Formatter> = None;

    #[doc(alias = "ges_formatter_can_load_uri")]
    pub fn can_load_uri(uri: &str) -> Result<(), glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::ges_formatter_can_load_uri(uri.to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "ges_formatter_can_save_uri")]
    pub fn can_save_uri(uri: &str) -> Result<(), glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::ges_formatter_can_save_uri(uri.to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "ges_formatter_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Asset {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_formatter_get_default()) }
    }
}

pub trait FormatterExt: IsA<Formatter> + 'static {
    #[cfg_attr(feature = "v1_18", deprecated = "Since 1.18")]
    #[allow(deprecated)]
    #[doc(alias = "ges_formatter_load_from_uri")]
    fn load_from_uri(&self, timeline: &impl IsA<Timeline>, uri: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::ges_formatter_load_from_uri(
                self.as_ref().to_glib_none().0,
                timeline.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg_attr(feature = "v1_18", deprecated = "Since 1.18")]
    #[allow(deprecated)]
    #[doc(alias = "ges_formatter_save_to_uri")]
    fn save_to_uri(
        &self,
        timeline: &impl IsA<Timeline>,
        uri: &str,
        overwrite: bool,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::ges_formatter_save_to_uri(
                self.as_ref().to_glib_none().0,
                timeline.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                overwrite.into_glib(),
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl<O: IsA<Formatter>> FormatterExt for O {}
