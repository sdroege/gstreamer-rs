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
use ControlSource;

pub trait ControlSourceExtManual: 'static {
    fn get_value_array(
        &self,
        timestamp: ClockTime,
        interval: ClockTime,
        values: &mut [f64],
    ) -> Result<(), glib::error::BoolError>;
}

impl<O: IsA<ControlSource>> ControlSourceExtManual for O {
    fn get_value_array(
        &self,
        timestamp: ClockTime,
        interval: ClockTime,
        values: &mut [f64],
    ) -> Result<(), glib::error::BoolError> {
        let n_values = values.len() as u32;
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_control_source_get_value_array(
                    self.as_ref().to_glib_none().0,
                    timestamp.to_glib(),
                    interval.to_glib(),
                    n_values,
                    values.to_glib_none().0,
                ),
                "Failed to get value array"
            )
        }
    }
}
