// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ClockTime;
use crate::ControlBinding;
use glib::prelude::*;
use glib::translate::*;

pub trait ControlBindingExtManual: 'static {
    #[doc(alias = "get_g_value_array")]
    fn g_value_array(
        &self,
        timestamp: ClockTime,
        interval: ClockTime,
        values: &mut [glib::Value],
    ) -> Result<(), glib::error::BoolError>;
}

impl<O: IsA<ControlBinding>> ControlBindingExtManual for O {
    fn g_value_array(
        &self,
        timestamp: ClockTime,
        interval: ClockTime,
        values: &mut [glib::Value],
    ) -> Result<(), glib::error::BoolError> {
        let n_values = values.len() as u32;
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_control_binding_get_g_value_array(
                    self.as_ref().to_glib_none().0,
                    timestamp.into_glib(),
                    interval.into_glib(),
                    n_values,
                    values.as_mut_ptr() as *mut glib::gobject_ffi::GValue,
                ),
                "Failed to get value array"
            )
        }
    }
}
