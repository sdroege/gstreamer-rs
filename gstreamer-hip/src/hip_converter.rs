// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{HipConverter, HipDevice, HipFenceData, HipStream, ffi};
use glib::{prelude::*, translate::*};

impl HipConverter {
    #[doc(alias = "gst_hip_converter_new")]
    pub fn new(
        device: &impl IsA<HipDevice>,
        in_info: &gst_video::VideoInfo,
        out_info: &gst_video::VideoInfo,
        config: Option<gst::Structure>,
    ) -> Result<Self, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<Self>::from_glib_full(ffi::gst_hip_converter_new(
                device.as_ref().to_glib_none().0,
                in_info.to_glib_none().0,
                out_info.to_glib_none().0,
                config
                    .map(|config| config.into_glib_ptr())
                    .unwrap_or(std::ptr::null_mut()),
            ))
            .ok_or_else(|| glib::bool_error!("Failed to create HIP converter"))
        }
    }
}

pub trait HipConverterExtManual: IsA<HipConverter> + 'static {
    #[doc(alias = "gst_hip_converter_convert_frame")]
    fn convert_frame(
        &self,
        input: &gst::BufferRef,
        output: &mut gst::BufferRef,
        fence_data: &HipFenceData,
        stream: Option<&HipStream>,
    ) -> Result<(), glib::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_hip_converter_convert_frame(
                    self.as_ref().to_glib_none().0,
                    input.as_mut_ptr(),
                    output.as_mut_ptr(),
                    fence_data.to_glib_none().0,
                    stream.to_glib_none().0
                ),
                "Failed to convert HIP frame"
            )
        }
    }
}

impl<O: IsA<HipConverter>> HipConverterExtManual for O {}
