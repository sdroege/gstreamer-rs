// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_audio_sys;

use std::ffi::CStr;
use std::fmt;
use std::str;

use glib::translate::{from_glib, FromGlib, ToGlib, ToGlibPtr};
use once_cell::sync::Lazy;

#[cfg(feature = "v1_18")]
pub static AUDIO_FORMATS_ALL: Lazy<Box<[::AudioFormat]>> = Lazy::new(|| unsafe {
    let mut len: u32 = 0;
    let mut res = Vec::with_capacity(len as usize);
    let formats = gst_audio_sys::gst_audio_formats_raw(&mut len);
    for i in 0..len {
        let format = formats.offset(i as isize);
        res.push(from_glib(*format));
    }
    res.into_boxed_slice()
});

#[cfg(not(feature = "v1_18"))]
pub static AUDIO_FORMATS_ALL: Lazy<Box<[::AudioFormat]>> = Lazy::new(|| {
    #[cfg(target_endian = "little")]
    {
        Box::new([
            ::AudioFormat::F64le,
            ::AudioFormat::F64be,
            ::AudioFormat::F32le,
            ::AudioFormat::F32be,
            ::AudioFormat::S32le,
            ::AudioFormat::S32be,
            ::AudioFormat::U32le,
            ::AudioFormat::U32be,
            ::AudioFormat::S2432le,
            ::AudioFormat::S2432be,
            ::AudioFormat::U2432le,
            ::AudioFormat::U2432be,
            ::AudioFormat::S24le,
            ::AudioFormat::S24be,
            ::AudioFormat::U24le,
            ::AudioFormat::U24be,
            ::AudioFormat::S20le,
            ::AudioFormat::S20be,
            ::AudioFormat::U20le,
            ::AudioFormat::U20be,
            ::AudioFormat::S18le,
            ::AudioFormat::S18be,
            ::AudioFormat::U18le,
            ::AudioFormat::U18be,
            ::AudioFormat::S16le,
            ::AudioFormat::S16be,
            ::AudioFormat::U16le,
            ::AudioFormat::U16be,
            ::AudioFormat::S8,
            ::AudioFormat::U8,
        ])
    }
    #[cfg(target_endian = "big")]
    {
        Box::new([
            ::AudioFormat::F64be,
            ::AudioFormat::F64le,
            ::AudioFormat::F32be,
            ::AudioFormat::F32le,
            ::AudioFormat::S32be,
            ::AudioFormat::S32le,
            ::AudioFormat::U32be,
            ::AudioFormat::U32le,
            ::AudioFormat::S2432be,
            ::AudioFormat::S2432le,
            ::AudioFormat::U2432be,
            ::AudioFormat::U2432le,
            ::AudioFormat::S24be,
            ::AudioFormat::S24le,
            ::AudioFormat::U24be,
            ::AudioFormat::U24le,
            ::AudioFormat::S20be,
            ::AudioFormat::S20le,
            ::AudioFormat::U20be,
            ::AudioFormat::U20le,
            ::AudioFormat::S18be,
            ::AudioFormat::S18le,
            ::AudioFormat::U18be,
            ::AudioFormat::U18le,
            ::AudioFormat::S16be,
            ::AudioFormat::S16le,
            ::AudioFormat::U16be,
            ::AudioFormat::U16le,
            ::AudioFormat::S8,
            ::AudioFormat::U8,
        ])
    }
});

impl ::AudioFormat {
    pub fn build_integer(
        sign: bool,
        endianness: ::AudioEndianness,
        width: i32,
        depth: i32,
    ) -> ::AudioFormat {
        assert_initialized_main_thread!();

        unsafe {
            from_glib(gst_audio_sys::gst_audio_format_build_integer(
                sign.to_glib(),
                endianness.to_glib(),
                width,
                depth,
            ))
        }
    }

    pub fn to_str<'a>(self) -> &'a str {
        if self == ::AudioFormat::Unknown {
            return "UNKNOWN";
        }

        unsafe {
            CStr::from_ptr(gst_audio_sys::gst_audio_format_to_string(self.to_glib()))
                .to_str()
                .unwrap()
        }
    }

    pub fn iter_raw() -> AudioFormatIterator {
        AudioFormatIterator::default()
    }
}

impl str::FromStr for ::AudioFormat {
    type Err = glib::BoolError;

    fn from_str(s: &str) -> Result<Self, glib::BoolError> {
        assert_initialized_main_thread!();

        unsafe {
            let fmt = ::AudioFormat::from_glib(gst_audio_sys::gst_audio_format_from_string(
                s.to_glib_none().0,
            ));
            if fmt == ::AudioFormat::Unknown {
                Err(glib_bool_error!("Failed to parse audio format from string"))
            } else {
                Ok(fmt)
            }
        }
    }
}

impl fmt::Display for ::AudioFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str((*self).to_str())
    }
}

impl PartialOrd for ::AudioFormat {
    fn partial_cmp(&self, other: &::AudioFormat) -> Option<std::cmp::Ordering> {
        ::AudioFormatInfo::from_format(*self).partial_cmp(&::AudioFormatInfo::from_format(*other))
    }
}

impl Ord for ::AudioFormat {
    fn cmp(&self, other: &::AudioFormat) -> std::cmp::Ordering {
        ::AudioFormatInfo::from_format(*self).cmp(&::AudioFormatInfo::from_format(*other))
    }
}

pub const AUDIO_FORMAT_UNKNOWN: ::AudioFormat = ::AudioFormat::Unknown;
pub const AUDIO_FORMAT_ENCODED: ::AudioFormat = ::AudioFormat::Encoded;
pub const AUDIO_FORMAT_S8: ::AudioFormat = ::AudioFormat::S8;
pub const AUDIO_FORMAT_U8: ::AudioFormat = ::AudioFormat::U8;

#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_S16: ::AudioFormat = ::AudioFormat::S16be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_U16: ::AudioFormat = ::AudioFormat::U16be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_S2432: ::AudioFormat = ::AudioFormat::S2432be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_U2432: ::AudioFormat = ::AudioFormat::U2432be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_S32: ::AudioFormat = ::AudioFormat::S32be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_U32: ::AudioFormat = ::AudioFormat::U32be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_S24: ::AudioFormat = ::AudioFormat::S24be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_U24: ::AudioFormat = ::AudioFormat::U24be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_S20: ::AudioFormat = ::AudioFormat::S20be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_U20: ::AudioFormat = ::AudioFormat::U20be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_S18: ::AudioFormat = ::AudioFormat::S18be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_U18: ::AudioFormat = ::AudioFormat::U18be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_F32: ::AudioFormat = ::AudioFormat::F32be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_F64: ::AudioFormat = ::AudioFormat::F64be;

#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_S16: ::AudioFormat = ::AudioFormat::S16le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_U16: ::AudioFormat = ::AudioFormat::U16le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_S2432: ::AudioFormat = ::AudioFormat::S2432le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_U2432: ::AudioFormat = ::AudioFormat::U2432le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_S32: ::AudioFormat = ::AudioFormat::S32le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_U32: ::AudioFormat = ::AudioFormat::U32le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_S24: ::AudioFormat = ::AudioFormat::S24le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_U24: ::AudioFormat = ::AudioFormat::U24le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_S20: ::AudioFormat = ::AudioFormat::S20le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_U20: ::AudioFormat = ::AudioFormat::U20le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_S18: ::AudioFormat = ::AudioFormat::S18le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_U18: ::AudioFormat = ::AudioFormat::U18le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_F32: ::AudioFormat = ::AudioFormat::F32le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_F64: ::AudioFormat = ::AudioFormat::F64le;

pub struct AudioFormatIterator {
    idx: usize,
    len: usize,
}

impl Default for AudioFormatIterator {
    fn default() -> Self {
        Self {
            idx: 0,
            len: AUDIO_FORMATS_ALL.len(),
        }
    }
}

impl Iterator for AudioFormatIterator {
    type Item = ::AudioFormat;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.len {
            None
        } else {
            let fmt = AUDIO_FORMATS_ALL[self.idx];
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

impl ExactSizeIterator for AudioFormatIterator {}

impl DoubleEndedIterator for AudioFormatIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx >= self.len {
            None
        } else {
            let fmt = AUDIO_FORMATS_ALL[self.len - 1];
            self.len -= 1;
            Some(fmt)
        }
    }
}
pub trait AudioFormatIteratorExt {
    fn into_audio_caps(
        self,
        layout: ::AudioLayout,
    ) -> Option<gst::caps::Builder<gst::caps::NoFeature>>;
}

impl<T> AudioFormatIteratorExt for T
where
    T: Iterator<Item = ::AudioFormat>,
{
    fn into_audio_caps(
        self,
        layout: ::AudioLayout,
    ) -> Option<gst::caps::Builder<gst::caps::NoFeature>> {
        let formats: Vec<::AudioFormat> = self.collect();
        if !formats.is_empty() {
            Some(::functions::audio_make_raw_caps(&formats, layout))
        } else {
            None
        }
    }
}

pub trait AudioFormatIteratorExtRef {
    fn into_audio_caps(
        self,
        layout: ::AudioLayout,
    ) -> Option<gst::caps::Builder<gst::caps::NoFeature>>;
}

impl<'a, T> AudioFormatIteratorExtRef for T
where
    T: Iterator<Item = &'a ::AudioFormat>,
{
    fn into_audio_caps(
        self,
        layout: ::AudioLayout,
    ) -> Option<gst::caps::Builder<gst::caps::NoFeature>> {
        let formats: Vec<::AudioFormat> = self.copied().collect();
        if !formats.is_empty() {
            Some(::functions::audio_make_raw_caps(&formats, layout))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use gst;
    use itertools::Itertools;

    #[test]
    fn test_display() {
        gst::init().unwrap();

        format!("{}", ::AudioFormat::S16be);
    }

    #[test]
    fn iter() {
        use super::*;
        gst::init().unwrap();

        assert!(::AudioFormat::iter_raw().count() > 0);
        assert_eq!(
            ::AudioFormat::iter_raw().count(),
            ::AudioFormat::iter_raw().len()
        );

        let mut i = ::AudioFormat::iter_raw();
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
        assert_eq!(count, ::AudioFormat::iter_raw().len());

        assert!(::AudioFormat::iter_raw().any(|f| f == ::AudioFormat::F64be));
        assert!(::AudioFormat::iter_raw()
            .find(|f| *f == ::AudioFormat::Encoded)
            .is_none());

        let caps = ::AudioFormat::iter_raw().into_audio_caps(::AudioLayout::Interleaved);
        assert!(caps.is_some());

        let caps = ::AudioFormat::iter_raw()
            .filter(|f| ::AudioFormatInfo::from_format(*f).is_little_endian())
            .into_audio_caps(::AudioLayout::Interleaved);
        assert!(caps.is_some());

        let caps = ::AudioFormat::iter_raw()
            .skip(1000)
            .into_audio_caps(::AudioLayout::Interleaved);
        assert!(caps.is_none());

        let caps = [::AudioFormat::S16le, ::AudioFormat::S16be]
            .iter()
            .into_audio_caps(::AudioLayout::Interleaved)
            .unwrap()
            .build();
        assert_eq!(caps.to_string(), "audio/x-raw, format=(string){ S16LE, S16BE }, rate=(int)[ 1, 2147483647 ], channels=(int)[ 1, 2147483647 ], layout=(string)interleaved");
    }

    #[test]
    fn sort() {
        gst::init().unwrap();

        assert!(
            ::AudioFormatInfo::from_format(::AudioFormat::F64be)
                > ::AudioFormatInfo::from_format(::AudioFormat::U8)
        );
        assert!(::AudioFormat::S20be > ::AudioFormat::S18be);

        let sorted: Vec<::AudioFormat> = ::AudioFormat::iter_raw().sorted().rev().collect();
        // FIXME: use is_sorted_by() once API is in stable
        assert_eq!(
            sorted,
            ::AudioFormat::iter_raw().collect::<Vec<::AudioFormat>>()
        );
    }
}
