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

use glib::translate::{from_glib, FromGlib, ToGlib, ToGlibPtr};

#[derive(PartialEq, Eq, Debug)]
pub enum AudioEndianness {
    Unknown,
    LittleEndian = 1234,
    BigEndian = 4321,
}

impl FromGlib<i32> for AudioEndianness {
    fn from_glib(value: i32) -> Self {
        match value {
            1234 => AudioEndianness::LittleEndian,
            4321 => AudioEndianness::BigEndian,
            _ => AudioEndianness::Unknown,
        }
    }
}

impl ToGlib for AudioEndianness {
    type GlibType = i32;

    fn to_glib(&self) -> i32 {
        match *self {
            AudioEndianness::LittleEndian => 1234,
            AudioEndianness::BigEndian => 4321,
            _ => 0,
        }
    }
}

impl ::AudioFormat {
    pub fn build_integer(
        sign: bool,
        endianness: AudioEndianness,
        width: i32,
        depth: i32,
    ) -> ::AudioFormat {
        unsafe {
            from_glib(ffi::gst_audio_format_build_integer(
                sign.to_glib(),
                endianness.to_glib(),
                width,
                depth,
            ))
        }
    }
}

impl str::FromStr for ::AudioFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        unsafe {
            let format = ffi::gst_audio_format_from_string(s.to_glib_none().0);
            if format == ffi::GST_AUDIO_FORMAT_UNKNOWN {
                Err(())
            } else {
                Ok(from_glib(format))
            }
        }
    }
}

#[cfg(target_endian = "big")]
pub const FORMAT_S16: ::AudioFormat = ::AudioFormat::S16be;
#[cfg(target_endian = "big")]
pub const FORMAT_U16: ::AudioFormat = ::AudioFormat::U16be;
#[cfg(target_endian = "big")]
pub const FORMAT_S2432: ::AudioFormat = ::AudioFormat::S2432be;
#[cfg(target_endian = "big")]
pub const FORMAT_U2432: ::AudioFormat = ::AudioFormat::U2432be;
#[cfg(target_endian = "big")]
pub const FORMAT_S32: ::AudioFormat = ::AudioFormat::S32be;
#[cfg(target_endian = "big")]
pub const FORMAT_U32: ::AudioFormat = ::AudioFormat::S32be;
#[cfg(target_endian = "big")]
pub const FORMAT_S24: ::AudioFormat = ::AudioFormat::S24be;
#[cfg(target_endian = "big")]
pub const FORMAT_U24: ::AudioFormat = ::AudioFormat::S24be;
#[cfg(target_endian = "big")]
pub const FORMAT_S20: ::AudioFormat = ::AudioFormat::S20be;
#[cfg(target_endian = "big")]
pub const FORMAT_U20: ::AudioFormat = ::AudioFormat::S20be;
#[cfg(target_endian = "big")]
pub const FORMAT_S18: ::AudioFormat = ::AudioFormat::S18be;
#[cfg(target_endian = "big")]
pub const FORMAT_U18: ::AudioFormat = ::AudioFormat::S18be;
#[cfg(target_endian = "big")]
pub const FORMAT_F32: ::AudioFormat = ::AudioFormat::F32be;
#[cfg(target_endian = "big")]
pub const FORMAT_F64: ::AudioFormat = ::AudioFormat::F64be;

#[cfg(target_endian = "little")]
pub const FORMAT_S16: ::AudioFormat = ::AudioFormat::S16le;
#[cfg(target_endian = "little")]
pub const FORMAT_U16: ::AudioFormat = ::AudioFormat::U16le;
#[cfg(target_endian = "little")]
pub const FORMAT_S2432: ::AudioFormat = ::AudioFormat::S2432le;
#[cfg(target_endian = "little")]
pub const FORMAT_U2432: ::AudioFormat = ::AudioFormat::U2432le;
#[cfg(target_endian = "little")]
pub const FORMAT_S32: ::AudioFormat = ::AudioFormat::S32le;
#[cfg(target_endian = "little")]
pub const FORMAT_U32: ::AudioFormat = ::AudioFormat::S32le;
#[cfg(target_endian = "little")]
pub const FORMAT_S24: ::AudioFormat = ::AudioFormat::S24le;
#[cfg(target_endian = "little")]
pub const FORMAT_U24: ::AudioFormat = ::AudioFormat::S24le;
#[cfg(target_endian = "little")]
pub const FORMAT_S20: ::AudioFormat = ::AudioFormat::S20le;
#[cfg(target_endian = "little")]
pub const FORMAT_U20: ::AudioFormat = ::AudioFormat::S20le;
#[cfg(target_endian = "little")]
pub const FORMAT_S18: ::AudioFormat = ::AudioFormat::S18le;
#[cfg(target_endian = "little")]
pub const FORMAT_U18: ::AudioFormat = ::AudioFormat::S18le;
#[cfg(target_endian = "little")]
pub const FORMAT_F32: ::AudioFormat = ::AudioFormat::F32le;
#[cfg(target_endian = "little")]
pub const FORMAT_F64: ::AudioFormat = ::AudioFormat::F64le;

pub struct AudioFormatInfo(&'static ffi::GstAudioFormatInfo);

impl AudioFormatInfo {
    pub fn from_format(format: ::AudioFormat) -> AudioFormatInfo {
        unsafe {
            let info = ffi::gst_audio_format_get_info(format.to_glib());
            assert!(!info.is_null());

            AudioFormatInfo(&*info)
        }
    }

    pub fn format(&self) -> ::AudioFormat {
        from_glib(self.0.format)
    }

    pub fn name(&self) -> &'static str {
        unsafe { CStr::from_ptr(self.0.name).to_str().unwrap() }
    }

    pub fn description(&self) -> &'static str {
        unsafe { CStr::from_ptr(self.0.description).to_str().unwrap() }
    }

    pub fn flags(&self) -> ::AudioFormatFlags {
        from_glib(self.0.flags)
    }

    pub fn endianness(&self) -> AudioEndianness {
        from_glib(self.0.endianness)
    }

    pub fn width(&self) -> i32 {
        self.0.width
    }

    pub fn depth(&self) -> i32 {
        self.0.depth
    }

    pub fn unpack_format(&self) -> ::AudioFormat {
        from_glib(self.0.unpack_format)
    }

    pub fn silence(&self) -> &'static [u8] {
        &self.0.silence
    }

    pub fn unpack(&self, flags: ::AudioPackFlags, dest: &mut [u8], src: &[u8]) {
        let unpack_format = Self::from_format(self.unpack_format());
        let unpack_width = unpack_format.width() as usize;

        if unpack_width == 0 || self.0.unpack_func.is_none() {
            panic!("No unpack format for {:?}", self);
        }

        let self_width = self.width() as usize;
        if self_width == 0 {
            panic!("No width for {:?}", self);
        }

        if src.len() % (self_width / 8) != 0 {
            panic!("Incomplete number of samples in src");
        }

        let nsamples = src.len() / (self_width / 8);

        if dest.len() != nsamples * (unpack_width / 8) {
            panic!("Invalid dest length");
        }

        unsafe {
            (self.0.unpack_func.as_ref().unwrap())(
                self.0,
                flags.to_glib(),
                dest.as_mut_ptr() as *mut _,
                src.as_ptr() as *mut _,
                nsamples as i32,
            );
        }
    }

    pub fn pack(&self, flags: ::AudioPackFlags, dest: &mut [u8], src: &[u8]) {
        let unpack_format = Self::from_format(self.unpack_format());
        let unpack_width = unpack_format.width() as usize;

        if unpack_width == 0 || self.0.pack_func.is_none() {
            panic!("No unpack format for {:?}", self);
        }

        let self_width = self.width() as usize;
        if self_width == 0 {
            panic!("No width for {:?}", self);
        }

        if src.len() % (unpack_width / 8) != 0 {
            panic!("Incomplete number of samples in src");
        }

        let nsamples = src.len() / (unpack_width / 8);

        if dest.len() != nsamples * (self_width / 8) {
            panic!("Invalid dest length");
        }

        unsafe {
            (self.0.pack_func.as_ref().unwrap())(
                self.0,
                flags.to_glib(),
                src.as_ptr() as *mut _,
                dest.as_mut_ptr() as *mut _,
                nsamples as i32,
            );
        }
    }

    pub fn fill_silence(&self, dest: &mut [u8]) {
        let self_width = self.width() as usize;

        if self_width == 0 {
            panic!("Filling with silence unsupported");
        }

        if dest.len() % (self_width / 8) != 0 {
            panic!("Incomplete number of samples in dest");
        }

        unsafe {
            ffi::gst_audio_format_fill_silence(self.0, dest.as_mut_ptr() as *mut _, dest.len())
        }
    }

    pub fn is_float(&self) -> bool {
        self.flags().contains(::AUDIO_FORMAT_FLAG_FLOAT)
    }

    pub fn is_integer(&self) -> bool {
        self.flags().contains(::AUDIO_FORMAT_FLAG_INTEGER)
    }

    pub fn is_signed(&self) -> bool {
        self.flags().contains(::AUDIO_FORMAT_FLAG_SIGNED)
    }


    pub fn is_little_endian(&self) -> bool {
        self.endianness() == AudioEndianness::LittleEndian
    }

    pub fn is_big_endian(&self) -> bool {
        self.endianness() == AudioEndianness::BigEndian
    }
}

unsafe impl Sync for AudioFormatInfo {}
unsafe impl Send for AudioFormatInfo {}

impl PartialEq for AudioFormatInfo {
    fn eq(&self, other: &Self) -> bool {
        self.format() == other.format()
    }
}

impl Eq for AudioFormatInfo {}

impl fmt::Debug for AudioFormatInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.name())
    }
}

impl From<::AudioFormat> for AudioFormatInfo {
    fn from(f: ::AudioFormat) -> Self {
        Self::from_format(f)
    }
}

impl str::FromStr for ::AudioFormatInfo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        unsafe {
            let format = ffi::gst_audio_format_from_string(s.to_glib_none().0);
            if format == ffi::GST_AUDIO_FORMAT_UNKNOWN {
                Err(())
            } else {
                Ok(AudioFormatInfo::from_format(from_glib(format)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gst;

    #[test]
    fn test_get() {
        gst::init().unwrap();

        let info = AudioFormatInfo::from_format(::AudioFormat::S16le);
        assert_eq!(info.name(), "S16LE");

        let other_info = "S16LE".parse().unwrap();
        assert_eq!(info, other_info);
    }

    #[test]
    fn pack_unpack() {
        gst::init().unwrap();

        let info = AudioFormatInfo::from_format(::AudioFormat::S16le);
        let unpack_info = AudioFormatInfo::from_format(info.unpack_format());

        assert!(unpack_info.width() > 0);

        let input = [0, 0, 255, 255, 128, 128, 64, 64];
        let mut unpacked = [0; 16];
        let mut output = [0; 8];

        info.unpack(::AUDIO_PACK_FLAG_NONE, &mut unpacked, &input);
        info.pack(::AUDIO_PACK_FLAG_NONE, &mut output, &unpacked);

        assert_eq!(input, output);
    }
}
