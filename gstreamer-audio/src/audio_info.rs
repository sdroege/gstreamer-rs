// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib_ffi;
use gobject_ffi;

use glib;
use glib::translate::{
    from_glib, from_glib_full, from_glib_none, FromGlibPtrNone, ToGlib, ToGlibPtr, ToGlibPtrMut,
};
use gst;
use gst::prelude::*;

use std::fmt;
use std::mem;
use std::ptr;

use array_init;

pub struct AudioInfo(ffi::GstAudioInfo, [::AudioChannelPosition; 64]);

impl fmt::Debug for AudioInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("AudioInfo")
            .field("rate", &self.rate())
            .field("channels", &self.channels())
            .field("positions", &self.positions())
            .field("flags", &self.flags())
            .field("layout", &self.layout())
            .finish()
    }
}

pub struct AudioInfoBuilder<'a> {
    format: ::AudioFormat,
    rate: u32,
    channels: u32,
    positions: Option<&'a [::AudioChannelPosition]>,
    flags: Option<::AudioFlags>,
    layout: Option<::AudioLayout>,
}

impl<'a> AudioInfoBuilder<'a> {
    pub fn build(self) -> Option<AudioInfo> {
        unsafe {
            let mut info = mem::uninitialized();

            let positions = if let Some(p) = self.positions {
                if p.len() != self.channels as usize || p.len() > 64 {
                    return None;
                }

                let positions: [ffi::GstAudioChannelPosition; 64] =
                    array_init::array_init_copy(|i| {
                        if i >= self.channels as usize {
                            ffi::GST_AUDIO_CHANNEL_POSITION_INVALID
                        } else {
                            p[i].to_glib()
                        }
                    });

                let valid: bool = from_glib(ffi::gst_audio_check_valid_channel_positions(
                    positions.as_ptr() as *mut _,
                    self.channels as i32,
                    true.to_glib(),
                ));
                if !valid {
                    return None;
                }

                Some(positions)
            } else {
                None
            };

            let positions_ptr = positions
                .as_ref()
                .map(|p| p.as_ptr())
                .unwrap_or(ptr::null());

            ffi::gst_audio_info_set_format(
                &mut info,
                self.format.to_glib(),
                self.rate as i32,
                self.channels as i32,
                positions_ptr as *mut _,
            );

            if info.finfo.is_null() || info.rate <= 0 || info.channels <= 0 {
                return None;
            }

            if let Some(flags) = self.flags {
                info.flags = flags.to_glib();
            }

            if let Some(layout) = self.layout {
                info.layout = layout.to_glib();
            }

            let positions = array_init::array_init_copy(|i| from_glib(info.position[i]));
            Some(AudioInfo(info, positions))
        }
    }

    pub fn positions(self, positions: &'a [::AudioChannelPosition]) -> AudioInfoBuilder<'a> {
        Self {
            positions: Some(positions),
            ..self
        }
    }

    pub fn flags(self, flags: ::AudioFlags) -> Self {
        Self {
            flags: Some(flags),
            ..self
        }
    }

    pub fn layout(self, layout: ::AudioLayout) -> Self {
        Self {
            layout: Some(layout),
            ..self
        }
    }
}

impl AudioInfo {
    #[cfg_attr(feature = "cargo-clippy", allow(new_ret_no_self))]
    pub fn new<'a>(format: ::AudioFormat, rate: u32, channels: u32) -> AudioInfoBuilder<'a> {
        assert_initialized_main_thread!();

        AudioInfoBuilder {
            format,
            rate,
            channels,
            positions: None,
            flags: None,
            layout: None,
        }
    }

    pub fn from_caps(caps: &gst::CapsRef) -> Option<Self> {
        skip_assert_initialized!();

        unsafe {
            let mut info = mem::uninitialized();
            if from_glib(ffi::gst_audio_info_from_caps(&mut info, caps.as_ptr())) {
                let positions = array_init::array_init_copy(|i| from_glib(info.position[i]));
                Some(AudioInfo(info, positions))
            } else {
                None
            }
        }
    }

    pub fn to_caps(&self) -> Option<gst::Caps> {
        unsafe { from_glib_full(ffi::gst_audio_info_to_caps(&self.0)) }
    }

    pub fn convert<V: Into<gst::GenericFormattedValue>, U: gst::SpecificFormattedValue>(
        &self,
        src_val: V,
    ) -> Option<U> {
        assert_initialized_main_thread!();

        let src_val = src_val.into();
        unsafe {
            let mut dest_val = mem::uninitialized();
            if from_glib(ffi::gst_audio_info_convert(
                &self.0,
                src_val.get_format().to_glib(),
                src_val.to_raw_value(),
                U::get_default_format().to_glib(),
                &mut dest_val,
            )) {
                Some(U::from_raw(U::get_default_format(), dest_val))
            } else {
                None
            }
        }
    }

    pub fn convert_generic<V: Into<gst::GenericFormattedValue>>(
        &self,
        src_val: V,
        dest_fmt: gst::Format,
    ) -> Option<gst::GenericFormattedValue> {
        assert_initialized_main_thread!();

        let src_val = src_val.into();
        unsafe {
            let mut dest_val = mem::uninitialized();
            if from_glib(ffi::gst_audio_info_convert(
                &self.0,
                src_val.get_format().to_glib(),
                src_val.to_raw_value(),
                dest_fmt.to_glib(),
                &mut dest_val,
            )) {
                Some(gst::GenericFormattedValue::new(dest_fmt, dest_val))
            } else {
                None
            }
        }
    }

    pub fn format(&self) -> ::AudioFormat {
        unsafe { from_glib((*self.0.finfo).format) }
    }

    pub fn format_info(&self) -> ::AudioFormatInfo {
        ::AudioFormatInfo::from_format(self.format())
    }

    pub fn layout(&self) -> ::AudioLayout {
        from_glib(self.0.layout)
    }

    pub fn flags(&self) -> ::AudioFlags {
        from_glib(self.0.flags)
    }

    pub fn rate(&self) -> u32 {
        self.0.rate as u32
    }

    pub fn channels(&self) -> u32 {
        self.0.channels as u32
    }

    pub fn bpf(&self) -> u32 {
        self.0.bpf as u32
    }

    pub fn bps(&self) -> u32 {
        (self.format_info().depth() as u32) >> 3
    }

    pub fn depth(&self) -> u32 {
        self.format_info().depth()
    }

    pub fn width(&self) -> u32 {
        self.format_info().width()
    }

    pub fn endianness(&self) -> ::AudioEndianness {
        self.format_info().endianness()
    }

    pub fn is_big_endian(&self) -> bool {
        self.format_info().is_big_endian()
    }

    pub fn is_little_endian(&self) -> bool {
        self.format_info().is_little_endian()
    }

    pub fn is_float(&self) -> bool {
        self.format_info().is_float()
    }

    pub fn is_integer(&self) -> bool {
        self.format_info().is_integer()
    }

    pub fn is_signed(&self) -> bool {
        self.format_info().is_signed()
    }

    pub fn positions(&self) -> Option<&[::AudioChannelPosition]> {
        if self.0.channels > 64 || self.is_unpositioned() {
            return None;
        }

        Some(&self.1[0..(self.0.channels as usize)])
    }

    pub fn is_unpositioned(&self) -> bool {
        self.flags().contains(::AudioFlags::UNPOSITIONED)
    }
}

impl Clone for AudioInfo {
    fn clone(&self) -> Self {
        unsafe { AudioInfo(ptr::read(&self.0), self.1) }
    }
}

impl PartialEq for AudioInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { from_glib(ffi::gst_audio_info_is_equal(&self.0, &other.0)) }
    }
}

impl Eq for AudioInfo {}

unsafe impl Send for AudioInfo {}

impl glib::types::StaticType for AudioInfo {
    fn static_type() -> glib::types::Type {
        unsafe { glib::translate::from_glib(ffi::gst_audio_info_get_type()) }
    }
}

#[doc(hidden)]
impl<'a> glib::value::FromValueOptional<'a> for AudioInfo {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Option::<AudioInfo>::from_glib_none(
            gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *mut ffi::GstAudioInfo
        )
    }
}

#[doc(hidden)]
impl glib::value::SetValue for AudioInfo {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstAudioInfo>::to_glib_none(this).0
                as glib_ffi::gpointer,
        )
    }
}

#[doc(hidden)]
impl glib::value::SetValueOptional for AudioInfo {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstAudioInfo>::to_glib_none(&this).0
                as glib_ffi::gpointer,
        )
    }
}

#[doc(hidden)]
impl glib::translate::Uninitialized for AudioInfo {
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl glib::translate::GlibPtrDefault for AudioInfo {
    type GlibType = *mut ffi::GstAudioInfo;
}

#[doc(hidden)]
impl<'a> glib::translate::ToGlibPtr<'a, *const ffi::GstAudioInfo> for AudioInfo {
    type Storage = &'a AudioInfo;

    fn to_glib_none(&'a self) -> glib::translate::Stash<'a, *const ffi::GstAudioInfo, Self> {
        glib::translate::Stash(&self.0, self)
    }

    fn to_glib_full(&self) -> *const ffi::GstAudioInfo {
        unimplemented!()
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*mut ffi::GstAudioInfo> for AudioInfo {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstAudioInfo) -> Self {
        AudioInfo(
            ptr::read(ptr),
            array_init::array_init_copy(|i| from_glib((*ptr).position[i])),
        )
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrFull<*mut ffi::GstAudioInfo> for AudioInfo {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GstAudioInfo) -> Self {
        let info = from_glib_none(ptr);
        glib_ffi::g_free(ptr as *mut _);
        info
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gst;

    #[test]
    fn test_new() {
        gst::init().unwrap();

        let info = AudioInfo::new(::AudioFormat::S16le, 48000, 2)
            .build()
            .unwrap();
        assert_eq!(info.format(), ::AudioFormat::S16le);
        assert_eq!(info.rate(), 48000);
        assert_eq!(info.channels(), 2);
        assert_eq!(
            &info.positions().unwrap(),
            &[
                ::AudioChannelPosition::FrontLeft,
                ::AudioChannelPosition::FrontRight,
            ]
        );

        let positions = [
            ::AudioChannelPosition::RearLeft,
            ::AudioChannelPosition::RearRight,
        ];
        let info = AudioInfo::new(::AudioFormat::S16le, 48000, 2)
            .positions(&positions)
            .build()
            .unwrap();
        assert_eq!(info.format(), ::AudioFormat::S16le);
        assert_eq!(info.rate(), 48000);
        assert_eq!(info.channels(), 2);
        assert_eq!(
            &info.positions().unwrap(),
            &[
                ::AudioChannelPosition::RearLeft,
                ::AudioChannelPosition::RearRight,
            ]
        );
    }

    #[test]
    fn test_from_to_caps() {
        gst::init().unwrap();

        let caps = gst::Caps::new_simple(
            "audio/x-raw",
            &[
                ("format", &"S16LE"),
                ("rate", &48000),
                ("channels", &2),
                ("layout", &"interleaved"),
                ("channel-mask", &gst::Bitmask::new(0x3)),
            ],
        );
        let info = AudioInfo::from_caps(&caps).unwrap();
        assert_eq!(info.format(), ::AudioFormat::S16le);
        assert_eq!(info.rate(), 48000);
        assert_eq!(info.channels(), 2);
        assert_eq!(
            &info.positions().unwrap(),
            &[
                ::AudioChannelPosition::FrontLeft,
                ::AudioChannelPosition::FrontRight,
            ]
        );

        let caps2 = info.to_caps().unwrap();
        assert_eq!(caps, caps2);

        let info2 = AudioInfo::from_caps(&caps2).unwrap();
        assert!(info == info2);
    }
}
