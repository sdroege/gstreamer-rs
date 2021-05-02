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

impl From<StateChangeSuccess> for StateChangeReturn {
    fn from(value: StateChangeSuccess) -> Self {
        skip_assert_initialized!();
        StateChangeReturn::from_ok(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
#[must_use]
#[error("Element failed to change its state")]
pub struct StateChangeError;

impl From<StateChangeError> for StateChangeReturn {
    fn from(value: StateChangeError) -> Self {
        skip_assert_initialized!();
        StateChangeReturn::from_error(value)
    }
}

impl From<Result<StateChangeSuccess, StateChangeError>> for StateChangeReturn {
    fn from(res: Result<StateChangeSuccess, StateChangeError>) -> Self {
        skip_assert_initialized!();
        match res {
            Ok(success) => StateChangeReturn::from_ok(success),
            Err(error) => StateChangeReturn::from_error(error),
        }
    }
}

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

    pub fn into_result_value<T, F: FnOnce() -> T>(self, func: F) -> Result<T, FlowError> {
        match self.into_result() {
            Ok(_) => Ok(func()),
            Err(err) => Err(err),
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

impl From<FlowSuccess> for FlowReturn {
    fn from(value: FlowSuccess) -> Self {
        skip_assert_initialized!();
        FlowReturn::from_ok(value)
    }
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

impl From<FlowError> for FlowReturn {
    fn from(value: FlowError) -> Self {
        skip_assert_initialized!();
        FlowReturn::from_error(value)
    }
}

impl From<Result<FlowSuccess, FlowError>> for FlowReturn {
    fn from(res: Result<FlowSuccess, FlowError>) -> Self {
        skip_assert_initialized!();
        match res {
            Ok(success) => FlowReturn::from_ok(success),
            Err(error) => FlowReturn::from_error(error),
        }
    }
}

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

impl From<PadLinkSuccess> for PadLinkReturn {
    fn from(value: PadLinkSuccess) -> Self {
        skip_assert_initialized!();
        PadLinkReturn::from_ok(value)
    }
}

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

impl From<PadLinkError> for PadLinkReturn {
    fn from(value: PadLinkError) -> Self {
        skip_assert_initialized!();
        PadLinkReturn::from_error(value)
    }
}

impl From<Result<PadLinkSuccess, PadLinkError>> for PadLinkReturn {
    fn from(res: Result<PadLinkSuccess, PadLinkError>) -> Self {
        skip_assert_initialized!();
        match res {
            Ok(success) => PadLinkReturn::from_ok(success),
            Err(error) => PadLinkReturn::from_error(error),
        }
    }
}

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

impl From<ClockSuccess> for ClockReturn {
    fn from(value: ClockSuccess) -> Self {
        skip_assert_initialized!();
        ClockReturn::from_ok(value)
    }
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

impl From<ClockError> for ClockReturn {
    fn from(value: ClockError) -> Self {
        skip_assert_initialized!();
        ClockReturn::from_error(value)
    }
}

impl From<Result<ClockSuccess, ClockError>> for ClockReturn {
    fn from(res: Result<ClockSuccess, ClockError>) -> Self {
        skip_assert_initialized!();
        match res {
            Ok(success) => ClockReturn::from_ok(success),
            Err(error) => ClockReturn::from_error(error),
        }
    }
}

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
    Unknown,
    Eos,
    Error,
    Warning,
    Info,
    Tag,
    Buffering,
    StateChanged,
    StateDirty,
    StepDone,
    ClockProvide,
    ClockLost,
    NewClock,
    StructureChange,
    StreamStatus,
    Application,
    Element,
    SegmentStart,
    SegmentDone,
    DurationChanged,
    Latency,
    AsyncStart,
    AsyncDone,
    RequestState,
    StepStart,
    Qos,
    Progress,
    Toc,
    ResetTime,
    StreamStart,
    NeedContext,
    HaveContext,
    Extended,
    DeviceAdded,
    DeviceRemoved,
    PropertyNotify,
    StreamCollection,
    StreamsSelected,
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
            0 => MessageType::Unknown,
            1 => MessageType::Eos,
            2 => MessageType::Error,
            4 => MessageType::Warning,
            8 => MessageType::Info,
            16 => MessageType::Tag,
            32 => MessageType::Buffering,
            64 => MessageType::StateChanged,
            128 => MessageType::StateDirty,
            256 => MessageType::StepDone,
            512 => MessageType::ClockProvide,
            1024 => MessageType::ClockLost,
            2048 => MessageType::NewClock,
            4096 => MessageType::StructureChange,
            8192 => MessageType::StreamStatus,
            16384 => MessageType::Application,
            32768 => MessageType::Element,
            65536 => MessageType::SegmentStart,
            131072 => MessageType::SegmentDone,
            262144 => MessageType::DurationChanged,
            524288 => MessageType::Latency,
            1048576 => MessageType::AsyncStart,
            2097152 => MessageType::AsyncDone,
            4194304 => MessageType::RequestState,
            8388608 => MessageType::StepStart,
            16777216 => MessageType::Qos,
            33554432 => MessageType::Progress,
            67108864 => MessageType::Toc,
            134217728 => MessageType::ResetTime,
            268435456 => MessageType::StreamStart,
            536870912 => MessageType::NeedContext,
            1073741824 => MessageType::HaveContext,
            2147483648 => MessageType::Extended,
            2147483649 => MessageType::DeviceAdded,
            2147483650 => MessageType::DeviceRemoved,
            2147483651 => MessageType::PropertyNotify,
            2147483652 => MessageType::StreamCollection,
            2147483653 => MessageType::StreamsSelected,
            2147483654 => MessageType::Redirect,
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
