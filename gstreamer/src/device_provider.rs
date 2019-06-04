// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use DeviceProvider;
use Plugin;
use Rank;

use glib::object::IsA;
use glib::translate::ToGlib;
use glib::translate::ToGlibPtr;

use std::ffi::CStr;

use gobject_sys;
use gst_sys;

impl DeviceProvider {
    pub fn register(
        plugin: Option<&Plugin>,
        name: &str,
        rank: Rank,
        type_: glib::types::Type,
    ) -> Result<(), glib::error::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_device_provider_register(
                    plugin.to_glib_none().0,
                    name.to_glib_none().0,
                    rank.to_glib() as u32,
                    type_.to_glib()
                ),
                "Failed to register device provider factory"
            )
        }
    }
}

pub trait DeviceProviderExtManual: 'static {
    fn get_metadata<'a>(&self, key: &str) -> Option<&'a str>;
}

impl<O: IsA<DeviceProvider>> DeviceProviderExtManual for O {
    fn get_metadata<'a>(&self, key: &str) -> Option<&'a str> {
        unsafe {
            let klass = (*(self.as_ptr() as *mut gobject_sys::GTypeInstance)).g_class
                as *mut gst_sys::GstDeviceProviderClass;

            let ptr = gst_sys::gst_device_provider_class_get_metadata(klass, key.to_glib_none().0);

            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }
}
