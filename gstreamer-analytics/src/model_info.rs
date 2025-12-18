// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ffi;
use crate::*;
use glib::translate::*;
use std::ptr;

use crate::{ModelInfo, ModelInfoTensorDirection, TensorDataType};

impl ModelInfo {
    #[doc(alias = "gst_analytics_modelinfo_find_tensor_name")]
    pub fn find_tensor_name(
        &self,
        dir: ModelInfoTensorDirection,
        index: usize,
        in_tensor_name: Option<&str>,
        data_type: TensorDataType,
        dims: &[usize],
    ) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gst_analytics_modelinfo_find_tensor_name(
                self.to_glib_none().0,
                dir.into_glib(),
                index,
                in_tensor_name.to_glib_none().0,
                data_type.into_glib(),
                dims.len(),
                dims.as_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_analytics_modelinfo_get_input_scales_offsets")]
    #[doc(alias = "get_input_scales_offsets")]
    pub fn input_scales_offsets(
        &self,
        tensor_name: &str,
        input_mins: &[f64],
        input_maxs: &[f64],
    ) -> Option<(glib::Slice<f64>, glib::Slice<f64>)> {
        unsafe {
            assert_eq!(input_mins.len(), input_maxs.len());

            let mut num_output_ranges = 0;
            let mut output_scales = ptr::null_mut();
            let mut output_offsets = ptr::null_mut();
            let res = from_glib(ffi::gst_analytics_modelinfo_get_input_scales_offsets(
                self.to_glib_none().0,
                tensor_name.to_glib_none().0,
                input_mins.len(),
                input_mins.as_ptr(),
                input_maxs.as_ptr(),
                &mut num_output_ranges,
                &mut output_scales,
                &mut output_offsets,
            ));
            if res {
                Some((
                    glib::Slice::from_glib_full_num(output_scales, num_output_ranges),
                    glib::Slice::from_glib_full_num(output_offsets, num_output_ranges),
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_analytics_modelinfo_get_target_ranges")]
    #[doc(alias = "get_target_ranges")]
    pub fn target_ranges(&self, tensor_name: &str) -> Option<(glib::Slice<f64>, glib::Slice<f64>)> {
        unsafe {
            let mut num_ranges = 0;
            let mut mins = ptr::null_mut();
            let mut maxs = ptr::null_mut();
            let res = from_glib(ffi::gst_analytics_modelinfo_get_target_ranges(
                mut_override(self.to_glib_none().0),
                tensor_name.to_glib_none().0,
                &mut num_ranges,
                &mut mins,
                &mut maxs,
            ));
            if res {
                Some((
                    glib::Slice::from_glib_full_num(mins, num_ranges),
                    glib::Slice::from_glib_full_num(maxs, num_ranges),
                ))
            } else {
                None
            }
        }
    }
}
