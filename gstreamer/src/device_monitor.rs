// Take a look at the license at the top of the repository in the LICENSE file.

use std::num::NonZeroU32;

use glib::{prelude::*, translate::*};

use crate::{Caps, DeviceMonitor};

#[derive(Debug, PartialEq, Eq)]
pub struct DeviceMonitorFilterId(NonZeroU32);

impl IntoGlib for DeviceMonitorFilterId {
    type GlibType = libc::c_uint;

    #[inline]
    fn into_glib(self) -> libc::c_uint {
        self.0.get()
    }
}

impl FromGlib<libc::c_uint> for DeviceMonitorFilterId {
    #[inline]
    unsafe fn from_glib(val: libc::c_uint) -> DeviceMonitorFilterId {
        skip_assert_initialized!();
        debug_assert_ne!(val, 0);
        DeviceMonitorFilterId(NonZeroU32::new_unchecked(val))
    }
}
mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DeviceMonitor>> Sealed for T {}
}

pub trait DeviceMonitorExtManual: sealed::Sealed + IsA<DeviceMonitor> + 'static {
    #[doc(alias = "gst_device_monitor_add_filter")]
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

    #[doc(alias = "gst_device_monitor_remove_filter")]
    fn remove_filter(
        &self,
        filter_id: DeviceMonitorFilterId,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_device_monitor_remove_filter(
                    self.as_ref().to_glib_none().0,
                    filter_id.into_glib()
                ),
                "Failed to remove the filter"
            )
        }
    }

    #[doc(alias = "gst_device_monitor_get_devices")]
    #[doc(alias = "get_devices")]
    fn devices(&self) -> glib::List<crate::Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_device_monitor_get_devices(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<DeviceMonitor>> DeviceMonitorExtManual for O {}
