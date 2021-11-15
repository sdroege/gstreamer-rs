// Take a look at the license at the top of the repository in the LICENSE file.

use crate::DeviceProviderFactory;
use glib::translate::*;

impl DeviceProviderFactory {
    #[doc(alias = "gst_device_provider_factory_list_get_device_providers")]
    pub fn factories(minrank: crate::Rank) -> glib::List<DeviceProviderFactory> {
        assert_initialized_main_thread!();
        unsafe {
            FromGlibPtrContainer::from_glib_full(
                ffi::gst_device_provider_factory_list_get_device_providers(minrank.into_glib()),
            )
        }
    }
}
