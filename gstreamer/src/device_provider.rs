// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use DeviceProvider;

use glib::IsA;
use glib::translate::ToGlibPtr;

use std::ffi::CStr;

use ffi;
use gobject_ffi;

pub trait DeviceProviderExtManual {
    fn get_metadata(&self, key: &str) -> Option<&'static str>;
}

impl<O: IsA<DeviceProvider>> DeviceProviderExtManual for O {
    fn get_metadata(&self, key: &str) -> Option<&'static str> {
        unsafe {
            let klass = (*(self.to_glib_none().0 as *mut gobject_ffi::GTypeInstance)).g_class as
                *mut ffi::GstDeviceProviderClass;

            let ptr = ffi::gst_device_provider_class_get_metadata(klass, key.to_glib_none().0);

            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }
}
