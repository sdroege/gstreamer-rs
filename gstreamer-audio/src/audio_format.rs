// Take a look at the license at the top of the repository in the LICENSE file.

use std::str;

use glib::translate::{from_glib, IntoGlib};
use once_cell::sync::Lazy;

#[cfg(feature = "v1_18")]
pub static AUDIO_FORMATS_ALL: Lazy<Box<[crate::AudioFormat]>> = Lazy::new(|| unsafe {
    let mut len: u32 = 0;
    let mut res = Vec::with_capacity(len as usize);
    let formats = ffi::gst_audio_formats_raw(&mut len);
    for i in 0..len {
        let format = formats.offset(i as isize);
        res.push(from_glib(*format));
    }
    res.into_boxed_slice()
});

#[cfg(not(feature = "v1_18"))]
pub static AUDIO_FORMATS_ALL: Lazy<Box<[crate::AudioFormat]>> = Lazy::new(|| {
    #[cfg(target_endian = "little")]
    {
        Box::new([
            crate::AudioFormat::F64le,
            crate::AudioFormat::F64be,
            crate::AudioFormat::F32le,
            crate::AudioFormat::F32be,
            crate::AudioFormat::S32le,
            crate::AudioFormat::S32be,
            crate::AudioFormat::U32le,
            crate::AudioFormat::U32be,
            crate::AudioFormat::S2432le,
            crate::AudioFormat::S2432be,
            crate::AudioFormat::U2432le,
            crate::AudioFormat::U2432be,
            crate::AudioFormat::S24le,
            crate::AudioFormat::S24be,
            crate::AudioFormat::U24le,
            crate::AudioFormat::U24be,
            crate::AudioFormat::S20le,
            crate::AudioFormat::S20be,
            crate::AudioFormat::U20le,
            crate::AudioFormat::U20be,
            crate::AudioFormat::S18le,
            crate::AudioFormat::S18be,
            crate::AudioFormat::U18le,
            crate::AudioFormat::U18be,
            crate::AudioFormat::S16le,
            crate::AudioFormat::S16be,
            crate::AudioFormat::U16le,
            crate::AudioFormat::U16be,
            crate::AudioFormat::S8,
            crate::AudioFormat::U8,
        ])
    }
    #[cfg(target_endian = "big")]
    {
        Box::new([
            crate::AudioFormat::F64be,
            crate::AudioFormat::F64le,
            crate::AudioFormat::F32be,
            crate::AudioFormat::F32le,
            crate::AudioFormat::S32be,
            crate::AudioFormat::S32le,
            crate::AudioFormat::U32be,
            crate::AudioFormat::U32le,
            crate::AudioFormat::S2432be,
            crate::AudioFormat::S2432le,
            crate::AudioFormat::U2432be,
            crate::AudioFormat::U2432le,
            crate::AudioFormat::S24be,
            crate::AudioFormat::S24le,
            crate::AudioFormat::U24be,
            crate::AudioFormat::U24le,
            crate::AudioFormat::S20be,
            crate::AudioFormat::S20le,
            crate::AudioFormat::U20be,
            crate::AudioFormat::U20le,
            crate::AudioFormat::S18be,
            crate::AudioFormat::S18le,
            crate::AudioFormat::U18be,
            crate::AudioFormat::U18le,
            crate::AudioFormat::S16be,
            crate::AudioFormat::S16le,
            crate::AudioFormat::U16be,
            crate::AudioFormat::U16le,
            crate::AudioFormat::S8,
            crate::AudioFormat::U8,
        ])
    }
});

impl crate::AudioFormat {
    #[doc(alias = "gst_audio_format_build_integer")]
    pub fn build_integer(
        sign: bool,
        endianness: crate::AudioEndianness,
        width: i32,
        depth: i32,
    ) -> Self {
        assert_initialized_main_thread!();

        unsafe {
            from_glib(ffi::gst_audio_format_build_integer(
                sign.into_glib(),
                endianness.into_glib(),
                width,
                depth,
            ))
        }
    }

    #[doc(alias = "gst_audio_format_to_string")]
    pub fn to_str<'a>(self) -> &'a glib::GStr {
        if self == Self::Unknown {
            return glib::gstr!("UNKNOWN");
        }
        unsafe {
            glib::GStr::from_ptr(
                ffi::gst_audio_format_to_string(self.into_glib())
                    .as_ref()
                    .expect("gst_audio_format_to_string returned NULL"),
            )
        }
    }

    pub fn iter_raw() -> AudioFormatIterator {
        AudioFormatIterator::default()
    }
}

impl str::FromStr for crate::AudioFormat {
    type Err = glib::BoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        skip_assert_initialized!();

        let fmt = Self::from_string(s);
        if fmt == Self::Unknown {
            Err(glib::bool_error!(
                "Failed to parse audio format from string"
            ))
        } else {
            Ok(fmt)
        }
    }
}

impl PartialOrd for crate::AudioFormat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        crate::AudioFormatInfo::from_format(*self)
            .partial_cmp(&crate::AudioFormatInfo::from_format(*other))
    }
}

impl Ord for crate::AudioFormat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        crate::AudioFormatInfo::from_format(*self).cmp(&crate::AudioFormatInfo::from_format(*other))
    }
}

pub const AUDIO_FORMAT_UNKNOWN: crate::AudioFormat = crate::AudioFormat::Unknown;
pub const AUDIO_FORMAT_ENCODED: crate::AudioFormat = crate::AudioFormat::Encoded;
pub const AUDIO_FORMAT_S8: crate::AudioFormat = crate::AudioFormat::S8;
pub const AUDIO_FORMAT_U8: crate::AudioFormat = crate::AudioFormat::U8;

#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_S16: crate::AudioFormat = crate::AudioFormat::S16be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_U16: crate::AudioFormat = crate::AudioFormat::U16be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_S2432: crate::AudioFormat = crate::AudioFormat::S2432be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_U2432: crate::AudioFormat = crate::AudioFormat::U2432be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_S32: crate::AudioFormat = crate::AudioFormat::S32be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_U32: crate::AudioFormat = crate::AudioFormat::U32be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_S24: crate::AudioFormat = crate::AudioFormat::S24be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_U24: crate::AudioFormat = crate::AudioFormat::U24be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_S20: crate::AudioFormat = crate::AudioFormat::S20be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_U20: crate::AudioFormat = crate::AudioFormat::U20be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_S18: crate::AudioFormat = crate::AudioFormat::S18be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_U18: crate::AudioFormat = crate::AudioFormat::U18be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_F32: crate::AudioFormat = crate::AudioFormat::F32be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_F64: crate::AudioFormat = crate::AudioFormat::F64be;

#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_S16: crate::AudioFormat = crate::AudioFormat::S16le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_U16: crate::AudioFormat = crate::AudioFormat::U16le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_S2432: crate::AudioFormat = crate::AudioFormat::S2432le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_U2432: crate::AudioFormat = crate::AudioFormat::U2432le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_S32: crate::AudioFormat = crate::AudioFormat::S32le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_U32: crate::AudioFormat = crate::AudioFormat::U32le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_S24: crate::AudioFormat = crate::AudioFormat::S24le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_U24: crate::AudioFormat = crate::AudioFormat::U24le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_S20: crate::AudioFormat = crate::AudioFormat::S20le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_U20: crate::AudioFormat = crate::AudioFormat::U20le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_S18: crate::AudioFormat = crate::AudioFormat::S18le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_U18: crate::AudioFormat = crate::AudioFormat::U18le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_F32: crate::AudioFormat = crate::AudioFormat::F32le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_F64: crate::AudioFormat = crate::AudioFormat::F64le;

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
    type Item = crate::AudioFormat;

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
            Some(AUDIO_FORMATS_ALL[end])
        }
    }

    fn last(self) -> Option<Self::Item> {
        if self.idx == self.len {
            None
        } else {
            Some(AUDIO_FORMATS_ALL[self.len - 1])
        }
    }
}

impl ExactSizeIterator for AudioFormatIterator {}

impl std::iter::FusedIterator for AudioFormatIterator {}

impl DoubleEndedIterator for AudioFormatIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx >= self.len {
            None
        } else {
            self.len -= 1;
            let fmt = AUDIO_FORMATS_ALL[self.len];
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
            let fmt = AUDIO_FORMATS_ALL[self.len];
            Some(fmt)
        }
    }
}
pub trait AudioFormatIteratorExt {
    fn into_audio_caps(
        self,
        layout: crate::AudioLayout,
    ) -> Option<crate::AudioCapsBuilder<gst::caps::NoFeature>>;
}

impl<T> AudioFormatIteratorExt for T
where
    T: Iterator<Item = crate::AudioFormat>,
{
    fn into_audio_caps(
        self,
        layout: crate::AudioLayout,
    ) -> Option<crate::AudioCapsBuilder<gst::caps::NoFeature>> {
        let formats: Vec<crate::AudioFormat> = self.collect();
        if !formats.is_empty() {
            Some(crate::functions::audio_make_raw_caps(&formats, layout))
        } else {
            None
        }
    }
}

pub trait AudioFormatIteratorExtRef {
    fn into_audio_caps(
        self,
        layout: crate::AudioLayout,
    ) -> Option<crate::AudioCapsBuilder<gst::caps::NoFeature>>;
}

impl<'a, T> AudioFormatIteratorExtRef for T
where
    T: Iterator<Item = &'a crate::AudioFormat>,
{
    fn into_audio_caps(
        self,
        layout: crate::AudioLayout,
    ) -> Option<crate::AudioCapsBuilder<gst::caps::NoFeature>> {
        let formats: Vec<crate::AudioFormat> = self.copied().collect();
        if !formats.is_empty() {
            Some(crate::functions::audio_make_raw_caps(&formats, layout))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    #[test]
    fn test_display() {
        gst::init().unwrap();

        format!("{}", crate::AudioFormat::S16be);
    }

    #[test]
    fn iter() {
        use super::*;
        gst::init().unwrap();

        assert!(crate::AudioFormat::iter_raw().count() > 0);
        assert_eq!(
            crate::AudioFormat::iter_raw().count(),
            crate::AudioFormat::iter_raw().len()
        );

        let mut i = crate::AudioFormat::iter_raw();
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
        assert_eq!(count, crate::AudioFormat::iter_raw().len());

        assert!(crate::AudioFormat::iter_raw().any(|f| f == crate::AudioFormat::F64be));
        assert!(!crate::AudioFormat::iter_raw().any(|f| f == crate::AudioFormat::Encoded));

        let caps = crate::AudioFormat::iter_raw().into_audio_caps(crate::AudioLayout::Interleaved);
        assert!(caps.is_some());

        let caps = crate::AudioFormat::iter_raw()
            .filter(|f| crate::AudioFormatInfo::from_format(*f).is_little_endian())
            .into_audio_caps(crate::AudioLayout::Interleaved);
        assert!(caps.is_some());

        let caps = crate::AudioFormat::iter_raw()
            .skip(1000)
            .into_audio_caps(crate::AudioLayout::Interleaved);
        assert!(caps.is_none());

        let caps = [crate::AudioFormat::S16le, crate::AudioFormat::S16be]
            .iter()
            .into_audio_caps(crate::AudioLayout::Interleaved)
            .unwrap()
            .build();
        assert_eq!(caps.to_string(), "audio/x-raw, rate=(int)[ 1, 2147483647 ], channels=(int)[ 1, 2147483647 ], layout=(string)interleaved, format=(string){ S16LE, S16BE }");
    }

    #[test]
    fn sort() {
        gst::init().unwrap();

        assert!(
            crate::AudioFormatInfo::from_format(crate::AudioFormat::F64be)
                > crate::AudioFormatInfo::from_format(crate::AudioFormat::U8)
        );
        assert!(crate::AudioFormat::S20be > crate::AudioFormat::S18be);

        let sorted: Vec<crate::AudioFormat> =
            crate::AudioFormat::iter_raw().sorted().rev().collect();
        // FIXME: use is_sorted_by() once API is in stable
        assert_eq!(
            sorted,
            crate::AudioFormat::iter_raw().collect::<Vec<crate::AudioFormat>>()
        );
    }
}
