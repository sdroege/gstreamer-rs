// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ffi;
use glib::{bitflags::bitflags, prelude::*, translate::*};

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstRTPBufferFlags")]
    pub struct RTPBufferFlags: u32 {
        #[doc(alias = "GST_RTP_BUFFER_FLAG_RETRANSMISSION")]
        const RETRANSMISSION = ffi::GST_RTP_BUFFER_FLAG_RETRANSMISSION as _;
        #[doc(alias = "GST_RTP_BUFFER_FLAG_REDUNDANT")]
        const REDUNDANT = ffi::GST_RTP_BUFFER_FLAG_REDUNDANT as _;
    }
}

#[doc(hidden)]
impl IntoGlib for RTPBufferFlags {
    type GlibType = ffi::GstRTPBufferFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstRTPBufferFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTPBufferFlags> for RTPBufferFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstRTPBufferFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for RTPBufferFlags {
    #[inline]
    #[doc(alias = "gst_rtp_buffer_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_rtp_buffer_flags_get_type()) }
    }
}

impl glib::HasParamSpec for RTPBufferFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for RTPBufferFlags {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for RTPBufferFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for RTPBufferFlags {
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

impl From<RTPBufferFlags> for glib::Value {
    #[inline]
    fn from(v: RTPBufferFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstRTPBufferMapFlags")]
    pub struct RTPBufferMapFlags: u32 {
        #[doc(alias = "GST_RTP_BUFFER_MAP_FLAG_SKIP_PADDING")]
        const SKIP_PADDING = ffi::GST_RTP_BUFFER_MAP_FLAG_SKIP_PADDING as _;
    }
}

#[doc(hidden)]
impl IntoGlib for RTPBufferMapFlags {
    type GlibType = ffi::GstRTPBufferMapFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstRTPBufferMapFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTPBufferMapFlags> for RTPBufferMapFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstRTPBufferMapFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for RTPBufferMapFlags {
    #[inline]
    #[doc(alias = "gst_rtp_buffer_map_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_rtp_buffer_map_flags_get_type()) }
    }
}

impl glib::HasParamSpec for RTPBufferMapFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for RTPBufferMapFlags {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for RTPBufferMapFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for RTPBufferMapFlags {
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

impl From<RTPBufferMapFlags> for glib::Value {
    #[inline]
    fn from(v: RTPBufferMapFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[cfg(feature = "v1_20")]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstRTPHeaderExtensionDirection")]
    pub struct RTPHeaderExtensionDirection: u32 {
        #[doc(alias = "GST_RTP_HEADER_EXTENSION_DIRECTION_INACTIVE")]
        const INACTIVE = ffi::GST_RTP_HEADER_EXTENSION_DIRECTION_INACTIVE as _;
        #[doc(alias = "GST_RTP_HEADER_EXTENSION_DIRECTION_SENDONLY")]
        const SENDONLY = ffi::GST_RTP_HEADER_EXTENSION_DIRECTION_SENDONLY as _;
        #[doc(alias = "GST_RTP_HEADER_EXTENSION_DIRECTION_RECVONLY")]
        const RECVONLY = ffi::GST_RTP_HEADER_EXTENSION_DIRECTION_RECVONLY as _;
        #[doc(alias = "GST_RTP_HEADER_EXTENSION_DIRECTION_SENDRECV")]
        const SENDRECV = ffi::GST_RTP_HEADER_EXTENSION_DIRECTION_SENDRECV as _;
        #[doc(alias = "GST_RTP_HEADER_EXTENSION_DIRECTION_INHERITED")]
        const INHERITED = ffi::GST_RTP_HEADER_EXTENSION_DIRECTION_INHERITED as _;
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl IntoGlib for RTPHeaderExtensionDirection {
    type GlibType = ffi::GstRTPHeaderExtensionDirection;

    #[inline]
    fn into_glib(self) -> ffi::GstRTPHeaderExtensionDirection {
        self.bits()
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl FromGlib<ffi::GstRTPHeaderExtensionDirection> for RTPHeaderExtensionDirection {
    #[inline]
    unsafe fn from_glib(value: ffi::GstRTPHeaderExtensionDirection) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl StaticType for RTPHeaderExtensionDirection {
    #[inline]
    #[doc(alias = "gst_rtp_header_extension_direction_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_rtp_header_extension_direction_get_type()) }
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl glib::HasParamSpec for RTPHeaderExtensionDirection {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl glib::value::ValueType for RTPHeaderExtensionDirection {
    type Type = Self;
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
unsafe impl<'a> glib::value::FromValue<'a> for RTPHeaderExtensionDirection {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl ToValue for RTPHeaderExtensionDirection {
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

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl From<RTPHeaderExtensionDirection> for glib::Value {
    #[inline]
    fn from(v: RTPHeaderExtensionDirection) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[cfg(feature = "v1_20")]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstRTPHeaderExtensionFlags")]
    pub struct RTPHeaderExtensionFlags: u32 {
        #[doc(alias = "GST_RTP_HEADER_EXTENSION_ONE_BYTE")]
        const ONE_BYTE = ffi::GST_RTP_HEADER_EXTENSION_ONE_BYTE as _;
        #[doc(alias = "GST_RTP_HEADER_EXTENSION_TWO_BYTE")]
        const TWO_BYTE = ffi::GST_RTP_HEADER_EXTENSION_TWO_BYTE as _;
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl IntoGlib for RTPHeaderExtensionFlags {
    type GlibType = ffi::GstRTPHeaderExtensionFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstRTPHeaderExtensionFlags {
        self.bits()
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl FromGlib<ffi::GstRTPHeaderExtensionFlags> for RTPHeaderExtensionFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstRTPHeaderExtensionFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl StaticType for RTPHeaderExtensionFlags {
    #[inline]
    #[doc(alias = "gst_rtp_header_extension_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_rtp_header_extension_flags_get_type()) }
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl glib::HasParamSpec for RTPHeaderExtensionFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl glib::value::ValueType for RTPHeaderExtensionFlags {
    type Type = Self;
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
unsafe impl<'a> glib::value::FromValue<'a> for RTPHeaderExtensionFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl ToValue for RTPHeaderExtensionFlags {
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

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl From<RTPHeaderExtensionFlags> for glib::Value {
    #[inline]
    fn from(v: RTPHeaderExtensionFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
