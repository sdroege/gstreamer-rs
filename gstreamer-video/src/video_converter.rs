// Copyright (C) 2020 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_video_sys;

use glib;
use glib::translate::ToGlibPtr;
use gst;

use std::convert;
use std::ops;
use std::ptr;

#[derive(Debug)]
pub struct VideoConverter(ptr::NonNull<gst_video_sys::GstVideoConverter>);

impl Drop for VideoConverter {
    fn drop(&mut self) {
        unsafe {
            gst_video_sys::gst_video_converter_free(self.0.as_ptr());
        }
    }
}

unsafe impl Send for VideoConverter {}
unsafe impl Sync for VideoConverter {}

impl VideoConverter {
    pub fn new(
        in_info: &::VideoInfo,
        out_info: &::VideoInfo,
        config: Option<VideoConverterConfig>,
    ) -> Result<Self, glib::BoolError> {
        assert_initialized_main_thread!();
        if in_info.fps() != out_info.fps() {
            return Err(glib_bool_error!("Can't do framerate conversion"));
        }

        if in_info.interlace_mode() != out_info.interlace_mode() {
            return Err(glib_bool_error!("Can't do interlacing conversion"));
        }

        unsafe {
            let ptr = gst_video_sys::gst_video_converter_new(
                in_info.to_glib_none().0 as *mut _,
                out_info.to_glib_none().0 as *mut _,
                config.map(|s| s.0.into_ptr()).unwrap_or(ptr::null_mut()),
            );
            if ptr.is_null() {
                Err(glib_bool_error!("Failed to create video converter"))
            } else {
                Ok(VideoConverter(ptr::NonNull::new_unchecked(ptr)))
            }
        }
    }

    pub fn get_config(&self) -> VideoConverterConfig {
        unsafe {
            VideoConverterConfig(
                gst::StructureRef::from_glib_borrow(gst_video_sys::gst_video_converter_get_config(
                    self.0.as_ptr(),
                ))
                .to_owned(),
            )
        }
    }

    pub fn set_config(&mut self, config: VideoConverterConfig) {
        unsafe {
            gst_video_sys::gst_video_converter_set_config(self.0.as_ptr(), config.0.into_ptr());
        }
    }

    pub fn frame<T>(
        &self,
        src: &::VideoFrame<T>,
        dest: &mut ::VideoFrame<::video_frame::Writable>,
    ) {
        unsafe {
            gst_video_sys::gst_video_converter_frame(
                self.0.as_ptr(),
                src.as_ptr(),
                dest.as_mut_ptr(),
            );
        }
    }

    pub fn frame_ref<T>(
        &self,
        src: &::VideoFrameRef<T>,
        dest: &mut ::VideoFrameRef<&mut gst::BufferRef>,
    ) {
        unsafe {
            gst_video_sys::gst_video_converter_frame(
                self.0.as_ptr(),
                src.as_ptr(),
                dest.as_mut_ptr(),
            );
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoConverterConfig(gst::Structure);

impl ops::Deref for VideoConverterConfig {
    type Target = gst::StructureRef;

    fn deref(&self) -> &gst::StructureRef {
        self.0.deref()
    }
}

impl ops::DerefMut for VideoConverterConfig {
    fn deref_mut(&mut self) -> &mut gst::StructureRef {
        self.0.deref_mut()
    }
}

impl AsRef<gst::StructureRef> for VideoConverterConfig {
    fn as_ref(&self) -> &gst::StructureRef {
        self.0.as_ref()
    }
}

impl AsMut<gst::StructureRef> for VideoConverterConfig {
    fn as_mut(&mut self) -> &mut gst::StructureRef {
        self.0.as_mut()
    }
}

impl Default for VideoConverterConfig {
    fn default() -> Self {
        VideoConverterConfig::new()
    }
}

impl convert::TryFrom<gst::Structure> for VideoConverterConfig {
    type Error = glib::BoolError;

    fn try_from(v: gst::Structure) -> Result<VideoConverterConfig, Self::Error> {
        skip_assert_initialized!();
        if v.get_name() == "GstVideoConverter" {
            Ok(VideoConverterConfig(v))
        } else {
            Err(glib_bool_error!("Structure is no VideoConverterConfig"))
        }
    }
}

impl<'a> convert::TryFrom<&'a gst::StructureRef> for VideoConverterConfig {
    type Error = glib::BoolError;

    fn try_from(v: &'a gst::StructureRef) -> Result<VideoConverterConfig, Self::Error> {
        skip_assert_initialized!();
        VideoConverterConfig::try_from(v.to_owned())
    }
}

impl From<VideoConverterConfig> for gst::Structure {
    fn from(v: VideoConverterConfig) -> gst::Structure {
        skip_assert_initialized!();
        v.0
    }
}

impl VideoConverterConfig {
    pub fn new() -> Self {
        VideoConverterConfig(gst::Structure::new_empty("GstVideoConverter"))
    }

    pub fn set_resampler_method(&mut self, v: ::VideoResamplerMethod) {
        self.0.set("GstVideoConverter.resampler-method", &v);
    }

    pub fn get_resampler_method(&self) -> ::VideoResamplerMethod {
        self.0
            .get_optional("GstVideoConverter.resampler-method")
            .expect("Wrong type")
            .unwrap_or(::VideoResamplerMethod::Cubic)
    }

    pub fn set_chroma_resampler_method(&mut self, v: ::VideoResamplerMethod) {
        self.0.set("GstVideoConverter.chroma-resampler-method", &v);
    }

    pub fn get_chroma_resampler_method(&self) -> ::VideoResamplerMethod {
        self.0
            .get_optional("GstVideoConverter.chroma-resampler-method")
            .expect("Wrong type")
            .unwrap_or(::VideoResamplerMethod::Linear)
    }

    pub fn set_resampler_taps(&mut self, v: u32) {
        self.0.set("GstVideoConverter.resampler-taps", &v);
    }

    pub fn get_resampler_taps(&self) -> u32 {
        self.0
            .get_optional("GstVideoConverter.resampler-taps")
            .expect("Wrong type")
            .unwrap_or(0)
    }

    pub fn set_dither_method(&mut self, v: ::VideoDitherMethod) {
        self.0.set("GstVideoConverter.dither-method", &v);
    }

    pub fn get_dither_method(&self) -> ::VideoDitherMethod {
        self.0
            .get_optional("GstVideoConverter.dither-method")
            .expect("Wrong type")
            .unwrap_or(::VideoDitherMethod::Bayer)
    }

    pub fn set_dither_quantization(&mut self, v: u32) {
        self.0.set("GstVideoConverter.dither-quantization", &v);
    }

    pub fn get_dither_quantization(&self) -> u32 {
        self.0
            .get_optional("GstVideoConverter.dither-quantization")
            .expect("Wrong type")
            .unwrap_or(1)
    }

    pub fn set_src_x(&mut self, v: i32) {
        self.0.set("GstVideoConverter.src-x", &v);
    }

    pub fn get_src_x(&self) -> i32 {
        self.0
            .get_optional("GstVideoConverter.src-x")
            .expect("Wrong type")
            .unwrap_or(0)
    }

    pub fn set_src_y(&mut self, v: i32) {
        self.0.set("GstVideoConverter.src-y", &v);
    }

    pub fn get_src_y(&self) -> i32 {
        self.0
            .get_optional("GstVideoConverter.src-y")
            .expect("Wrong type")
            .unwrap_or(0)
    }

    pub fn set_src_width(&mut self, v: Option<i32>) {
        if let Some(v) = v {
            self.0.set("GstVideoConverter.src-width", &v);
        } else {
            self.0.remove_field("GstVideoConverter.src-width");
        }
    }

    pub fn get_src_width(&self) -> Option<i32> {
        self.0
            .get_optional("GstVideoConverter.src-width")
            .expect("Wrong type")
    }

    pub fn set_src_height(&mut self, v: Option<i32>) {
        if let Some(v) = v {
            self.0.set("GstVideoConverter.src-height", &v);
        } else {
            self.0.remove_field("GstVideoConverter.src-height");
        }
    }

    pub fn get_src_height(&self) -> Option<i32> {
        self.0
            .get_optional("GstVideoConverter.src-height")
            .expect("Wrong type")
    }

    pub fn set_dest_x(&mut self, v: i32) {
        self.0.set("GstVideoConverter.dest-x", &v);
    }

    pub fn get_dest_x(&self) -> i32 {
        self.0
            .get_optional("GstVideoConverter.dest-x")
            .expect("Wrong type")
            .unwrap_or(0)
    }

    pub fn set_dest_y(&mut self, v: i32) {
        self.0.set("GstVideoConverter.dest-y", &v);
    }

    pub fn get_dest_y(&self) -> i32 {
        self.0
            .get_optional("GstVideoConverter.dest-y")
            .expect("Wrong type")
            .unwrap_or(0)
    }

    pub fn set_dest_width(&mut self, v: Option<i32>) {
        if let Some(v) = v {
            self.0.set("GstVideoConverter.dest-width", &v);
        } else {
            self.0.remove_field("GstVideoConverter.dest-width");
        }
    }

    pub fn get_dest_width(&self) -> Option<i32> {
        self.0
            .get_optional("GstVideoConverter.dest-width")
            .expect("Wrong type")
    }

    pub fn set_dest_height(&mut self, v: Option<i32>) {
        if let Some(v) = v {
            self.0.set("GstVideoConverter.dest-height", &v);
        } else {
            self.0.remove_field("GstVideoConverter.dest-height");
        }
    }

    pub fn get_dest_height(&self) -> Option<i32> {
        self.0
            .get_optional("GstVideoConverter.dest-height")
            .expect("Wrong type")
    }

    pub fn set_fill_border(&mut self, v: bool) {
        self.0.set("GstVideoConverter.fill-border", &v);
    }

    pub fn get_fill_border(&self) -> bool {
        self.0
            .get_optional("GstVideoConverter.fill-border")
            .expect("Wrong type")
            .unwrap_or(true)
    }

    pub fn set_alpha_value(&mut self, v: f64) {
        self.0.set("GstVideoConverter.alpha-value", &v);
    }

    pub fn get_alpha_value(&self) -> f64 {
        self.0
            .get_optional("GstVideoConverter.alpha-value")
            .expect("Wrong type")
            .unwrap_or(1.0)
    }

    pub fn set_alpha_mode(&mut self, v: ::VideoAlphaMode) {
        self.0.set("GstVideoConverter.alpha-mode", &v);
    }

    pub fn get_alpha_mode(&self) -> ::VideoAlphaMode {
        self.0
            .get_optional("GstVideoConverter.alpha-mode")
            .expect("Wrong type")
            .unwrap_or(::VideoAlphaMode::Copy)
    }

    pub fn set_border_argb(&mut self, v: u32) {
        self.0.set("GstVideoConverter.border-argb", &v);
    }

    pub fn get_border_argb(&self) -> u32 {
        self.0
            .get_optional("GstVideoConverter.border-argb")
            .expect("Wrong type")
            .unwrap_or(0xff_00_00_00)
    }

    pub fn set_chroma_mode(&mut self, v: ::VideoChromaMode) {
        self.0.set("GstVideoConverter.chroma-mode", &v);
    }

    pub fn get_chroma_mode(&self) -> ::VideoChromaMode {
        self.0
            .get_optional("GstVideoConverter.chroma-mode")
            .expect("Wrong type")
            .unwrap_or(::VideoChromaMode::Full)
    }

    pub fn set_matrix_mode(&mut self, v: ::VideoMatrixMode) {
        self.0.set("GstVideoConverter.matrix-mode", &v);
    }

    pub fn get_matrix_mode(&self) -> ::VideoMatrixMode {
        self.0
            .get_optional("GstVideoConverter.matrix-mode")
            .expect("Wrong type")
            .unwrap_or(::VideoMatrixMode::Full)
    }

    pub fn set_gamma_mode(&mut self, v: ::VideoGammaMode) {
        self.0.set("GstVideoConverter.gamma-mode", &v);
    }

    pub fn get_gamma_mode(&self) -> ::VideoGammaMode {
        self.0
            .get_optional("GstVideoConverter.gamma-mode")
            .expect("Wrong type")
            .unwrap_or(::VideoGammaMode::None)
    }

    pub fn set_primaries_mode(&mut self, v: ::VideoPrimariesMode) {
        self.0.set("GstVideoConverter.primaries-mode", &v);
    }

    pub fn get_primaries_mode(&self) -> ::VideoPrimariesMode {
        self.0
            .get_optional("GstVideoConverter.primaries-mode")
            .expect("Wrong type")
            .unwrap_or(::VideoPrimariesMode::None)
    }

    pub fn set_threads(&mut self, v: u32) {
        self.0.set("GstVideoConverter.threads", &v);
    }

    pub fn get_threads(&self) -> u32 {
        self.0
            .get_optional("GstVideoConverter.threads")
            .expect("Wrong type")
            .unwrap_or(1)
    }
}
