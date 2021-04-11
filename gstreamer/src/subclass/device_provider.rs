// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use crate::Device;
use crate::DeviceProvider;
use crate::LoggableError;

#[derive(Debug, Clone)]
pub struct DeviceProviderMetadata {
    long_name: String,
    classification: String,
    description: String,
    author: String,
    additional: Vec<(String, String)>,
}

pub trait DeviceProviderImpl: DeviceProviderImplExt + ObjectImpl + Send + Sync {
    fn metadata() -> Option<&'static DeviceProviderMetadata> {
        None
    }

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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstDeviceProviderClass;
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstDeviceProviderClass;
            let f = (*parent_class).start.ok_or_else(|| {
                loggable_error!(crate::CAT_RUST, "Parent function `start` is not defined")
            })?;
            result_from_gboolean!(
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstDeviceProviderClass;
            if let Some(f) = (*parent_class).stop {
                f(device_provider
                    .unsafe_cast_ref::<DeviceProvider>()
                    .to_glib_none()
                    .0);
            }
        }
    }
}

unsafe impl<T: DeviceProviderImpl> IsSubclassable<T> for DeviceProvider {
    fn class_init(klass: &mut glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::class_init(klass);
        let klass = klass.as_mut();
        klass.probe = Some(device_provider_probe::<T>);
        klass.start = Some(device_provider_start::<T>);
        klass.stop = Some(device_provider_stop::<T>);

        unsafe {
            if let Some(metadata) = T::metadata() {
                ffi::gst_device_provider_class_set_metadata(
                    klass,
                    metadata.long_name.to_glib_none().0,
                    metadata.classification.to_glib_none().0,
                    metadata.description.to_glib_none().0,
                    metadata.author.to_glib_none().0,
                );

                for (key, value) in &metadata.additional {
                    ffi::gst_device_provider_class_add_metadata(
                        klass,
                        key.to_glib_none().0,
                        value.to_glib_none().0,
                    );
                }
            }
        }
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <glib::Object as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn device_provider_probe<T: DeviceProviderImpl>(
    ptr: *mut ffi::GstDeviceProvider,
) -> *mut glib::ffi::GList {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<DeviceProvider> = from_glib_borrow(ptr);

    imp.probe(wrap.unsafe_cast_ref()).to_glib_full()
}

unsafe extern "C" fn device_provider_start<T: DeviceProviderImpl>(
    ptr: *mut ffi::GstDeviceProvider,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
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
    let imp = instance.impl_();
    let wrap: Borrowed<DeviceProvider> = from_glib_borrow(ptr);

    imp.stop(wrap.unsafe_cast_ref());
}
