// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_sys;
use Plugin;
use PluginFlags;
use Structure;
use StructureRef;

use glib::translate::*;
use glib::IsA;

impl Plugin {
    pub fn get_cache_data(&self) -> Option<&StructureRef> {
        unsafe {
            let cache_data = gst_sys::gst_plugin_get_cache_data(self.to_glib_none().0);
            if cache_data.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(cache_data))
            }
        }
    }

    pub fn set_cache_data(&self, cache_data: Structure) {
        unsafe {
            gst_sys::gst_plugin_set_cache_data(self.to_glib_none().0, cache_data.into_ptr());
        }
    }
}

pub trait GstPluginExtManual: 'static {
    fn get_plugin_flags(&self) -> PluginFlags;

    fn get_plugin_name(&self) -> glib::GString;
}

impl<O: IsA<::Plugin>> GstPluginExtManual for O {
    fn get_plugin_flags(&self) -> PluginFlags {
        unsafe {
            let ptr: *mut gst_sys::GstObject = self.as_ptr() as *mut _;
            let _guard = ::utils::MutexGuard::lock(&(*ptr).lock);
            from_glib((*ptr).flags)
        }
    }

    fn get_plugin_name(&self) -> glib::GString {
        unsafe { from_glib_none(gst_sys::gst_plugin_get_name(self.as_ref().to_glib_none().0)) }
    }
}
