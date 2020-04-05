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
use DeviceClass;
use Element;
use LoggableError;

use std::ptr;

pub trait DeviceImpl: DeviceImplExt + ObjectImpl + Send + Sync + 'static {
    fn create_element(
        &self,
        device: &Device,
        name: Option<&str>,
    ) -> Result<Element, LoggableError> {
        self.parent_create_element(device, name)
    }

    fn reconfigure_element(&self, device: &Device, element: &Element) -> Result<(), LoggableError> {
        self.parent_reconfigure_element(device, element)
    }
}

pub trait DeviceImplExt {
    fn parent_create_element(
        &self,
        device: &Device,
        name: Option<&str>,
    ) -> Result<Element, LoggableError>;

    fn parent_reconfigure_element(
        &self,
        device: &Device,
        element: &Element,
    ) -> Result<(), LoggableError>;
}

impl<T: DeviceImpl + ObjectImpl> DeviceImplExt for T {
    fn parent_create_element(
        &self,
        device: &Device,
        name: Option<&str>,
    ) -> Result<Element, LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstDeviceClass;
            if let Some(f) = (*parent_class).create_element {
                let ptr = f(device.to_glib_none().0, name.to_glib_none().0);

                // Don't steal floating reference here but pass it further to the caller
                Option::<_>::from_glib_full(ptr).ok_or_else(|| {
                    gst_loggable_error!(
                        ::CAT_RUST,
                        "Failed to create element using the parent function"
                    )
                })
            } else {
                Err(gst_loggable_error!(
                    ::CAT_RUST,
                    "Parent function `create_element` is not defined"
                ))
            }
        }
    }

    fn parent_reconfigure_element(
        &self,
        device: &Device,
        element: &Element,
    ) -> Result<(), LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstDeviceClass;
            let f = (*parent_class).reconfigure_element.ok_or_else(|| {
                gst_loggable_error!(
                    ::CAT_RUST,
                    "Parent function `reconfigure_element` is not defined"
                )
            })?;
            gst_result_from_gboolean!(
                f(device.to_glib_none().0, element.to_glib_none().0),
                ::CAT_RUST,
                "Failed to reconfigure the element using the parent function"
            )
        }
    }
}

unsafe impl<T: ObjectSubclass + DeviceImpl> IsSubclassable<T> for DeviceClass {
    fn override_vfuncs(&mut self) {
        <glib::ObjectClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gst_sys::GstDeviceClass);
            klass.create_element = Some(device_create_element::<T>);
            klass.reconfigure_element = Some(device_reconfigure_element::<T>);
        }
    }
}

unsafe extern "C" fn device_create_element<T: ObjectSubclass>(
    ptr: *mut gst_sys::GstDevice,
    name: *const libc::c_char,
) -> *mut gst_sys::GstElement
where
    T: DeviceImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Device> = from_glib_borrow(ptr);

    match imp.create_element(
        &wrap,
        Option::<glib::GString>::from_glib_borrow(name)
            .as_ref()
            .as_ref()
            .map(|s| s.as_str()),
    ) {
        Ok(element) => {
            // The reference we're going to return, the initial reference is going to
            // be dropped here now
            let element_ptr = element.to_glib_full();
            drop(element);
            // See https://gitlab.freedesktop.org/gstreamer/gstreamer/issues/444
            gobject_sys::g_object_force_floating(element_ptr as *mut gobject_sys::GObject);
            element_ptr
        }
        Err(err) => {
            err.log_with_object(&*wrap);
            ptr::null_mut()
        }
    }
}

unsafe extern "C" fn device_reconfigure_element<T: ObjectSubclass>(
    ptr: *mut gst_sys::GstDevice,
    element: *mut gst_sys::GstElement,
) -> glib_sys::gboolean
where
    T: DeviceImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Device> = from_glib_borrow(ptr);

    match imp.reconfigure_element(&wrap, &from_glib_borrow(element)) {
        Ok(()) => true,
        Err(err) => {
            err.log_with_object(&*wrap);
            false
        }
    }
    .to_glib()
}
