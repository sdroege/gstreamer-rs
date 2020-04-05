// Copyright (C) 2017,2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib_sys;
use gst_sys;

use glib::translate::*;

use super::prelude::*;
use glib::subclass::prelude::*;

use Bin;
use BinClass;
use Element;
use LoggableError;
use Message;

pub trait BinImpl: BinImplExt + ElementImpl + Send + Sync + 'static {
    fn add_element(&self, bin: &Bin, element: &Element) -> Result<(), LoggableError> {
        self.parent_add_element(bin, element)
    }

    fn remove_element(&self, bin: &Bin, element: &Element) -> Result<(), LoggableError> {
        self.parent_remove_element(bin, element)
    }

    fn handle_message(&self, bin: &Bin, message: Message) {
        self.parent_handle_message(bin, message)
    }
}

pub trait BinImplExt {
    fn parent_add_element(&self, bin: &Bin, element: &Element) -> Result<(), LoggableError>;

    fn parent_remove_element(&self, bin: &Bin, element: &Element) -> Result<(), LoggableError>;

    fn parent_handle_message(&self, bin: &Bin, message: Message);
}

impl<T: BinImpl + ObjectImpl> BinImplExt for T {
    fn parent_add_element(&self, bin: &Bin, element: &Element) -> Result<(), LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstBinClass;
            let f = (*parent_class).add_element.ok_or_else(|| {
                gst_loggable_error!(::CAT_RUST, "Parent function `add_element` is not defined")
            })?;
            gst_result_from_gboolean!(
                f(bin.to_glib_none().0, element.to_glib_none().0),
                ::CAT_RUST,
                "Failed to add the element using the parent function"
            )
        }
    }

    fn parent_remove_element(&self, bin: &Bin, element: &Element) -> Result<(), LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstBinClass;
            let f = (*parent_class).remove_element.ok_or_else(|| {
                gst_loggable_error!(
                    ::CAT_RUST,
                    "Parent function `remove_element` is not defined"
                )
            })?;
            gst_result_from_gboolean!(
                f(bin.to_glib_none().0, element.to_glib_none().0),
                ::CAT_RUST,
                "Failed to remove the element using the parent function"
            )
        }
    }

    fn parent_handle_message(&self, bin: &Bin, message: Message) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstBinClass;
            if let Some(ref f) = (*parent_class).handle_message {
                f(bin.to_glib_none().0, message.into_ptr());
            }
        }
    }
}

unsafe impl<T: ObjectSubclass + BinImpl> IsSubclassable<T> for BinClass
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(&mut self) {
        <::ElementClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gst_sys::GstBinClass);
            klass.add_element = Some(bin_add_element::<T>);
            klass.remove_element = Some(bin_remove_element::<T>);
            klass.handle_message = Some(bin_handle_message::<T>);
        }
    }
}

unsafe extern "C" fn bin_add_element<T: ObjectSubclass>(
    ptr: *mut gst_sys::GstBin,
    element: *mut gst_sys::GstElement,
) -> glib_sys::gboolean
where
    T: BinImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Bin> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.add_element(&wrap, &from_glib_none(element)) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn bin_remove_element<T: ObjectSubclass>(
    ptr: *mut gst_sys::GstBin,
    element: *mut gst_sys::GstElement,
) -> glib_sys::gboolean
where
    T: BinImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Bin> = from_glib_borrow(ptr);

    // If we get a floating reference passed simply return FALSE here. It can't be
    // stored inside this bin, and if we continued to use it we would take ownership
    // of this floating reference.
    if gobject_sys::g_object_is_floating(element as *mut gobject_sys::GObject) != glib_sys::GFALSE {
        return glib_sys::GFALSE;
    }

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.remove_element(&wrap, &from_glib_none(element)) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn bin_handle_message<T: ObjectSubclass>(
    ptr: *mut gst_sys::GstBin,
    message: *mut gst_sys::GstMessage,
) where
    T: BinImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Bin> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), (), {
        imp.handle_message(&wrap, from_glib_full(message))
    });
}
