// Take a look at the license at the top of the repository in the LICENSE file.

use crate::TimelineElement;
use glib::prelude::*;
use glib::translate::*;
use std::ptr;

pub trait TimelineElementExtManual: 'static {
    fn child_property(&self, name: &str) -> Option<glib::Value>;
    fn set_child_property(
        &self,
        name: &str,
        value: &dyn glib::ToValue,
    ) -> Result<(), glib::BoolError>;
}

impl<O: IsA<TimelineElement>> TimelineElementExtManual for O {
    fn child_property(&self, name: &str) -> Option<glib::Value> {
        unsafe {
            let found: bool = from_glib(ffi::ges_timeline_element_lookup_child(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                ptr::null_mut(),
                ptr::null_mut(),
            ));
            if !found {
                return None;
            }

            let mut value = glib::Value::uninitialized();
            ffi::ges_timeline_element_get_child_property(
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
            let found: bool = from_glib(ffi::ges_timeline_element_lookup_child(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                ptr::null_mut(),
                ptr::null_mut(),
            ));
            if !found {
                return Err(glib::bool_error!("Child property not found"));
            }

            let value = value.to_value();
            ffi::ges_timeline_element_set_child_property(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            );

            Ok(())
        }
    }
}
