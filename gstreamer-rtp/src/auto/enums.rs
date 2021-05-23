// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstRTCPFBType")]
pub enum RTCPFBType {
    #[doc(alias = "GST_RTCP_FB_TYPE_INVALID")]
    FbTypeInvalid,
    #[doc(alias = "GST_RTCP_RTPFB_TYPE_NACK")]
    RtpfbTypeNack,
    #[doc(alias = "GST_RTCP_RTPFB_TYPE_TMMBR")]
    RtpfbTypeTmmbr,
    #[doc(alias = "GST_RTCP_RTPFB_TYPE_TMMBN")]
    RtpfbTypeTmmbn,
    #[doc(alias = "GST_RTCP_RTPFB_TYPE_RTCP_SR_REQ")]
    RtpfbTypeRtcpSrReq,
    #[doc(alias = "GST_RTCP_RTPFB_TYPE_TWCC")]
    RtpfbTypeTwcc,
    #[doc(alias = "GST_RTCP_PSFB_TYPE_SLI")]
    PsfbTypeSli,
    #[doc(alias = "GST_RTCP_PSFB_TYPE_TSTN")]
    PsfbTypeTstn,
    #[doc(alias = "GST_RTCP_PSFB_TYPE_VBCN")]
    PsfbTypeVbcn,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for RTCPFBType {
    type GlibType = ffi::GstRTCPFBType;

    fn into_glib(self) -> ffi::GstRTCPFBType {
        match self {
            Self::FbTypeInvalid => ffi::GST_RTCP_FB_TYPE_INVALID,
            Self::RtpfbTypeNack => ffi::GST_RTCP_RTPFB_TYPE_NACK,
            Self::RtpfbTypeTmmbr => ffi::GST_RTCP_RTPFB_TYPE_TMMBR,
            Self::RtpfbTypeTmmbn => ffi::GST_RTCP_RTPFB_TYPE_TMMBN,
            Self::RtpfbTypeRtcpSrReq => ffi::GST_RTCP_RTPFB_TYPE_RTCP_SR_REQ,
            Self::RtpfbTypeTwcc => ffi::GST_RTCP_RTPFB_TYPE_TWCC,
            Self::PsfbTypeSli => ffi::GST_RTCP_PSFB_TYPE_SLI,
            Self::PsfbTypeTstn => ffi::GST_RTCP_PSFB_TYPE_TSTN,
            Self::PsfbTypeVbcn => ffi::GST_RTCP_PSFB_TYPE_VBCN,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTCPFBType> for RTCPFBType {
    unsafe fn from_glib(value: ffi::GstRTCPFBType) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_RTCP_FB_TYPE_INVALID => Self::FbTypeInvalid,
            ffi::GST_RTCP_RTPFB_TYPE_NACK => Self::RtpfbTypeNack,
            ffi::GST_RTCP_RTPFB_TYPE_TMMBR => Self::RtpfbTypeTmmbr,
            ffi::GST_RTCP_RTPFB_TYPE_TMMBN => Self::RtpfbTypeTmmbn,
            ffi::GST_RTCP_RTPFB_TYPE_RTCP_SR_REQ => Self::RtpfbTypeRtcpSrReq,
            ffi::GST_RTCP_RTPFB_TYPE_TWCC => Self::RtpfbTypeTwcc,
            ffi::GST_RTCP_PSFB_TYPE_SLI => Self::PsfbTypeSli,
            ffi::GST_RTCP_PSFB_TYPE_TSTN => Self::PsfbTypeTstn,
            ffi::GST_RTCP_PSFB_TYPE_VBCN => Self::PsfbTypeVbcn,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for RTCPFBType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtcpfb_type_get_type()) }
    }
}

impl glib::value::ValueType for RTCPFBType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for RTCPFBType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for RTCPFBType {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstRTCPSDESType")]
pub enum RTCPSDESType {
    #[doc(alias = "GST_RTCP_SDES_INVALID")]
    Invalid,
    #[doc(alias = "GST_RTCP_SDES_END")]
    End,
    #[doc(alias = "GST_RTCP_SDES_CNAME")]
    Cname,
    #[doc(alias = "GST_RTCP_SDES_NAME")]
    Name,
    #[doc(alias = "GST_RTCP_SDES_EMAIL")]
    Email,
    #[doc(alias = "GST_RTCP_SDES_PHONE")]
    Phone,
    #[doc(alias = "GST_RTCP_SDES_LOC")]
    Loc,
    #[doc(alias = "GST_RTCP_SDES_TOOL")]
    Tool,
    #[doc(alias = "GST_RTCP_SDES_NOTE")]
    Note,
    #[doc(alias = "GST_RTCP_SDES_PRIV")]
    Priv,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for RTCPSDESType {
    type GlibType = ffi::GstRTCPSDESType;

    fn into_glib(self) -> ffi::GstRTCPSDESType {
        match self {
            Self::Invalid => ffi::GST_RTCP_SDES_INVALID,
            Self::End => ffi::GST_RTCP_SDES_END,
            Self::Cname => ffi::GST_RTCP_SDES_CNAME,
            Self::Name => ffi::GST_RTCP_SDES_NAME,
            Self::Email => ffi::GST_RTCP_SDES_EMAIL,
            Self::Phone => ffi::GST_RTCP_SDES_PHONE,
            Self::Loc => ffi::GST_RTCP_SDES_LOC,
            Self::Tool => ffi::GST_RTCP_SDES_TOOL,
            Self::Note => ffi::GST_RTCP_SDES_NOTE,
            Self::Priv => ffi::GST_RTCP_SDES_PRIV,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTCPSDESType> for RTCPSDESType {
    unsafe fn from_glib(value: ffi::GstRTCPSDESType) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_RTCP_SDES_INVALID => Self::Invalid,
            ffi::GST_RTCP_SDES_END => Self::End,
            ffi::GST_RTCP_SDES_CNAME => Self::Cname,
            ffi::GST_RTCP_SDES_NAME => Self::Name,
            ffi::GST_RTCP_SDES_EMAIL => Self::Email,
            ffi::GST_RTCP_SDES_PHONE => Self::Phone,
            ffi::GST_RTCP_SDES_LOC => Self::Loc,
            ffi::GST_RTCP_SDES_TOOL => Self::Tool,
            ffi::GST_RTCP_SDES_NOTE => Self::Note,
            ffi::GST_RTCP_SDES_PRIV => Self::Priv,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for RTCPSDESType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtcpsdes_type_get_type()) }
    }
}

impl glib::value::ValueType for RTCPSDESType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for RTCPSDESType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for RTCPSDESType {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstRTCPType")]
pub enum RTCPType {
    #[doc(alias = "GST_RTCP_TYPE_INVALID")]
    Invalid,
    #[doc(alias = "GST_RTCP_TYPE_SR")]
    Sr,
    #[doc(alias = "GST_RTCP_TYPE_RR")]
    Rr,
    #[doc(alias = "GST_RTCP_TYPE_SDES")]
    Sdes,
    #[doc(alias = "GST_RTCP_TYPE_BYE")]
    Bye,
    #[doc(alias = "GST_RTCP_TYPE_APP")]
    App,
    #[doc(alias = "GST_RTCP_TYPE_RTPFB")]
    Rtpfb,
    #[doc(alias = "GST_RTCP_TYPE_PSFB")]
    Psfb,
    #[doc(alias = "GST_RTCP_TYPE_XR")]
    Xr,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for RTCPType {
    type GlibType = ffi::GstRTCPType;

    fn into_glib(self) -> ffi::GstRTCPType {
        match self {
            Self::Invalid => ffi::GST_RTCP_TYPE_INVALID,
            Self::Sr => ffi::GST_RTCP_TYPE_SR,
            Self::Rr => ffi::GST_RTCP_TYPE_RR,
            Self::Sdes => ffi::GST_RTCP_TYPE_SDES,
            Self::Bye => ffi::GST_RTCP_TYPE_BYE,
            Self::App => ffi::GST_RTCP_TYPE_APP,
            Self::Rtpfb => ffi::GST_RTCP_TYPE_RTPFB,
            Self::Psfb => ffi::GST_RTCP_TYPE_PSFB,
            Self::Xr => ffi::GST_RTCP_TYPE_XR,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTCPType> for RTCPType {
    unsafe fn from_glib(value: ffi::GstRTCPType) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_RTCP_TYPE_INVALID => Self::Invalid,
            ffi::GST_RTCP_TYPE_SR => Self::Sr,
            ffi::GST_RTCP_TYPE_RR => Self::Rr,
            ffi::GST_RTCP_TYPE_SDES => Self::Sdes,
            ffi::GST_RTCP_TYPE_BYE => Self::Bye,
            ffi::GST_RTCP_TYPE_APP => Self::App,
            ffi::GST_RTCP_TYPE_RTPFB => Self::Rtpfb,
            ffi::GST_RTCP_TYPE_PSFB => Self::Psfb,
            ffi::GST_RTCP_TYPE_XR => Self::Xr,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for RTCPType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtcp_type_get_type()) }
    }
}

impl glib::value::ValueType for RTCPType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for RTCPType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for RTCPType {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstRTCPXRType")]
pub enum RTCPXRType {
    #[doc(alias = "GST_RTCP_XR_TYPE_INVALID")]
    Invalid,
    #[doc(alias = "GST_RTCP_XR_TYPE_LRLE")]
    Lrle,
    #[doc(alias = "GST_RTCP_XR_TYPE_DRLE")]
    Drle,
    #[doc(alias = "GST_RTCP_XR_TYPE_PRT")]
    Prt,
    #[doc(alias = "GST_RTCP_XR_TYPE_RRT")]
    Rrt,
    #[doc(alias = "GST_RTCP_XR_TYPE_DLRR")]
    Dlrr,
    #[doc(alias = "GST_RTCP_XR_TYPE_SSUMM")]
    Ssumm,
    #[doc(alias = "GST_RTCP_XR_TYPE_VOIP_METRICS")]
    VoipMetrics,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
#[doc(hidden)]
impl IntoGlib for RTCPXRType {
    type GlibType = ffi::GstRTCPXRType;

    fn into_glib(self) -> ffi::GstRTCPXRType {
        match self {
            Self::Invalid => ffi::GST_RTCP_XR_TYPE_INVALID,
            Self::Lrle => ffi::GST_RTCP_XR_TYPE_LRLE,
            Self::Drle => ffi::GST_RTCP_XR_TYPE_DRLE,
            Self::Prt => ffi::GST_RTCP_XR_TYPE_PRT,
            Self::Rrt => ffi::GST_RTCP_XR_TYPE_RRT,
            Self::Dlrr => ffi::GST_RTCP_XR_TYPE_DLRR,
            Self::Ssumm => ffi::GST_RTCP_XR_TYPE_SSUMM,
            Self::VoipMetrics => ffi::GST_RTCP_XR_TYPE_VOIP_METRICS,
            Self::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
#[doc(hidden)]
impl FromGlib<ffi::GstRTCPXRType> for RTCPXRType {
    unsafe fn from_glib(value: ffi::GstRTCPXRType) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_RTCP_XR_TYPE_INVALID => Self::Invalid,
            ffi::GST_RTCP_XR_TYPE_LRLE => Self::Lrle,
            ffi::GST_RTCP_XR_TYPE_DRLE => Self::Drle,
            ffi::GST_RTCP_XR_TYPE_PRT => Self::Prt,
            ffi::GST_RTCP_XR_TYPE_RRT => Self::Rrt,
            ffi::GST_RTCP_XR_TYPE_DLRR => Self::Dlrr,
            ffi::GST_RTCP_XR_TYPE_SSUMM => Self::Ssumm,
            ffi::GST_RTCP_XR_TYPE_VOIP_METRICS => Self::VoipMetrics,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
impl StaticType for RTCPXRType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtcpxr_type_get_type()) }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
impl glib::value::ValueType for RTCPXRType {
    type Type = Self;
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
unsafe impl<'a> FromValue<'a> for RTCPXRType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
impl ToValue for RTCPXRType {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstRTPPayload")]
pub enum RTPPayload {
    #[doc(alias = "GST_RTP_PAYLOAD_PCMU")]
    Pcmu,
    #[doc(alias = "GST_RTP_PAYLOAD_1016")]
    _1016,
    #[doc(alias = "GST_RTP_PAYLOAD_G721")]
    G721,
    #[doc(alias = "GST_RTP_PAYLOAD_GSM")]
    Gsm,
    #[doc(alias = "GST_RTP_PAYLOAD_G723")]
    G723,
    #[doc(alias = "GST_RTP_PAYLOAD_DVI4_8000")]
    Dvi48000,
    #[doc(alias = "GST_RTP_PAYLOAD_DVI4_16000")]
    Dvi416000,
    #[doc(alias = "GST_RTP_PAYLOAD_LPC")]
    Lpc,
    #[doc(alias = "GST_RTP_PAYLOAD_PCMA")]
    Pcma,
    #[doc(alias = "GST_RTP_PAYLOAD_G722")]
    G722,
    #[doc(alias = "GST_RTP_PAYLOAD_L16_STEREO")]
    L16Stereo,
    #[doc(alias = "GST_RTP_PAYLOAD_L16_MONO")]
    L16Mono,
    #[doc(alias = "GST_RTP_PAYLOAD_QCELP")]
    Qcelp,
    #[doc(alias = "GST_RTP_PAYLOAD_CN")]
    Cn,
    #[doc(alias = "GST_RTP_PAYLOAD_MPA")]
    Mpa,
    #[doc(alias = "GST_RTP_PAYLOAD_G728")]
    G728,
    #[doc(alias = "GST_RTP_PAYLOAD_DVI4_11025")]
    Dvi411025,
    #[doc(alias = "GST_RTP_PAYLOAD_DVI4_22050")]
    Dvi422050,
    #[doc(alias = "GST_RTP_PAYLOAD_G729")]
    G729,
    #[doc(alias = "GST_RTP_PAYLOAD_CELLB")]
    Cellb,
    #[doc(alias = "GST_RTP_PAYLOAD_JPEG")]
    Jpeg,
    #[doc(alias = "GST_RTP_PAYLOAD_NV")]
    Nv,
    #[doc(alias = "GST_RTP_PAYLOAD_H261")]
    H261,
    #[doc(alias = "GST_RTP_PAYLOAD_MPV")]
    Mpv,
    #[doc(alias = "GST_RTP_PAYLOAD_MP2T")]
    Mp2t,
    #[doc(alias = "GST_RTP_PAYLOAD_H263")]
    H263,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for RTPPayload {
    type GlibType = ffi::GstRTPPayload;

    fn into_glib(self) -> ffi::GstRTPPayload {
        match self {
            Self::Pcmu => ffi::GST_RTP_PAYLOAD_PCMU,
            Self::_1016 => ffi::GST_RTP_PAYLOAD_1016,
            Self::G721 => ffi::GST_RTP_PAYLOAD_G721,
            Self::Gsm => ffi::GST_RTP_PAYLOAD_GSM,
            Self::G723 => ffi::GST_RTP_PAYLOAD_G723,
            Self::Dvi48000 => ffi::GST_RTP_PAYLOAD_DVI4_8000,
            Self::Dvi416000 => ffi::GST_RTP_PAYLOAD_DVI4_16000,
            Self::Lpc => ffi::GST_RTP_PAYLOAD_LPC,
            Self::Pcma => ffi::GST_RTP_PAYLOAD_PCMA,
            Self::G722 => ffi::GST_RTP_PAYLOAD_G722,
            Self::L16Stereo => ffi::GST_RTP_PAYLOAD_L16_STEREO,
            Self::L16Mono => ffi::GST_RTP_PAYLOAD_L16_MONO,
            Self::Qcelp => ffi::GST_RTP_PAYLOAD_QCELP,
            Self::Cn => ffi::GST_RTP_PAYLOAD_CN,
            Self::Mpa => ffi::GST_RTP_PAYLOAD_MPA,
            Self::G728 => ffi::GST_RTP_PAYLOAD_G728,
            Self::Dvi411025 => ffi::GST_RTP_PAYLOAD_DVI4_11025,
            Self::Dvi422050 => ffi::GST_RTP_PAYLOAD_DVI4_22050,
            Self::G729 => ffi::GST_RTP_PAYLOAD_G729,
            Self::Cellb => ffi::GST_RTP_PAYLOAD_CELLB,
            Self::Jpeg => ffi::GST_RTP_PAYLOAD_JPEG,
            Self::Nv => ffi::GST_RTP_PAYLOAD_NV,
            Self::H261 => ffi::GST_RTP_PAYLOAD_H261,
            Self::Mpv => ffi::GST_RTP_PAYLOAD_MPV,
            Self::Mp2t => ffi::GST_RTP_PAYLOAD_MP2T,
            Self::H263 => ffi::GST_RTP_PAYLOAD_H263,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTPPayload> for RTPPayload {
    unsafe fn from_glib(value: ffi::GstRTPPayload) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_RTP_PAYLOAD_PCMU => Self::Pcmu,
            ffi::GST_RTP_PAYLOAD_1016 => Self::_1016,
            ffi::GST_RTP_PAYLOAD_G721 => Self::G721,
            ffi::GST_RTP_PAYLOAD_GSM => Self::Gsm,
            ffi::GST_RTP_PAYLOAD_G723 => Self::G723,
            ffi::GST_RTP_PAYLOAD_DVI4_8000 => Self::Dvi48000,
            ffi::GST_RTP_PAYLOAD_DVI4_16000 => Self::Dvi416000,
            ffi::GST_RTP_PAYLOAD_LPC => Self::Lpc,
            ffi::GST_RTP_PAYLOAD_PCMA => Self::Pcma,
            ffi::GST_RTP_PAYLOAD_G722 => Self::G722,
            ffi::GST_RTP_PAYLOAD_L16_STEREO => Self::L16Stereo,
            ffi::GST_RTP_PAYLOAD_L16_MONO => Self::L16Mono,
            ffi::GST_RTP_PAYLOAD_QCELP => Self::Qcelp,
            ffi::GST_RTP_PAYLOAD_CN => Self::Cn,
            ffi::GST_RTP_PAYLOAD_MPA => Self::Mpa,
            ffi::GST_RTP_PAYLOAD_G728 => Self::G728,
            ffi::GST_RTP_PAYLOAD_DVI4_11025 => Self::Dvi411025,
            ffi::GST_RTP_PAYLOAD_DVI4_22050 => Self::Dvi422050,
            ffi::GST_RTP_PAYLOAD_G729 => Self::G729,
            ffi::GST_RTP_PAYLOAD_CELLB => Self::Cellb,
            ffi::GST_RTP_PAYLOAD_JPEG => Self::Jpeg,
            ffi::GST_RTP_PAYLOAD_NV => Self::Nv,
            ffi::GST_RTP_PAYLOAD_H261 => Self::H261,
            ffi::GST_RTP_PAYLOAD_MPV => Self::Mpv,
            ffi::GST_RTP_PAYLOAD_MP2T => Self::Mp2t,
            ffi::GST_RTP_PAYLOAD_H263 => Self::H263,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for RTPPayload {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtp_payload_get_type()) }
    }
}

impl glib::value::ValueType for RTPPayload {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for RTPPayload {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for RTPPayload {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstRTPProfile")]
pub enum RTPProfile {
    #[doc(alias = "GST_RTP_PROFILE_UNKNOWN")]
    Unknown,
    #[doc(alias = "GST_RTP_PROFILE_AVP")]
    Avp,
    #[doc(alias = "GST_RTP_PROFILE_SAVP")]
    Savp,
    #[doc(alias = "GST_RTP_PROFILE_AVPF")]
    Avpf,
    #[doc(alias = "GST_RTP_PROFILE_SAVPF")]
    Savpf,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for RTPProfile {
    type GlibType = ffi::GstRTPProfile;

    fn into_glib(self) -> ffi::GstRTPProfile {
        match self {
            Self::Unknown => ffi::GST_RTP_PROFILE_UNKNOWN,
            Self::Avp => ffi::GST_RTP_PROFILE_AVP,
            Self::Savp => ffi::GST_RTP_PROFILE_SAVP,
            Self::Avpf => ffi::GST_RTP_PROFILE_AVPF,
            Self::Savpf => ffi::GST_RTP_PROFILE_SAVPF,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTPProfile> for RTPProfile {
    unsafe fn from_glib(value: ffi::GstRTPProfile) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_RTP_PROFILE_UNKNOWN => Self::Unknown,
            ffi::GST_RTP_PROFILE_AVP => Self::Avp,
            ffi::GST_RTP_PROFILE_SAVP => Self::Savp,
            ffi::GST_RTP_PROFILE_AVPF => Self::Avpf,
            ffi::GST_RTP_PROFILE_SAVPF => Self::Savpf,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for RTPProfile {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtp_profile_get_type()) }
    }
}

impl glib::value::ValueType for RTPProfile {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for RTPProfile {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for RTPProfile {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
