// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use gst_sys;
use std::mem;
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

#[cfg(any(feature = "v1_12", feature = "dox"))]
pub fn calculate_linear_regression(
    xy: &[(u64, u64)],
    temp: Option<&mut [(u64, u64)]>,
) -> Option<(u64, u64, u64, u64, f64)> {
    unsafe {
        assert_eq!(mem::size_of::<u64>() * 2, mem::size_of::<(u64, u64)>());
        assert_eq!(mem::align_of::<u64>(), mem::align_of::<(u64, u64)>());
        assert!(temp.as_ref().map(|temp| temp.len()).unwrap_or(xy.len()) >= xy.len());

        let mut m_num = mem::uninitialized();
        let mut m_denom = mem::uninitialized();
        let mut b = mem::uninitialized();
        let mut xbase = mem::uninitialized();
        let mut r_squared = mem::uninitialized();

        let res = from_glib(gst_sys::gst_calculate_linear_regression(
            xy.as_ptr() as *const u64,
            temp.map(|temp| temp.as_mut_ptr() as *mut u64)
                .unwrap_or(ptr::null_mut()),
            xy.len() as u32,
            &mut m_num,
            &mut m_denom,
            &mut b,
            &mut xbase,
            &mut r_squared,
        ));
        if res {
            Some((m_num, m_denom, b, xbase, r_squared))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_linear_regression() {
        ::init().unwrap();

        let values = [(0, 0), (1, 1), (2, 2), (3, 3)];

        let (m_num, m_denom, b, xbase, _) = calculate_linear_regression(&values, None).unwrap();
        assert_eq!((m_num, m_denom, b, xbase), (10, 10, 3, 3));

        let mut temp = [(0, 0); 4];
        let (m_num, m_denom, b, xbase, _) =
            calculate_linear_regression(&values, Some(&mut temp)).unwrap();
        assert_eq!((m_num, m_denom, b, xbase), (10, 10, 3, 3));
    }
}
