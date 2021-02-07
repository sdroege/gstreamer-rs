// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::translate::*;

use glib::subclass::prelude::*;

use crate::URIHandler;
use crate::URIType;

pub trait URIHandlerImpl: super::element::ElementImpl {
    const URI_TYPE: URIType;
    fn get_protocols() -> &'static [&'static str];
    fn get_uri(&self, element: &Self::Type) -> Option<String>;
    fn set_uri(&self, element: &Self::Type, uri: &str) -> Result<(), glib::Error>;
}

// Send+Sync wrapper around a NULL-terminated C string array
struct CStrV(*const *const libc::c_char);
unsafe impl Send for CStrV {}
unsafe impl Sync for CStrV {}

unsafe impl<T: URIHandlerImpl> IsImplementable<T> for URIHandler {
    unsafe extern "C" fn interface_init(
        iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
        let uri_handler_iface = &mut *(iface as *mut ffi::GstURIHandlerInterface);

        // Store the protocols in the interface data for later use
        let mut data = T::type_data();
        let protocols = T::get_protocols();
        let protocols = protocols.to_glib_full();
        let data = data.as_mut();

        data.set_class_data(URIHandler::static_type(), CStrV(protocols));

        uri_handler_iface.get_type = Some(uri_handler_get_type::<T>);
        uri_handler_iface.get_protocols = Some(uri_handler_get_protocols::<T>);
        uri_handler_iface.get_uri = Some(uri_handler_get_uri::<T>);
        uri_handler_iface.set_uri = Some(uri_handler_set_uri::<T>);
    }
}

unsafe extern "C" fn uri_handler_get_type<T: URIHandlerImpl>(
    _type_: glib::ffi::GType,
) -> ffi::GstURIType {
    <T as URIHandlerImpl>::URI_TYPE.to_glib()
}

unsafe extern "C" fn uri_handler_get_protocols<T: URIHandlerImpl>(
    _type_: glib::ffi::GType,
) -> *const *const libc::c_char {
    let data = <T as ObjectSubclass>::type_data();
    data.as_ref()
        .get_class_data::<CStrV>(URIHandler::static_type())
        .unwrap_or(&CStrV(std::ptr::null()))
        .0
}

unsafe extern "C" fn uri_handler_get_uri<T: URIHandlerImpl>(
    uri_handler: *mut ffi::GstURIHandler,
) -> *mut libc::c_char {
    let instance = &*(uri_handler as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_uri(&from_glib_borrow::<_, URIHandler>(uri_handler).unsafe_cast_ref())
        .to_glib_full()
}

unsafe extern "C" fn uri_handler_set_uri<T: URIHandlerImpl>(
    uri_handler: *mut ffi::GstURIHandler,
    uri: *const libc::c_char,
    err: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(uri_handler as *mut T::Instance);
    let imp = instance.get_impl();

    match imp.set_uri(
        &from_glib_borrow::<_, URIHandler>(uri_handler).unsafe_cast_ref(),
        glib::GString::from_glib_borrow(uri).as_str(),
    ) {
        Ok(()) => true.to_glib(),
        Err(error) => {
            *err = error.to_glib_full() as *mut _;
            false.to_glib()
        }
    }
}
