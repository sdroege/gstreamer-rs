// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ffi;
use glib::{bitflags::bitflags, prelude::*, translate::*};

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstAudioFlags")]
    pub struct AudioFlags: u32 {
        #[doc(alias = "GST_AUDIO_FLAG_UNPOSITIONED")]
        const UNPOSITIONED = ffi::GST_AUDIO_FLAG_UNPOSITIONED as _;
    }
}

#[doc(hidden)]
impl IntoGlib for AudioFlags {
    type GlibType = ffi::GstAudioFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstAudioFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioFlags> for AudioFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstAudioFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for AudioFlags {
    #[inline]
    #[doc(alias = "gst_audio_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_audio_flags_get_type()) }
    }
}

impl glib::HasParamSpec for AudioFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for AudioFlags {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for AudioFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for AudioFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<AudioFlags> for glib::Value {
    #[inline]
    fn from(v: AudioFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstAudioFormatFlags")]
    pub struct AudioFormatFlags: u32 {
        #[doc(alias = "GST_AUDIO_FORMAT_FLAG_INTEGER")]
        const INTEGER = ffi::GST_AUDIO_FORMAT_FLAG_INTEGER as _;
        #[doc(alias = "GST_AUDIO_FORMAT_FLAG_FLOAT")]
        const FLOAT = ffi::GST_AUDIO_FORMAT_FLAG_FLOAT as _;
        #[doc(alias = "GST_AUDIO_FORMAT_FLAG_SIGNED")]
        const SIGNED = ffi::GST_AUDIO_FORMAT_FLAG_SIGNED as _;
        #[doc(alias = "GST_AUDIO_FORMAT_FLAG_COMPLEX")]
        const COMPLEX = ffi::GST_AUDIO_FORMAT_FLAG_COMPLEX as _;
        #[doc(alias = "GST_AUDIO_FORMAT_FLAG_UNPACK")]
        const UNPACK = ffi::GST_AUDIO_FORMAT_FLAG_UNPACK as _;
    }
}

#[doc(hidden)]
impl IntoGlib for AudioFormatFlags {
    type GlibType = ffi::GstAudioFormatFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstAudioFormatFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioFormatFlags> for AudioFormatFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstAudioFormatFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for AudioFormatFlags {
    #[inline]
    #[doc(alias = "gst_audio_format_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_audio_format_flags_get_type()) }
    }
}

impl glib::HasParamSpec for AudioFormatFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for AudioFormatFlags {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for AudioFormatFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for AudioFormatFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<AudioFormatFlags> for glib::Value {
    #[inline]
    fn from(v: AudioFormatFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstAudioPackFlags")]
    pub struct AudioPackFlags: u32 {
        #[doc(alias = "GST_AUDIO_PACK_FLAG_TRUNCATE_RANGE")]
        const TRUNCATE_RANGE = ffi::GST_AUDIO_PACK_FLAG_TRUNCATE_RANGE as _;
    }
}

#[doc(hidden)]
impl IntoGlib for AudioPackFlags {
    type GlibType = ffi::GstAudioPackFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstAudioPackFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioPackFlags> for AudioPackFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstAudioPackFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for AudioPackFlags {
    #[inline]
    #[doc(alias = "gst_audio_pack_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_audio_pack_flags_get_type()) }
    }
}

impl glib::HasParamSpec for AudioPackFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for AudioPackFlags {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for AudioPackFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for AudioPackFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<AudioPackFlags> for glib::Value {
    #[inline]
    fn from(v: AudioPackFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
