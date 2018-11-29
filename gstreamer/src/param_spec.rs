// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib;
use glib::translate::*;

pub struct ParamSpec(());

impl ParamSpec {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn array(
        name: &str,
        nick: &str,
        blurb: &str,
        element_spec: &glib::ParamSpec,
        flags: glib::ParamFlags,
    ) -> glib::ParamSpec {
        unsafe {
            from_glib_full(ffi::gst_param_spec_array(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                element_spec.to_glib_none().0,
                flags.to_glib(),
            ))
        }
    }

    pub fn fraction(
        name: &str,
        nick: &str,
        blurb: &str,
        min: ::Fraction,
        max: ::Fraction,
        default: ::Fraction,
        flags: glib::ParamFlags,
    ) -> glib::ParamSpec {
        unsafe {
            from_glib_full(ffi::gst_param_spec_fraction(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                *min.numer(),
                *min.denom(),
                *max.numer(),
                *max.denom(),
                *default.numer(),
                *default.denom(),
                flags.to_glib(),
            ))
        }
    }
}
