// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use gst_sys;
use std::ptr;

use Element;
use Error;
use ParseContext;
use ParseFlags;

pub fn parse_bin_from_description_full(
    bin_description: &str,
    ghost_unlinked_pads: bool,
    mut context: Option<&mut ParseContext>,
    flags: ParseFlags,
) -> Result<Element, Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = gst_sys::gst_parse_bin_from_description_full(
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

pub fn parse_launch_full(
    pipeline_description: &str,
    mut context: Option<&mut ParseContext>,
    flags: ParseFlags,
) -> Result<Element, Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = gst_sys::gst_parse_launch_full(
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

pub fn parse_launchv_full(
    argv: &[&str],
    mut context: Option<&mut ParseContext>,
    flags: ParseFlags,
) -> Result<Element, Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = gst_sys::gst_parse_launchv_full(
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

pub fn util_group_id_next() -> ::GroupId {
    assert_initialized_main_thread!();
    unsafe {
        let v = from_glib(gst_sys::gst_util_group_id_next());
        if v == ::GROUP_ID_INVALID {
            return from_glib(gst_sys::gst_util_group_id_next());
        }
        v
    }
}

pub fn util_seqnum_next() -> ::Seqnum {
    assert_initialized_main_thread!();
    unsafe {
        let v = from_glib(gst_sys::gst_util_seqnum_next());
        if v == ::SEQNUM_INVALID {
            return from_glib(gst_sys::gst_util_seqnum_next());
        }
        v
    }
}
