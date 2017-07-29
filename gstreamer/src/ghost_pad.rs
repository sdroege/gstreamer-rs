// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Pad;
use PadTemplate;
use GhostPad;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

impl GhostPad {
    pub fn new<'a, P: Into<Option<&'a str>>, Q: IsA<Pad>>(name: P, target: &Q) -> Option<GhostPad> {
        skip_assert_initialized!();
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            Option::<Pad>::from_glib_none(ffi::gst_ghost_pad_new(name.0, target.to_glib_none().0)).map(|o| Downcast::downcast_unchecked(o))
        }
    }

    pub fn new_from_template<'a, P: Into<Option<&'a str>>, Q: IsA<Pad>>(name: P, target: &Q, templ: &PadTemplate) -> Option<GhostPad> {
        skip_assert_initialized!();
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            Option::<Pad>::from_glib_none(ffi::gst_ghost_pad_new_from_template(name.0, target.to_glib_none().0, templ.to_glib_none().0)).map(|o| Downcast::downcast_unchecked(o))
        }
    }
}
