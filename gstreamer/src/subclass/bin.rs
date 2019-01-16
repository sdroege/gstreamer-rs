// Copyright (C) 2017,2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib_ffi;

use glib::translate::*;

use super::prelude::*;
use glib::subclass::prelude::*;

use Bin;
use BinClass;
use Element;
use Message;

pub trait BinImpl: ElementImpl + Send + Sync + 'static {
    fn add_element(&self, bin: &Bin, element: &Element) -> bool {
        self.parent_add_element(bin, element)
    }

    fn remove_element(&self, bin: &Bin, element: &Element) -> bool {
        self.parent_remove_element(bin, element)
    }

    fn handle_message(&self, bin: &Bin, message: Message) {
        self.parent_handle_message(bin, message)
    }

    fn parent_add_element(&self, bin: &Bin, element: &Element) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBinClass;
            (*parent_class)
                .add_element
                .map(|f| from_glib(f(bin.to_glib_none().0, element.to_glib_none().0)))
                .unwrap_or(false)
        }
    }

    fn parent_remove_element(&self, bin: &Bin, element: &Element) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBinClass;
            (*parent_class)
                .remove_element
                .map(|f| from_glib(f(bin.to_glib_none().0, element.to_glib_none().0)))
                .unwrap_or(false)
        }
    }

    fn parent_handle_message(&self, bin: &Bin, message: Message) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBinClass;
            (*parent_class)
                .handle_message
                .map(move |f| f(bin.to_glib_none().0, message.into_ptr()));
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
            let klass = &mut *(self as *mut Self as *mut ffi::GstBinClass);
            klass.add_element = Some(bin_add_element::<T>);
            klass.remove_element = Some(bin_remove_element::<T>);
            klass.handle_message = Some(bin_handle_message::<T>);
        }
    }
}

unsafe extern "C" fn bin_add_element<T: ObjectSubclass>(
    ptr: *mut ffi::GstBin,
    element: *mut ffi::GstElement,
) -> glib_ffi::gboolean
where
    T: BinImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Bin = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.add_element(&wrap, &from_glib_borrow(element))
    })
    .to_glib()
}

unsafe extern "C" fn bin_remove_element<T: ObjectSubclass>(
    ptr: *mut ffi::GstBin,
    element: *mut ffi::GstElement,
) -> glib_ffi::gboolean
where
    T: BinImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Bin = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.remove_element(&wrap, &from_glib_borrow(element))
    })
    .to_glib()
}

unsafe extern "C" fn bin_handle_message<T: ObjectSubclass>(
    ptr: *mut ffi::GstBin,
    message: *mut ffi::GstMessage,
) where
    T: BinImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Bin = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), (), {
        imp.handle_message(&wrap, from_glib_full(message))
    });
}
