// Copyright (C) 2017-2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib_sys;
use gst_sys;

use glib;
use glib::prelude::*;
use glib::translate::*;

use glib::subclass::prelude::*;

use libc;

use URIHandler;
use URIType;

pub trait URIHandlerImpl: super::element::ElementImpl + Send + Sync + 'static {
    fn get_uri(&self, element: &URIHandler) -> Option<String>;
    fn set_uri(&self, element: &URIHandler, uri: &str) -> Result<(), glib::Error>;
    fn get_uri_type() -> URIType;
    fn get_protocols() -> Vec<String>;
}

unsafe impl<T: ObjectSubclass + URIHandlerImpl> IsImplementable<T> for URIHandler {
    unsafe extern "C" fn interface_init(
        iface: glib_sys::gpointer,
        _iface_data: glib_sys::gpointer,
    ) {
        let uri_handler_iface = &mut *(iface as *mut gst_sys::GstURIHandlerInterface);

        // Store the protocols in the interface data for later use
        let mut data = T::type_data();
        let protocols = T::get_protocols();
        let protocols: *mut *const libc::c_char = protocols.to_glib_full();
        let data = data.as_mut();
        if data.interface_data.is_null() {
            data.interface_data = Box::into_raw(Box::new(Vec::new()));
        }
        (*(data.interface_data as *mut Vec<(glib_sys::GType, glib_sys::gpointer)>))
            .push((URIHandler::static_type().to_glib(), protocols as *mut _));

        uri_handler_iface.get_type = Some(uri_handler_get_type::<T>);
        uri_handler_iface.get_protocols = Some(uri_handler_get_protocols::<T>);
        uri_handler_iface.get_uri = Some(uri_handler_get_uri::<T>);
        uri_handler_iface.set_uri = Some(uri_handler_set_uri::<T>);
    }
}

unsafe extern "C" fn uri_handler_get_type<T: ObjectSubclass>(
    _type_: glib_sys::GType,
) -> gst_sys::GstURIType
where
    T: URIHandlerImpl,
{
    <T as URIHandlerImpl>::get_uri_type().to_glib()
}

unsafe extern "C" fn uri_handler_get_protocols<T: ObjectSubclass>(
    _type_: glib_sys::GType,
) -> *const *const libc::c_char
where
    T: URIHandlerImpl,
{
    let data = <T as ObjectSubclass>::type_data();
    data.as_ref()
        .get_interface_data(URIHandler::static_type().to_glib()) as *const _
}

unsafe extern "C" fn uri_handler_get_uri<T: ObjectSubclass>(
    uri_handler: *mut gst_sys::GstURIHandler,
) -> *mut libc::c_char
where
    T: URIHandlerImpl,
{
    let instance = &*(uri_handler as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_uri(&from_glib_borrow(uri_handler)).to_glib_full()
}

unsafe extern "C" fn uri_handler_set_uri<T: ObjectSubclass>(
    uri_handler: *mut gst_sys::GstURIHandler,
    uri: *const libc::c_char,
    err: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean
where
    T: URIHandlerImpl,
{
    let instance = &*(uri_handler as *mut T::Instance);
    let imp = instance.get_impl();

    match imp.set_uri(
        &from_glib_borrow(uri_handler),
        glib::GString::from_glib_borrow(uri).as_str(),
    ) {
        Ok(()) => true.to_glib(),
        Err(error) => {
            *err = error.to_glib_full() as *mut _;
            false.to_glib()
        }
    }
}
