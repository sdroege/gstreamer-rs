// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};

use crate::{ffi, ClockTime, ControlBinding};

pub trait ControlBindingExtManual: IsA<ControlBinding> + 'static {
    fn property_name(&self) -> &glib::GStr {
        unsafe {
            let ptr: *mut ffi::GstControlBinding = self.as_ref().to_glib_none().0;
            glib::GStr::from_ptr((*ptr).name)
        }
    }

    fn property_spec(&self) -> &glib::ParamSpec {
        unsafe {
            let ptr: *mut ffi::GstControlBinding = self.as_ref().to_glib_none().0;
            glib::ParamSpec::from_glib_ptr_borrow(&(*ptr).pspec)
        }
    }

    #[doc(alias = "get_g_value_array")]
    #[doc(alias = "gst_control_binding_get_g_value_array")]
    fn g_value_array(
        &self,
        timestamp: ClockTime,
        interval: ClockTime,
        values: &mut [glib::Value],
    ) -> Result<(), glib::error::BoolError> {
        let n_values = values.len() as u32;
        unsafe {
            glib::result_from_gboolean!(
                crate::ffi::gst_control_binding_get_g_value_array(
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

impl<O: IsA<ControlBinding>> ControlBindingExtManual for O {}
