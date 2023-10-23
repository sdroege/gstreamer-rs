// Take a look at the license at the top of the repository in the LICENSE file.

use std::{cmp, ops};

use glib::{
    translate::*,
    value::{FromValue, ToValue, Value},
    StaticType, Type,
};
use thiserror::Error;

use crate::{ClockReturn, State, StateChange, StateChangeReturn};

macro_rules! impl_return_result_traits {
    ($ffi_type:ident, $ret_type:ident, $ok_type:ident, $err_type:ident) => {
        impl From<$ok_type> for $ret_type {
            #[inline]
            fn from(value: $ok_type) -> Self {
                skip_assert_initialized!();
                $ret_type::from_ok(value)
            }
        }

        impl IntoGlib for $ok_type {
            type GlibType = <$ret_type as IntoGlib>::GlibType;

            #[inline]
            fn into_glib(self) -> Self::GlibType {
                $ret_type::from_ok(self).into_glib()
            }
        }

        impl From<$err_type> for $ret_type {
            #[inline]
            fn from(value: $err_type) -> Self {
                skip_assert_initialized!();
                $ret_type::from_error(value)
            }
        }

        impl IntoGlib for $err_type {
            type GlibType = <$ret_type as IntoGlib>::GlibType;

            #[inline]
            fn into_glib(self) -> Self::GlibType {
                $ret_type::from_error(self).into_glib()
            }
        }

        impl From<Result<$ok_type, $err_type>> for $ret_type {
            #[inline]
            fn from(res: Result<$ok_type, $err_type>) -> Self {
                skip_assert_initialized!();
                match res {
                    Ok(success) => $ret_type::from_ok(success),
                    Err(error) => $ret_type::from_error(error),
                }
            }
        }

        impl TryFromGlib<ffi::$ffi_type> for $ok_type {
            type Error = $err_type;

            #[inline]
            unsafe fn try_from_glib(val: ffi::$ffi_type) -> Result<$ok_type, $err_type> {
                skip_assert_initialized!();
                $ret_type::from_glib(val).into_result()
            }
        }
    };
}

impl StateChangeReturn {
    #[inline]
    pub fn into_result(self) -> Result<StateChangeSuccess, StateChangeError> {
        match self {
            StateChangeReturn::Failure => Err(StateChangeError),
            _ => Ok(unsafe { std::mem::transmute(self) }),
        }
    }

    #[inline]
    pub fn from_error(_: StateChangeError) -> Self {
        skip_assert_initialized!();
        StateChangeReturn::Failure
    }

    #[inline]
    pub fn from_ok(v: StateChangeSuccess) -> Self {
        skip_assert_initialized!();
        unsafe { std::mem::transmute(v) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum StateChangeSuccess {
    Success = ffi::GST_STATE_CHANGE_SUCCESS,
    Async = ffi::GST_STATE_CHANGE_ASYNC,
    NoPreroll = ffi::GST_STATE_CHANGE_NO_PREROLL,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
#[must_use]
#[error("Element failed to change its state")]
pub struct StateChangeError;

impl_return_result_traits!(
    GstStateChangeReturn,
    StateChangeReturn,
    StateChangeSuccess,
    StateChangeError
);

#[must_use]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[repr(i32)]
#[doc(alias = "GstFlowReturn")]
pub enum FlowReturn {
    #[doc(alias = "GST_FLOW_CUSTOM_SUCCESS_2")]
    CustomSuccess2 = ffi::GST_FLOW_CUSTOM_SUCCESS_2,
    #[doc(alias = "GST_FLOW_CUSTOM_SUCCESS_1")]
    CustomSuccess1 = ffi::GST_FLOW_CUSTOM_SUCCESS_1,
    #[doc(alias = "GST_FLOW_CUSTOM_SUCCESS")]
    CustomSuccess = ffi::GST_FLOW_CUSTOM_SUCCESS,
    #[doc(alias = "GST_FLOW_OK")]
    Ok = ffi::GST_FLOW_OK,
    #[doc(alias = "GST_FLOW_NOT_LINKED")]
    NotLinked = ffi::GST_FLOW_NOT_LINKED,
    #[doc(alias = "GST_FLOW_FLUSHING")]
    Flushing = ffi::GST_FLOW_FLUSHING,
    #[doc(alias = "GST_FLOW_EOS")]
    Eos = ffi::GST_FLOW_EOS,
    #[doc(alias = "GST_FLOW_NOT_NEGOTIATED")]
    NotNegotiated = ffi::GST_FLOW_NOT_NEGOTIATED,
    #[doc(alias = "GST_FLOW_ERROR")]
    Error = ffi::GST_FLOW_ERROR,
    #[doc(alias = "GST_FLOW_NOT_SUPPORTED")]
    NotSupported = ffi::GST_FLOW_NOT_SUPPORTED,
    #[doc(alias = "GST_FLOW_CUSTOM_ERROR")]
    CustomError = ffi::GST_FLOW_CUSTOM_ERROR,
    #[doc(alias = "GST_FLOW_CUSTOM_ERROR_1")]
    CustomError1 = ffi::GST_FLOW_CUSTOM_ERROR_1,
    #[doc(alias = "GST_FLOW_CUSTOM_ERROR_2")]
    CustomError2 = ffi::GST_FLOW_CUSTOM_ERROR_2,
}

#[doc(hidden)]
impl IntoGlib for FlowReturn {
    type GlibType = ffi::GstFlowReturn;

    #[inline]
    fn into_glib(self) -> ffi::GstFlowReturn {
        self as ffi::GstFlowReturn
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstFlowReturn> for FlowReturn {
    #[inline]
    unsafe fn from_glib(value: ffi::GstFlowReturn) -> Self {
        skip_assert_initialized!();

        if value < ffi::GST_FLOW_NOT_SUPPORTED
            && (value > ffi::GST_FLOW_CUSTOM_ERROR || value < ffi::GST_FLOW_CUSTOM_ERROR_2)
        {
            FlowReturn::Error
        } else if value > 0
            && (value < ffi::GST_FLOW_CUSTOM_SUCCESS || value > ffi::GST_FLOW_CUSTOM_SUCCESS_2)
        {
            FlowReturn::Ok
        } else {
            std::mem::transmute(value)
        }
    }
}

impl StaticType for FlowReturn {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_flow_return_get_type()) }
    }
}

impl glib::value::ValueType for FlowReturn {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for FlowReturn {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for FlowReturn {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<FlowReturn> for glib::Value {
    #[inline]
    fn from(v: FlowReturn) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

impl FlowReturn {
    #[inline]
    pub fn into_result(self) -> Result<FlowSuccess, FlowError> {
        if self.into_glib() >= 0 {
            Ok(unsafe { std::mem::transmute(self) })
        } else {
            Err(unsafe { std::mem::transmute(self) })
        }
    }

    #[inline]
    pub fn from_error(v: FlowError) -> Self {
        skip_assert_initialized!();
        unsafe { std::mem::transmute(v) }
    }

    #[inline]
    pub fn from_ok(v: FlowSuccess) -> Self {
        skip_assert_initialized!();
        unsafe { std::mem::transmute(v) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum FlowSuccess {
    CustomSuccess2 = ffi::GST_FLOW_CUSTOM_SUCCESS_2,
    CustomSuccess1 = ffi::GST_FLOW_CUSTOM_SUCCESS_1,
    CustomSuccess = ffi::GST_FLOW_CUSTOM_SUCCESS,
    Ok = ffi::GST_FLOW_OK,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
#[must_use]
#[repr(i32)]
pub enum FlowError {
    #[error("Pad is not linked")]
    NotLinked = ffi::GST_FLOW_NOT_LINKED,
    #[error("Pad is flushing")]
    Flushing = ffi::GST_FLOW_FLUSHING,
    #[error("Pad is EOS")]
    Eos = ffi::GST_FLOW_EOS,
    #[error("Pad is not negotiated")]
    NotNegotiated = ffi::GST_FLOW_NOT_NEGOTIATED,
    #[error("Some (fatal) error occurred. Element generating this error should post an error message with more details")]
    Error = ffi::GST_FLOW_ERROR,
    #[error("This operation is not supported")]
    NotSupported = ffi::GST_FLOW_NOT_SUPPORTED,
    #[error("Elements can use values starting from this (and lower) to define custom error codes")]
    CustomError = ffi::GST_FLOW_CUSTOM_ERROR,
    #[error("Pre-defined custom error code")]
    CustomError1 = ffi::GST_FLOW_CUSTOM_ERROR_1,
    #[error("Pre-defined custom error code")]
    CustomError2 = ffi::GST_FLOW_CUSTOM_ERROR_2,
}

impl_return_result_traits!(GstFlowReturn, FlowReturn, FlowSuccess, FlowError);

#[must_use]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[repr(i32)]
#[doc(alias = "GstPadLinkReturn")]
pub enum PadLinkReturn {
    #[doc(alias = "GST_PAD_LINK_OK")]
    Ok = ffi::GST_PAD_LINK_OK,
    #[doc(alias = "GST_PAD_LINK_WRONG_HIERARCHY")]
    WrongHierarchy = ffi::GST_PAD_LINK_WRONG_HIERARCHY,
    #[doc(alias = "GST_PAD_LINK_WAS_LINKED")]
    WasLinked = ffi::GST_PAD_LINK_WAS_LINKED,
    #[doc(alias = "GST_PAD_LINK_WRONG_DIRECTION")]
    WrongDirection = ffi::GST_PAD_LINK_WRONG_DIRECTION,
    #[doc(alias = "GST_PAD_LINK_NOFORMAT")]
    Noformat = ffi::GST_PAD_LINK_NOFORMAT,
    #[doc(alias = "GST_PAD_LINK_NOSCHED")]
    Nosched = ffi::GST_PAD_LINK_NOSCHED,
    #[doc(alias = "GST_PAD_LINK_REFUSED")]
    Refused = ffi::GST_PAD_LINK_REFUSED,
}

#[doc(hidden)]
impl IntoGlib for PadLinkReturn {
    type GlibType = ffi::GstPadLinkReturn;

    #[inline]
    fn into_glib(self) -> ffi::GstPadLinkReturn {
        self as ffi::GstPadLinkReturn
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPadLinkReturn> for PadLinkReturn {
    #[inline]
    unsafe fn from_glib(value: ffi::GstPadLinkReturn) -> Self {
        skip_assert_initialized!();

        if value >= 0 {
            PadLinkReturn::Ok
        } else if value < ffi::GST_PAD_LINK_REFUSED {
            PadLinkReturn::Refused
        } else {
            std::mem::transmute(value)
        }
    }
}

impl StaticType for PadLinkReturn {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_pad_link_return_get_type()) }
    }
}

impl glib::value::ValueType for PadLinkReturn {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PadLinkReturn {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for PadLinkReturn {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<PadLinkReturn> for glib::Value {
    #[inline]
    fn from(v: PadLinkReturn) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

impl PadLinkReturn {
    #[inline]
    pub fn into_result(self) -> Result<PadLinkSuccess, PadLinkError> {
        if self == PadLinkReturn::Ok {
            Ok(PadLinkSuccess)
        } else {
            Err(unsafe { std::mem::transmute(self) })
        }
    }

    #[inline]
    pub fn from_error(v: PadLinkError) -> Self {
        skip_assert_initialized!();
        unsafe { std::mem::transmute(v) }
    }

    #[inline]
    pub fn from_ok(_: PadLinkSuccess) -> Self {
        skip_assert_initialized!();
        PadLinkReturn::Ok
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PadLinkSuccess;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
#[must_use]
#[repr(i32)]
pub enum PadLinkError {
    #[error("Pads have no common grandparent")]
    WrongHierarchy = ffi::GST_PAD_LINK_WRONG_HIERARCHY,
    #[error("Pad was already linked")]
    WasLinked = ffi::GST_PAD_LINK_WAS_LINKED,
    #[error("Pads have wrong direction")]
    WrongDirection = ffi::GST_PAD_LINK_WRONG_DIRECTION,
    #[error("Pads do not have common format")]
    Noformat = ffi::GST_PAD_LINK_NOFORMAT,
    #[error("Pads cannot cooperate in scheduling")]
    Nosched = ffi::GST_PAD_LINK_NOSCHED,
    #[error("Refused for some other reason")]
    Refused = ffi::GST_PAD_LINK_REFUSED,
}

impl_return_result_traits!(
    GstPadLinkReturn,
    PadLinkReturn,
    PadLinkSuccess,
    PadLinkError
);

impl ClockReturn {
    #[inline]
    pub fn into_result(self) -> Result<ClockSuccess, ClockError> {
        match self {
            ClockReturn::Ok => Ok(ClockSuccess::Ok),
            ClockReturn::Done => Ok(ClockSuccess::Done),
            _ => Err(unsafe { std::mem::transmute(self) }),
        }
    }

    #[inline]
    pub fn from_error(v: ClockError) -> Self {
        skip_assert_initialized!();
        unsafe { std::mem::transmute(v) }
    }

    #[inline]
    pub fn from_ok(v: ClockSuccess) -> Self {
        skip_assert_initialized!();
        unsafe { std::mem::transmute(v) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum ClockSuccess {
    Ok = ffi::GST_CLOCK_OK,
    Done = ffi::GST_CLOCK_DONE,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
#[must_use]
#[repr(i32)]
pub enum ClockError {
    #[error("The operation was scheduled too late")]
    Early = ffi::GST_CLOCK_EARLY,
    #[error("The clockID was unscheduled")]
    Unscheduled = ffi::GST_CLOCK_UNSCHEDULED,
    #[error("The ClockID is busy")]
    Busy = ffi::GST_CLOCK_BUSY,
    #[error("A bad time was provided to a function")]
    Badtime = ffi::GST_CLOCK_BADTIME,
    #[error("An error occurred")]
    Error = ffi::GST_CLOCK_ERROR,
    #[error("Operation is not supported")]
    Unsupported = ffi::GST_CLOCK_UNSUPPORTED,
}

impl_return_result_traits!(GstClockReturn, ClockReturn, ClockSuccess, ClockError);

impl PartialEq for crate::TypeFindProbability {
    #[inline]
    fn eq(&self, other: &crate::TypeFindProbability) -> bool {
        (self.into_glib() as u32).eq(&(other.into_glib() as u32))
    }
}

impl Eq for crate::TypeFindProbability {}

impl PartialOrd for crate::TypeFindProbability {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for crate::TypeFindProbability {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (self.into_glib() as u32).cmp(&(other.into_glib() as u32))
    }
}

impl ops::Add<u32> for crate::TypeFindProbability {
    type Output = crate::TypeFindProbability;

    #[inline]
    fn add(self, rhs: u32) -> crate::TypeFindProbability {
        let res = (self.into_glib() as u32).saturating_add(rhs);
        unsafe { from_glib(res as i32) }
    }
}

impl ops::AddAssign<u32> for crate::TypeFindProbability {
    #[inline]
    fn add_assign(&mut self, rhs: u32) {
        let res = (self.into_glib() as u32).saturating_add(rhs);
        *self = unsafe { from_glib(res as i32) };
    }
}

impl ops::Sub<u32> for crate::TypeFindProbability {
    type Output = crate::TypeFindProbability;

    #[inline]
    fn sub(self, rhs: u32) -> crate::TypeFindProbability {
        let res = (self.into_glib() as u32).saturating_sub(rhs);
        unsafe { from_glib(res as i32) }
    }
}

impl ops::SubAssign<u32> for crate::TypeFindProbability {
    #[inline]
    fn sub_assign(&mut self, rhs: u32) {
        let res = (self.into_glib() as u32).saturating_sub(rhs);
        *self = unsafe { from_glib(res as i32) };
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
#[must_use]
pub enum TagError {
    #[error("The value type doesn't match with the specified Tag")]
    TypeMismatch,
}

// This cannot be done automatically because in GStreamer it's exposed as a bitflag but works as an
// enum instead
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[doc(alias = "GstMessageType")]
#[non_exhaustive]
pub enum MessageType {
    #[doc(alias = "GST_MESSAGE_UNKNOWN")]
    Unknown,
    #[doc(alias = "GST_MESSAGE_EOS")]
    Eos,
    #[doc(alias = "GST_MESSAGE_ERROR")]
    Error,
    #[doc(alias = "GST_MESSAGE_WARNING")]
    Warning,
    #[doc(alias = "GST_MESSAGE_INFO")]
    Info,
    #[doc(alias = "GST_MESSAGE_TAG")]
    Tag,
    #[doc(alias = "GST_MESSAGE_BUFFERING")]
    Buffering,
    #[doc(alias = "GST_MESSAGE_STATE_CHANGED")]
    StateChanged,
    #[doc(alias = "GST_MESSAGE_STATE_DIRTY")]
    StateDirty,
    #[doc(alias = "GST_MESSAGE_STEP_DONE")]
    StepDone,
    #[doc(alias = "GST_MESSAGE_CLOCK_PROVIDE")]
    ClockProvide,
    #[doc(alias = "GST_MESSAGE_CLOCK_LOST")]
    ClockLost,
    #[doc(alias = "GST_MESSAGE_NEW_CLOCK")]
    NewClock,
    #[doc(alias = "GST_MESSAGE_STRUCTURE_CHANGE")]
    StructureChange,
    #[doc(alias = "GST_MESSAGE_STREAM_STATUS")]
    StreamStatus,
    #[doc(alias = "GST_MESSAGE_APPLICATION")]
    Application,
    #[doc(alias = "GST_MESSAGE_ELEMENT")]
    Element,
    #[doc(alias = "GST_MESSAGE_SEGMENT_START")]
    SegmentStart,
    #[doc(alias = "GST_MESSAGE_SEGMENT_DONE")]
    SegmentDone,
    #[doc(alias = "GST_MESSAGE_DURATION_CHANGED")]
    DurationChanged,
    #[doc(alias = "GST_MESSAGE_LATENCY")]
    Latency,
    #[doc(alias = "GST_MESSAGE_ASYNC_START")]
    AsyncStart,
    #[doc(alias = "GST_MESSAGE_ASYNC_DONE")]
    AsyncDone,
    #[doc(alias = "GST_MESSAGE_REQUEST_STATE")]
    RequestState,
    #[doc(alias = "GST_MESSAGE_STEP_START")]
    StepStart,
    #[doc(alias = "GST_MESSAGE_QOS")]
    Qos,
    #[doc(alias = "GST_MESSAGE_PROGRESS")]
    Progress,
    #[doc(alias = "GST_MESSAGE_TOC")]
    Toc,
    #[doc(alias = "GST_MESSAGE_RESET_TIME")]
    ResetTime,
    #[doc(alias = "GST_MESSAGE_STREAM_START")]
    StreamStart,
    #[doc(alias = "GST_MESSAGE_NEED_CONTEXT")]
    NeedContext,
    #[doc(alias = "GST_MESSAGE_HAVE_CONTEXT")]
    HaveContext,
    #[doc(alias = "GST_MESSAGE_EXTENDED")]
    Extended,
    #[doc(alias = "GST_MESSAGE_DEVICE_ADDED")]
    DeviceAdded,
    #[doc(alias = "GST_MESSAGE_DEVICE_REMOVED")]
    DeviceRemoved,
    #[doc(alias = "GST_MESSAGE_PROPERTY_NOTIFY")]
    PropertyNotify,
    #[doc(alias = "GST_MESSAGE_STREAM_COLLECTION")]
    StreamCollection,
    #[doc(alias = "GST_MESSAGE_STREAMS_SELECTED")]
    StreamsSelected,
    #[doc(alias = "GST_MESSAGE_REDIRECT")]
    Redirect,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for MessageType {
    type GlibType = ffi::GstMessageType;

    fn into_glib(self) -> ffi::GstMessageType {
        match self {
            MessageType::Unknown => ffi::GST_MESSAGE_UNKNOWN,
            MessageType::Eos => ffi::GST_MESSAGE_EOS,
            MessageType::Error => ffi::GST_MESSAGE_ERROR,
            MessageType::Warning => ffi::GST_MESSAGE_WARNING,
            MessageType::Info => ffi::GST_MESSAGE_INFO,
            MessageType::Tag => ffi::GST_MESSAGE_TAG,
            MessageType::Buffering => ffi::GST_MESSAGE_BUFFERING,
            MessageType::StateChanged => ffi::GST_MESSAGE_STATE_CHANGED,
            MessageType::StateDirty => ffi::GST_MESSAGE_STATE_DIRTY,
            MessageType::StepDone => ffi::GST_MESSAGE_STEP_DONE,
            MessageType::ClockProvide => ffi::GST_MESSAGE_CLOCK_PROVIDE,
            MessageType::ClockLost => ffi::GST_MESSAGE_CLOCK_LOST,
            MessageType::NewClock => ffi::GST_MESSAGE_NEW_CLOCK,
            MessageType::StructureChange => ffi::GST_MESSAGE_STRUCTURE_CHANGE,
            MessageType::StreamStatus => ffi::GST_MESSAGE_STREAM_STATUS,
            MessageType::Application => ffi::GST_MESSAGE_APPLICATION,
            MessageType::Element => ffi::GST_MESSAGE_ELEMENT,
            MessageType::SegmentStart => ffi::GST_MESSAGE_SEGMENT_START,
            MessageType::SegmentDone => ffi::GST_MESSAGE_SEGMENT_DONE,
            MessageType::DurationChanged => ffi::GST_MESSAGE_DURATION_CHANGED,
            MessageType::Latency => ffi::GST_MESSAGE_LATENCY,
            MessageType::AsyncStart => ffi::GST_MESSAGE_ASYNC_START,
            MessageType::AsyncDone => ffi::GST_MESSAGE_ASYNC_DONE,
            MessageType::RequestState => ffi::GST_MESSAGE_REQUEST_STATE,
            MessageType::StepStart => ffi::GST_MESSAGE_STEP_START,
            MessageType::Qos => ffi::GST_MESSAGE_QOS,
            MessageType::Progress => ffi::GST_MESSAGE_PROGRESS,
            MessageType::Toc => ffi::GST_MESSAGE_TOC,
            MessageType::ResetTime => ffi::GST_MESSAGE_RESET_TIME,
            MessageType::StreamStart => ffi::GST_MESSAGE_STREAM_START,
            MessageType::NeedContext => ffi::GST_MESSAGE_NEED_CONTEXT,
            MessageType::HaveContext => ffi::GST_MESSAGE_HAVE_CONTEXT,
            MessageType::Extended => ffi::GST_MESSAGE_EXTENDED,
            MessageType::DeviceAdded => ffi::GST_MESSAGE_DEVICE_ADDED,
            MessageType::DeviceRemoved => ffi::GST_MESSAGE_DEVICE_REMOVED,
            MessageType::PropertyNotify => ffi::GST_MESSAGE_PROPERTY_NOTIFY,
            MessageType::StreamCollection => ffi::GST_MESSAGE_STREAM_COLLECTION,
            MessageType::StreamsSelected => ffi::GST_MESSAGE_STREAMS_SELECTED,
            MessageType::Redirect => ffi::GST_MESSAGE_REDIRECT,
            MessageType::__Unknown(value) => value as u32,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstMessageType> for MessageType {
    #[allow(clippy::unreadable_literal)]
    unsafe fn from_glib(value: ffi::GstMessageType) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_MESSAGE_UNKNOWN => MessageType::Unknown,
            ffi::GST_MESSAGE_EOS => MessageType::Eos,
            ffi::GST_MESSAGE_ERROR => MessageType::Error,
            ffi::GST_MESSAGE_WARNING => MessageType::Warning,
            ffi::GST_MESSAGE_INFO => MessageType::Info,
            ffi::GST_MESSAGE_TAG => MessageType::Tag,
            ffi::GST_MESSAGE_BUFFERING => MessageType::Buffering,
            ffi::GST_MESSAGE_STATE_CHANGED => MessageType::StateChanged,
            ffi::GST_MESSAGE_STATE_DIRTY => MessageType::StateDirty,
            ffi::GST_MESSAGE_STEP_DONE => MessageType::StepDone,
            ffi::GST_MESSAGE_CLOCK_PROVIDE => MessageType::ClockProvide,
            ffi::GST_MESSAGE_CLOCK_LOST => MessageType::ClockLost,
            ffi::GST_MESSAGE_NEW_CLOCK => MessageType::NewClock,
            ffi::GST_MESSAGE_STRUCTURE_CHANGE => MessageType::StructureChange,
            ffi::GST_MESSAGE_STREAM_STATUS => MessageType::StreamStatus,
            ffi::GST_MESSAGE_APPLICATION => MessageType::Application,
            ffi::GST_MESSAGE_ELEMENT => MessageType::Element,
            ffi::GST_MESSAGE_SEGMENT_START => MessageType::SegmentStart,
            ffi::GST_MESSAGE_SEGMENT_DONE => MessageType::SegmentDone,
            ffi::GST_MESSAGE_DURATION_CHANGED => MessageType::DurationChanged,
            ffi::GST_MESSAGE_LATENCY => MessageType::Latency,
            ffi::GST_MESSAGE_ASYNC_START => MessageType::AsyncStart,
            ffi::GST_MESSAGE_ASYNC_DONE => MessageType::AsyncDone,
            ffi::GST_MESSAGE_REQUEST_STATE => MessageType::RequestState,
            ffi::GST_MESSAGE_STEP_START => MessageType::StepStart,
            ffi::GST_MESSAGE_QOS => MessageType::Qos,
            ffi::GST_MESSAGE_PROGRESS => MessageType::Progress,
            ffi::GST_MESSAGE_TOC => MessageType::Toc,
            ffi::GST_MESSAGE_RESET_TIME => MessageType::ResetTime,
            ffi::GST_MESSAGE_STREAM_START => MessageType::StreamStart,
            ffi::GST_MESSAGE_NEED_CONTEXT => MessageType::NeedContext,
            ffi::GST_MESSAGE_HAVE_CONTEXT => MessageType::HaveContext,
            ffi::GST_MESSAGE_EXTENDED => MessageType::Extended,
            ffi::GST_MESSAGE_DEVICE_ADDED => MessageType::DeviceAdded,
            ffi::GST_MESSAGE_DEVICE_REMOVED => MessageType::DeviceRemoved,
            ffi::GST_MESSAGE_PROPERTY_NOTIFY => MessageType::PropertyNotify,
            ffi::GST_MESSAGE_STREAM_COLLECTION => MessageType::StreamCollection,
            ffi::GST_MESSAGE_STREAMS_SELECTED => MessageType::StreamsSelected,
            ffi::GST_MESSAGE_REDIRECT => MessageType::Redirect,
            value => MessageType::__Unknown(value as i32),
        }
    }
}

impl StaticType for MessageType {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_message_type_get_type()) }
    }
}

impl glib::value::ValueType for MessageType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for MessageType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0) as ffi::GstMessageType)
    }
}

impl ToValue for MessageType {
    #[inline]
    fn to_value(&self) -> Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib() as i32)
        }
        value
    }

    #[inline]
    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl From<MessageType> for glib::Value {
    #[inline]
    fn from(v: MessageType) -> glib::Value {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

impl State {
    #[must_use]
    #[inline]
    pub fn next(self, pending: Self) -> Self {
        let current = self.into_glib();
        let pending = pending.into_glib();

        let sign = (pending - current).signum();

        unsafe { from_glib(current + sign) }
    }
}

impl StateChange {
    #[inline]
    pub fn new(current: State, next: State) -> Self {
        skip_assert_initialized!();
        let current = current.into_glib();
        let next = next.into_glib();
        unsafe { from_glib((current << 3) | next) }
    }

    #[inline]
    pub fn current(self) -> State {
        unsafe { from_glib(self.into_glib() >> 3) }
    }

    #[inline]
    pub fn next(self) -> State {
        unsafe { from_glib(self.into_glib() & 0x7) }
    }
}
