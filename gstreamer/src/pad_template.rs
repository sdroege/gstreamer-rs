// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib::translate::*;
use PadTemplate;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use StaticPadTemplate;

#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use gst_sys;

impl PadTemplate {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn from_static_pad_template_with_gtype(
        pad_template: &StaticPadTemplate,
        pad_type: glib::types::Type,
    ) -> Result<PadTemplate, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_none(
                gst_sys::gst_pad_template_new_from_static_pad_template_with_gtype(
                    mut_override(pad_template.to_glib_none().0),
                    pad_type.to_glib(),
                ),
            )
            .ok_or_else(|| glib_bool_error!("Failed to create PadTemplate"))
        }
    }
}
