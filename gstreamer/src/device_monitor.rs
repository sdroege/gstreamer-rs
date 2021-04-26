// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Caps;
use crate::DeviceMonitor;

use glib::prelude::*;
use glib::translate::*;

use std::num::NonZeroU32;

impl DeviceMonitor {
    pub fn new() -> DeviceMonitor {
        assert_initialized_main_thread!();
        let (major, minor, _, _) = crate::version();
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

#[derive(Debug, PartialEq, Eq)]
pub struct DeviceMonitorFilterId(NonZeroU32);

impl ToGlib for DeviceMonitorFilterId {
    type GlibType = libc::c_uint;

    fn to_glib(&self) -> libc::c_uint {
        self.0.get()
    }
}

impl FromGlib<libc::c_uint> for DeviceMonitorFilterId {
    unsafe fn from_glib(val: libc::c_uint) -> DeviceMonitorFilterId {
        skip_assert_initialized!();
        assert_ne!(val, 0);
        DeviceMonitorFilterId(NonZeroU32::new_unchecked(val))
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
            ffi::gst_device_monitor_add_filter(
                self.as_ref().to_glib_none().0,
                classes.to_glib_none().0,
                caps.to_glib_none().0,
            )
        };

        if id == 0 {
            None
        } else {
            Some(unsafe { from_glib(id) })
        }
    }

    fn remove_filter(
        &self,
        filter_id: DeviceMonitorFilterId,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_device_monitor_remove_filter(
                    self.as_ref().to_glib_none().0,
                    filter_id.to_glib()
                ),
                "Failed to remove the filter"
            )
        }
    }
}
