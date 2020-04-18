// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{cmp, ops};
use thiserror::Error;
use ClockReturn;
use FlowReturn;
use PadLinkReturn;
use State;
use StateChange;
use StateChangeReturn;

use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
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

impl PartialEq for ::TypeFindProbability {
    fn eq(&self, other: &::TypeFindProbability) -> bool {
        (self.to_glib() as u32).eq(&(other.to_glib() as u32))
    }
}

impl Eq for ::TypeFindProbability {}

impl PartialOrd for ::TypeFindProbability {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        (self.to_glib() as u32).partial_cmp(&(other.to_glib() as u32))
    }
}

impl Ord for ::TypeFindProbability {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (self.to_glib() as u32).cmp(&(other.to_glib() as u32))
    }
}

impl ops::Add<u32> for ::TypeFindProbability {
    type Output = ::TypeFindProbability;

    fn add(self, rhs: u32) -> ::TypeFindProbability {
        let res = (self.to_glib() as u32).saturating_add(rhs);
        from_glib(res as i32)
    }
}

impl ops::AddAssign<u32> for ::TypeFindProbability {
    fn add_assign(&mut self, rhs: u32) {
        let res = (self.to_glib() as u32).saturating_add(rhs);
        *self = from_glib(res as i32);
    }
}

impl ops::Sub<u32> for ::TypeFindProbability {
    type Output = ::TypeFindProbability;

    fn sub(self, rhs: u32) -> ::TypeFindProbability {
        let res = (self.to_glib() as u32).saturating_sub(rhs);
        from_glib(res as i32)
    }
}

impl ops::SubAssign<u32> for ::TypeFindProbability {
    fn sub_assign(&mut self, rhs: u32) {
        let res = (self.to_glib() as u32).saturating_sub(rhs);
        *self = from_glib(res as i32);
    }
}

impl PartialEq for ::Rank {
    fn eq(&self, other: &::Rank) -> bool {
        (self.to_glib() as u32).eq(&(other.to_glib() as u32))
    }
}

impl Eq for ::Rank {}

impl PartialOrd for ::Rank {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        (self.to_glib() as u32).partial_cmp(&(other.to_glib() as u32))
    }
}

impl Ord for ::Rank {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (self.to_glib() as u32).cmp(&(other.to_glib() as u32))
    }
}

impl ops::Add<u32> for ::Rank {
    type Output = ::Rank;

    fn add(self, rhs: u32) -> ::Rank {
        let res = (self.to_glib() as u32).saturating_add(rhs);
        from_glib(res as i32)
    }
}

impl ops::AddAssign<u32> for ::Rank {
    fn add_assign(&mut self, rhs: u32) {
        let res = (self.to_glib() as u32).saturating_add(rhs);
        *self = from_glib(res as i32);
    }
}

impl ops::Sub<u32> for ::Rank {
    type Output = ::Rank;

    fn sub(self, rhs: u32) -> ::Rank {
        let res = (self.to_glib() as u32).saturating_sub(rhs);
        from_glib(res as i32)
    }
}

impl ops::SubAssign<u32> for ::Rank {
    fn sub_assign(&mut self, rhs: u32) {
        let res = (self.to_glib() as u32).saturating_sub(rhs);
        *self = from_glib(res as i32);
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
impl ToGlib for MessageType {
    type GlibType = gst_sys::GstMessageType;

    fn to_glib(&self) -> gst_sys::GstMessageType {
        match *self {
            MessageType::Unknown => gst_sys::GST_MESSAGE_UNKNOWN,
            MessageType::Eos => gst_sys::GST_MESSAGE_EOS,
            MessageType::Error => gst_sys::GST_MESSAGE_ERROR,
            MessageType::Warning => gst_sys::GST_MESSAGE_WARNING,
            MessageType::Info => gst_sys::GST_MESSAGE_INFO,
            MessageType::Tag => gst_sys::GST_MESSAGE_TAG,
            MessageType::Buffering => gst_sys::GST_MESSAGE_BUFFERING,
            MessageType::StateChanged => gst_sys::GST_MESSAGE_STATE_CHANGED,
            MessageType::StateDirty => gst_sys::GST_MESSAGE_STATE_DIRTY,
            MessageType::StepDone => gst_sys::GST_MESSAGE_STEP_DONE,
            MessageType::ClockProvide => gst_sys::GST_MESSAGE_CLOCK_PROVIDE,
            MessageType::ClockLost => gst_sys::GST_MESSAGE_CLOCK_LOST,
            MessageType::NewClock => gst_sys::GST_MESSAGE_NEW_CLOCK,
            MessageType::StructureChange => gst_sys::GST_MESSAGE_STRUCTURE_CHANGE,
            MessageType::StreamStatus => gst_sys::GST_MESSAGE_STREAM_STATUS,
            MessageType::Application => gst_sys::GST_MESSAGE_APPLICATION,
            MessageType::Element => gst_sys::GST_MESSAGE_ELEMENT,
            MessageType::SegmentStart => gst_sys::GST_MESSAGE_SEGMENT_START,
            MessageType::SegmentDone => gst_sys::GST_MESSAGE_SEGMENT_DONE,
            MessageType::DurationChanged => gst_sys::GST_MESSAGE_DURATION_CHANGED,
            MessageType::Latency => gst_sys::GST_MESSAGE_LATENCY,
            MessageType::AsyncStart => gst_sys::GST_MESSAGE_ASYNC_START,
            MessageType::AsyncDone => gst_sys::GST_MESSAGE_ASYNC_DONE,
            MessageType::RequestState => gst_sys::GST_MESSAGE_REQUEST_STATE,
            MessageType::StepStart => gst_sys::GST_MESSAGE_STEP_START,
            MessageType::Qos => gst_sys::GST_MESSAGE_QOS,
            MessageType::Progress => gst_sys::GST_MESSAGE_PROGRESS,
            MessageType::Toc => gst_sys::GST_MESSAGE_TOC,
            MessageType::ResetTime => gst_sys::GST_MESSAGE_RESET_TIME,
            MessageType::StreamStart => gst_sys::GST_MESSAGE_STREAM_START,
            MessageType::NeedContext => gst_sys::GST_MESSAGE_NEED_CONTEXT,
            MessageType::HaveContext => gst_sys::GST_MESSAGE_HAVE_CONTEXT,
            MessageType::Extended => gst_sys::GST_MESSAGE_EXTENDED,
            MessageType::DeviceAdded => gst_sys::GST_MESSAGE_DEVICE_ADDED,
            MessageType::DeviceRemoved => gst_sys::GST_MESSAGE_DEVICE_REMOVED,
            MessageType::PropertyNotify => gst_sys::GST_MESSAGE_PROPERTY_NOTIFY,
            MessageType::StreamCollection => gst_sys::GST_MESSAGE_STREAM_COLLECTION,
            MessageType::StreamsSelected => gst_sys::GST_MESSAGE_STREAMS_SELECTED,
            MessageType::Redirect => gst_sys::GST_MESSAGE_REDIRECT,
            MessageType::__Unknown(value) => value as u32,
        }
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstMessageType> for MessageType {
    #[allow(clippy::unreadable_literal)]
    fn from_glib(value: gst_sys::GstMessageType) -> Self {
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
        unsafe { from_glib(gst_sys::gst_message_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for MessageType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for MessageType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for MessageType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

impl State {
    pub fn next(self, pending: Self) -> Self {
        let current = self.to_glib();
        let pending = pending.to_glib();

        let sign = (pending - current).signum();

        from_glib(current + sign)
    }
}

impl StateChange {
    pub fn new(current: State, next: State) -> Self {
        skip_assert_initialized!();
        let current = current.to_glib();
        let next = next.to_glib();
        from_glib((current << 3) | next)
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
}
