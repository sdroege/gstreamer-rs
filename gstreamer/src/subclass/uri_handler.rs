// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::translate::*;

use glib::subclass::prelude::*;

use crate::URIHandler;
use crate::URIType;

use std::ptr;

pub trait URIHandlerImpl: super::element::ElementImpl {
    const URI_TYPE: URIType;
    fn protocols() -> &'static [&'static str];
    fn uri(&self) -> Option<String>;
    fn set_uri(&self, uri: &str) -> Result<(), glib::Error>;
}

pub trait URIHandlerImplExt: ObjectSubclass {
    fn parent_protocols() -> Vec<String>;
    fn parent_uri(&self) -> Option<String>;
    fn parent_set_uri(&self, uri: &str) -> Result<(), glib::Error>;
}

impl<T: URIHandlerImpl> URIHandlerImplExt for T {
    fn parent_protocols() -> Vec<String> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<URIHandler>()
                as *const ffi::GstURIHandlerInterface;

            let func = (*parent_iface)
                .get_protocols
                .expect("no parent \"protocols\" implementation");
            let ret = func(Self::ParentType::static_type().into_glib());
            FromGlibPtrContainer::from_glib_none(ret)
        }
    }

    fn parent_uri(&self) -> Option<String> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<URIHandler>()
                as *const ffi::GstURIHandlerInterface;

            let func = (*parent_iface)
                .get_uri
                .expect("no parent \"uri\" implementation");
            let ret = func(self.obj().unsafe_cast_ref::<URIHandler>().to_glib_none().0);
            from_glib_full(ret)
        }
    }

    fn parent_set_uri(&self, uri: &str) -> Result<(), glib::Error> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<URIHandler>()
                as *const ffi::GstURIHandlerInterface;

            let func = (*parent_iface)
                .set_uri
                .expect("no parent \"set_uri\" implementation");

            let mut err = ptr::null_mut();
            func(
                self.obj().unsafe_cast_ref::<URIHandler>().to_glib_none().0,
                uri.to_glib_none().0,
                &mut err,
            );

            if !err.is_null() {
                Err(from_glib_full(err))
            } else {
                Ok(())
            }
        }
    }
}

unsafe impl<T: URIHandlerImpl> IsImplementable<T> for URIHandler {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        // Store the protocols in the interface data for later use
        unsafe {
            let mut data = T::type_data();
            let protocols = T::protocols();
            let data = data.as_mut();
            data.set_class_data(Self::static_type(), glib::StrV::from(protocols));
        }

        iface.get_type = Some(uri_handler_get_type::<T>);
        iface.get_protocols = Some(uri_handler_get_protocols::<T>);
        iface.get_uri = Some(uri_handler_get_uri::<T>);
        iface.set_uri = Some(uri_handler_set_uri::<T>);
    }
}

unsafe extern "C" fn uri_handler_get_type<T: URIHandlerImpl>(
    _type_: glib::ffi::GType,
) -> ffi::GstURIType {
    <T as URIHandlerImpl>::URI_TYPE.into_glib()
}

unsafe extern "C" fn uri_handler_get_protocols<T: URIHandlerImpl>(
    _type_: glib::ffi::GType,
) -> *const *const libc::c_char {
    let data = <T as ObjectSubclassType>::type_data();
    data.as_ref()
        .class_data::<glib::StrV>(URIHandler::static_type())
        .map(|p| p.as_ptr() as *const *const _)
        .unwrap_or(ptr::null())
}

unsafe extern "C" fn uri_handler_get_uri<T: URIHandlerImpl>(
    uri_handler: *mut ffi::GstURIHandler,
) -> *mut libc::c_char {
    let instance = &*(uri_handler as *mut T::Instance);
    let imp = instance.imp();

    imp.uri().to_glib_full()
}

unsafe extern "C" fn uri_handler_set_uri<T: URIHandlerImpl>(
    uri_handler: *mut ffi::GstURIHandler,
    uri: *const libc::c_char,
    err: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(uri_handler as *mut T::Instance);
    let imp = instance.imp();

    match imp.set_uri(glib::GString::from_glib_borrow(uri).as_str()) {
        Ok(()) => true.into_glib(),
        Err(error) => {
            if !err.is_null() {
                *err = error.into_glib_ptr();
            }
            false.into_glib()
        }
    }
}
