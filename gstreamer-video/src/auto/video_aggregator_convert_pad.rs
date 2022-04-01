// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::VideoAggregatorPad;
use glib::object::IsA;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GstVideoAggregatorConvertPad")]
    pub struct VideoAggregatorConvertPad(Object<ffi::GstVideoAggregatorConvertPad, ffi::GstVideoAggregatorConvertPadClass>) @extends VideoAggregatorPad, gst_base::AggregatorPad, gst::Pad, gst::Object;

    match fn {
        type_ => || ffi::gst_video_aggregator_convert_pad_get_type(),
    }
}

impl VideoAggregatorConvertPad {
    pub const NONE: Option<&'static VideoAggregatorConvertPad> = None;
}

unsafe impl Send for VideoAggregatorConvertPad {}
unsafe impl Sync for VideoAggregatorConvertPad {}

pub trait VideoAggregatorConvertPadExt: 'static {
    #[doc(alias = "gst_video_aggregator_convert_pad_update_conversion_info")]
    fn update_conversion_info(&self);
}

impl<O: IsA<VideoAggregatorConvertPad>> VideoAggregatorConvertPadExt for O {
    fn update_conversion_info(&self) {
        unsafe {
            ffi::gst_video_aggregator_convert_pad_update_conversion_info(
                self.as_ref().to_glib_none().0,
            );
        }
    }
}