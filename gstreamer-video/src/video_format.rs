// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_video_sys;

use once_cell::sync::Lazy;
use std::ffi::CStr;
use std::fmt;
use std::str;

use glib::translate::{from_glib, FromGlib, ToGlib, ToGlibPtr};

#[cfg(feature = "v1_18")]
pub static VIDEO_FORMATS_ALL: Lazy<Box<[::VideoFormat]>> = Lazy::new(|| unsafe {
    let mut len: u32 = 0;
    let mut res = Vec::with_capacity(len as usize);
    let formats = gst_video_sys::gst_video_formats_raw(&mut len);
    for i in 0..len {
        let format = formats.offset(i as isize);
        res.push(from_glib(*format));
    }
    res.into_boxed_slice()
});

#[cfg(not(feature = "v1_18"))]
pub static VIDEO_FORMATS_ALL: Lazy<Box<[::VideoFormat]>> = Lazy::new(|| {
    Box::new([
        ::VideoFormat::I420,
        ::VideoFormat::Yv12,
        ::VideoFormat::Yuy2,
        ::VideoFormat::Uyvy,
        ::VideoFormat::Ayuv,
        ::VideoFormat::Vuya,
        ::VideoFormat::Rgbx,
        ::VideoFormat::Bgrx,
        ::VideoFormat::Xrgb,
        ::VideoFormat::Xbgr,
        ::VideoFormat::Rgba,
        ::VideoFormat::Bgra,
        ::VideoFormat::Argb,
        ::VideoFormat::Abgr,
        ::VideoFormat::Rgb,
        ::VideoFormat::Bgr,
        ::VideoFormat::Y41b,
        ::VideoFormat::Y42b,
        ::VideoFormat::Yvyu,
        ::VideoFormat::Y444,
        ::VideoFormat::V210,
        ::VideoFormat::V216,
        ::VideoFormat::Y210,
        ::VideoFormat::Y410,
        ::VideoFormat::Nv12,
        ::VideoFormat::Nv21,
        ::VideoFormat::Gray8,
        ::VideoFormat::Gray16Be,
        ::VideoFormat::Gray16Le,
        ::VideoFormat::V308,
        ::VideoFormat::Rgb16,
        ::VideoFormat::Bgr16,
        ::VideoFormat::Rgb15,
        ::VideoFormat::Bgr15,
        ::VideoFormat::Uyvp,
        ::VideoFormat::A420,
        ::VideoFormat::Rgb8p,
        ::VideoFormat::Yuv9,
        ::VideoFormat::Yvu9,
        ::VideoFormat::Iyu1,
        ::VideoFormat::Argb64,
        ::VideoFormat::Ayuv64,
        ::VideoFormat::R210,
        ::VideoFormat::I42010be,
        ::VideoFormat::I42010le,
        ::VideoFormat::I42210be,
        ::VideoFormat::I42210le,
        ::VideoFormat::Y44410be,
        ::VideoFormat::Y44410le,
        ::VideoFormat::Gbr,
        ::VideoFormat::Gbr10be,
        ::VideoFormat::Gbr10le,
        ::VideoFormat::Nv16,
        ::VideoFormat::Nv24,
        ::VideoFormat::Nv1264z32,
        ::VideoFormat::A42010be,
        ::VideoFormat::A42010le,
        ::VideoFormat::A42210be,
        ::VideoFormat::A42210le,
        ::VideoFormat::A44410be,
        ::VideoFormat::A44410le,
        ::VideoFormat::Nv61,
        ::VideoFormat::P01010be,
        ::VideoFormat::P01010le,
        ::VideoFormat::Iyu2,
        ::VideoFormat::Vyuy,
        ::VideoFormat::Gbra,
        ::VideoFormat::Gbra10be,
        ::VideoFormat::Gbra10le,
        ::VideoFormat::Bgr10a2Le,
        ::VideoFormat::Rgb10a2Le,
        ::VideoFormat::Gbr12be,
        ::VideoFormat::Gbr12le,
        ::VideoFormat::Gbra12be,
        ::VideoFormat::Gbra12le,
        ::VideoFormat::P012Be,
        ::VideoFormat::P012Le,
        ::VideoFormat::I42012be,
        ::VideoFormat::I42012le,
        ::VideoFormat::Y212Be,
        ::VideoFormat::Y212Le,
        ::VideoFormat::I42212be,
        ::VideoFormat::I42212le,
        ::VideoFormat::Y412Be,
        ::VideoFormat::Y412Le,
        ::VideoFormat::Y44412be,
        ::VideoFormat::Y44412le,
        ::VideoFormat::Gray10Le32,
        ::VideoFormat::Nv1210le32,
        ::VideoFormat::Nv1610le32,
        ::VideoFormat::Nv1210le40,
        ::VideoFormat::Y44416be,
        ::VideoFormat::Y44416le,
        ::VideoFormat::P016Be,
        ::VideoFormat::P016Le,
    ])
});

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum VideoEndianness {
    Unknown,
    LittleEndian = 1234,
    BigEndian = 4321,
}

impl FromGlib<i32> for VideoEndianness {
    fn from_glib(value: i32) -> Self {
        skip_assert_initialized!();

        match value {
            1234 => VideoEndianness::LittleEndian,
            4321 => VideoEndianness::BigEndian,
            _ => VideoEndianness::Unknown,
        }
    }
}

impl ToGlib for VideoEndianness {
    type GlibType = i32;

    fn to_glib(&self) -> i32 {
        match *self {
            VideoEndianness::LittleEndian => 1234,
            VideoEndianness::BigEndian => 4321,
            _ => 0,
        }
    }
}

impl ::VideoFormat {
    pub fn from_fourcc(fourcc: u32) -> ::VideoFormat {
        assert_initialized_main_thread!();

        unsafe { from_glib(gst_video_sys::gst_video_format_from_fourcc(fourcc)) }
    }

    pub fn from_masks(
        depth: u32,
        bpp: u32,
        endianness: ::VideoEndianness,
        red_mask: u32,
        blue_mask: u32,
        green_mask: u32,
        alpha_mask: u32,
    ) -> ::VideoFormat {
        assert_initialized_main_thread!();

        unsafe {
            from_glib(gst_video_sys::gst_video_format_from_masks(
                depth as i32,
                bpp as i32,
                endianness.to_glib(),
                red_mask,
                blue_mask,
                green_mask,
                alpha_mask,
            ))
        }
    }

    pub fn to_str<'a>(self) -> &'a str {
        if self == ::VideoFormat::Unknown {
            return "UNKNOWN";
        }

        unsafe {
            CStr::from_ptr(gst_video_sys::gst_video_format_to_string(self.to_glib()))
                .to_str()
                .unwrap()
        }
    }

    pub fn iter_raw() -> VideoFormatIterator {
        VideoFormatIterator::default()
    }
}

impl str::FromStr for ::VideoFormat {
    type Err = glib::BoolError;

    fn from_str(s: &str) -> Result<Self, glib::BoolError> {
        assert_initialized_main_thread!();

        unsafe {
            let fmt = ::VideoFormat::from_glib(gst_video_sys::gst_video_format_from_string(
                s.to_glib_none().0,
            ));

            if fmt == ::VideoFormat::Unknown {
                Err(glib_bool_error!("Failed to parse video format from string"))
            } else {
                Ok(fmt)
            }
        }
    }
}

impl fmt::Display for ::VideoFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str((*self).to_str())
    }
}
pub struct VideoFormatIterator {
    idx: usize,
    len: usize,
}

impl Default for VideoFormatIterator {
    fn default() -> Self {
        Self {
            idx: 0,
            len: VIDEO_FORMATS_ALL.len(),
        }
    }
}

impl Iterator for VideoFormatIterator {
    type Item = ::VideoFormat;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.len {
            None
        } else {
            let fmt = VIDEO_FORMATS_ALL[self.idx];
            self.idx += 1;
            Some(fmt)
        }
    }
}

impl ExactSizeIterator for VideoFormatIterator {
    fn len(&self) -> usize {
        self.len
    }
}

impl DoubleEndedIterator for VideoFormatIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx >= self.len {
            None
        } else {
            let fmt = VIDEO_FORMATS_ALL[self.len - 1];
            self.len -= 1;
            Some(fmt)
        }
    }
}
pub trait VideoFormatIteratorExt {
    fn into_video_caps(self) -> Option<gst::caps::Builder<gst::caps::NoFeature>>;
}

impl<T> VideoFormatIteratorExt for T
where
    T: Iterator<Item = ::VideoFormat>,
{
    fn into_video_caps(self) -> Option<gst::caps::Builder<gst::caps::NoFeature>> {
        let formats: Vec<::VideoFormat> = self.collect();
        if !formats.is_empty() {
            Some(::functions::video_make_raw_caps(&formats))
        } else {
            None
        }
    }
}

pub trait VideoFormatIteratorExtRef {
    fn into_video_caps(self) -> Option<gst::caps::Builder<gst::caps::NoFeature>>;
}

impl<'a, T> VideoFormatIteratorExtRef for T
where
    T: Iterator<Item = &'a ::VideoFormat>,
{
    fn into_video_caps(self) -> Option<gst::caps::Builder<gst::caps::NoFeature>> {
        let formats: Vec<::VideoFormat> = self.copied().collect();
        if !formats.is_empty() {
            Some(::functions::video_make_raw_caps(&formats))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use gst;

    #[test]
    fn test_display() {
        gst::init().unwrap();

        format!("{}", ::VideoFormat::Nv16);
    }

    #[test]
    fn iter() {
        use super::*;
        gst::init().unwrap();

        assert!(::VideoFormat::iter_raw().count() > 0);
        assert_eq!(
            ::VideoFormat::iter_raw().count(),
            ::VideoFormat::iter_raw().len()
        );

        let mut i = ::VideoFormat::iter_raw();
        let mut count = 0;
        loop {
            if i.next().is_none() {
                break;
            }
            count += 1;
            if i.next_back().is_none() {
                break;
            }
            count += 1;
        }
        assert_eq!(count, ::VideoFormat::iter_raw().len());

        assert!(::VideoFormat::iter_raw().any(|f| f == ::VideoFormat::P016Be));
        assert!(::VideoFormat::iter_raw()
            .find(|f| *f == ::VideoFormat::Encoded)
            .is_none());

        let caps = ::VideoFormat::iter_raw().into_video_caps();
        assert!(caps.is_some());

        let caps = ::VideoFormat::iter_raw()
            .filter(|f| ::VideoFormatInfo::from_format(*f).is_gray())
            .into_video_caps();
        assert!(caps.is_some());

        let caps = ::VideoFormat::iter_raw().skip(1000).into_video_caps();
        assert!(caps.is_none());

        let caps = [::VideoFormat::Nv12, ::VideoFormat::Nv16]
            .iter()
            .into_video_caps()
            .unwrap()
            .build();
        assert_eq!(caps.to_string(), "video/x-raw, format=(string){ NV12, NV16 }, width=(int)[ 1, 2147483647 ], height=(int)[ 1, 2147483647 ], framerate=(fraction)[ 0/1, 2147483647/1 ]");
    }
}
