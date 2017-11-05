// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use StateChangeReturn;
use FlowReturn;
use PadLinkReturn;
use ClockReturn;

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
        StateChangeReturn::Failure
    }

    pub fn from_ok(v: StateChangeSuccess) -> Self {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[must_use]
pub struct StateChangeError;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[must_use]
pub enum FlowError {
    NotLinked,
    Flushing,
    Eos,
    NotNegotiated,
    Error,
    NotSupported,
    CustomError,
    CustomError1,
    CustomError2,
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
        PadLinkReturn::Ok
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PadLinkSuccess;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[must_use]
pub enum PadLinkError {
    WrongHierarchy,
    WasLinked,
    WrongDirection,
    Noformat,
    Nosched,
    Refused,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[must_use]
pub enum ClockError {
    Early,
    Unscheduled,
    Busy,
    Badtime,
    Error,
    Unsupported,
}
