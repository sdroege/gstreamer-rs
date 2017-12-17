// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Plugin;
use Structure;
use StructureRef;
use ffi;

use glib::translate::*;

impl Plugin {
    pub fn get_cache_data(&self) -> Option<&StructureRef> {
        unsafe {
            let cache_data = ffi::gst_plugin_get_cache_data(self.to_glib_none().0);
            if cache_data.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(cache_data))
            }
        }
    }

    pub fn set_cache_data(&self, cache_data: Structure) {
        unsafe {
            ffi::gst_plugin_set_cache_data(self.to_glib_none().0, cache_data.into_ptr());
        }
    }
}
