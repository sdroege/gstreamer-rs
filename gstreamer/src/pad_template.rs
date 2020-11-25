// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::PadTemplate;
#[cfg(any(feature = "v1_14", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_14")))]
use crate::StaticPadTemplate;
#[cfg(any(feature = "v1_14", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_14")))]
use glib::translate::*;

impl PadTemplate {
    #[cfg(any(feature = "v1_14", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_14")))]
    pub fn from_static_pad_template_with_gtype(
        pad_template: &StaticPadTemplate,
        pad_type: glib::types::Type,
    ) -> Result<PadTemplate, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_none(
                ffi::gst_pad_template_new_from_static_pad_template_with_gtype(
                    mut_override(pad_template.to_glib_none().0),
                    pad_type.to_glib(),
                ),
            )
            .ok_or_else(|| glib::glib_bool_error!("Failed to create PadTemplate"))
        }
    }
}
