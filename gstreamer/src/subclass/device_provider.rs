// Copyright (C) 2019 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use crate::Device;
use crate::DeviceProvider;
use crate::LoggableError;

pub trait DeviceProviderImpl: DeviceProviderImplExt + ObjectImpl + Send + Sync {
    fn probe(&self, device_provider: &Self::Type) -> Vec<Device> {
        self.parent_probe(device_provider)
    }

    fn start(&self, device_provider: &Self::Type) -> Result<(), LoggableError> {
        self.parent_start(device_provider)
    }

    fn stop(&self, device_provider: &Self::Type) {
        self.parent_stop(device_provider)
    }
}

pub trait DeviceProviderImplExt: ObjectSubclass {
    fn parent_probe(&self, device_provider: &Self::Type) -> Vec<Device>;

    fn parent_start(&self, device_provider: &Self::Type) -> Result<(), LoggableError>;

    fn parent_stop(&self, device_provider: &Self::Type);
}

impl<T: DeviceProviderImpl> DeviceProviderImplExt for T {
    fn parent_probe(&self, device_provider: &Self::Type) -> Vec<Device> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstDeviceProviderClass;
            if let Some(f) = (*parent_class).probe {
                FromGlibPtrContainer::from_glib_full(f(device_provider
                    .unsafe_cast_ref::<DeviceProvider>()
                    .to_glib_none()
                    .0))
            } else {
                Vec::new()
            }
        }
    }

    fn parent_start(&self, device_provider: &Self::Type) -> Result<(), LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstDeviceProviderClass;
            let f = (*parent_class).start.ok_or_else(|| {
                gst_loggable_error!(crate::CAT_RUST, "Parent function `start` is not defined")
            })?;
            gst_result_from_gboolean!(
                f(device_provider
                    .unsafe_cast_ref::<DeviceProvider>()
                    .to_glib_none()
                    .0),
                crate::CAT_RUST,
                "Failed to start the device provider using the parent function"
            )
        }
    }

    fn parent_stop(&self, device_provider: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstDeviceProviderClass;
            if let Some(f) = (*parent_class).stop {
                f(device_provider
                    .unsafe_cast_ref::<DeviceProvider>()
                    .to_glib_none()
                    .0);
            }
        }
    }
}

pub unsafe trait DeviceProviderClassSubclassExt: Sized + 'static {
    fn set_metadata(
        &mut self,
        long_name: &str,
        classification: &str,
        description: &str,
        author: &str,
    ) {
        unsafe {
            ffi::gst_device_provider_class_set_metadata(
                self as *mut Self as *mut ffi::GstDeviceProviderClass,
                long_name.to_glib_none().0,
                classification.to_glib_none().0,
                description.to_glib_none().0,
                author.to_glib_none().0,
            );
        }
    }

    fn add_metadata(&mut self, key: &str, value: &str) {
        unsafe {
            ffi::gst_device_provider_class_add_metadata(
                self as *mut Self as *mut ffi::GstDeviceProviderClass,
                key.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }
}

unsafe impl DeviceProviderClassSubclassExt for glib::Class<DeviceProvider> {}

unsafe impl<T: DeviceProviderImpl> IsSubclassable<T> for DeviceProvider {
    fn override_vfuncs(klass: &mut glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::override_vfuncs(klass);
        let klass = klass.as_mut();
        klass.probe = Some(device_provider_probe::<T>);
        klass.start = Some(device_provider_start::<T>);
        klass.stop = Some(device_provider_stop::<T>);
    }
}

unsafe extern "C" fn device_provider_probe<T: DeviceProviderImpl>(
    ptr: *mut ffi::GstDeviceProvider,
) -> *mut glib::ffi::GList {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<DeviceProvider> = from_glib_borrow(ptr);

    imp.probe(wrap.unsafe_cast_ref()).to_glib_full()
}

unsafe extern "C" fn device_provider_start<T: DeviceProviderImpl>(
    ptr: *mut ffi::GstDeviceProvider,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<DeviceProvider> = from_glib_borrow(ptr);

    match imp.start(wrap.unsafe_cast_ref()) {
        Ok(()) => true,
        Err(err) => {
            err.log_with_object(&*wrap);
            false
        }
    }
    .to_glib()
}

unsafe extern "C" fn device_provider_stop<T: DeviceProviderImpl>(ptr: *mut ffi::GstDeviceProvider) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<DeviceProvider> = from_glib_borrow(ptr);

    imp.stop(wrap.unsafe_cast_ref());
}
