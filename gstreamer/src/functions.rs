// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib::translate::*;
use std::ptr;

use ParseContext;
use ParseFlags;
use Element;
use Error;

pub fn parse_bin_from_description_full<'a, P: Into<Option<&'a mut ParseContext>>>(
    bin_description: &str,
    ghost_unlinked_pads: bool,
    context: P,
    flags: ParseFlags,
) -> Result<Element, Error> {
    assert_initialized_main_thread!();
    let mut context = context.into();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_parse_bin_from_description_full(
            bin_description.to_glib_none().0,
            ghost_unlinked_pads.to_glib(),
            context.to_glib_none_mut().0,
            flags.to_glib(),
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_none(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn parse_launch_full<'a, P: Into<Option<&'a mut ParseContext>>>(
    pipeline_description: &str,
    context: P,
    flags: ParseFlags,
) -> Result<Element, Error> {
    assert_initialized_main_thread!();
    let mut context = context.into();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_parse_launch_full(
            pipeline_description.to_glib_none().0,
            context.to_glib_none_mut().0,
            flags.to_glib(),
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_none(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn parse_launchv_full<'a, P: Into<Option<&'a mut ParseContext>>>(
    argv: &[&str],
    context: P,
    flags: ParseFlags,
) -> Result<Element, Error> {
    assert_initialized_main_thread!();
    let mut context = context.into();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_parse_launchv_full(
            argv.to_glib_none().0,
            context.to_glib_none_mut().0,
            flags.to_glib(),
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_none(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}
