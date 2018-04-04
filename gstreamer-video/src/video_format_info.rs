// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;

use std::ffi::CStr;
use std::fmt;
use std::str;

use glib;
use glib::translate::{from_glib, ToGlib};

pub struct VideoFormatInfo(&'static ffi::GstVideoFormatInfo);

impl VideoFormatInfo {
    pub fn from_format(format: ::VideoFormat) -> VideoFormatInfo {
        assert_initialized_main_thread!();

        unsafe {
            let info = ffi::gst_video_format_get_info(format.to_glib());
            assert!(!info.is_null());

            VideoFormatInfo(&*info)
        }
    }

    pub fn format(&self) -> ::VideoFormat {
        from_glib(self.0.format)
    }

    pub fn name<'a>(&self) -> &'a str {
        unsafe { CStr::from_ptr(self.0.name).to_str().unwrap() }
    }

    pub fn description<'a>(&self) -> &'a str {
        unsafe { CStr::from_ptr(self.0.description).to_str().unwrap() }
    }

    pub fn flags(&self) -> ::VideoFormatFlags {
        from_glib(self.0.flags)
    }

    pub fn bits(&self) -> u32 {
        self.0.bits
    }

    pub fn n_components(&self) -> u32 {
        self.0.n_components
    }

    pub fn shift(&self) -> &[u32] {
        &self.0.shift[0..(self.0.n_components as usize)]
    }

    pub fn depth(&self) -> &[u32] {
        &self.0.depth[0..(self.0.n_components as usize)]
    }

    pub fn pixel_stride(&self) -> &[i32] {
        &self.0.pixel_stride[0..(self.0.n_components as usize)]
    }

    pub fn n_planes(&self) -> u32 {
        self.0.n_planes
    }

    pub fn plane(&self) -> &[u32] {
        &self.0.plane[0..(self.0.n_components as usize)]
    }

    pub fn poffset(&self) -> &[u32] {
        &self.0.poffset[0..(self.0.n_components as usize)]
    }

    pub fn w_sub(&self) -> &[u32] {
        &self.0.w_sub[0..(self.0.n_components as usize)]
    }

    pub fn h_sub(&self) -> &[u32] {
        &self.0.h_sub[0..(self.0.n_components as usize)]
    }

    pub fn tile_mode(&self) -> ::VideoTileMode {
        from_glib(self.0.tile_mode)
    }

    pub fn tile_ws(&self) -> u32 {
        self.0.tile_ws
    }

    pub fn tile_hs(&self) -> u32 {
        self.0.tile_hs
    }

    pub fn unpack_format(&self) -> ::VideoFormat {
        from_glib(self.0.unpack_format)
    }

    pub fn pack_lines(&self) -> i32 {
        self.0.pack_lines
    }

    pub fn has_alpha(&self) -> bool {
        self.0.flags & ffi::GST_VIDEO_FORMAT_FLAG_ALPHA != 0
    }

    pub fn has_palette(&self) -> bool {
        self.0.flags & ffi::GST_VIDEO_FORMAT_FLAG_PALETTE != 0
    }

    pub fn is_complex(&self) -> bool {
        self.0.flags & ffi::GST_VIDEO_FORMAT_FLAG_COMPLEX != 0
    }

    pub fn is_gray(&self) -> bool {
        self.0.flags & ffi::GST_VIDEO_FORMAT_FLAG_GRAY != 0
    }

    pub fn is_le(&self) -> bool {
        self.0.flags & ffi::GST_VIDEO_FORMAT_FLAG_LE != 0
    }

    pub fn is_rgb(&self) -> bool {
        self.0.flags & ffi::GST_VIDEO_FORMAT_FLAG_RGB != 0
    }

    pub fn is_tiled(&self) -> bool {
        self.0.flags & ffi::GST_VIDEO_FORMAT_FLAG_TILED != 0
    }

    pub fn is_yuv(&self) -> bool {
        self.0.flags & ffi::GST_VIDEO_FORMAT_FLAG_YUV != 0
    }

    pub fn scale_width(&self, component: u8, width: u32) -> u32 {
        (-((-(i64::from(width))) >> self.w_sub()[component as usize])) as u32
    }

    pub fn scale_height(&self, component: u8, height: u32) -> u32 {
        (-((-(i64::from(height))) >> self.h_sub()[component as usize])) as u32
    }

    // TODO: pack/unpack
}

unsafe impl Sync for VideoFormatInfo {}
unsafe impl Send for VideoFormatInfo {}

impl PartialEq for VideoFormatInfo {
    fn eq(&self, other: &Self) -> bool {
        self.format() == other.format()
    }
}

impl Eq for VideoFormatInfo {}

impl fmt::Debug for VideoFormatInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.name())
    }
}

impl fmt::Display for VideoFormatInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.name())
    }
}

impl str::FromStr for ::VideoFormatInfo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        skip_assert_initialized!();
        let format = s.parse()?;
        Ok(VideoFormatInfo::from_format(format))
    }
}

impl From<::VideoFormat> for VideoFormatInfo {
    fn from(f: ::VideoFormat) -> Self {
        skip_assert_initialized!();
        Self::from_format(f)
    }
}

#[doc(hidden)]
impl glib::translate::GlibPtrDefault for VideoFormatInfo {
    type GlibType = *mut ffi::GstVideoFormatInfo;
}

#[doc(hidden)]
impl<'a> glib::translate::ToGlibPtr<'a, *const ffi::GstVideoFormatInfo> for VideoFormatInfo {
    type Storage = &'a VideoFormatInfo;

    fn to_glib_none(&'a self) -> glib::translate::Stash<'a, *const ffi::GstVideoFormatInfo, Self> {
        glib::translate::Stash(self.0, self)
    }

    fn to_glib_full(&self) -> *const ffi::GstVideoFormatInfo {
        unimplemented!()
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*mut ffi::GstVideoFormatInfo> for VideoFormatInfo {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstVideoFormatInfo) -> Self {
        VideoFormatInfo(&*ptr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gst;

    #[test]
    fn test_get() {
        gst::init().unwrap();

        let info = VideoFormatInfo::from_format(::VideoFormat::I420);
        assert_eq!(info.name(), "I420");

        let other_info = "I420".parse().unwrap();
        assert_eq!(info, other_info);

        assert_eq!(info.scale_width(0, 128), 128);
        assert_eq!(info.scale_width(1, 128), 64);
        assert_eq!(info.scale_width(2, 128), 64);
    }
}
