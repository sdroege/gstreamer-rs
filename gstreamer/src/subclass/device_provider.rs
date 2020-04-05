// Copyright (C) 2019 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib_sys;
use gst_sys;

use glib::translate::*;

use glib::subclass::prelude::*;

use Device;
use DeviceProvider;
use DeviceProviderClass;
use LoggableError;

pub trait DeviceProviderImpl: DeviceProviderImplExt + ObjectImpl + Send + Sync + 'static {
    fn probe(&self, device_provider: &DeviceProvider) -> Vec<Device> {
        self.parent_probe(device_provider)
    }

    fn start(&self, device_provider: &DeviceProvider) -> Result<(), LoggableError> {
        self.parent_start(device_provider)
    }

    fn stop(&self, device_provider: &DeviceProvider) {
        self.parent_stop(device_provider)
    }
}

pub trait DeviceProviderImplExt {
    fn parent_probe(&self, device_provider: &DeviceProvider) -> Vec<Device>;

    fn parent_start(&self, device_provider: &DeviceProvider) -> Result<(), LoggableError>;

    fn parent_stop(&self, device_provider: &DeviceProvider);
}

impl<T: DeviceProviderImpl + ObjectImpl> DeviceProviderImplExt for T {
    fn parent_probe(&self, device_provider: &DeviceProvider) -> Vec<Device> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_sys::GstDeviceProviderClass;
            if let Some(f) = (*parent_class).probe {
                FromGlibPtrContainer::from_glib_full(f(device_provider.to_glib_none().0))
            } else {
                Vec::new()
            }
        }
    }

    fn parent_start(&self, device_provider: &DeviceProvider) -> Result<(), LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_sys::GstDeviceProviderClass;
            let f = (*parent_class).start.ok_or_else(|| {
                gst_loggable_error!(::CAT_RUST, "Parent function `start` is not defined")
            })?;
            gst_result_from_gboolean!(
                f(device_provider.to_glib_none().0),
                ::CAT_RUST,
                "Failed to start the device provider using the parent function"
            )
        }
    }

    fn parent_stop(&self, device_provider: &DeviceProvider) {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_sys::GstDeviceProviderClass;
            if let Some(f) = (*parent_class).stop {
                f(device_provider.to_glib_none().0);
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
            gst_sys::gst_device_provider_class_set_metadata(
                self as *mut Self as *mut gst_sys::GstDeviceProviderClass,
                long_name.to_glib_none().0,
                classification.to_glib_none().0,
                description.to_glib_none().0,
                author.to_glib_none().0,
            );
        }
    }

    fn add_metadata(&mut self, key: &str, value: &str) {
        unsafe {
            gst_sys::gst_device_provider_class_add_metadata(
                self as *mut Self as *mut gst_sys::GstDeviceProviderClass,
                key.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }
}

unsafe impl DeviceProviderClassSubclassExt for DeviceProviderClass {}

unsafe impl<T: ObjectSubclass + DeviceProviderImpl> IsSubclassable<T> for DeviceProviderClass {
    fn override_vfuncs(&mut self) {
        <glib::ObjectClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gst_sys::GstDeviceProviderClass);
            klass.probe = Some(device_provider_probe::<T>);
            klass.start = Some(device_provider_start::<T>);
            klass.stop = Some(device_provider_stop::<T>);
        }
    }
}

unsafe extern "C" fn device_provider_probe<T: ObjectSubclass>(
    ptr: *mut gst_sys::GstDeviceProvider,
) -> *mut glib_sys::GList
where
    T: DeviceProviderImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<DeviceProvider> = from_glib_borrow(ptr);

    imp.probe(&wrap).to_glib_full()
}

unsafe extern "C" fn device_provider_start<T: ObjectSubclass>(
    ptr: *mut gst_sys::GstDeviceProvider,
) -> glib_sys::gboolean
where
    T: DeviceProviderImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<DeviceProvider> = from_glib_borrow(ptr);

    match imp.start(&wrap) {
        Ok(()) => true,
        Err(err) => {
            err.log_with_object(&*wrap);
            false
        }
    }
    .to_glib()
}

unsafe extern "C" fn device_provider_stop<T: ObjectSubclass>(ptr: *mut gst_sys::GstDeviceProvider)
where
    T: DeviceProviderImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<DeviceProvider> = from_glib_borrow(ptr);

    imp.stop(&wrap);
}
