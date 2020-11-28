// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use once_cell::sync::Lazy;
use std::ffi::CStr;
use std::fmt;
use std::str;

use glib::translate::{from_glib, FromGlib, ToGlib, ToGlibPtr};

#[cfg(feature = "v1_18")]
pub static VIDEO_FORMATS_ALL: Lazy<Box<[crate::VideoFormat]>> = Lazy::new(|| unsafe {
    let mut len: u32 = 0;
    let mut res = Vec::with_capacity(len as usize);
    let formats = ffi::gst_video_formats_raw(&mut len);
    for i in 0..len {
        let format = formats.offset(i as isize);
        res.push(from_glib(*format));
    }
    res.into_boxed_slice()
});

#[cfg(not(feature = "v1_18"))]
pub static VIDEO_FORMATS_ALL: Lazy<Box<[crate::VideoFormat]>> = Lazy::new(|| {
    #[cfg(target_endian = "little")]
    {
        Box::new([
            crate::VideoFormat::Ayuv64,
            crate::VideoFormat::Argb64,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Gbra12le,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Gbra12be,
            crate::VideoFormat::A44410le,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Gbra10le,
            crate::VideoFormat::A44410be,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Gbra10be,
            crate::VideoFormat::A42210le,
            crate::VideoFormat::A42210be,
            crate::VideoFormat::A42010le,
            crate::VideoFormat::A42010be,
            #[cfg(feature = "v1_16")]
            crate::VideoFormat::Bgr10a2Le,
            #[cfg(feature = "v1_16")]
            crate::VideoFormat::Y410,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Gbra,
            crate::VideoFormat::Abgr,
            #[cfg(feature = "v1_16")]
            crate::VideoFormat::Vuya,
            crate::VideoFormat::Bgra,
            crate::VideoFormat::Ayuv,
            crate::VideoFormat::Argb,
            crate::VideoFormat::Rgba,
            crate::VideoFormat::A420,
            crate::VideoFormat::V216,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Y44412le,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Gbr12le,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Y44412be,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Gbr12be,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::I42212le,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::I42212be,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::I42012le,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::I42012be,
            crate::VideoFormat::Y44410le,
            crate::VideoFormat::Gbr10le,
            crate::VideoFormat::Y44410be,
            crate::VideoFormat::Gbr10be,
            crate::VideoFormat::R210,
            crate::VideoFormat::I42210le,
            crate::VideoFormat::I42210be,
            #[cfg(feature = "v1_14")]
            crate::VideoFormat::Nv1610le32,
            #[cfg(feature = "v1_16")]
            crate::VideoFormat::Y210,
            crate::VideoFormat::V210,
            crate::VideoFormat::Uyvp,
            crate::VideoFormat::I42010le,
            crate::VideoFormat::I42010be,
            #[cfg(feature = "v1_10")]
            crate::VideoFormat::P01010le,
            #[cfg(feature = "v1_14")]
            crate::VideoFormat::Nv1210le32,
            #[cfg(feature = "v1_16")]
            crate::VideoFormat::Nv1210le40,
            #[cfg(feature = "v1_10")]
            crate::VideoFormat::P01010be,
            crate::VideoFormat::Y444,
            crate::VideoFormat::Gbr,
            crate::VideoFormat::Nv24,
            crate::VideoFormat::Xbgr,
            crate::VideoFormat::Bgrx,
            crate::VideoFormat::Xrgb,
            crate::VideoFormat::Rgbx,
            crate::VideoFormat::Bgr,
            #[cfg(feature = "v1_10")]
            crate::VideoFormat::Iyu2,
            crate::VideoFormat::V308,
            crate::VideoFormat::Rgb,
            crate::VideoFormat::Y42b,
            crate::VideoFormat::Nv61,
            crate::VideoFormat::Nv16,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Vyuy,
            crate::VideoFormat::Uyvy,
            crate::VideoFormat::Yvyu,
            crate::VideoFormat::Yuy2,
            crate::VideoFormat::I420,
            crate::VideoFormat::Yv12,
            crate::VideoFormat::Nv21,
            crate::VideoFormat::Nv12,
            crate::VideoFormat::Nv1264z32,
            crate::VideoFormat::Y41b,
            crate::VideoFormat::Iyu1,
            crate::VideoFormat::Yvu9,
            crate::VideoFormat::Yuv9,
            crate::VideoFormat::Rgb16,
            crate::VideoFormat::Bgr16,
            crate::VideoFormat::Rgb15,
            crate::VideoFormat::Bgr15,
            crate::VideoFormat::Rgb8p,
            crate::VideoFormat::Gray16Le,
            crate::VideoFormat::Gray16Be,
            #[cfg(feature = "v1_14")]
            crate::VideoFormat::Gray10Le32,
            crate::VideoFormat::Gray8,
        ])
    }
    #[cfg(target_endian = "big")]
    {
        Box::new([
            crate::VideoFormat::Ayuv64,
            crate::VideoFormat::Argb64,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Gbra12be,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Gbra12le,
            crate::VideoFormat::A44410be,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Gbra10be,
            crate::VideoFormat::A44410le,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Gbra10le,
            crate::VideoFormat::A42210be,
            crate::VideoFormat::A42210le,
            crate::VideoFormat::A42010be,
            crate::VideoFormat::A42010le,
            #[cfg(feature = "v1_16")]
            crate::VideoFormat::Y410,
            #[cfg(feature = "v1_16")]
            crate::VideoFormat::Bgr10a2Le,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Gbra,
            crate::VideoFormat::Abgr,
            #[cfg(feature = "v1_16")]
            crate::VideoFormat::Vuya,
            crate::VideoFormat::Bgra,
            crate::VideoFormat::Ayuv,
            crate::VideoFormat::Argb,
            crate::VideoFormat::Rgba,
            crate::VideoFormat::A420,
            crate::VideoFormat::V216,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Y44412be,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Gbr12be,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Y44412le,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Gbr12le,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::I42212be,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::I42212le,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::I42012be,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::I42012le,
            crate::VideoFormat::Y44410be,
            crate::VideoFormat::Gbr10be,
            crate::VideoFormat::Y44410le,
            crate::VideoFormat::Gbr10le,
            crate::VideoFormat::R210,
            crate::VideoFormat::I42210be,
            crate::VideoFormat::I42210le,
            #[cfg(feature = "v1_14")]
            crate::VideoFormat::Nv1610le32,
            #[cfg(feature = "v1_16")]
            crate::VideoFormat::Y210,
            crate::VideoFormat::V210,
            crate::VideoFormat::Uyvp,
            crate::VideoFormat::I42010be,
            crate::VideoFormat::I42010le,
            #[cfg(feature = "v1_10")]
            crate::VideoFormat::P01010be,
            #[cfg(feature = "v1_10")]
            crate::VideoFormat::P01010le,
            #[cfg(feature = "v1_14")]
            crate::VideoFormat::Nv1210le32,
            #[cfg(feature = "v1_16")]
            crate::VideoFormat::Nv1210le40,
            crate::VideoFormat::Y444,
            crate::VideoFormat::Gbr,
            crate::VideoFormat::Nv24,
            crate::VideoFormat::Xbgr,
            crate::VideoFormat::Bgrx,
            crate::VideoFormat::Xrgb,
            crate::VideoFormat::Rgbx,
            crate::VideoFormat::Bgr,
            #[cfg(feature = "v1_10")]
            crate::VideoFormat::Iyu2,
            crate::VideoFormat::V308,
            crate::VideoFormat::Rgb,
            crate::VideoFormat::Y42b,
            crate::VideoFormat::Nv61,
            crate::VideoFormat::Nv16,
            #[cfg(feature = "v1_12")]
            crate::VideoFormat::Vyuy,
            crate::VideoFormat::Uyvy,
            crate::VideoFormat::Yvyu,
            crate::VideoFormat::Yuy2,
            crate::VideoFormat::I420,
            crate::VideoFormat::Yv12,
            crate::VideoFormat::Nv21,
            crate::VideoFormat::Nv12,
            crate::VideoFormat::Nv1264z32,
            crate::VideoFormat::Y41b,
            crate::VideoFormat::Iyu1,
            crate::VideoFormat::Yvu9,
            crate::VideoFormat::Yuv9,
            crate::VideoFormat::Rgb16,
            crate::VideoFormat::Bgr16,
            crate::VideoFormat::Rgb15,
            crate::VideoFormat::Bgr15,
            crate::VideoFormat::Rgb8p,
            crate::VideoFormat::Gray16Be,
            crate::VideoFormat::Gray16Le,
            #[cfg(feature = "v1_14")]
            crate::VideoFormat::Gray10Le32,
            crate::VideoFormat::Gray8,
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

impl crate::VideoFormat {
    pub fn from_fourcc(fourcc: u32) -> crate::VideoFormat {
        assert_initialized_main_thread!();

        unsafe { from_glib(ffi::gst_video_format_from_fourcc(fourcc)) }
    }

    pub fn from_masks(
        depth: u32,
        bpp: u32,
        endianness: crate::VideoEndianness,
        red_mask: u32,
        blue_mask: u32,
        green_mask: u32,
        alpha_mask: u32,
    ) -> crate::VideoFormat {
        assert_initialized_main_thread!();

        unsafe {
            from_glib(ffi::gst_video_format_from_masks(
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
        if self == crate::VideoFormat::Unknown {
            return "UNKNOWN";
        }

        unsafe {
            CStr::from_ptr(ffi::gst_video_format_to_string(self.to_glib()))
                .to_str()
                .unwrap()
        }
    }

    pub fn iter_raw() -> VideoFormatIterator {
        VideoFormatIterator::default()
    }
}

impl str::FromStr for crate::VideoFormat {
    type Err = glib::BoolError;

    fn from_str(s: &str) -> Result<Self, glib::BoolError> {
        assert_initialized_main_thread!();

        unsafe {
            let fmt = crate::VideoFormat::from_glib(ffi::gst_video_format_from_string(
                s.to_glib_none().0,
            ));

            if fmt == crate::VideoFormat::Unknown {
                Err(glib::glib_bool_error!(
                    "Failed to parse video format from string"
                ))
            } else {
                Ok(fmt)
            }
        }
    }
}

impl fmt::Display for crate::VideoFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str((*self).to_str())
    }
}

impl PartialOrd for crate::VideoFormat {
    fn partial_cmp(&self, other: &crate::VideoFormat) -> Option<std::cmp::Ordering> {
        crate::VideoFormatInfo::from_format(*self)
            .partial_cmp(&crate::VideoFormatInfo::from_format(*other))
    }
}

impl Ord for crate::VideoFormat {
    fn cmp(&self, other: &crate::VideoFormat) -> std::cmp::Ordering {
        crate::VideoFormatInfo::from_format(*self).cmp(&crate::VideoFormatInfo::from_format(*other))
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
    type Item = crate::VideoFormat;

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
    T: Iterator<Item = crate::VideoFormat>,
{
    fn into_video_caps(self) -> Option<gst::caps::Builder<gst::caps::NoFeature>> {
        let formats: Vec<crate::VideoFormat> = self.collect();
        if !formats.is_empty() {
            Some(crate::functions::video_make_raw_caps(&formats))
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
    T: Iterator<Item = &'a crate::VideoFormat>,
{
    fn into_video_caps(self) -> Option<gst::caps::Builder<gst::caps::NoFeature>> {
        let formats: Vec<crate::VideoFormat> = self.copied().collect();
        if !formats.is_empty() {
            Some(crate::functions::video_make_raw_caps(&formats))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_display() {
        gst::init().unwrap();

        format!("{}", crate::VideoFormat::Nv16);
    }

    #[test]
    fn iter() {
        use super::*;
        gst::init().unwrap();

        assert!(crate::VideoFormat::iter_raw().count() > 0);
        assert_eq!(
            crate::VideoFormat::iter_raw().count(),
            crate::VideoFormat::iter_raw().len()
        );

        let mut i = crate::VideoFormat::iter_raw();
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
        assert_eq!(count, crate::VideoFormat::iter_raw().len());

        assert!(crate::VideoFormat::iter_raw().any(|f| f == crate::VideoFormat::Nv12));
        assert!(crate::VideoFormat::iter_raw()
            .find(|f| *f == crate::VideoFormat::Encoded)
            .is_none());

        let caps = crate::VideoFormat::iter_raw().into_video_caps();
        assert!(caps.is_some());

        let caps = crate::VideoFormat::iter_raw()
            .filter(|f| crate::VideoFormatInfo::from_format(*f).is_gray())
            .into_video_caps();
        assert!(caps.is_some());

        let caps = crate::VideoFormat::iter_raw().skip(1000).into_video_caps();
        assert!(caps.is_none());

        let caps = [crate::VideoFormat::Nv12, crate::VideoFormat::Nv16]
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
            crate::VideoFormatInfo::from_format(crate::VideoFormat::Nv16)
                > crate::VideoFormatInfo::from_format(crate::VideoFormat::Nv12)
        );
        assert!(crate::VideoFormat::I420 > crate::VideoFormat::Yv12);

        let sorted: Vec<crate::VideoFormat> =
            crate::VideoFormat::iter_raw().sorted().rev().collect();
        // FIXME: use is_sorted_by() once API is in stable
        assert_eq!(
            sorted,
            crate::VideoFormat::iter_raw().collect::<Vec<crate::VideoFormat>>()
        );
    }
}
