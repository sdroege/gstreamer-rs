// Take a look at the license at the top of the repository in the LICENSE file.

use crate::DeviceProviderFactory;
use crate::ELEMENT_METADATA_AUTHOR;
use crate::ELEMENT_METADATA_DESCRIPTION;
use crate::ELEMENT_METADATA_DOC_URI;
use crate::ELEMENT_METADATA_ICON_NAME;
use crate::ELEMENT_METADATA_KLASS;
use crate::ELEMENT_METADATA_LONGNAME;
use glib::translate::*;
use std::ffi::CStr;

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

    #[doc(alias = "gst_device_provider_factory_get_metadata")]
    #[doc(alias = "get_metadata")]
    pub fn metadata(&self, key: &str) -> Option<&str> {
        unsafe {
            let ptr = ffi::gst_device_provider_factory_get_metadata(
                self.to_glib_none().0,
                key.to_glib_none().0,
            );

            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    #[doc(alias = "get_longname")]
    #[doc(alias = "gst_device_provider_factory_get_longname")]
    pub fn longname(&self) -> &str {
        self.metadata(&ELEMENT_METADATA_LONGNAME).unwrap()
    }

    #[doc(alias = "get_klass")]
    #[doc(alias = "gst_device_provider_factory_get_klass")]
    pub fn klass(&self) -> &str {
        self.metadata(&ELEMENT_METADATA_KLASS).unwrap()
    }

    #[doc(alias = "get_description")]
    #[doc(alias = "gst_device_provider_factory_get_description")]
    pub fn description(&self) -> &str {
        self.metadata(&ELEMENT_METADATA_DESCRIPTION).unwrap()
    }

    #[doc(alias = "get_author")]
    #[doc(alias = "gst_device_provider_factory_get_author")]
    pub fn author(&self) -> &str {
        self.metadata(&ELEMENT_METADATA_AUTHOR).unwrap()
    }

    #[doc(alias = "get_documentation_uri")]
    #[doc(alias = "gst_device_provider_factory_get_documentation_uri")]
    pub fn documentation_uri(&self) -> Option<&str> {
        self.metadata(&ELEMENT_METADATA_DOC_URI)
    }

    #[doc(alias = "get_icon_name")]
    #[doc(alias = "gst_device_provider_factory_get_icon_name")]
    pub fn icon_name(&self) -> Option<&str> {
        self.metadata(&ELEMENT_METADATA_ICON_NAME)
    }
}
