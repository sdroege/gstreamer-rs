// Copyright (C) 2018 Thibault Saunier <tsaunier@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;
use TimelineElement;

pub trait TimelineElementExtManual: 'static {
    fn get_child_property(&self, name: &str) -> Option<glib::Value>;
    fn set_child_property(&self, name: &str, value: &glib::ToValue) -> Result<(), glib::BoolError>;
}

impl<O: IsA<TimelineElement>> TimelineElementExtManual for O {
    fn get_child_property(&self, name: &str) -> Option<glib::Value> {
        unsafe {
            let found: bool = from_glib(ffi::ges_timeline_element_lookup_child(
                self.to_glib_none().0,
                name.to_glib_none().0,
                ptr::null_mut(),
                ptr::null_mut(),
            ));
            if !found {
                return None;
            }

            let mut value = glib::Value::uninitialized();
            ffi::ges_timeline_element_get_child_property(
                self.to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            Some(value)
        }
    }

    fn set_child_property(&self, name: &str, value: &glib::ToValue) -> Result<(), glib::BoolError> {
        unsafe {
            let found: bool = from_glib(ffi::ges_timeline_element_lookup_child(
                self.to_glib_none().0,
                name.to_glib_none().0,
                ptr::null_mut(),
                ptr::null_mut(),
            ));
            if !found {
                return Err(glib_bool_error!("Child property not found"));
            }

            let value = value.to_value();
            ffi::ges_timeline_element_set_child_property(
                self.to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            );

            Ok(())
        }
    }
}
