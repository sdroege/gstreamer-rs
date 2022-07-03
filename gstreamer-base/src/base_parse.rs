// Take a look at the license at the top of the repository in the LICENSE file.

use crate::BaseParse;
use crate::BaseParseFrame;
use glib::prelude::*;
use glib::translate::*;
use std::mem;

pub trait BaseParseExtManual: 'static {
    #[doc(alias = "get_sink_pad")]
    fn sink_pad(&self) -> &gst::Pad;
    #[doc(alias = "get_src_pad")]
    fn src_pad(&self) -> &gst::Pad;

    #[doc(alias = "gst_base_parse_set_duration")]
    fn set_duration(&self, duration: impl gst::FormattedValue, interval: u32);
    #[doc(alias = "gst_base_parse_set_frame_rate")]
    fn set_frame_rate(&self, fps: gst::Fraction, lead_in: u32, lead_out: u32);

    #[doc(alias = "gst_base_parse_convert_default")]
    fn convert_default<U: gst::SpecificFormattedValueFullRange>(
        &self,
        src_val: impl gst::FormattedValue,
    ) -> Option<U>;
    fn convert_default_generic(
        &self,
        src_val: impl gst::FormattedValue,
        dest_format: gst::Format,
    ) -> Option<gst::GenericFormattedValue>;

    #[doc(alias = "gst_base_parse_finish_frame")]
    fn finish_frame(
        &self,
        frame: BaseParseFrame,
        size: u32,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;
}

impl<O: IsA<BaseParse>> BaseParseExtManual for O {
    fn sink_pad(&self) -> &gst::Pad {
        unsafe {
            let elt = &*(self.as_ptr() as *const ffi::GstBaseParse);
            &*(&elt.sinkpad as *const *mut gst::ffi::GstPad as *const gst::Pad)
        }
    }

    fn src_pad(&self) -> &gst::Pad {
        unsafe {
            let elt = &*(self.as_ptr() as *const ffi::GstBaseParse);
            &*(&elt.srcpad as *const *mut gst::ffi::GstPad as *const gst::Pad)
        }
    }

    fn set_duration(&self, duration: impl gst::FormattedValue, interval: u32) {
        unsafe {
            ffi::gst_base_parse_set_duration(
                self.as_ref().to_glib_none().0,
                duration.format().into_glib(),
                duration.into_raw_value(),
                interval as i32,
            );
        }
    }

    fn set_frame_rate(&self, fps: gst::Fraction, lead_in: u32, lead_out: u32) {
        let (fps_num, fps_den) = fps.into();
        unsafe {
            ffi::gst_base_parse_set_frame_rate(
                self.as_ref().to_glib_none().0,
                fps_num as u32,
                fps_den as u32,
                lead_in,
                lead_out,
            );
        }
    }

    fn convert_default<U: gst::SpecificFormattedValueFullRange>(
        &self,
        src_val: impl gst::FormattedValue,
    ) -> Option<U> {
        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_base_parse_convert_default(
                self.as_ref().to_glib_none().0,
                src_val.format().into_glib(),
                src_val.into_raw_value(),
                U::default_format().into_glib(),
                dest_val.as_mut_ptr(),
            ));
            if ret {
                Some(U::from_raw(U::default_format(), dest_val.assume_init()))
            } else {
                None
            }
        }
    }

    fn convert_default_generic(
        &self,
        src_val: impl gst::FormattedValue,
        dest_format: gst::Format,
    ) -> Option<gst::GenericFormattedValue> {
        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_base_parse_convert_default(
                self.as_ref().to_glib_none().0,
                src_val.format().into_glib(),
                src_val.into_raw_value(),
                dest_format.into_glib(),
                dest_val.as_mut_ptr(),
            ));
            if ret {
                Some(gst::GenericFormattedValue::new(
                    dest_format,
                    dest_val.assume_init(),
                ))
            } else {
                None
            }
        }
    }

    fn finish_frame(
        &self,
        frame: BaseParseFrame,
        size: u32,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_base_parse_finish_frame(
                self.as_ref().to_glib_none().0,
                frame.to_glib_none().0,
                i32::try_from(size).expect("size higher than i32::MAX"),
            ))
        }
    }
}
