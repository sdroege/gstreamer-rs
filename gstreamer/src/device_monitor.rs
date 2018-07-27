// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib::translate::*;
use DeviceMonitor;

impl DeviceMonitor {
    pub fn new() -> DeviceMonitor {
        assert_initialized_main_thread!();
        let (major, minor, _, _) = ::version();
        if (major, minor) > (1, 12) {
            unsafe { from_glib_full(ffi::gst_device_monitor_new()) }
        } else {
            // Work-around for 1.14 switching from transfer-floating to transfer-full
            unsafe { from_glib_none(ffi::gst_device_monitor_new()) }
        }
    }
}

impl Default for DeviceMonitor {
    fn default() -> Self {
        Self::new()
    }
}
