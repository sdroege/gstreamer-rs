// Take a look at the license at the top of the repository in the LICENSE file.

use std::{ffi::CStr, str};

use glib::translate::{from_glib, FromGlib, IntoGlib};
use once_cell::sync::Lazy;

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
            crate::VideoFormat::Gbra12le,
            crate::VideoFormat::Gbra12be,
            crate::VideoFormat::A44410le,
            crate::VideoFormat::Gbra10le,
            crate::VideoFormat::A44410be,
            crate::VideoFormat::Gbra10be,
            crate::VideoFormat::A42210le,
            crate::VideoFormat::A42210be,
            crate::VideoFormat::A42010le,
            crate::VideoFormat::A42010be,
            #[cfg(feature = "v1_16")]
            crate::VideoFormat::Bgr10a2Le,
            #[cfg(feature = "v1_16")]
            crate::VideoFormat::Y410,
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
            crate::VideoFormat::Y44412le,
            crate::VideoFormat::Gbr12le,
            crate::VideoFormat::Y44412be,
            crate::VideoFormat::Gbr12be,
            crate::VideoFormat::I42212le,
            crate::VideoFormat::I42212be,
            crate::VideoFormat::I42012le,
            crate::VideoFormat::I42012be,
            crate::VideoFormat::Y44410le,
            crate::VideoFormat::Gbr10le,
            crate::VideoFormat::Y44410be,
            crate::VideoFormat::Gbr10be,
            crate::VideoFormat::R210,
            crate::VideoFormat::I42210le,
            crate::VideoFormat::I42210be,
            crate::VideoFormat::Nv1610le32,
            #[cfg(feature = "v1_16")]
            crate::VideoFormat::Y210,
            crate::VideoFormat::V210,
            crate::VideoFormat::Uyvp,
            crate::VideoFormat::I42010le,
            crate::VideoFormat::I42010be,
            crate::VideoFormat::P01010le,
            crate::VideoFormat::Nv1210le32,
            #[cfg(feature = "v1_16")]
            crate::VideoFormat::Nv1210le40,
            crate::VideoFormat::P01010be,
            crate::VideoFormat::Y444,
            crate::VideoFormat::Gbr,
            crate::VideoFormat::Nv24,
            crate::VideoFormat::Xbgr,
            crate::VideoFormat::Bgrx,
            crate::VideoFormat::Xrgb,
            crate::VideoFormat::Rgbx,
            crate::VideoFormat::Bgr,
            crate::VideoFormat::Iyu2,
            crate::VideoFormat::V308,
            crate::VideoFormat::Rgb,
            crate::VideoFormat::Y42b,
            crate::VideoFormat::Nv61,
            crate::VideoFormat::Nv16,
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
            crate::VideoFormat::Gray10Le32,
            crate::VideoFormat::Gray8,
        ])
    }
    #[cfg(target_endian = "big")]
    {
        Box::new([
            crate::VideoFormat::Ayuv64,
            crate::VideoFormat::Argb64,
            crate::VideoFormat::Gbra12be,
            crate::VideoFormat::Gbra12le,
            crate::VideoFormat::A44410be,
            crate::VideoFormat::Gbra10be,
            crate::VideoFormat::A44410le,
            crate::VideoFormat::Gbra10le,
            crate::VideoFormat::A42210be,
            crate::VideoFormat::A42210le,
            crate::VideoFormat::A42010be,
            crate::VideoFormat::A42010le,
            #[cfg(feature = "v1_16")]
            crate::VideoFormat::Y410,
            #[cfg(feature = "v1_16")]
            crate::VideoFormat::Bgr10a2Le,
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
            crate::VideoFormat::Y44412be,
            crate::VideoFormat::Gbr12be,
            crate::VideoFormat::Y44412le,
            crate::VideoFormat::Gbr12le,
            crate::VideoFormat::I42212be,
            crate::VideoFormat::I42212le,
            crate::VideoFormat::I42012be,
            crate::VideoFormat::I42012le,
            crate::VideoFormat::Y44410be,
            crate::VideoFormat::Gbr10be,
            crate::VideoFormat::Y44410le,
            crate::VideoFormat::Gbr10le,
            crate::VideoFormat::R210,
            crate::VideoFormat::I42210be,
            crate::VideoFormat::I42210le,
            crate::VideoFormat::Nv1610le32,
            #[cfg(feature = "v1_16")]
            crate::VideoFormat::Y210,
            crate::VideoFormat::V210,
            crate::VideoFormat::Uyvp,
            crate::VideoFormat::I42010be,
            crate::VideoFormat::I42010le,
            crate::VideoFormat::P01010be,
            crate::VideoFormat::P01010le,
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
            crate::VideoFormat::Iyu2,
            crate::VideoFormat::V308,
            crate::VideoFormat::Rgb,
            crate::VideoFormat::Y42b,
            crate::VideoFormat::Nv61,
            crate::VideoFormat::Nv16,
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
    unsafe fn from_glib(value: i32) -> Self {
        skip_assert_initialized!();

        match value {
            1234 => Self::LittleEndian,
            4321 => Self::BigEndian,
            _ => Self::Unknown,
        }
    }
}

impl IntoGlib for VideoEndianness {
    type GlibType = i32;

    fn into_glib(self) -> i32 {
        match self {
            Self::LittleEndian => 1234,
            Self::BigEndian => 4321,
            _ => 0,
        }
    }
}

impl crate::VideoFormat {
    #[doc(alias = "gst_video_format_from_masks")]
    pub fn from_masks(
        depth: u32,
        bpp: u32,
        endianness: crate::VideoEndianness,
        red_mask: u32,
        blue_mask: u32,
        green_mask: u32,
        alpha_mask: u32,
    ) -> Self {
        assert_initialized_main_thread!();

        unsafe {
            from_glib(ffi::gst_video_format_from_masks(
                depth as i32,
                bpp as i32,
                endianness.into_glib(),
                red_mask,
                blue_mask,
                green_mask,
                alpha_mask,
            ))
        }
    }

    #[doc(alias = "gst_video_format_to_string")]
    pub fn to_str<'a>(self) -> &'a str {
        if self == Self::Unknown {
            return "UNKNOWN";
        }
        unsafe {
            CStr::from_ptr(
                ffi::gst_video_format_to_string(self.into_glib())
                    .as_ref()
                    .expect("gst_video_format_to_string returned NULL"),
            )
            .to_str()
            .expect("gst_video_format_to_string returned an invalid string")
        }
    }

    pub fn iter_raw() -> VideoFormatIterator {
        VideoFormatIterator::default()
    }
}

impl str::FromStr for crate::VideoFormat {
    type Err = glib::BoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        skip_assert_initialized!();

        let fmt = Self::from_string(s);
        if fmt == Self::Unknown {
            Err(glib::bool_error!(
                "Failed to parse video format from string"
            ))
        } else {
            Ok(fmt)
        }
    }
}

impl PartialOrd for crate::VideoFormat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        crate::VideoFormatInfo::from_format(*self)
            .partial_cmp(&crate::VideoFormatInfo::from_format(*other))
    }
}

impl Ord for crate::VideoFormat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
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

        let remaining = self.len - self.idx;

        (remaining, Some(remaining))
    }

    fn count(self) -> usize {
        self.len - self.idx
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.idx.overflowing_add(n);
        if end >= self.len || overflow {
            self.idx = self.len;
            None
        } else {
            self.idx = end + 1;
            Some(VIDEO_FORMATS_ALL[end])
        }
    }

    fn last(self) -> Option<Self::Item> {
        if self.idx == self.len {
            None
        } else {
            Some(VIDEO_FORMATS_ALL[self.len - 1])
        }
    }
}

impl ExactSizeIterator for VideoFormatIterator {}

impl std::iter::FusedIterator for VideoFormatIterator {}

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

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.len.overflowing_sub(n);
        if end <= self.idx || overflow {
            self.idx = self.len;
            None
        } else {
            self.len = end - 1;
            let fmt = VIDEO_FORMATS_ALL[self.len];
            Some(fmt)
        }
    }
}
pub trait VideoFormatIteratorExt {
    fn into_video_caps(self) -> Option<crate::VideoCapsBuilder<gst::caps::NoFeature>>;
}

impl<T> VideoFormatIteratorExt for T
where
    T: Iterator<Item = crate::VideoFormat>,
{
    fn into_video_caps(self) -> Option<crate::VideoCapsBuilder<gst::caps::NoFeature>> {
        let formats: Vec<crate::VideoFormat> = self.collect();
        if !formats.is_empty() {
            Some(crate::functions::video_make_raw_caps(&formats))
        } else {
            None
        }
    }
}

pub trait VideoFormatIteratorExtRef {
    fn into_video_caps(self) -> Option<crate::VideoCapsBuilder<gst::caps::NoFeature>>;
}

impl<'a, T> VideoFormatIteratorExtRef for T
where
    T: Iterator<Item = &'a crate::VideoFormat>,
{
    fn into_video_caps(self) -> Option<crate::VideoCapsBuilder<gst::caps::NoFeature>> {
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
    fn enum_to_string() {
        gst::init().unwrap();

        assert_eq!(&format!("{}", crate::VideoFormat::Argb), "ARGB");
        assert_eq!(&format!("{:?}", crate::VideoFormat::Argb), "Argb");
        assert_eq!(crate::VideoFormat::Argb.to_str(), "ARGB");

        assert_eq!(&format!("{}", crate::VideoFormat::Unknown), "UNKNOWN");
        assert_eq!(&format!("{:?}", crate::VideoFormat::Unknown), "Unknown");
        assert_eq!(crate::VideoFormat::Unknown.to_str(), "UNKNOWN");

        assert_eq!(
            &format!("{:?}", crate::VideoFormat::__Unknown(-1)),
            "__Unknown(-1)"
        );
    }

    #[test]
    #[should_panic(expected = "gst_video_format_to_string returned NULL")]
    fn enum_to_string_panics() {
        assert_eq!(&format!("{}", crate::VideoFormat::__Unknown(-1)), "UNKNOWN");
        assert_eq!(crate::VideoFormat::__Unknown(-1).to_str(), "UNKNOWN");
    }

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
        assert!(!crate::VideoFormat::iter_raw().any(|f| f == crate::VideoFormat::Encoded));

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
