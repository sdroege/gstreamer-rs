// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib_sys;
use gobject_sys;
use gst_video_sys;

use glib;
use glib::translate::{
    from_glib, from_glib_full, from_glib_none, FromGlib, FromGlibPtrFull, FromGlibPtrNone, ToGlib,
    ToGlibPtr, ToGlibPtrMut,
};
use gst;
use gst::prelude::*;

use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::ptr;
use std::str;

pub const VIDEO_MAX_PLANES: usize = gst_video_sys::GST_VIDEO_MAX_PLANES as usize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum VideoColorRange {
    Unknown,
    Range0255,
    Range16235,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for VideoColorRange {
    type GlibType = gst_video_sys::GstVideoColorRange;

    fn to_glib(&self) -> gst_video_sys::GstVideoColorRange {
        match *self {
            VideoColorRange::Unknown => gst_video_sys::GST_VIDEO_COLOR_RANGE_UNKNOWN,
            VideoColorRange::Range0255 => gst_video_sys::GST_VIDEO_COLOR_RANGE_0_255,
            VideoColorRange::Range16235 => gst_video_sys::GST_VIDEO_COLOR_RANGE_16_235,
            VideoColorRange::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<gst_video_sys::GstVideoColorRange> for VideoColorRange {
    fn from_glib(value: gst_video_sys::GstVideoColorRange) -> Self {
        skip_assert_initialized!();
        match value as i32 {
            0 => VideoColorRange::Unknown,
            1 => VideoColorRange::Range0255,
            2 => VideoColorRange::Range16235,
            value => VideoColorRange::__Unknown(value),
        }
    }
}

impl glib::StaticType for VideoColorRange {
    fn static_type() -> glib::Type {
        unsafe { from_glib(gst_video_sys::gst_video_color_range_get_type()) }
    }
}

impl<'a> glib::value::FromValueOptional<'a> for VideoColorRange {
    unsafe fn from_value_optional(value: &glib::value::Value) -> Option<Self> {
        Some(glib::value::FromValue::from_value(value))
    }
}

impl<'a> glib::value::FromValue<'a> for VideoColorRange {
    unsafe fn from_value(value: &glib::value::Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl glib::value::SetValue for VideoColorRange {
    unsafe fn set_value(value: &mut glib::value::Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib() as i32)
    }
}

pub struct VideoColorimetry(gst_video_sys::GstVideoColorimetry);

impl VideoColorimetry {
    pub fn new(
        range: VideoColorRange,
        matrix: ::VideoColorMatrix,
        transfer: ::VideoTransferFunction,
        primaries: ::VideoColorPrimaries,
    ) -> Self {
        assert_initialized_main_thread!();

        let colorimetry = unsafe {
            let mut colorimetry: gst_video_sys::GstVideoColorimetry = mem::zeroed();

            colorimetry.range = range.to_glib();
            colorimetry.matrix = matrix.to_glib();
            colorimetry.transfer = transfer.to_glib();
            colorimetry.primaries = primaries.to_glib();

            colorimetry
        };

        VideoColorimetry(colorimetry)
    }

    pub fn range(&self) -> ::VideoColorRange {
        from_glib(self.0.range)
    }

    pub fn matrix(&self) -> ::VideoColorMatrix {
        from_glib(self.0.matrix)
    }

    pub fn transfer(&self) -> ::VideoTransferFunction {
        from_glib(self.0.transfer)
    }

    pub fn primaries(&self) -> ::VideoColorPrimaries {
        from_glib(self.0.primaries)
    }
}

impl Clone for VideoColorimetry {
    fn clone(&self) -> Self {
        unsafe { VideoColorimetry(ptr::read(&self.0)) }
    }
}

impl PartialEq for VideoColorimetry {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            from_glib(gst_video_sys::gst_video_colorimetry_is_equal(
                &self.0, &other.0,
            ))
        }
    }
}

impl Eq for VideoColorimetry {}

impl str::FromStr for ::VideoColorimetry {
    type Err = glib::error::BoolError;

    fn from_str(s: &str) -> Result<Self, glib::error::BoolError> {
        assert_initialized_main_thread!();

        unsafe {
            let mut colorimetry = mem::MaybeUninit::zeroed();
            let valid: bool = from_glib(gst_video_sys::gst_video_colorimetry_from_string(
                colorimetry.as_mut_ptr(),
                s.to_glib_none().0,
            ));
            if valid {
                Ok(VideoColorimetry(colorimetry.assume_init()))
            } else {
                Err(glib_bool_error!("Invalid colorimetry info"))
            }
        }
    }
}

impl fmt::Debug for ::VideoColorimetry {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("VideoColorimetry")
            .field("range", &self.0.range)
            .field("matrix", &self.0.matrix)
            .field("transfer", &self.0.transfer)
            .field("primaries", &self.0.primaries)
            .finish()
    }
}

impl fmt::Display for ::VideoColorimetry {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let s = unsafe {
            glib::GString::from_glib_full(gst_video_sys::gst_video_colorimetry_to_string(&self.0))
        };
        f.write_str(&s)
    }
}

impl str::FromStr for ::VideoChromaSite {
    type Err = glib::error::BoolError;

    fn from_str(s: &str) -> Result<Self, glib::error::BoolError> {
        assert_initialized_main_thread!();

        unsafe {
            let chroma_site = from_glib(gst_video_sys::gst_video_chroma_from_string(
                s.to_glib_none().0,
            ));
            if chroma_site == ::VideoChromaSite::empty() {
                Err(glib_bool_error!("Invalid chroma site"))
            } else {
                Ok(chroma_site)
            }
        }
    }
}

impl fmt::Display for ::VideoChromaSite {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let s = unsafe {
            glib::GString::from_glib_full(gst_video_sys::gst_video_chroma_to_string(self.to_glib()))
        };
        f.write_str(&s)
    }
}

impl ::VideoTransferFunction {
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    pub fn from_iso(iso: u32) -> Result<::VideoTransferFunction, glib::BoolError> {
        assert_initialized_main_thread!();

        unsafe {
            let value = from_glib(gst_video_sys::gst_video_transfer_function_from_iso(iso));
            match value {
                ::VideoTransferFunction::__Unknown(_) => Err(glib_bool_error!("Invalid ISO value")),
                _ => Ok(value),
            }
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    pub fn to_iso(&self) -> u32 {
        unsafe { gst_video_sys::gst_video_transfer_function_to_iso(self.to_glib()) }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    pub fn is_equivalent(
        &self,
        from_bpp: u32,
        to_func: ::VideoTransferFunction,
        to_bpp: u32,
    ) -> bool {
        unsafe {
            from_glib(gst_video_sys::gst_video_transfer_function_is_equivalent(
                self.to_glib(),
                from_bpp,
                to_func.to_glib(),
                to_bpp,
            ))
        }
    }
}

impl ::VideoColorMatrix {
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    pub fn from_iso(iso: u32) -> Result<::VideoColorMatrix, glib::BoolError> {
        assert_initialized_main_thread!();

        unsafe {
            let value = from_glib(gst_video_sys::gst_video_color_matrix_from_iso(iso));
            match value {
                ::VideoColorMatrix::__Unknown(_) => Err(glib_bool_error!("Invalid ISO value")),
                _ => Ok(value),
            }
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    pub fn to_iso(&self) -> u32 {
        unsafe { gst_video_sys::gst_video_color_matrix_to_iso(self.to_glib()) }
    }
}

impl ::VideoColorPrimaries {
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    pub fn from_iso(iso: u32) -> Result<::VideoColorPrimaries, glib::BoolError> {
        assert_initialized_main_thread!();

        unsafe {
            let value = from_glib(gst_video_sys::gst_video_color_primaries_from_iso(iso));
            match value {
                ::VideoColorPrimaries::__Unknown(_) => Err(glib_bool_error!("Invalid ISO value")),
                _ => Ok(value),
            }
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    pub fn to_iso(&self) -> u32 {
        unsafe { gst_video_sys::gst_video_color_primaries_to_iso(self.to_glib()) }
    }
}

impl From<::VideoMultiviewFramePacking> for ::VideoMultiviewMode {
    fn from(v: ::VideoMultiviewFramePacking) -> Self {
        skip_assert_initialized!();
        from_glib(v.to_glib())
    }
}

impl std::convert::TryFrom<::VideoMultiviewMode> for ::VideoMultiviewFramePacking {
    type Error = glib::BoolError;

    fn try_from(v: ::VideoMultiviewMode) -> Result<::VideoMultiviewFramePacking, glib::BoolError> {
        skip_assert_initialized!();

        let v2 = from_glib(v.to_glib());

        if let ::VideoMultiviewFramePacking::__Unknown(_) = v2 {
            Err(glib_bool_error!("Invalid frame packing mode"))
        } else {
            Ok(v2)
        }
    }
}

pub struct VideoInfo(pub(crate) gst_video_sys::GstVideoInfo);

impl fmt::Debug for VideoInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut b = f.debug_struct("VideoInfo");

        b.field("format", &self.format())
            .field("format-info", &self.format_info())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("interlace_mode", &self.interlace_mode())
            .field("flags", &self.flags())
            .field("size", &self.size())
            .field("views", &self.views())
            .field("chroma_site", &self.chroma_site())
            .field("colorimetry", &self.colorimetry())
            .field("par", &self.par())
            .field("fps", &self.fps())
            .field("offset", &self.offset())
            .field("stride", &self.stride())
            .field("multiview_mode", &self.multiview_mode())
            .field("multiview_flags", &self.multiview_flags());

        #[cfg(any(feature = "v1_12", feature = "dox"))]
        {
            b.field("field_order", &self.field_order());
        };

        b.finish()
    }
}

#[derive(Debug)]
pub struct VideoInfoBuilder<'a> {
    format: ::VideoFormat,
    width: u32,
    height: u32,
    interlace_mode: Option<::VideoInterlaceMode>,
    flags: Option<::VideoFlags>,
    size: Option<usize>,
    views: Option<u32>,
    chroma_site: Option<::VideoChromaSite>,
    colorimetry: Option<&'a ::VideoColorimetry>,
    par: Option<gst::Fraction>,
    fps: Option<gst::Fraction>,
    offset: Option<&'a [usize]>,
    stride: Option<&'a [i32]>,
    multiview_mode: Option<::VideoMultiviewMode>,
    multiview_flags: Option<::VideoMultiviewFlags>,
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    field_order: Option<::VideoFieldOrder>,
}

impl<'a> VideoInfoBuilder<'a> {
    pub fn build(self) -> Result<VideoInfo, glib::error::BoolError> {
        unsafe {
            let mut info = mem::MaybeUninit::uninit();

            #[cfg(not(feature = "v1_12"))]
            let res: bool = {
                // The bool return value is new with 1.11.1, see
                // https://gitlab.freedesktop.org/gstreamer/gst-plugins-base/commit/17cdd369e6f2f73329d27dfceb50011f40f1ceb0
                let res = if gst::version() < (1, 11, 1, 0) {
                    gst_video_sys::gst_video_info_set_format(
                        info.as_mut_ptr(),
                        self.format.to_glib(),
                        self.width,
                        self.height,
                    );

                    true
                } else {
                    from_glib(gst_video_sys::gst_video_info_set_format(
                        info.as_mut_ptr(),
                        self.format.to_glib(),
                        self.width,
                        self.height,
                    ))
                };

                if res {
                    if let Some(interlace_mode) = self.interlace_mode {
                        let info = info.as_mut_ptr();
                        (*info).interlace_mode = interlace_mode.to_glib();
                    }
                }

                res
            };
            #[cfg(all(feature = "v1_12", not(feature = "v1_16")))]
            let res: bool = {
                let res = from_glib(gst_video_sys::gst_video_info_set_format(
                    info.as_mut_ptr(),
                    self.format.to_glib(),
                    self.width,
                    self.height,
                ));

                if res {
                    if let Some(interlace_mode) = self.interlace_mode {
                        let info = info.as_mut_ptr();
                        (*info).interlace_mode = interlace_mode.to_glib();
                    }
                }

                res
            };
            #[cfg(feature = "v1_16")]
            let res: bool = {
                from_glib(if let Some(interlace_mode) = self.interlace_mode {
                    gst_video_sys::gst_video_info_set_interlaced_format(
                        info.as_mut_ptr(),
                        self.format.to_glib(),
                        interlace_mode.to_glib(),
                        self.width,
                        self.height,
                    )
                } else {
                    gst_video_sys::gst_video_info_set_format(
                        info.as_mut_ptr(),
                        self.format.to_glib(),
                        self.width,
                        self.height,
                    )
                })
            };

            if !res {
                return Err(glib_bool_error!("Failed to build VideoInfo"));
            }

            let mut info = info.assume_init();

            if info.finfo.is_null() || info.width <= 0 || info.height <= 0 {
                return Err(glib_bool_error!("Failed to build VideoInfo"));
            }

            if let Some(flags) = self.flags {
                info.flags = flags.to_glib();
            }

            if let Some(size) = self.size {
                info.size = size;
            }

            if let Some(views) = self.views {
                info.views = views as i32;
            }

            if let Some(chroma_site) = self.chroma_site {
                info.chroma_site = chroma_site.to_glib();
            }

            if let Some(colorimetry) = self.colorimetry {
                ptr::write(&mut info.colorimetry, ptr::read(&colorimetry.0));
            }

            if let Some(par) = self.par {
                info.par_n = *par.numer();
                info.par_d = *par.denom();
            }

            if let Some(fps) = self.fps {
                info.fps_n = *fps.numer();
                info.fps_d = *fps.denom();
            }

            if let Some(offset) = self.offset {
                if offset.len() != ((*info.finfo).n_planes as usize) {
                    return Err(glib_bool_error!("Failed to build VideoInfo"));
                }

                let n_planes = (*info.finfo).n_planes as usize;
                info.offset[..n_planes].copy_from_slice(&offset[..n_planes]);
            }

            if let Some(stride) = self.stride {
                if stride.len() != ((*info.finfo).n_planes as usize) {
                    return Err(glib_bool_error!("Failed to build VideoInfo"));
                }

                let n_planes = (*info.finfo).n_planes as usize;
                info.stride[..n_planes].copy_from_slice(&stride[..n_planes]);
            }

            if let Some(multiview_mode) = self.multiview_mode {
                let ptr = &mut info.ABI._gst_reserved as *mut _ as *mut i32;
                ptr::write(ptr.offset(0), multiview_mode.to_glib());
            }

            if let Some(multiview_flags) = self.multiview_flags {
                let ptr = &mut info.ABI._gst_reserved as *mut _ as *mut u32;
                ptr::write(ptr.offset(1), multiview_flags.to_glib());
            }

            #[cfg(any(feature = "v1_12", feature = "dox"))]
            {
                if let Some(field_order) = self.field_order {
                    let ptr = &mut info.ABI._gst_reserved as *mut _ as *mut i32;
                    ptr::write(ptr.offset(2), field_order.to_glib());
                }
            }

            Ok(VideoInfo(info))
        }
    }

    pub fn interlace_mode(self, interlace_mode: ::VideoInterlaceMode) -> VideoInfoBuilder<'a> {
        Self {
            interlace_mode: Some(interlace_mode),
            ..self
        }
    }

    pub fn flags(self, flags: ::VideoFlags) -> Self {
        Self {
            flags: Some(flags),
            ..self
        }
    }

    pub fn size(self, size: usize) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }

    pub fn views(self, views: u32) -> Self {
        Self {
            views: Some(views),
            ..self
        }
    }

    pub fn chroma_site(self, chroma_site: ::VideoChromaSite) -> Self {
        Self {
            chroma_site: Some(chroma_site),
            ..self
        }
    }

    pub fn colorimetry(self, colorimetry: &'a ::VideoColorimetry) -> VideoInfoBuilder<'a> {
        Self {
            colorimetry: Some(colorimetry),
            ..self
        }
    }

    pub fn par<T: Into<gst::Fraction>>(self, par: T) -> Self {
        Self {
            par: Some(par.into()),
            ..self
        }
    }

    pub fn fps<T: Into<gst::Fraction>>(self, fps: T) -> Self {
        Self {
            fps: Some(fps.into()),
            ..self
        }
    }

    pub fn offset(self, offset: &'a [usize]) -> VideoInfoBuilder<'a> {
        Self {
            offset: Some(offset),
            ..self
        }
    }

    pub fn stride(self, stride: &'a [i32]) -> VideoInfoBuilder<'a> {
        Self {
            stride: Some(stride),
            ..self
        }
    }

    pub fn multiview_mode(self, multiview_mode: ::VideoMultiviewMode) -> Self {
        Self {
            multiview_mode: Some(multiview_mode),
            ..self
        }
    }

    pub fn multiview_flags(self, multiview_flags: ::VideoMultiviewFlags) -> Self {
        Self {
            multiview_flags: Some(multiview_flags),
            ..self
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn field_order(self, field_order: ::VideoFieldOrder) -> Self {
        Self {
            field_order: Some(field_order),
            ..self
        }
    }
}

impl VideoInfo {
    pub fn builder<'a>(format: ::VideoFormat, width: u32, height: u32) -> VideoInfoBuilder<'a> {
        assert_initialized_main_thread!();

        #[cfg(not(any(feature = "v1_12", feature = "dox")))]
        {
            VideoInfoBuilder {
                format,
                width,
                height,
                interlace_mode: None,
                flags: None,
                size: None,
                views: None,
                chroma_site: None,
                colorimetry: None,
                par: None,
                fps: None,
                offset: None,
                stride: None,
                multiview_mode: None,
                multiview_flags: None,
            }
        }
        #[cfg(any(feature = "v1_12", feature = "dox"))]
        {
            VideoInfoBuilder {
                format,
                width,
                height,
                interlace_mode: None,
                flags: None,
                size: None,
                views: None,
                chroma_site: None,
                colorimetry: None,
                par: None,
                fps: None,
                offset: None,
                stride: None,
                multiview_mode: None,
                multiview_flags: None,
                field_order: None,
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.0.finfo.is_null() && self.0.width > 0 && self.0.height > 0 && self.0.size > 0
    }

    pub fn from_caps(caps: &gst::CapsRef) -> Result<Self, glib::error::BoolError> {
        skip_assert_initialized!();

        unsafe {
            let mut info = mem::MaybeUninit::uninit();
            if from_glib(gst_video_sys::gst_video_info_from_caps(
                info.as_mut_ptr(),
                caps.as_ptr(),
            )) {
                Ok(VideoInfo(info.assume_init()))
            } else {
                Err(glib_bool_error!("Failed to create VideoInfo from caps"))
            }
        }
    }

    pub fn to_caps(&self) -> Result<gst::Caps, glib::error::BoolError> {
        unsafe {
            let result = from_glib_full(gst_video_sys::gst_video_info_to_caps(
                &self.0 as *const _ as *mut _,
            ));
            match result {
                Some(c) => Ok(c),
                None => Err(glib_bool_error!("Failed to create caps from VideoInfo")),
            }
        }
    }

    pub fn format(&self) -> ::VideoFormat {
        if self.0.finfo.is_null() {
            return ::VideoFormat::Unknown;
        }

        unsafe { from_glib((*self.0.finfo).format) }
    }

    pub fn format_info(&self) -> ::VideoFormatInfo {
        ::VideoFormatInfo::from_format(self.format())
    }

    pub fn width(&self) -> u32 {
        self.0.width as u32
    }

    pub fn height(&self) -> u32 {
        self.0.height as u32
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn field_height(&self) -> u32 {
        if self.0.interlace_mode == gst_video_sys::GST_VIDEO_INTERLACE_MODE_ALTERNATE {
            (self.0.height as u32 + 1) / 2
        } else {
            self.0.height as u32
        }
    }

    pub fn interlace_mode(&self) -> ::VideoInterlaceMode {
        from_glib(self.0.interlace_mode)
    }

    pub fn flags(&self) -> ::VideoFlags {
        from_glib(self.0.flags)
    }

    pub fn size(&self) -> usize {
        self.0.size
    }

    pub fn views(&self) -> u32 {
        self.0.views as u32
    }

    pub fn chroma_site(&self) -> ::VideoChromaSite {
        from_glib(self.0.chroma_site)
    }

    pub fn colorimetry(&self) -> VideoColorimetry {
        unsafe { VideoColorimetry(ptr::read(&self.0.colorimetry)) }
    }

    pub fn par(&self) -> gst::Fraction {
        gst::Fraction::new(self.0.par_n, self.0.par_d)
    }

    pub fn fps(&self) -> gst::Fraction {
        gst::Fraction::new(self.0.fps_n, self.0.fps_d)
    }

    pub fn offset(&self) -> &[usize] {
        &self.0.offset[0..(self.format_info().n_planes() as usize)]
    }

    pub fn stride(&self) -> &[i32] {
        &self.0.stride[0..(self.format_info().n_planes() as usize)]
    }

    pub fn multiview_mode(&self) -> ::VideoMultiviewMode {
        unsafe {
            let ptr = &self.0.ABI._gst_reserved as *const _ as *const i32;
            from_glib(ptr::read(ptr.offset(0)))
        }
    }

    pub fn multiview_flags(&self) -> ::VideoMultiviewFlags {
        unsafe {
            let ptr = &self.0.ABI._gst_reserved as *const _ as *const u32;
            from_glib(ptr::read(ptr.offset(1)))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn field_order(&self) -> ::VideoFieldOrder {
        unsafe {
            let ptr = &self.0.ABI._gst_reserved as *const _ as *const i32;
            from_glib(ptr::read(ptr.offset(2)))
        }
    }

    pub fn has_alpha(&self) -> bool {
        self.format_info().has_alpha()
    }

    pub fn is_gray(&self) -> bool {
        self.format_info().is_gray()
    }

    pub fn is_rgb(&self) -> bool {
        self.format_info().is_rgb()
    }

    pub fn is_yuv(&self) -> bool {
        self.format_info().is_yuv()
    }

    pub fn is_interlaced(&self) -> bool {
        self.interlace_mode() != ::VideoInterlaceMode::Progressive
    }

    pub fn n_planes(&self) -> u32 {
        self.format_info().n_planes()
    }

    pub fn n_components(&self) -> u32 {
        self.format_info().n_components()
    }

    pub fn convert<V: Into<gst::GenericFormattedValue>, U: gst::SpecificFormattedValue>(
        &self,
        src_val: V,
    ) -> Option<U> {
        skip_assert_initialized!();

        let src_val = src_val.into();
        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            if from_glib(gst_video_sys::gst_video_info_convert(
                &self.0 as *const _ as *mut _,
                src_val.get_format().to_glib(),
                src_val.to_raw_value(),
                U::get_default_format().to_glib(),
                dest_val.as_mut_ptr(),
            )) {
                Some(U::from_raw(U::get_default_format(), dest_val.assume_init()))
            } else {
                None
            }
        }
    }

    pub fn convert_generic<V: Into<gst::GenericFormattedValue>>(
        &self,
        src_val: V,
        dest_fmt: gst::Format,
    ) -> Option<gst::GenericFormattedValue> {
        skip_assert_initialized!();

        let src_val = src_val.into();
        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            if from_glib(gst_video_sys::gst_video_info_convert(
                &self.0 as *const _ as *mut _,
                src_val.get_format().to_glib(),
                src_val.to_raw_value(),
                dest_fmt.to_glib(),
                dest_val.as_mut_ptr(),
            )) {
                Some(gst::GenericFormattedValue::new(
                    dest_fmt,
                    dest_val.assume_init(),
                ))
            } else {
                None
            }
        }
    }

    pub fn align(&mut self, align: &mut ::VideoAlignment) -> bool {
        #[cfg(not(feature = "v1_12"))]
        unsafe {
            // The bool return value is new with 1.11.1, see
            // https://gitlab.freedesktop.org/gstreamer/gst-plugins-base/commit/17cdd369e6f2f73329d27dfceb50011f40f1ceb0
            if gst::version() < (1, 11, 1, 0) {
                gst_video_sys::gst_video_info_align(&mut self.0, &mut align.0);

                true
            } else {
                from_glib(gst_video_sys::gst_video_info_align(
                    &mut self.0,
                    &mut align.0,
                ))
            }
        }
        #[cfg(feature = "v1_12")]
        unsafe {
            from_glib(gst_video_sys::gst_video_info_align(
                &mut self.0,
                &mut align.0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn align_full(
        &mut self,
        align: &mut crate::VideoAlignment,
    ) -> Result<([usize; crate::VIDEO_MAX_PLANES]), glib::BoolError> {
        let mut plane_size = [0; crate::VIDEO_MAX_PLANES];

        unsafe {
            glib::glib_result_from_gboolean!(
                gst_video_sys::gst_video_info_align_full(
                    &mut self.0,
                    &mut align.0,
                    plane_size.as_mut_ptr()
                ),
                "Failed to align VideoInfo"
            )?;
        }

        Ok(plane_size)
    }
}

impl Clone for VideoInfo {
    fn clone(&self) -> Self {
        unsafe { VideoInfo(ptr::read(&self.0)) }
    }
}

impl PartialEq for VideoInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { from_glib(gst_video_sys::gst_video_info_is_equal(&self.0, &other.0)) }
    }
}

impl Eq for VideoInfo {}

unsafe impl Send for VideoInfo {}
unsafe impl Sync for VideoInfo {}

impl glib::types::StaticType for VideoInfo {
    fn static_type() -> glib::types::Type {
        unsafe { glib::translate::from_glib(gst_video_sys::gst_video_info_get_type()) }
    }
}

#[doc(hidden)]
impl<'a> glib::value::FromValueOptional<'a> for VideoInfo {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Option::<VideoInfo>::from_glib_none(gobject_sys::g_value_get_boxed(value.to_glib_none().0)
            as *mut gst_video_sys::GstVideoInfo)
    }
}

#[doc(hidden)]
impl glib::value::SetValue for VideoInfo {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        gobject_sys::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const gst_video_sys::GstVideoInfo>::to_glib_none(this).0
                as glib_sys::gpointer,
        )
    }
}

#[doc(hidden)]
impl glib::value::SetValueOptional for VideoInfo {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        gobject_sys::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const gst_video_sys::GstVideoInfo>::to_glib_none(&this).0
                as glib_sys::gpointer,
        )
    }
}

#[doc(hidden)]
impl glib::translate::Uninitialized for VideoInfo {
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl glib::translate::GlibPtrDefault for VideoInfo {
    type GlibType = *mut gst_video_sys::GstVideoInfo;
}

#[doc(hidden)]
impl<'a> glib::translate::ToGlibPtr<'a, *const gst_video_sys::GstVideoInfo> for VideoInfo {
    type Storage = &'a VideoInfo;

    fn to_glib_none(
        &'a self,
    ) -> glib::translate::Stash<'a, *const gst_video_sys::GstVideoInfo, Self> {
        glib::translate::Stash(&self.0, self)
    }

    fn to_glib_full(&self) -> *const gst_video_sys::GstVideoInfo {
        unimplemented!()
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*mut gst_video_sys::GstVideoInfo> for VideoInfo {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut gst_video_sys::GstVideoInfo) -> Self {
        VideoInfo(ptr::read(ptr))
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrFull<*mut gst_video_sys::GstVideoInfo> for VideoInfo {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut gst_video_sys::GstVideoInfo) -> Self {
        let info = from_glib_none(ptr);
        glib_sys::g_free(ptr as *mut _);
        info
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl ::VideoFieldOrder {
    pub fn to_str<'a>(self) -> &'a str {
        unsafe {
            CStr::from_ptr(gst_video_sys::gst_video_field_order_to_string(
                self.to_glib(),
            ))
            .to_str()
            .unwrap()
        }
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl str::FromStr for ::VideoFieldOrder {
    type Err = glib::error::BoolError;

    fn from_str(s: &str) -> Result<Self, glib::error::BoolError> {
        assert_initialized_main_thread!();

        unsafe {
            Ok(from_glib(gst_video_sys::gst_video_field_order_from_string(
                s.to_glib_none().0,
            )))
        }
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl fmt::Display for ::VideoFieldOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str((*self).to_str())
    }
}

impl ::VideoInterlaceMode {
    pub fn to_str<'a>(self) -> &'a str {
        unsafe {
            CStr::from_ptr(gst_video_sys::gst_video_interlace_mode_to_string(
                self.to_glib(),
            ))
            .to_str()
            .unwrap()
        }
    }
}

impl str::FromStr for ::VideoInterlaceMode {
    type Err = glib::error::BoolError;

    fn from_str(s: &str) -> Result<Self, glib::error::BoolError> {
        assert_initialized_main_thread!();

        unsafe {
            Ok(from_glib(
                gst_video_sys::gst_video_interlace_mode_from_string(s.to_glib_none().0),
            ))
        }
    }
}

impl fmt::Display for ::VideoInterlaceMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str((*self).to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gst;

    #[test]
    fn test_new() {
        gst::init().unwrap();

        let info = VideoInfo::builder(::VideoFormat::I420, 320, 240)
            .build()
            .unwrap();
        assert_eq!(info.format(), ::VideoFormat::I420);
        assert_eq!(info.width(), 320);
        assert_eq!(info.height(), 240);
        assert_eq!(info.size(), 320 * 240 + 2 * 160 * 120);
        assert_eq!(info.multiview_mode(), ::VideoMultiviewMode::None);
        assert_eq!(&info.offset(), &[0, 320 * 240, 320 * 240 + 160 * 120]);
        assert_eq!(&info.stride(), &[320, 160, 160]);

        let offsets = [0, 640 * 240 + 16, 640 * 240 + 16 + 320 * 120 + 16];
        let strides = [640, 320, 320];
        let info = VideoInfo::builder(::VideoFormat::I420, 320, 240)
            .offset(&offsets)
            .stride(&strides)
            .size(640 * 240 + 16 + 320 * 120 + 16 + 320 * 120 + 16)
            .multiview_mode(::VideoMultiviewMode::SideBySide)
            .build()
            .unwrap();
        assert_eq!(info.format(), ::VideoFormat::I420);
        assert_eq!(info.width(), 320);
        assert_eq!(info.height(), 240);
        assert_eq!(
            info.size(),
            640 * 240 + 16 + 320 * 120 + 16 + 320 * 120 + 16
        );
        assert_eq!(info.multiview_mode(), ::VideoMultiviewMode::SideBySide);
        assert_eq!(
            &info.offset(),
            &[0, 640 * 240 + 16, 640 * 240 + 16 + 320 * 120 + 16]
        );
        assert_eq!(&info.stride(), &[640, 320, 320]);
    }

    #[test]
    fn test_from_to_caps() {
        gst::init().unwrap();

        let caps = gst::Caps::new_simple(
            "video/x-raw",
            &[
                ("format", &"I420"),
                ("width", &320),
                ("height", &240),
                ("framerate", &gst::Fraction::new(30, 1)),
                ("pixel-aspect-ratio", &gst::Fraction::new(1, 1)),
                ("interlace-mode", &"progressive"),
                ("chroma-site", &"mpeg2"),
                ("colorimetry", &"bt709"),
            ],
        );
        let info = VideoInfo::from_caps(&caps).unwrap();
        assert_eq!(info.format(), ::VideoFormat::I420);
        assert_eq!(info.width(), 320);
        assert_eq!(info.height(), 240);
        assert_eq!(info.fps(), gst::Fraction::new(30, 1));
        assert_eq!(info.interlace_mode(), ::VideoInterlaceMode::Progressive);
        assert_eq!(info.chroma_site(), ::VideoChromaSite::MPEG2);
        assert_eq!(info.colorimetry(), "bt709".parse().unwrap());

        let caps2 = info.to_caps().unwrap();
        assert_eq!(caps, caps2);

        let info2 = VideoInfo::from_caps(&caps2).unwrap();
        assert!(info == info2);
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[test]
    fn test_video_align() {
        gst::init().unwrap();

        let mut info = ::VideoInfo::builder(::VideoFormat::Nv16, 1920, 1080)
            .build()
            .expect("Failed to create VideoInfo");

        assert_eq!(info.stride(), [1920, 1920]);
        assert_eq!(info.offset(), [0, 2_073_600]);

        let mut align = ::VideoAlignment::new(0, 0, 0, 8, &[0; VIDEO_MAX_PLANES]);
        assert!(info.align(&mut align));

        assert_eq!(info.stride(), [1928, 1928]);
        assert_eq!(info.offset(), [0, 2_082_240]);

        #[cfg(feature = "v1_18")]
        {
            let mut info = crate::VideoInfo::builder(crate::VideoFormat::Nv16, 1920, 1080)
                .build()
                .expect("Failed to create VideoInfo");

            let mut align = crate::VideoAlignment::new(0, 0, 0, 8, &[0; VIDEO_MAX_PLANES]);
            let plane_size = info.align_full(&mut align).unwrap();
            assert_eq!(plane_size, [2082240, 2082240, 0, 0]);
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[test]
    fn test_display() {
        use std::str::FromStr;

        gst::init().unwrap();

        format!("{}", ::VideoColorimetry::from_str("sRGB").unwrap());
        format!("{}", ::VideoFieldOrder::TopFieldFirst);
        format!("{}", ::VideoInterlaceMode::Progressive);
    }
}
