// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CStr;

use glib::{prelude::*, translate::*};

use crate::{DeviceProvider, Plugin, Rank};

impl DeviceProvider {
    #[doc(alias = "gst_device_provider_register")]
    pub fn register(
        plugin: Option<&Plugin>,
        name: &str,
        rank: Rank,
        type_: glib::types::Type,
    ) -> Result<(), glib::error::BoolError> {
        skip_assert_initialized!();
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

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DeviceProvider>> Sealed for T {}
}

pub trait DeviceProviderExtManual: sealed::Sealed + IsA<DeviceProvider> + 'static {
    #[doc(alias = "get_metadata")]
    #[doc(alias = "gst_device_provider_class_get_metadata")]
    fn metadata<'a>(&self, key: &str) -> Option<&'a str> {
        unsafe {
            self.unsafe_cast_ref::<DeviceProvider>()
                .class()
                .metadata(key)
        }
    }

    #[doc(alias = "gst_device_provider_get_devices")]
    #[doc(alias = "get_devices")]
    fn devices(&self) -> glib::List<crate::Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_device_provider_get_devices(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<DeviceProvider>> DeviceProviderExtManual for O {}

pub unsafe trait DeviceProviderClassExt {
    #[doc(alias = "get_metadata")]
    #[doc(alias = "gst_device_provider_class_get_metadata")]
    fn metadata<'a>(&self, key: &str) -> Option<&'a str> {
        unsafe {
            let klass = self as *const _ as *const ffi::GstDeviceProviderClass;

            let ptr = ffi::gst_device_provider_class_get_metadata(
                mut_override(klass),
                key.to_glib_none().0,
            );

            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }
}

unsafe impl<T: IsA<DeviceProvider> + glib::object::IsClass> DeviceProviderClassExt
    for glib::object::Class<T>
{
}
