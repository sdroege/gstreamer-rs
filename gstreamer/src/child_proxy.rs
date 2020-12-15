// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ChildProxy;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;

pub trait ChildProxyExtManual: 'static {
    fn get_child_property(&self, name: &str) -> Option<glib::Value>;
    fn set_child_property(
        &self,
        name: &str,
        value: &dyn glib::ToValue,
    ) -> Result<(), glib::BoolError>;
}

impl<O: IsA<ChildProxy>> ChildProxyExtManual for O {
    fn get_child_property(&self, name: &str) -> Option<glib::Value> {
        unsafe {
            let found: bool = from_glib(ffi::gst_child_proxy_lookup(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                ptr::null_mut(),
                ptr::null_mut(),
            ));
            if !found {
                return None;
            }

            let mut value = glib::Value::uninitialized();
            ffi::gst_child_proxy_get_property(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            Some(value)
        }
    }

    fn set_child_property(
        &self,
        name: &str,
        value: &dyn glib::ToValue,
    ) -> Result<(), glib::BoolError> {
        unsafe {
            let found: bool = from_glib(ffi::gst_child_proxy_lookup(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                ptr::null_mut(),
                ptr::null_mut(),
            ));
            if !found {
                return Err(glib::glib_bool_error!("Child property not found"));
            }

            let value = value.to_value();
            ffi::gst_child_proxy_set_property(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            );

            Ok(())
        }
    }
}
