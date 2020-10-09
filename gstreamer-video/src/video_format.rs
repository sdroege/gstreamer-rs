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
    #[cfg(target_endian = "little")]
    {
        Box::new([
            ::VideoFormat::Ayuv64,
            ::VideoFormat::Argb64,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Gbra12le,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Gbra12be,
            ::VideoFormat::A44410le,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Gbra10le,
            ::VideoFormat::A44410be,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Gbra10be,
            ::VideoFormat::A42210le,
            ::VideoFormat::A42210be,
            ::VideoFormat::A42010le,
            ::VideoFormat::A42010be,
            #[cfg(feature = "v1_16")]
            ::VideoFormat::Bgr10a2Le,
            #[cfg(feature = "v1_16")]
            ::VideoFormat::Y410,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Gbra,
            ::VideoFormat::Abgr,
            #[cfg(feature = "v1_16")]
            ::VideoFormat::Vuya,
            ::VideoFormat::Bgra,
            ::VideoFormat::Ayuv,
            ::VideoFormat::Argb,
            ::VideoFormat::Rgba,
            ::VideoFormat::A420,
            ::VideoFormat::V216,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Y44412le,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Gbr12le,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Y44412be,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Gbr12be,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::I42212le,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::I42212be,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::I42012le,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::I42012be,
            ::VideoFormat::Y44410le,
            ::VideoFormat::Gbr10le,
            ::VideoFormat::Y44410be,
            ::VideoFormat::Gbr10be,
            ::VideoFormat::R210,
            ::VideoFormat::I42210le,
            ::VideoFormat::I42210be,
            #[cfg(feature = "v1_14")]
            ::VideoFormat::Nv1610le32,
            #[cfg(feature = "v1_16")]
            ::VideoFormat::Y210,
            ::VideoFormat::V210,
            ::VideoFormat::Uyvp,
            ::VideoFormat::I42010le,
            ::VideoFormat::I42010be,
            #[cfg(feature = "v1_10")]
            ::VideoFormat::P01010le,
            #[cfg(feature = "v1_14")]
            ::VideoFormat::Nv1210le32,
            #[cfg(feature = "v1_16")]
            ::VideoFormat::Nv1210le40,
            #[cfg(feature = "v1_10")]
            ::VideoFormat::P01010be,
            ::VideoFormat::Y444,
            ::VideoFormat::Gbr,
            ::VideoFormat::Nv24,
            ::VideoFormat::Xbgr,
            ::VideoFormat::Bgrx,
            ::VideoFormat::Xrgb,
            ::VideoFormat::Rgbx,
            ::VideoFormat::Bgr,
            #[cfg(feature = "v1_10")]
            ::VideoFormat::Iyu2,
            ::VideoFormat::V308,
            ::VideoFormat::Rgb,
            ::VideoFormat::Y42b,
            ::VideoFormat::Nv61,
            ::VideoFormat::Nv16,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Vyuy,
            ::VideoFormat::Uyvy,
            ::VideoFormat::Yvyu,
            ::VideoFormat::Yuy2,
            ::VideoFormat::I420,
            ::VideoFormat::Yv12,
            ::VideoFormat::Nv21,
            ::VideoFormat::Nv12,
            ::VideoFormat::Nv1264z32,
            ::VideoFormat::Y41b,
            ::VideoFormat::Iyu1,
            ::VideoFormat::Yvu9,
            ::VideoFormat::Yuv9,
            ::VideoFormat::Rgb16,
            ::VideoFormat::Bgr16,
            ::VideoFormat::Rgb15,
            ::VideoFormat::Bgr15,
            ::VideoFormat::Rgb8p,
            ::VideoFormat::Gray16Le,
            ::VideoFormat::Gray16Be,
            #[cfg(feature = "v1_14")]
            ::VideoFormat::Gray10Le32,
            ::VideoFormat::Gray8,
        ])
    }
    #[cfg(target_endian = "big")]
    {
        Box::new([
            ::VideoFormat::Ayuv64,
            ::VideoFormat::Argb64,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Gbra12be,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Gbra12le,
            ::VideoFormat::A44410be,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Gbra10be,
            ::VideoFormat::A44410le,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Gbra10le,
            ::VideoFormat::A42210be,
            ::VideoFormat::A42210le,
            ::VideoFormat::A42010be,
            ::VideoFormat::A42010le,
            #[cfg(feature = "v1_16")]
            ::VideoFormat::Y410,
            #[cfg(feature = "v1_16")]
            ::VideoFormat::Bgr10a2Le,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Gbra,
            ::VideoFormat::Abgr,
            #[cfg(feature = "v1_16")]
            ::VideoFormat::Vuya,
            ::VideoFormat::Bgra,
            ::VideoFormat::Ayuv,
            ::VideoFormat::Argb,
            ::VideoFormat::Rgba,
            ::VideoFormat::A420,
            ::VideoFormat::V216,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Y44412be,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Gbr12be,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Y44412le,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Gbr12le,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::I42212be,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::I42212le,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::I42012be,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::I42012le,
            ::VideoFormat::Y44410be,
            ::VideoFormat::Gbr10be,
            ::VideoFormat::Y44410le,
            ::VideoFormat::Gbr10le,
            ::VideoFormat::R210,
            ::VideoFormat::I42210be,
            ::VideoFormat::I42210le,
            #[cfg(feature = "v1_14")]
            ::VideoFormat::Nv1610le32,
            #[cfg(feature = "v1_16")]
            ::VideoFormat::Y210,
            ::VideoFormat::V210,
            ::VideoFormat::Uyvp,
            ::VideoFormat::I42010be,
            ::VideoFormat::I42010le,
            #[cfg(feature = "v1_10")]
            ::VideoFormat::P01010be,
            #[cfg(feature = "v1_10")]
            ::VideoFormat::P01010le,
            #[cfg(feature = "v1_14")]
            ::VideoFormat::Nv1210le32,
            #[cfg(feature = "v1_16")]
            ::VideoFormat::Nv1210le40,
            ::VideoFormat::Y444,
            ::VideoFormat::Gbr,
            ::VideoFormat::Nv24,
            ::VideoFormat::Xbgr,
            ::VideoFormat::Bgrx,
            ::VideoFormat::Xrgb,
            ::VideoFormat::Rgbx,
            ::VideoFormat::Bgr,
            #[cfg(feature = "v1_10")]
            ::VideoFormat::Iyu2,
            ::VideoFormat::V308,
            ::VideoFormat::Rgb,
            ::VideoFormat::Y42b,
            ::VideoFormat::Nv61,
            ::VideoFormat::Nv16,
            #[cfg(feature = "v1_12")]
            ::VideoFormat::Vyuy,
            ::VideoFormat::Uyvy,
            ::VideoFormat::Yvyu,
            ::VideoFormat::Yuy2,
            ::VideoFormat::I420,
            ::VideoFormat::Yv12,
            ::VideoFormat::Nv21,
            ::VideoFormat::Nv12,
            ::VideoFormat::Nv1264z32,
            ::VideoFormat::Y41b,
            ::VideoFormat::Iyu1,
            ::VideoFormat::Yvu9,
            ::VideoFormat::Yuv9,
            ::VideoFormat::Rgb16,
            ::VideoFormat::Bgr16,
            ::VideoFormat::Rgb15,
            ::VideoFormat::Bgr15,
            ::VideoFormat::Rgb8p,
            ::VideoFormat::Gray16Be,
            ::VideoFormat::Gray16Le,
            #[cfg(feature = "v1_14")]
            ::VideoFormat::Gray10Le32,
            ::VideoFormat::Gray8,
        ])
    }
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

impl PartialOrd for ::VideoFormat {
    fn partial_cmp(&self, other: &::VideoFormat) -> Option<std::cmp::Ordering> {
        ::VideoFormatInfo::from_format(*self).partial_cmp(&::VideoFormatInfo::from_format(*other))
    }
}

impl Ord for ::VideoFormat {
    fn cmp(&self, other: &::VideoFormat) -> std::cmp::Ordering {
        ::VideoFormatInfo::from_format(*self).cmp(&::VideoFormatInfo::from_format(*other))
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.idx == self.len {
            return (0, Some(0));
        }

        let remaining = (self.len - self.idx) as usize;

        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for VideoFormatIterator {}

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

        assert!(::VideoFormat::iter_raw().any(|f| f == ::VideoFormat::Nv12));
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

    #[cfg(feature = "v1_18")]
    #[test]
    fn sort() {
        use itertools::Itertools;

        gst::init().unwrap();

        assert!(
            ::VideoFormatInfo::from_format(::VideoFormat::Nv16)
                > ::VideoFormatInfo::from_format(::VideoFormat::Nv12)
        );
        assert!(::VideoFormat::I420 > ::VideoFormat::Yv12);

        let sorted: Vec<::VideoFormat> = ::VideoFormat::iter_raw().sorted().rev().collect();
        // FIXME: use is_sorted_by() once API is in stable
        assert_eq!(
            sorted,
            ::VideoFormat::iter_raw().collect::<Vec<::VideoFormat>>()
        );
    }
}
