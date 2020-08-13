// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use auto::functions::parse_bin_from_description;
use glib;
use glib::translate::*;
use glib::Cast;
use gst_sys;
use std::ptr;

use Bin;
use Element;
use Object;
use ParseContext;
use ParseFlags;

pub fn parse_bin_from_description_with_name(
    bin_description: &str,
    ghost_unlinked_pads: bool,
    bin_name: &str,
) -> Result<Bin, glib::Error> {
    assert_initialized_main_thread!();
    let bin = parse_bin_from_description(bin_description, ghost_unlinked_pads)?;
    if !bin_name.is_empty() {
        let obj = bin.clone().upcast::<Object>();
        unsafe {
            gst_sys::gst_object_set_name(obj.to_glib_none().0, bin_name.to_glib_none().0);
        }
    }
    Ok(bin)
}

pub fn parse_bin_from_description_full(
    bin_description: &str,
    ghost_unlinked_pads: bool,
    mut context: Option<&mut ParseContext>,
    flags: ParseFlags,
) -> Result<Element, glib::Error> {
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

pub fn parse_bin_from_description_with_name_full(
    bin_description: &str,
    ghost_unlinked_pads: bool,
    bin_name: &str,
    context: Option<&mut ParseContext>,
    flags: ParseFlags,
) -> Result<Element, glib::Error> {
    assert_initialized_main_thread!();
    let bin =
        parse_bin_from_description_full(bin_description, ghost_unlinked_pads, context, flags)?;
    if !bin_name.is_empty() {
        let obj = bin.clone().upcast::<Object>();
        unsafe {
            gst_sys::gst_object_set_name(obj.to_glib_none().0, bin_name.to_glib_none().0);
        }
    }
    Ok(bin)
}

pub fn parse_launch_full(
    pipeline_description: &str,
    mut context: Option<&mut ParseContext>,
    flags: ParseFlags,
) -> Result<Element, glib::Error> {
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
) -> Result<Element, glib::Error> {
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

#[cfg(any(feature = "v1_12", feature = "dox"))]
pub fn calculate_linear_regression(
    xy: &[(u64, u64)],
    temp: Option<&mut [(u64, u64)]>,
) -> Option<(u64, u64, u64, u64, f64)> {
    skip_assert_initialized!();
    use std::mem;

    unsafe {
        assert_eq!(mem::size_of::<u64>() * 2, mem::size_of::<(u64, u64)>());
        assert_eq!(mem::align_of::<u64>(), mem::align_of::<(u64, u64)>());
        assert!(
            temp.as_ref()
                .map(|temp| temp.len())
                .unwrap_or_else(|| xy.len())
                >= xy.len()
        );

        let mut m_num = mem::MaybeUninit::uninit();
        let mut m_denom = mem::MaybeUninit::uninit();
        let mut b = mem::MaybeUninit::uninit();
        let mut xbase = mem::MaybeUninit::uninit();
        let mut r_squared = mem::MaybeUninit::uninit();

        let res = from_glib(gst_sys::gst_calculate_linear_regression(
            xy.as_ptr() as *const u64,
            temp.map(|temp| temp.as_mut_ptr() as *mut u64)
                .unwrap_or(ptr::null_mut()),
            xy.len() as u32,
            m_num.as_mut_ptr(),
            m_denom.as_mut_ptr(),
            b.as_mut_ptr(),
            xbase.as_mut_ptr(),
            r_squared.as_mut_ptr(),
        ));
        if res {
            Some((
                m_num.assume_init(),
                m_denom.assume_init(),
                b.assume_init(),
                xbase.assume_init(),
                r_squared.assume_init(),
            ))
        } else {
            None
        }
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
pub fn type_is_plugin_api(type_: glib::types::Type) -> Option<::PluginAPIFlags> {
    assert_initialized_main_thread!();
    unsafe {
        use std::mem;

        let mut flags = mem::MaybeUninit::uninit();
        let ret = from_glib(gst_sys::gst_type_is_plugin_api(
            type_.to_glib(),
            flags.as_mut_ptr(),
        ));
        let flags = flags.assume_init();
        if ret {
            Some(from_glib(flags))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prelude::*;

    #[cfg(feature = "v1_12")]
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

    #[test]
    fn test_parse_bin_from_description_with_name() {
        ::init().unwrap();

        let bin =
            parse_bin_from_description_with_name("fakesrc ! fakesink", false, "all_fake").unwrap();
        let name = bin.get_name();
        assert_eq!(name, "all_fake");

        let bin = parse_bin_from_description_with_name("fakesrc ! fakesink", false, "").unwrap();
        let name = bin.get_name();
        assert_ne!(name, "");
    }
}
