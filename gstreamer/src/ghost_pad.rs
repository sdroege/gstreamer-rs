// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::object::IsA;
use glib::translate::*;
use gst_sys;
use GhostPad;
use Object;
use PadMode;

impl GhostPad {
    pub fn activate_mode_default<P: IsA<GhostPad>, Q: IsA<Object>>(
        pad: &P,
        parent: Option<&Q>,
        mode: PadMode,
        active: bool,
    ) -> Result<(), glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_ghost_pad_activate_mode_default(
                    pad.to_glib_none().0 as *mut gst_sys::GstPad,
                    parent.map(|p| p.as_ref()).to_glib_none().0,
                    mode.to_glib(),
                    active.to_glib(),
                ),
                "Failed to invoke the default activate mode function of the ghost pad"
            )
        }
    }

    pub fn internal_activate_mode_default<P: IsA<GhostPad>, Q: IsA<Object>>(
        pad: &P,
        parent: Option<&Q>,
        mode: PadMode,
        active: bool,
    ) -> Result<(), glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_ghost_pad_internal_activate_mode_default(
                    pad.to_glib_none().0 as *mut gst_sys::GstPad,
                    parent.map(|p| p.as_ref()).to_glib_none().0,
                    mode.to_glib(),
                    active.to_glib(),
                ),
                concat!(
                    "Failed to invoke the default activate mode function of a proxy pad ",
                    "that is owned by the ghost pad"
                )
            )
        }
    }
}
