// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::{
    from_glib, from_glib_full, from_glib_none, IntoGlib, ToGlibPtr, ToGlibPtrMut,
};
use gst::prelude::*;

use std::fmt;
use std::mem;
use std::ptr;

#[doc(alias = "GstAudioInfo")]
pub struct AudioInfo(ffi::GstAudioInfo, [crate::AudioChannelPosition; 64]);

impl fmt::Debug for AudioInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AudioInfo")
            .field("format-info", &self.format_info())
            .field("rate", &self.rate())
            .field("channels", &self.channels())
            .field("positions", &self.positions())
            .field("flags", &self.flags())
            .field("layout", &self.layout())
            .finish()
    }
}

#[derive(Debug)]
#[must_use = "The builder must be built to be used"]
pub struct AudioInfoBuilder<'a> {
    format: crate::AudioFormat,
    rate: u32,
    channels: u32,
    positions: Option<&'a [crate::AudioChannelPosition]>,
    flags: Option<crate::AudioFlags>,
    layout: Option<crate::AudioLayout>,
}

impl<'a> AudioInfoBuilder<'a> {
    #[must_use = "The built AudioInfo must be used"]
    pub fn build(self) -> Result<AudioInfo, glib::error::BoolError> {
        unsafe {
            let mut info = mem::MaybeUninit::uninit();

            let positions = if let Some(p) = self.positions {
                if p.len() != self.channels as usize || p.len() > 64 {
                    return Err(glib::bool_error!("Invalid positions length"));
                }

                let positions: [ffi::GstAudioChannelPosition; 64] = std::array::from_fn(|i| {
                    if i >= self.channels as usize {
                        ffi::GST_AUDIO_CHANNEL_POSITION_INVALID
                    } else {
                        p[i].into_glib()
                    }
                });

                let valid: bool = from_glib(ffi::gst_audio_check_valid_channel_positions(
                    positions.as_ptr() as *mut _,
                    self.channels as i32,
                    true.into_glib(),
                ));
                if !valid {
                    return Err(glib::bool_error!("channel positions are invalid"));
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
                info.as_mut_ptr(),
                self.format.into_glib(),
                self.rate as i32,
                self.channels as i32,
                positions_ptr as *mut _,
            );

            let mut info = info.assume_init();

            if info.finfo.is_null() || info.rate <= 0 || info.channels <= 0 {
                return Err(glib::bool_error!("Failed to build AudioInfo"));
            }

            if let Some(flags) = self.flags {
                info.flags = flags.into_glib();
            }

            if let Some(layout) = self.layout {
                info.layout = layout.into_glib();
            }

            let positions = std::array::from_fn(|i| from_glib(info.position[i]));
            Ok(AudioInfo(info, positions))
        }
    }

    pub fn positions(self, positions: &'a [crate::AudioChannelPosition]) -> AudioInfoBuilder<'a> {
        Self {
            positions: Some(positions),
            ..self
        }
    }

    pub fn flags(self, flags: crate::AudioFlags) -> Self {
        Self {
            flags: Some(flags),
            ..self
        }
    }

    pub fn layout(self, layout: crate::AudioLayout) -> Self {
        Self {
            layout: Some(layout),
            ..self
        }
    }
}

impl AudioInfo {
    pub fn builder<'a>(
        format: crate::AudioFormat,
        rate: u32,
        channels: u32,
    ) -> AudioInfoBuilder<'a> {
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

    pub fn is_valid(&self) -> bool {
        !self.0.finfo.is_null() && self.0.channels > 0 && self.0.rate > 0 && self.0.bpf > 0
    }

    #[doc(alias = "gst_audio_info_from_caps")]
    pub fn from_caps(caps: &gst::CapsRef) -> Result<Self, glib::error::BoolError> {
        skip_assert_initialized!();

        unsafe {
            let mut info = mem::MaybeUninit::uninit();
            if from_glib(ffi::gst_audio_info_from_caps(
                info.as_mut_ptr(),
                caps.as_ptr(),
            )) {
                let info = info.assume_init();
                let positions = std::array::from_fn(|i| from_glib(info.position[i]));
                Ok(Self(info, positions))
            } else {
                Err(glib::bool_error!("Failed to create AudioInfo from caps"))
            }
        }
    }

    #[doc(alias = "gst_audio_info_to_caps")]
    pub fn to_caps(&self) -> Result<gst::Caps, glib::error::BoolError> {
        unsafe {
            let result = from_glib_full(ffi::gst_audio_info_to_caps(&self.0));
            match result {
                Some(c) => Ok(c),
                None => Err(glib::bool_error!("Failed to create caps from AudioInfo")),
            }
        }
    }

    #[doc(alias = "gst_audio_info_convert")]
    pub fn convert<U: gst::format::SpecificFormattedValueFullRange>(
        &self,
        src_val: impl gst::format::FormattedValue,
    ) -> Option<U> {
        assert_initialized_main_thread!();
        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            if from_glib(ffi::gst_audio_info_convert(
                &self.0,
                src_val.format().into_glib(),
                src_val.into_raw_value(),
                U::default_format().into_glib(),
                dest_val.as_mut_ptr(),
            )) {
                Some(U::from_raw(U::default_format(), dest_val.assume_init()))
            } else {
                None
            }
        }
    }

    pub fn convert_generic(
        &self,
        src_val: impl gst::format::FormattedValue,
        dest_fmt: gst::Format,
    ) -> Option<gst::GenericFormattedValue> {
        assert_initialized_main_thread!();
        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            if from_glib(ffi::gst_audio_info_convert(
                &self.0,
                src_val.format().into_glib(),
                src_val.into_raw_value(),
                dest_fmt.into_glib(),
                dest_val.as_mut_ptr(),
            )) {
                Some(gst::GenericFormattedValue::new(
                    dest_fmt,
                    dest_val.assume_init(),
                ))
            } else {
                None
            }
        }
    }

    pub fn format(&self) -> crate::AudioFormat {
        if self.0.finfo.is_null() {
            return crate::AudioFormat::Unknown;
        }

        unsafe { from_glib((*self.0.finfo).format) }
    }

    pub fn format_info(&self) -> crate::AudioFormatInfo {
        crate::AudioFormatInfo::from_format(self.format())
    }

    pub fn layout(&self) -> crate::AudioLayout {
        unsafe { from_glib(self.0.layout) }
    }

    pub fn flags(&self) -> crate::AudioFlags {
        unsafe { from_glib(self.0.flags) }
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

    pub fn endianness(&self) -> crate::AudioEndianness {
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

    pub fn positions(&self) -> Option<&[crate::AudioChannelPosition]> {
        if self.0.channels > 64 || self.is_unpositioned() {
            return None;
        }

        Some(&self.1[0..(self.0.channels as usize)])
    }

    pub fn is_unpositioned(&self) -> bool {
        self.flags().contains(crate::AudioFlags::UNPOSITIONED)
    }
}

impl Clone for AudioInfo {
    fn clone(&self) -> Self {
        unsafe { Self(ptr::read(&self.0), self.1) }
    }
}

impl PartialEq for AudioInfo {
    #[doc(alias = "gst_audio_info_is_equal")]
    fn eq(&self, other: &Self) -> bool {
        unsafe { from_glib(ffi::gst_audio_info_is_equal(&self.0, &other.0)) }
    }
}

impl Eq for AudioInfo {}

unsafe impl Send for AudioInfo {}
unsafe impl Sync for AudioInfo {}

impl glib::types::StaticType for AudioInfo {
    fn static_type() -> glib::types::Type {
        unsafe { glib::translate::from_glib(ffi::gst_audio_info_get_type()) }
    }
}

impl glib::value::ValueType for AudioInfo {
    type Type = Self;
}

#[doc(hidden)]
unsafe impl<'a> glib::value::FromValue<'a> for AudioInfo {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib_none(
            glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *mut ffi::GstAudioInfo
        )
    }
}

#[doc(hidden)]
impl glib::value::ToValue for AudioInfo {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                self.to_glib_none().0 as *mut _,
            )
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[doc(hidden)]
impl glib::value::ToValueOptional for AudioInfo {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                s.to_glib_none().0 as *mut _,
            )
        }
        value
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
    type Storage = &'a Self;

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
        Self(
            ptr::read(ptr),
            std::array::from_fn(|i| from_glib((*ptr).position[i])),
        )
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrFull<*mut ffi::GstAudioInfo> for AudioInfo {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GstAudioInfo) -> Self {
        let info = from_glib_none(ptr);
        glib::ffi::g_free(ptr as *mut _);
        info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        gst::init().unwrap();

        let info = AudioInfo::builder(crate::AudioFormat::S16le, 48000, 2)
            .build()
            .unwrap();
        assert_eq!(info.format(), crate::AudioFormat::S16le);
        assert_eq!(info.rate(), 48000);
        assert_eq!(info.channels(), 2);
        assert_eq!(
            &info.positions().unwrap(),
            &[
                crate::AudioChannelPosition::FrontLeft,
                crate::AudioChannelPosition::FrontRight,
            ]
        );

        let positions = [
            crate::AudioChannelPosition::RearLeft,
            crate::AudioChannelPosition::RearRight,
        ];
        let info = AudioInfo::builder(crate::AudioFormat::S16le, 48000, 2)
            .positions(&positions)
            .build()
            .unwrap();
        assert_eq!(info.format(), crate::AudioFormat::S16le);
        assert_eq!(info.rate(), 48000);
        assert_eq!(info.channels(), 2);
        assert_eq!(
            &info.positions().unwrap(),
            &[
                crate::AudioChannelPosition::RearLeft,
                crate::AudioChannelPosition::RearRight,
            ]
        );
    }

    #[test]
    fn test_from_to_caps() {
        gst::init().unwrap();

        let caps = crate::AudioCapsBuilder::new_interleaved()
            .format(crate::AudioFormat::S16le)
            .rate(48000)
            .channels(2)
            .field("channel-mask", gst::Bitmask::new(0x3))
            .build();
        let info = AudioInfo::from_caps(&caps).unwrap();
        assert_eq!(info.format(), crate::AudioFormat::S16le);
        assert_eq!(info.rate(), 48000);
        assert_eq!(info.channels(), 2);
        assert_eq!(
            &info.positions().unwrap(),
            &[
                crate::AudioChannelPosition::FrontLeft,
                crate::AudioChannelPosition::FrontRight,
            ]
        );

        let caps2 = info.to_caps().unwrap();
        assert_eq!(caps, caps2);

        let info2 = AudioInfo::from_caps(&caps2).unwrap();
        assert!(info == info2);
    }
}
