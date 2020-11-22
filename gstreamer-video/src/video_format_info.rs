// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cmp::Ordering;
use std::ffi::CStr;
use std::fmt;
use std::str;

use glib::translate::{from_glib, ToGlib};

pub struct VideoFormatInfo(&'static ffi::GstVideoFormatInfo);

impl VideoFormatInfo {
    pub fn from_format(format: crate::VideoFormat) -> VideoFormatInfo {
        assert_initialized_main_thread!();

        unsafe {
            let info = ffi::gst_video_format_get_info(format.to_glib());
            assert!(!info.is_null());

            VideoFormatInfo(&*info)
        }
    }

    pub fn format(&self) -> crate::VideoFormat {
        from_glib(self.0.format)
    }

    pub fn name<'a>(&self) -> &'a str {
        unsafe { CStr::from_ptr(self.0.name).to_str().unwrap() }
    }

    pub fn description<'a>(&self) -> &'a str {
        unsafe { CStr::from_ptr(self.0.description).to_str().unwrap() }
    }

    pub fn flags(&self) -> crate::VideoFormatFlags {
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

    pub fn tile_mode(&self) -> crate::VideoTileMode {
        from_glib(self.0.tile_mode)
    }

    pub fn tile_ws(&self) -> u32 {
        self.0.tile_ws
    }

    pub fn tile_hs(&self) -> u32 {
        self.0.tile_hs
    }

    pub fn unpack_format(&self) -> crate::VideoFormat {
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

    #[allow(clippy::too_many_arguments)]
    pub fn unpack(
        &self,
        flags: crate::VideoPackFlags,
        dest: &mut [u8],
        src: &[&[u8]],
        stride: &[i32],
        x: i32,
        y: i32,
        width: i32,
    ) {
        let unpack_format = Self::from_format(self.unpack_format());

        if unpack_format.pixel_stride()[0] == 0 || self.0.unpack_func.is_none() {
            panic!("No unpack format for {:?}", self);
        }

        if src.len() != self.n_planes() as usize {
            panic!(
                "Wrong number of planes provided for format: {} != {}",
                src.len(),
                self.n_planes()
            );
        }

        if stride.len() != self.n_planes() as usize {
            panic!(
                "Wrong number of strides provided for format: {} != {}",
                stride.len(),
                self.n_planes()
            );
        }

        if dest.len() < unpack_format.pixel_stride()[0] as usize * width as usize {
            panic!("Too small destination slice");
        }

        for plane in 0..(self.n_planes()) {
            if stride[plane as usize]
                < self.scale_width(plane as u8, width as u32) as i32
                    * self.pixel_stride()[plane as usize]
            {
                panic!("Too small source stride for plane {}", plane);
            }

            let plane_size = y * stride[plane as usize]
                + self.scale_width(plane as u8, (x + width) as u32) as i32
                    * self.pixel_stride()[plane as usize];

            if src[plane as usize].len() < plane_size as usize {
                panic!("Too small source plane size for plane {}", plane);
            }
        }

        unsafe {
            use std::ptr;

            let mut src_ptr = [ptr::null() as *const u8; ffi::GST_VIDEO_MAX_PLANES as usize];
            for plane in 0..(self.n_planes()) {
                src_ptr[plane as usize] = src[plane as usize].as_ptr();
            }

            (self.0.unpack_func.as_ref().unwrap())(
                self.0,
                flags.to_glib(),
                dest.as_mut_ptr() as *mut _,
                src_ptr.as_ptr() as *const _,
                stride.as_ptr() as *const i32,
                x,
                y,
                width,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn pack(
        &self,
        flags: crate::VideoPackFlags,
        src: &[u8],
        src_stride: i32,
        dest: &mut [&mut [u8]],
        dest_stride: &[i32],
        chroma_site: crate::VideoChromaSite,
        y: i32,
        width: i32,
    ) {
        let unpack_format = Self::from_format(self.unpack_format());

        if unpack_format.pixel_stride()[0] == 0 || self.0.unpack_func.is_none() {
            panic!("No unpack format for {:?}", self);
        }

        if dest.len() != self.n_planes() as usize {
            panic!(
                "Wrong number of planes provided for format: {} != {}",
                dest.len(),
                self.n_planes()
            );
        }

        if dest_stride.len() != self.n_planes() as usize {
            panic!(
                "Wrong number of strides provided for format: {} != {}",
                dest_stride.len(),
                self.n_planes()
            );
        }

        if src.len() < unpack_format.pixel_stride()[0] as usize * width as usize {
            panic!("Too small source slice");
        }

        for plane in 0..(self.n_planes()) {
            if dest_stride[plane as usize]
                < self.scale_width(plane as u8, width as u32) as i32
                    * self.pixel_stride()[plane as usize]
            {
                panic!("Too small destination stride for plane {}", plane);
            }

            let plane_size = y * dest_stride[plane as usize]
                + self.scale_width(plane as u8, width as u32) as i32
                    * self.pixel_stride()[plane as usize];

            if dest[plane as usize].len() < plane_size as usize {
                panic!("Too small destination plane size for plane {}", plane);
            }
        }

        unsafe {
            use std::ptr;

            let mut dest_ptr = [ptr::null_mut() as *mut u8; ffi::GST_VIDEO_MAX_PLANES as usize];
            for plane in 0..(self.n_planes()) {
                dest_ptr[plane as usize] = dest[plane as usize].as_mut_ptr();
            }

            (self.0.pack_func.as_ref().unwrap())(
                self.0,
                flags.to_glib(),
                src.as_ptr() as *mut _,
                src_stride,
                dest_ptr.as_mut_ptr() as *mut _,
                dest_stride.as_ptr() as *const i32,
                chroma_site.to_glib(),
                y,
                width,
            );
        }
    }
}

unsafe impl Sync for VideoFormatInfo {}
unsafe impl Send for VideoFormatInfo {}

impl PartialEq for VideoFormatInfo {
    fn eq(&self, other: &Self) -> bool {
        self.format() == other.format()
    }
}

impl Eq for VideoFormatInfo {}

impl PartialOrd for VideoFormatInfo {
    fn partial_cmp(&self, other: &VideoFormatInfo) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VideoFormatInfo {
    // See GST_VIDEO_FORMATS_ALL for the sorting algorithm
    fn cmp(&self, other: &VideoFormatInfo) -> Ordering {
        self.n_components()
            .cmp(&other.n_components())
            .then_with(|| self.depth().cmp(&other.depth()))
            .then_with(|| self.w_sub().cmp(&other.w_sub()).reverse())
            .then_with(|| self.h_sub().cmp(&other.h_sub()).reverse())
            .then_with(|| self.n_planes().cmp(&other.n_planes()))
            .then_with(|| {
                // Format using native endianess is considered as bigger
                match (
                    self.flags().contains(crate::VideoFormatFlags::LE),
                    other.flags().contains(crate::VideoFormatFlags::LE),
                ) {
                    (true, false) => {
                        // a LE, b BE
                        #[cfg(target_endian = "little")]
                        {
                            Ordering::Greater
                        }
                        #[cfg(target_endian = "big")]
                        {
                            Ordering::Less
                        }
                    }
                    (false, true) => {
                        // a BE, b LE
                        #[cfg(target_endian = "little")]
                        {
                            Ordering::Less
                        }
                        #[cfg(target_endian = "big")]
                        {
                            Ordering::Greater
                        }
                    }
                    _ => Ordering::Equal,
                }
            })
            .then_with(|| self.pixel_stride().cmp(&other.pixel_stride()))
            .then_with(|| self.poffset().cmp(&other.poffset()))
            .then_with(|| {
                // Prefer non-complex formats
                match (
                    self.flags().contains(crate::VideoFormatFlags::COMPLEX),
                    other.flags().contains(crate::VideoFormatFlags::COMPLEX),
                ) {
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    _ => Ordering::Equal,
                }
            })
            .then_with(|| {
                // tiebreaker: YUV > RGB
                if self.flags().contains(crate::VideoFormatFlags::RGB)
                    && other.flags().contains(crate::VideoFormatFlags::YUV)
                {
                    Ordering::Less
                } else if self.flags().contains(crate::VideoFormatFlags::YUV)
                    && other.flags().contains(crate::VideoFormatFlags::RGB)
                {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            })
            .then_with(|| {
                // Manual tiebreaker
                match (self.format(), other.format()) {
                    // I420 is more commonly used in GStreamer
                    (crate::VideoFormat::I420, crate::VideoFormat::Yv12) => Ordering::Greater,
                    (crate::VideoFormat::Yv12, crate::VideoFormat::I420) => Ordering::Less,
                    _ => Ordering::Equal,
                }
            })
            .then_with(|| {
                // tie, sort by name
                self.name().cmp(&other.name())
            })
    }
}

impl fmt::Debug for VideoFormatInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("VideoFormatInfo")
            .field("format", &self.format())
            .field("name", &self.name())
            .field("description", &self.description())
            .field("flags", &self.flags())
            .field("bits", &self.bits())
            .field("n-components", &self.n_components())
            .field("shift", &self.shift())
            .field("depth", &self.depth())
            .field("pixel-stride", &self.pixel_stride())
            .field("n-planes", &self.n_planes())
            .field("plane", &self.plane())
            .field("poffset", &self.poffset())
            .field("w-sub", &self.w_sub())
            .field("h-sub", &self.h_sub())
            .field("unpack-format", &self.unpack_format())
            .field("pack-lines", &self.pack_lines())
            .field("tile-mode", &self.tile_mode())
            .field("tile-ws", &self.tile_ws())
            .field("tile-hs", &self.tile_hs())
            .finish()
    }
}

impl fmt::Display for VideoFormatInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.name())
    }
}

impl str::FromStr for crate::VideoFormatInfo {
    type Err = glib::BoolError;

    fn from_str(s: &str) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();
        let format = s.parse()?;
        Ok(VideoFormatInfo::from_format(format))
    }
}

impl From<crate::VideoFormat> for VideoFormatInfo {
    fn from(f: crate::VideoFormat) -> Self {
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

    #[test]
    fn test_get() {
        gst::init().unwrap();

        let info = VideoFormatInfo::from_format(crate::VideoFormat::I420);
        assert_eq!(info.name(), "I420");

        let other_info = "I420".parse().unwrap();
        assert_eq!(info, other_info);

        assert_eq!(info.scale_width(0, 128), 128);
        assert_eq!(info.scale_width(1, 128), 64);
        assert_eq!(info.scale_width(2, 128), 64);
    }

    #[test]
    fn test_unpack() {
        gst::init().unwrap();

        // One line black 320 pixel I420
        let input = &[&[0; 320][..], &[128; 160][..], &[128; 160][..]];
        // One line of AYUV
        let intermediate = &mut [0; 320 * 4][..];
        // One line of 320 pixel I420
        let output = &mut [&mut [0; 320][..], &mut [0; 160][..], &mut [0; 160][..]];

        let info = VideoFormatInfo::from_format(crate::VideoFormat::I420);
        assert_eq!(info.unpack_format(), crate::VideoFormat::Ayuv);
        info.unpack(
            crate::VideoPackFlags::empty(),
            intermediate,
            input,
            &[320, 160, 160][..],
            0,
            0,
            320,
        );

        for pixel in intermediate.chunks_exact(4) {
            assert_eq!(&[255, 0, 128, 128][..], pixel);
        }

        info.pack(
            crate::VideoPackFlags::empty(),
            &intermediate[..(4 * 320)],
            4 * 320,
            output,
            &[320, 160, 160][..],
            crate::VideoChromaSite::NONE,
            0,
            320,
        );
        assert_eq!(input, output);
    }
}
