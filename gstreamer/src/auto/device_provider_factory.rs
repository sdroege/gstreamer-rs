// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::DeviceProvider;
use crate::Object;
use crate::PluginFeature;
use crate::Rank;
use glib::translate::*;

glib::wrapper! {
    pub struct DeviceProviderFactory(Object<ffi::GstDeviceProviderFactory, ffi::GstDeviceProviderFactoryClass>) @extends PluginFeature, Object;

    match fn {
        type_ => || ffi::gst_device_provider_factory_get_type(),
    }
}

impl DeviceProviderFactory {
    #[doc(alias = "gst_device_provider_factory_get")]
    pub fn get(&self) -> Option<DeviceProvider> {
        unsafe { from_glib_full(ffi::gst_device_provider_factory_get(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_device_provider_factory_get_device_provider_type")]
    pub fn device_provider_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gst_device_provider_factory_get_device_provider_type(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_device_provider_factory_get_metadata")]
    pub fn metadata(&self, key: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_device_provider_factory_get_metadata(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_device_provider_factory_get_metadata_keys")]
    pub fn metadata_keys(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(
                ffi::gst_device_provider_factory_get_metadata_keys(self.to_glib_none().0),
            )
        }
    }

    #[doc(alias = "gst_device_provider_factory_has_classes")]
    pub fn has_classes(&self, classes: Option<&str>) -> bool {
        unsafe {
            from_glib(ffi::gst_device_provider_factory_has_classes(
                self.to_glib_none().0,
                classes.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_device_provider_factory_has_classesv")]
    pub fn has_classesv(&self, classes: &[&str]) -> bool {
        unsafe {
            from_glib(ffi::gst_device_provider_factory_has_classesv(
                self.to_glib_none().0,
                classes.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_device_provider_factory_find")]
    pub fn find(name: &str) -> Option<DeviceProviderFactory> {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_device_provider_factory_find(name.to_glib_none().0)) }
    }

    #[doc(alias = "gst_device_provider_factory_get_by_name")]
    pub fn by_name(factoryname: &str) -> Option<DeviceProvider> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_device_provider_factory_get_by_name(
                factoryname.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_device_provider_factory_list_get_device_providers")]
    pub fn list_get_device_providers(minrank: Rank) -> Vec<DeviceProviderFactory> {
        assert_initialized_main_thread!();
        unsafe {
            FromGlibPtrContainer::from_glib_full(
                ffi::gst_device_provider_factory_list_get_device_providers(minrank.into_glib()),
            )
        }
    }
}

unsafe impl Send for DeviceProviderFactory {}
unsafe impl Sync for DeviceProviderFactory {}
