// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ClockReturn;
use crate::FlowReturn;
use crate::PadLinkReturn;
use crate::State;
use crate::StateChange;
use crate::StateChangeReturn;
use std::{cmp, ops};
use thiserror::Error;

use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::value::Value;
use glib::StaticType;
use glib::Type;

macro_rules! impl_return_result_traits {
    ($ffi_type:ident, $ret_type:ident, $ok_type:ident, $err_type:ident) => {
        impl From<$ok_type> for $ret_type {
            fn from(value: $ok_type) -> Self {
                skip_assert_initialized!();
                $ret_type::from_ok(value)
            }
        }

        impl IntoGlib for $ok_type {
            type GlibType = <$ret_type as IntoGlib>::GlibType;

            fn into_glib(self) -> Self::GlibType {
                $ret_type::from_ok(self).into_glib()
            }
        }

        impl From<$err_type> for $ret_type {
            fn from(value: $err_type) -> Self {
                skip_assert_initialized!();
                $ret_type::from_error(value)
            }
        }

        impl IntoGlib for $err_type {
            type GlibType = <$ret_type as IntoGlib>::GlibType;

            fn into_glib(self) -> Self::GlibType {
                $ret_type::from_error(self).into_glib()
            }
        }

        impl From<Result<$ok_type, $err_type>> for $ret_type {
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
            unsafe fn try_from_glib(val: ffi::$ffi_type) -> Result<$ok_type, $err_type> {
                skip_assert_initialized!();
                $ret_type::from_glib(val).into_result()
            }
        }
    };
}

impl StateChangeReturn {
    pub fn into_result(self) -> Result<StateChangeSuccess, StateChangeError> {
        match self {
            StateChangeReturn::Success => Ok(StateChangeSuccess::Success),
            StateChangeReturn::Async => Ok(StateChangeSuccess::Async),
            StateChangeReturn::NoPreroll => Ok(StateChangeSuccess::NoPreroll),
            StateChangeReturn::Failure => Err(StateChangeError),
            _ => Err(StateChangeError),
        }
    }

    pub fn from_error(_: StateChangeError) -> Self {
        skip_assert_initialized!();
        StateChangeReturn::Failure
    }

    pub fn from_ok(v: StateChangeSuccess) -> Self {
        skip_assert_initialized!();
        match v {
            StateChangeSuccess::Success => StateChangeReturn::Success,
            StateChangeSuccess::Async => StateChangeReturn::Async,
            StateChangeSuccess::NoPreroll => StateChangeReturn::NoPreroll,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum StateChangeSuccess {
    Success,
    Async,
    NoPreroll,
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

impl FlowReturn {
    pub fn into_result(self) -> Result<FlowSuccess, FlowError> {
        match self {
            FlowReturn::CustomSuccess2 => Ok(FlowSuccess::CustomSuccess2),
            FlowReturn::CustomSuccess1 => Ok(FlowSuccess::CustomSuccess1),
            FlowReturn::CustomSuccess => Ok(FlowSuccess::CustomSuccess),
            FlowReturn::Ok => Ok(FlowSuccess::Ok),
            FlowReturn::NotLinked => Err(FlowError::NotLinked),
            FlowReturn::Flushing => Err(FlowError::Flushing),
            FlowReturn::Eos => Err(FlowError::Eos),
            FlowReturn::NotNegotiated => Err(FlowError::NotNegotiated),
            FlowReturn::Error => Err(FlowError::Error),
            FlowReturn::NotSupported => Err(FlowError::NotSupported),
            FlowReturn::CustomError => Err(FlowError::CustomError),
            FlowReturn::CustomError1 => Err(FlowError::CustomError1),
            FlowReturn::CustomError2 => Err(FlowError::CustomError2),
            _ => Err(FlowError::Error),
        }
    }

    pub fn from_error(v: FlowError) -> Self {
        skip_assert_initialized!();
        match v {
            FlowError::NotLinked => FlowReturn::NotLinked,
            FlowError::Flushing => FlowReturn::Flushing,
            FlowError::Eos => FlowReturn::Eos,
            FlowError::NotNegotiated => FlowReturn::NotNegotiated,
            FlowError::Error => FlowReturn::Error,
            FlowError::NotSupported => FlowReturn::NotSupported,
            FlowError::CustomError => FlowReturn::CustomError,
            FlowError::CustomError1 => FlowReturn::CustomError1,
            FlowError::CustomError2 => FlowReturn::CustomError2,
        }
    }

    pub fn from_ok(v: FlowSuccess) -> Self {
        skip_assert_initialized!();
        match v {
            FlowSuccess::CustomSuccess2 => FlowReturn::CustomSuccess2,
            FlowSuccess::CustomSuccess1 => FlowReturn::CustomSuccess1,
            FlowSuccess::CustomSuccess => FlowReturn::CustomSuccess,
            FlowSuccess::Ok => FlowReturn::Ok,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum FlowSuccess {
    CustomSuccess2,
    CustomSuccess1,
    CustomSuccess,
    Ok,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
#[must_use]
pub enum FlowError {
    #[error("Pad is not linked")]
    NotLinked,
    #[error("Pad is flushing")]
    Flushing,
    #[error("Pad is EOS")]
    Eos,
    #[error("Pad is not negotiated")]
    NotNegotiated,
    #[error("Some (fatal) error occurred. Element generating this error should post an error message with more details")]
    Error,
    #[error("This operation is not supported")]
    NotSupported,
    #[error("Elements can use values starting from this (and lower) to define custom error codes")]
    CustomError,
    #[error("Pre-defined custom error code")]
    CustomError1,
    #[error("Pre-defined custom error code")]
    CustomError2,
}

impl_return_result_traits!(GstFlowReturn, FlowReturn, FlowSuccess, FlowError);

impl PadLinkReturn {
    pub fn into_result(self) -> Result<PadLinkSuccess, PadLinkError> {
        match self {
            PadLinkReturn::Ok => Ok(PadLinkSuccess),
            PadLinkReturn::WrongHierarchy => Err(PadLinkError::WrongHierarchy),
            PadLinkReturn::WasLinked => Err(PadLinkError::WasLinked),
            PadLinkReturn::WrongDirection => Err(PadLinkError::WrongDirection),
            PadLinkReturn::Noformat => Err(PadLinkError::Noformat),
            PadLinkReturn::Nosched => Err(PadLinkError::Nosched),
            PadLinkReturn::Refused => Err(PadLinkError::Refused),
            _ => Err(PadLinkError::Refused),
        }
    }

    pub fn from_error(v: PadLinkError) -> Self {
        skip_assert_initialized!();
        match v {
            PadLinkError::WrongHierarchy => PadLinkReturn::WrongHierarchy,
            PadLinkError::WasLinked => PadLinkReturn::WasLinked,
            PadLinkError::WrongDirection => PadLinkReturn::WrongDirection,
            PadLinkError::Noformat => PadLinkReturn::Noformat,
            PadLinkError::Nosched => PadLinkReturn::Nosched,
            PadLinkError::Refused => PadLinkReturn::Refused,
        }
    }

    pub fn from_ok(_: PadLinkSuccess) -> Self {
        skip_assert_initialized!();
        PadLinkReturn::Ok
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PadLinkSuccess;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
#[must_use]
pub enum PadLinkError {
    #[error("Pads have no common grandparent")]
    WrongHierarchy,
    #[error("Pad was already linked")]
    WasLinked,
    #[error("Pads have wrong direction")]
    WrongDirection,
    #[error("Pads do not have common format")]
    Noformat,
    #[error("Pads cannot cooperate in scheduling")]
    Nosched,
    #[error("Refused for some other reason")]
    Refused,
}

impl_return_result_traits!(
    GstPadLinkReturn,
    PadLinkReturn,
    PadLinkSuccess,
    PadLinkError
);

impl ClockReturn {
    pub fn into_result(self) -> Result<ClockSuccess, ClockError> {
        match self {
            ClockReturn::Ok => Ok(ClockSuccess::Ok),
            ClockReturn::Done => Ok(ClockSuccess::Done),
            ClockReturn::Early => Err(ClockError::Early),
            ClockReturn::Unscheduled => Err(ClockError::Unscheduled),
            ClockReturn::Busy => Err(ClockError::Busy),
            ClockReturn::Badtime => Err(ClockError::Badtime),
            ClockReturn::Error => Err(ClockError::Error),
            ClockReturn::Unsupported => Err(ClockError::Unsupported),
            _ => Err(ClockError::Error),
        }
    }

    pub fn from_error(v: ClockError) -> Self {
        skip_assert_initialized!();
        match v {
            ClockError::Early => ClockReturn::Early,
            ClockError::Unscheduled => ClockReturn::Unscheduled,
            ClockError::Busy => ClockReturn::Busy,
            ClockError::Badtime => ClockReturn::Badtime,
            ClockError::Error => ClockReturn::Error,
            ClockError::Unsupported => ClockReturn::Unsupported,
        }
    }

    pub fn from_ok(v: ClockSuccess) -> Self {
        skip_assert_initialized!();
        match v {
            ClockSuccess::Ok => ClockReturn::Ok,
            ClockSuccess::Done => ClockReturn::Done,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ClockSuccess {
    Ok,
    Done,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
#[must_use]
pub enum ClockError {
    #[error("The operation was scheduled too late")]
    Early,
    #[error("The clockID was unscheduled")]
    Unscheduled,
    #[error("The ClockID is busy")]
    Busy,
    #[error("A bad time was provided to a function")]
    Badtime,
    #[error("An error occurred")]
    Error,
    #[error("Operation is not supported")]
    Unsupported,
}

impl_return_result_traits!(GstClockReturn, ClockReturn, ClockSuccess, ClockError);

impl PartialEq for crate::TypeFindProbability {
    fn eq(&self, other: &crate::TypeFindProbability) -> bool {
        (self.into_glib() as u32).eq(&(other.into_glib() as u32))
    }
}

impl Eq for crate::TypeFindProbability {}

impl PartialOrd for crate::TypeFindProbability {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        (self.into_glib() as u32).partial_cmp(&(other.into_glib() as u32))
    }
}

impl Ord for crate::TypeFindProbability {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (self.into_glib() as u32).cmp(&(other.into_glib() as u32))
    }
}

impl ops::Add<u32> for crate::TypeFindProbability {
    type Output = crate::TypeFindProbability;

    fn add(self, rhs: u32) -> crate::TypeFindProbability {
        let res = (self.into_glib() as u32).saturating_add(rhs);
        unsafe { from_glib(res as i32) }
    }
}

impl ops::AddAssign<u32> for crate::TypeFindProbability {
    fn add_assign(&mut self, rhs: u32) {
        let res = (self.into_glib() as u32).saturating_add(rhs);
        *self = unsafe { from_glib(res as i32) };
    }
}

impl ops::Sub<u32> for crate::TypeFindProbability {
    type Output = crate::TypeFindProbability;

    fn sub(self, rhs: u32) -> crate::TypeFindProbability {
        let res = (self.into_glib() as u32).saturating_sub(rhs);
        unsafe { from_glib(res as i32) }
    }
}

impl ops::SubAssign<u32> for crate::TypeFindProbability {
    fn sub_assign(&mut self, rhs: u32) {
        let res = (self.into_glib() as u32).saturating_sub(rhs);
        *self = unsafe { from_glib(res as i32) };
    }
}

impl PartialEq for crate::Rank {
    fn eq(&self, other: &crate::Rank) -> bool {
        (self.into_glib() as u32).eq(&(other.into_glib() as u32))
    }
}

impl Eq for crate::Rank {}

impl PartialOrd for crate::Rank {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        (self.into_glib() as u32).partial_cmp(&(other.into_glib() as u32))
    }
}

impl Ord for crate::Rank {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (self.into_glib() as u32).cmp(&(other.into_glib() as u32))
    }
}

impl ops::Add<u32> for crate::Rank {
    type Output = crate::Rank;

    fn add(self, rhs: u32) -> crate::Rank {
        let res = (self.into_glib() as u32).saturating_add(rhs);
        unsafe { from_glib(res as i32) }
    }
}

impl ops::AddAssign<u32> for crate::Rank {
    fn add_assign(&mut self, rhs: u32) {
        let res = (self.into_glib() as u32).saturating_add(rhs);
        *self = unsafe { from_glib(res as i32) };
    }
}

impl ops::Sub<u32> for crate::Rank {
    type Output = crate::Rank;

    fn sub(self, rhs: u32) -> crate::Rank {
        let res = (self.into_glib() as u32).saturating_sub(rhs);
        unsafe { from_glib(res as i32) }
    }
}

impl ops::SubAssign<u32> for crate::Rank {
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
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_message_type_get_type()) }
    }
}

impl glib::value::ValueType for MessageType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for MessageType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0) as ffi::GstMessageType)
    }
}

impl ToValue for MessageType {
    fn to_value(&self) -> Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib() as i32)
        }
        value
    }

    fn value_type(&self) -> Type {
        Self::static_type()
    }
}

impl State {
    pub fn next(self, pending: Self) -> Self {
        let current = self.into_glib();
        let pending = pending.into_glib();

        let sign = (pending - current).signum();

        unsafe { from_glib(current + sign) }
    }
}

impl StateChange {
    pub fn new(current: State, next: State) -> Self {
        skip_assert_initialized!();
        let current = current.into_glib();
        let next = next.into_glib();
        unsafe { from_glib((current << 3) | next) }
    }

    pub fn current(self) -> State {
        match self {
            StateChange::NullToReady => State::Null,
            StateChange::ReadyToPaused => State::Ready,
            StateChange::PausedToPlaying => State::Paused,
            StateChange::PlayingToPaused => State::Playing,
            StateChange::PausedToReady => State::Paused,
            StateChange::ReadyToNull => State::Ready,
            StateChange::NullToNull => State::Null,
            StateChange::ReadyToReady => State::Ready,
            StateChange::PausedToPaused => State::Paused,
            StateChange::PlayingToPlaying => State::Playing,
            StateChange::__Unknown(value) => State::__Unknown(value >> 3),
        }
    }

    pub fn next(self) -> State {
        match self {
            StateChange::NullToReady => State::Ready,
            StateChange::ReadyToPaused => State::Paused,
            StateChange::PausedToPlaying => State::Playing,
            StateChange::PlayingToPaused => State::Paused,
            StateChange::PausedToReady => State::Ready,
            StateChange::ReadyToNull => State::Null,
            StateChange::NullToNull => State::Null,
            StateChange::ReadyToReady => State::Ready,
            StateChange::PausedToPaused => State::Paused,
            StateChange::PlayingToPlaying => State::Playing,
            StateChange::__Unknown(value) => State::__Unknown(value & 0x7),
        }
    }

    #[doc(alias = "get_name")]
    #[doc(alias = "gst_state_change_get_name")]
    pub fn name<'a>(self) -> &'a str {
        cfg_if::cfg_if! {
            if #[cfg(feature = "v1_14")] {
                // This implementation is autogenerated on 1.14 and up
                use std::ffi::CStr;
                unsafe {
                    CStr::from_ptr(
                        ffi::gst_state_change_get_name(self.into_glib())
                            .as_ref()
                            .expect("gst_state_change_get_name returned NULL"),
                    )
                    .to_str()
                    .expect("gst_state_change_get_name returned an invalid string")
                }
            } else {
                match self {
                    Self::NullToReady => "NULL->READY",
                    Self::ReadyToPaused => "READY->PAUSED",
                    Self::PausedToPlaying => "PAUSED->PLAYING",
                    Self::PlayingToPaused => "PLAYING->PAUSED",
                    Self::PausedToReady => "PAUSED->READY",
                    Self::ReadyToNull => "READY->NULL",
                    Self::NullToNull => "NULL->NULL",
                    Self::ReadyToReady => "READY->READY",
                    Self::PausedToPaused => "PAUSED->PAUSED",
                    Self::PlayingToPlaying => "PLAYING->PLAYING",
                    _ => "Unknown state return",
                }
            }
        }
    }
}
