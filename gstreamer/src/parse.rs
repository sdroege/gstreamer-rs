// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use glib::{prelude::*, translate::*};

use crate::{Bin, Element, Object, ParseContext, ParseFlags};

pub use crate::auto::functions::parse_bin_from_description as bin_from_description;
pub use crate::auto::functions::parse_launch as launch;
pub use crate::auto::functions::parse_launchv as launchv;

#[doc(alias = "gst_parse_bin_from_description_full")]
pub fn bin_from_description_with_name(
    bin_description: &str,
    ghost_unlinked_pads: bool,
    bin_name: &str,
) -> Result<Bin, glib::Error> {
    skip_assert_initialized!();
    let bin = bin_from_description(bin_description, ghost_unlinked_pads)?;
    if !bin_name.is_empty() {
        let obj = bin.clone().upcast::<Object>();
        unsafe {
            ffi::gst_object_set_name(obj.to_glib_none().0, bin_name.to_glib_none().0);
        }
    }
    Ok(bin)
}

#[doc(alias = "gst_parse_bin_from_description_full")]
pub fn bin_from_description_full(
    bin_description: &str,
    ghost_unlinked_pads: bool,
    mut context: Option<&mut ParseContext>,
    flags: ParseFlags,
) -> Result<Element, glib::Error> {
    skip_assert_initialized!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_parse_bin_from_description_full(
            bin_description.to_glib_none().0,
            ghost_unlinked_pads.into_glib(),
            context.to_glib_none_mut().0,
            flags.into_glib(),
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_none(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "gst_parse_bin_from_description_full")]
pub fn bin_from_description_with_name_full(
    bin_description: &str,
    ghost_unlinked_pads: bool,
    bin_name: &str,
    context: Option<&mut ParseContext>,
    flags: ParseFlags,
) -> Result<Element, glib::Error> {
    skip_assert_initialized!();
    let bin = bin_from_description_full(bin_description, ghost_unlinked_pads, context, flags)?;
    if !bin_name.is_empty() {
        let obj = bin.clone().upcast::<Object>();
        unsafe {
            ffi::gst_object_set_name(obj.to_glib_none().0, bin_name.to_glib_none().0);
        }
    }
    Ok(bin)
}

#[doc(alias = "gst_parse_launch_full")]
pub fn launch_full(
    pipeline_description: &str,
    mut context: Option<&mut ParseContext>,
    flags: ParseFlags,
) -> Result<Element, glib::Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_parse_launch_full(
            pipeline_description.to_glib_none().0,
            context.to_glib_none_mut().0,
            flags.into_glib(),
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_none(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "gst_parse_launchv_full")]
pub fn launchv_full(
    argv: &[&str],
    mut context: Option<&mut ParseContext>,
    flags: ParseFlags,
) -> Result<Element, glib::Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_parse_launchv_full(
            argv.to_glib_none().0,
            context.to_glib_none_mut().0,
            flags.into_glib(),
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_none(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_parse_bin_from_description_with_name() {
        crate::init().unwrap();

        let bin = bin_from_description_with_name("fakesrc ! fakesink", false, "all_fake").unwrap();
        let name = bin.name();
        assert_eq!(name, "all_fake");

        let bin = bin_from_description_with_name("fakesrc ! fakesink", false, "").unwrap();
        let name = bin.name();
        assert_ne!(name, "");
    }
}
