// Copyright (C) 2018-2019 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Caps;
use DeviceMonitor;

use glib;
use glib::object::IsA;
use glib::translate::*;

use gst_sys;

use std::num::NonZeroU32;

impl DeviceMonitor {
    pub fn new() -> DeviceMonitor {
        assert_initialized_main_thread!();
        let (major, minor, _, _) = ::version();
        if (major, minor) > (1, 12) {
            unsafe { from_glib_full(gst_sys::gst_device_monitor_new()) }
        } else {
            // Work-around for 1.14 switching from transfer-floating to transfer-full
            unsafe { from_glib_none(gst_sys::gst_device_monitor_new()) }
        }
    }
}

impl Default for DeviceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DeviceMonitorFilterId(NonZeroU32);

impl ToGlib for DeviceMonitorFilterId {
    type GlibType = libc::c_uint;

    fn to_glib(&self) -> libc::c_uint {
        self.0.get()
    }
}

impl FromGlib<libc::c_uint> for DeviceMonitorFilterId {
    fn from_glib(val: libc::c_uint) -> DeviceMonitorFilterId {
        skip_assert_initialized!();
        assert_ne!(val, 0);
        DeviceMonitorFilterId(unsafe { NonZeroU32::new_unchecked(val) })
    }
}

pub trait DeviceMonitorExtManual: 'static {
    fn add_filter(
        &self,
        classes: Option<&str>,
        caps: Option<&Caps>,
    ) -> Option<DeviceMonitorFilterId>;

    fn remove_filter(&self, filter_id: DeviceMonitorFilterId)
        -> Result<(), glib::error::BoolError>;
}

impl<O: IsA<DeviceMonitor>> DeviceMonitorExtManual for O {
    fn add_filter(
        &self,
        classes: Option<&str>,
        caps: Option<&Caps>,
    ) -> Option<DeviceMonitorFilterId> {
        let id = unsafe {
            gst_sys::gst_device_monitor_add_filter(
                self.as_ref().to_glib_none().0,
                classes.to_glib_none().0,
                caps.to_glib_none().0,
            )
        };

        if id == 0 {
            None
        } else {
            Some(from_glib(id))
        }
    }

    fn remove_filter(
        &self,
        filter_id: DeviceMonitorFilterId,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_device_monitor_remove_filter(
                    self.as_ref().to_glib_none().0,
                    filter_id.to_glib()
                ),
                "Failed to remove the filter"
            )
        }
    }
}
