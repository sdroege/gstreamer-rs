// Take a look at the license at the top of the repository in the LICENSE file.

use crate::DeviceProvider;
use crate::Plugin;
use crate::Rank;

use glib::prelude::*;
use glib::translate::*;

use std::ffi::CStr;

impl DeviceProvider {
    #[doc(alias = "gst_device_provider_register")]
    pub fn register(
        plugin: Option<&Plugin>,
        name: &str,
        rank: Rank,
        type_: glib::types::Type,
    ) -> Result<(), glib::error::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_device_provider_register(
                    plugin.to_glib_none().0,
                    name.to_glib_none().0,
                    rank.into_glib() as u32,
                    type_.into_glib()
                ),
                "Failed to register device provider factory"
            )
        }
    }
}

pub trait DeviceProviderExtManual: 'static {
    #[doc(alias = "get_metadata")]
    #[doc(alias = "gst_device_provider_class_get_metadata")]
    fn metadata<'a>(&self, key: &str) -> Option<&'a str>;
}

impl<O: IsA<DeviceProvider>> DeviceProviderExtManual for O {
    fn metadata<'a>(&self, key: &str) -> Option<&'a str> {
        unsafe {
            let klass = (*(self.as_ptr() as *mut glib::gobject_ffi::GTypeInstance)).g_class
                as *mut ffi::GstDeviceProviderClass;

            let ptr = ffi::gst_device_provider_class_get_metadata(klass, key.to_glib_none().0);

            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }
}
