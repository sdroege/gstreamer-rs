// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::object::IsA;
use glib::translate::*;
use gst_sys;
use ClockTime;
use ControlBinding;

pub trait ControlBindingExtManual: 'static {
    fn get_g_value_array(
        &self,
        timestamp: ClockTime,
        interval: ClockTime,
        values: &mut [glib::Value],
    ) -> Result<(), glib::error::BoolError>;
}

impl<O: IsA<ControlBinding>> ControlBindingExtManual for O {
    fn get_g_value_array(
        &self,
        timestamp: ClockTime,
        interval: ClockTime,
        values: &mut [glib::Value],
    ) -> Result<(), glib::error::BoolError> {
        let n_values = values.len() as u32;
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_control_binding_get_g_value_array(
                    self.as_ref().to_glib_none().0,
                    timestamp.to_glib(),
                    interval.to_glib(),
                    n_values,
                    values.as_mut_ptr() as *mut gobject_sys::GValue,
                ),
                "Failed to get value array"
            )
        }
    }
}
