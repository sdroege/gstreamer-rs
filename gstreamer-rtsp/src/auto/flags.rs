// This file was generated by gir (https://github.com/gtk-rs/gir @ 9ae0a4a+)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib::StaticType;
use glib::Type;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use gobject_ffi;

bitflags! {
    pub struct RTSPEvent: u32 {
        const READ = 1;
        const WRITE = 2;
    }
}

#[doc(hidden)]
impl ToGlib for RTSPEvent {
    type GlibType = ffi::GstRTSPEvent;

    fn to_glib(&self) -> ffi::GstRTSPEvent {
        ffi::GstRTSPEvent::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTSPEvent> for RTSPEvent {
    fn from_glib(value: ffi::GstRTSPEvent) -> RTSPEvent {
        skip_assert_initialized!();
        RTSPEvent::from_bits_truncate(value.bits())
    }
}

impl StaticType for RTSPEvent {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtsp_event_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RTSPEvent {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RTSPEvent {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstRTSPEvent::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for RTSPEvent {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct RTSPLowerTrans: u32 {
        const UNKNOWN = 0;
        const UDP = 1;
        const UDP_MCAST = 2;
        const TCP = 4;
        const HTTP = 16;
        const TLS = 32;
    }
}

#[doc(hidden)]
impl ToGlib for RTSPLowerTrans {
    type GlibType = ffi::GstRTSPLowerTrans;

    fn to_glib(&self) -> ffi::GstRTSPLowerTrans {
        ffi::GstRTSPLowerTrans::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTSPLowerTrans> for RTSPLowerTrans {
    fn from_glib(value: ffi::GstRTSPLowerTrans) -> RTSPLowerTrans {
        skip_assert_initialized!();
        RTSPLowerTrans::from_bits_truncate(value.bits())
    }
}

impl StaticType for RTSPLowerTrans {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtsp_lower_trans_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RTSPLowerTrans {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RTSPLowerTrans {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstRTSPLowerTrans::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for RTSPLowerTrans {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct RTSPMethod: u32 {
        const INVALID = 0;
        const DESCRIBE = 1;
        const ANNOUNCE = 2;
        const GET_PARAMETER = 4;
        const OPTIONS = 8;
        const PAUSE = 16;
        const PLAY = 32;
        const RECORD = 64;
        const REDIRECT = 128;
        const SETUP = 256;
        const SET_PARAMETER = 512;
        const TEARDOWN = 1024;
        const GET = 2048;
        const POST = 4096;
    }
}

#[doc(hidden)]
impl ToGlib for RTSPMethod {
    type GlibType = ffi::GstRTSPMethod;

    fn to_glib(&self) -> ffi::GstRTSPMethod {
        ffi::GstRTSPMethod::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTSPMethod> for RTSPMethod {
    fn from_glib(value: ffi::GstRTSPMethod) -> RTSPMethod {
        skip_assert_initialized!();
        RTSPMethod::from_bits_truncate(value.bits())
    }
}

impl StaticType for RTSPMethod {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtsp_method_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RTSPMethod {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RTSPMethod {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstRTSPMethod::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for RTSPMethod {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct RTSPProfile: u32 {
        const UNKNOWN = 0;
        const AVP = 1;
        const SAVP = 2;
        const AVPF = 4;
        const SAVPF = 8;
    }
}

#[doc(hidden)]
impl ToGlib for RTSPProfile {
    type GlibType = ffi::GstRTSPProfile;

    fn to_glib(&self) -> ffi::GstRTSPProfile {
        ffi::GstRTSPProfile::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTSPProfile> for RTSPProfile {
    fn from_glib(value: ffi::GstRTSPProfile) -> RTSPProfile {
        skip_assert_initialized!();
        RTSPProfile::from_bits_truncate(value.bits())
    }
}

impl StaticType for RTSPProfile {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtsp_profile_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RTSPProfile {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RTSPProfile {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstRTSPProfile::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for RTSPProfile {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct RTSPTransMode: u32 {
        const UNKNOWN = 0;
        const RTP = 1;
        const RDT = 2;
    }
}

#[doc(hidden)]
impl ToGlib for RTSPTransMode {
    type GlibType = ffi::GstRTSPTransMode;

    fn to_glib(&self) -> ffi::GstRTSPTransMode {
        ffi::GstRTSPTransMode::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTSPTransMode> for RTSPTransMode {
    fn from_glib(value: ffi::GstRTSPTransMode) -> RTSPTransMode {
        skip_assert_initialized!();
        RTSPTransMode::from_bits_truncate(value.bits())
    }
}

impl StaticType for RTSPTransMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtsp_trans_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RTSPTransMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RTSPTransMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstRTSPTransMode::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for RTSPTransMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

