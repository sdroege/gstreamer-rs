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

use gst;
use gst::miniobject::MiniObject;
use glib;
use glib::translate::{from_glib, from_glib_full, from_glib_none, FromGlibPtrNone, ToGlib,
                      ToGlibPtr, ToGlibPtrMut};

use std::mem;
use std::ptr;

pub struct AudioInfo(ffi::GstAudioInfo);

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
            let mut positions_raw = Vec::new();

            let positions_ptr = match self.positions {
                Some(p) => {
                    if p.len() != self.channels as usize {
                        return None;
                    }

                    positions_raw.reserve(self.channels as usize);
                    for i in p {
                        positions_raw.push(i.to_glib());
                    }

                    let valid: bool = from_glib(ffi::gst_audio_check_valid_channel_positions(
                        positions_raw.as_mut_ptr(),
                        self.channels as i32,
                        true.to_glib(),
                    ));
                    if !valid {
                        return None;
                    }

                    positions_raw.as_ptr()
                }
                None => ptr::null(),
            };


            ffi::gst_audio_info_set_format(
                &mut info,
                self.format.to_glib(),
                self.rate as i32,
                self.channels as i32,
                positions_ptr,
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

            Some(AudioInfo(info))
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
    pub fn new<'a>(format: ::AudioFormat, rate: u32, channels: u32) -> AudioInfoBuilder<'a> {
        AudioInfoBuilder {
            format: format,
            rate: rate,
            channels: channels,
            positions: None,
            flags: None,
            layout: None,
        }
    }

    pub fn from_caps(caps: &gst::Caps) -> Option<Self> {
        unsafe {
            let mut info = mem::uninitialized();
            if from_glib(ffi::gst_audio_info_from_caps(&mut info, caps.as_ptr())) {
                Some(AudioInfo(info))
            } else {
                None
            }
        }
    }

    pub fn to_caps(&self) -> Option<gst::Caps> {
        unsafe {
            let caps = ffi::gst_audio_info_to_caps(&self.0);
            if caps.is_null() {
                None
            } else {
                Some(from_glib_full(caps))
            }
        }
    }

    pub fn convert(
        &self,
        src_fmt: gst::Format,
        src_val: i64,
        dest_fmt: gst::Format,
    ) -> Option<i64> {
        unsafe {
            let mut dest_val = mem::uninitialized();
            if from_glib(ffi::gst_audio_info_convert(
                &self.0,
                src_fmt.to_glib(),
                src_val,
                dest_fmt.to_glib(),
                &mut dest_val,
            )) {
                Some(dest_val)
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

    pub fn rate(&self) -> u32 {
        self.0.rate as u32
    }

    pub fn channels(&self) -> u32 {
        self.0.channels as u32
    }

    pub fn bpf(&self) -> u32 {
        self.0.bpf as u32
    }

    pub fn positions(&self) -> Vec<::AudioChannelPosition> {
        let mut v = Vec::with_capacity(self.0.channels as usize);
        for i in 0..(self.0.channels as usize) {
            v.push(from_glib(self.0.position[i]));
        }

        v
    }
}

impl Clone for AudioInfo {
    fn clone(&self) -> Self {
        unsafe { AudioInfo(ptr::read(&self.0)) }
    }
}

impl PartialEq for AudioInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { from_glib(ffi::gst_audio_info_is_equal(&self.0, &other.0)) }
    }
}

impl Eq for AudioInfo {}

impl glib::types::StaticType for AudioInfo {
    fn static_type() -> glib::types::Type {
        unsafe { glib::translate::from_glib(ffi::gst_audio_info_get_type()) }
    }
}

#[doc(hidden)]
impl<'a> glib::value::FromValueOptional<'a> for AudioInfo {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Option::<AudioInfo>::from_glib_none(
            gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *mut ffi::GstAudioInfo,
        )
    }
}

#[doc(hidden)]
impl glib::value::SetValue for AudioInfo {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstAudioInfo>::to_glib_none(this).0 as
                glib_ffi::gpointer,
        )
    }
}

#[doc(hidden)]
impl glib::value::SetValueOptional for AudioInfo {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstAudioInfo>::to_glib_none(&this).0 as
                glib_ffi::gpointer,
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
        AudioInfo(ptr::read(ptr))
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
            &info.positions(),
            &[
                ::AudioChannelPosition::FrontLeft,
                ::AudioChannelPosition::FrontRight
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
            &info.positions(),
            &[
                ::AudioChannelPosition::RearLeft,
                ::AudioChannelPosition::RearRight
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
            &info.positions(),
            &[
                ::AudioChannelPosition::FrontLeft,
                ::AudioChannelPosition::FrontRight
            ]
        );

        let caps2 = info.to_caps().unwrap();
        assert_eq!(caps, caps2);

        let info2 = AudioInfo::from_caps(&caps2).unwrap();
        assert!(info == info2);
    }
}
