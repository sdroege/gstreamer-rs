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
    pub fn from_string(s: &str) -> ::VideoFormat {
        assert_initialized_main_thread!();

        unsafe { from_glib(ffi::gst_video_format_from_string(s.to_glib_none().0)) }
    }

    pub fn from_fourcc(fourcc: u32) -> ::VideoFormat {
        assert_initialized_main_thread!();

        unsafe { from_glib(ffi::gst_video_format_from_fourcc(fourcc)) }
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

    pub fn to_string(&self) -> &'static str {
        unsafe {
            CStr::from_ptr(ffi::gst_video_format_to_string(self.to_glib()))
                .to_str()
                .unwrap()
        }
    }
}

impl str::FromStr for ::VideoFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        skip_assert_initialized!();

        let format = Self::from_string(s);
        if format == ::VideoFormat::Unknown {
            Err(())
        } else {
            Ok(format)
        }
    }
}

impl fmt::Display for ::VideoFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.to_string())
    }
}
