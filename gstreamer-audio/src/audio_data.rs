// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::slice;
use std::fmt;
use libc::uintptr_t;

use std::error::Error as StdError;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    WrongAlignment,
    WrongEndianness,
    IncompleteSamples,
    UnsupportedFormat,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        use self::Error::*;

        match *self {
            WrongAlignment => "Wrong Alignment",
            WrongEndianness => "Wrong Endianness",
            IncompleteSamples => "Incomplete Samples",
            UnsupportedFormat => "Unsupported Format",
        }
    }
}

pub enum AudioData<'a> {
    S8(&'a [i8]),
    U8(&'a [u8]),
    S16(&'a [i16]),
    U16(&'a [u16]),
    S32(&'a [i32]),
    U32(&'a [u32]),
    F32(&'a [f32]),
    F64(&'a [f64]),
}

impl<'a> AudioData<'a> {
    pub fn new(data: &'a [u8], format: ::AudioFormat) -> Result<AudioData<'a>, Error> {
        use AudioFormat::*;

        let alignment = if (data.as_ptr() as uintptr_t) % 8 == 0 {
            8
        } else if (data.as_ptr() as uintptr_t) % 4 == 0 {
            4
        } else if (data.as_ptr() as uintptr_t) % 2 == 0 {
            2
        } else {
            1
        };

        let format_info = ::AudioFormatInfo::from_format(format);
        let width = (format_info.width() / 8) as usize;

        if width != 1 && cfg!(target_endian = "big") && format_info.is_little_endian() {
            return Err(Error::WrongEndianness);
        } else if width != 1 && cfg!(target_endian = "little") && format_info.is_big_endian() {
            return Err(Error::WrongEndianness);
        }

        if alignment < width {
            return Err(Error::WrongAlignment);
        }

        if data.len() % width != 0 {
            return Err(Error::IncompleteSamples);
        }

        match format {
            S8 => Ok(AudioData::S8(unsafe {
                slice::from_raw_parts(data.as_ptr() as *const _, data.len())
            })),
            U8 => Ok(AudioData::U8(unsafe {
                slice::from_raw_parts(data.as_ptr() as *const _, data.len())
            })),
            S16le | S16be => Ok(AudioData::S16(unsafe {
                slice::from_raw_parts(data.as_ptr() as *const _, data.len() / 2)
            })),
            U16le | U16be => Ok(AudioData::U16(unsafe {
                slice::from_raw_parts(data.as_ptr() as *const _, data.len() / 2)
            })),
            S32le | S2432le | S32be | S2432be => Ok(AudioData::S32(unsafe {
                slice::from_raw_parts(data.as_ptr() as *const _, data.len() / 4)
            })),
            U32le | U2432le | U32be | U2432be => Ok(AudioData::U32(unsafe {
                slice::from_raw_parts(data.as_ptr() as *const _, data.len() / 4)
            })),
            F32le | F32be => Ok(AudioData::F32(unsafe {
                slice::from_raw_parts(data.as_ptr() as *const _, data.len() / 4)
            })),
            F64le | F64be => Ok(AudioData::F64(unsafe {
                slice::from_raw_parts(data.as_ptr() as *const _, data.len() / 8)
            })),
            _ => Err(Error::UnsupportedFormat),
        }
    }
}

pub enum AudioDataMut<'a> {
    S8(&'a mut [i8]),
    U8(&'a mut [u8]),
    S16(&'a mut [i16]),
    U16(&'a mut [u16]),
    S32(&'a mut [i32]),
    U32(&'a mut [u32]),
    F32(&'a mut [f32]),
    F64(&'a mut [f64]),
}

impl<'a> AudioDataMut<'a> {
    pub fn new(data: &'a mut [u8], format: ::AudioFormat) -> Result<AudioDataMut<'a>, Error> {
        use AudioFormat::*;

        let alignment = if (data.as_ptr() as uintptr_t) % 8 == 0 {
            8
        } else if (data.as_ptr() as uintptr_t) % 4 == 0 {
            4
        } else if (data.as_ptr() as uintptr_t) % 2 == 0 {
            2
        } else {
            1
        };

        let format_info = ::AudioFormatInfo::from_format(format);
        let width = (format_info.width() / 8) as usize;

        if width != 1 && cfg!(target_endian = "big") && format_info.is_little_endian() {
            return Err(Error::WrongEndianness);
        } else if width != 1 && cfg!(target_endian = "little") && format_info.is_big_endian() {
            return Err(Error::WrongEndianness);
        }

        if alignment < width {
            return Err(Error::WrongAlignment);
        }

        if data.len() % width != 0 {
            return Err(Error::IncompleteSamples);
        }

        match format {
            S8 => Ok(AudioDataMut::S8(unsafe {
                slice::from_raw_parts_mut(data.as_ptr() as *mut _, data.len())
            })),
            U8 => Ok(AudioDataMut::U8(unsafe {
                slice::from_raw_parts_mut(data.as_ptr() as *mut _, data.len())
            })),
            S16le | S16be => Ok(AudioDataMut::S16(unsafe {
                slice::from_raw_parts_mut(data.as_ptr() as *mut _, data.len() / 2)
            })),
            U16le | U16be => Ok(AudioDataMut::U16(unsafe {
                slice::from_raw_parts_mut(data.as_ptr() as *mut _, data.len() / 2)
            })),
            S32le | S2432le | S32be | S2432be => Ok(AudioDataMut::S32(unsafe {
                slice::from_raw_parts_mut(data.as_ptr() as *mut _, data.len() / 4)
            })),
            U32le | U2432le | U32be | U2432be => Ok(AudioDataMut::U32(unsafe {
                slice::from_raw_parts_mut(data.as_ptr() as *mut _, data.len() / 4)
            })),
            F32le | F32be => Ok(AudioDataMut::F32(unsafe {
                slice::from_raw_parts_mut(data.as_ptr() as *mut _, data.len() / 4)
            })),
            F64le | F64be => Ok(AudioDataMut::F64(unsafe {
                slice::from_raw_parts_mut(data.as_ptr() as *mut _, data.len() / 8)
            })),
            _ => Err(Error::UnsupportedFormat),
        }
    }
}
