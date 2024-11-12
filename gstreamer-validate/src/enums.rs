// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ffi;
use glib::translate::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum ActionSuccess {
    Ok = ffi::GST_VALIDATE_EXECUTE_ACTION_OK,
    Async = ffi::GST_VALIDATE_EXECUTE_ACTION_ASYNC,
    NonBlocking = ffi::GST_VALIDATE_EXECUTE_ACTION_NON_BLOCKING,
    InProgress = ffi::GST_VALIDATE_EXECUTE_ACTION_IN_PROGRESS,
    Done = ffi::GST_VALIDATE_EXECUTE_ACTION_DONE,
}

impl ActionSuccess {
    pub fn from_value(value: impl Into<i32>) -> Option<Self> {
        skip_assert_initialized!();
        Some(match value.into() {
            ffi::GST_VALIDATE_EXECUTE_ACTION_OK => ActionSuccess::Ok,
            ffi::GST_VALIDATE_EXECUTE_ACTION_ASYNC => ActionSuccess::Async,
            ffi::GST_VALIDATE_EXECUTE_ACTION_NON_BLOCKING => ActionSuccess::NonBlocking,
            ffi::GST_VALIDATE_EXECUTE_ACTION_IN_PROGRESS => ActionSuccess::InProgress,
            ffi::GST_VALIDATE_EXECUTE_ACTION_DONE => ActionSuccess::Done,
            _ => return None,
        })
    }
}

impl IntoGlib for ActionSuccess {
    type GlibType = ffi::GstValidateActionReturn;

    #[inline]
    fn into_glib(self) -> ffi::GstValidateActionReturn {
        skip_assert_initialized!();
        self as ffi::GstValidateActionReturn
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum ActionError {
    Error(String),
}

impl ActionError {
    pub fn from_value(value: impl Into<i32>) -> Self {
        skip_assert_initialized!();
        match value.into() {
            ffi::GST_VALIDATE_EXECUTE_ACTION_ERROR => {
                ActionError::Error("Execution failed".to_string())
            }
            _ => ActionError::Error("Unknown error".to_string()),
        }
    }
}

impl IntoGlib for ActionError {
    type GlibType = ffi::GstValidateActionReturn;

    #[inline]
    fn into_glib(self) -> ffi::GstValidateActionReturn {
        match self {
            ActionError::Error(_) => ffi::GST_VALIDATE_EXECUTE_ACTION_ERROR,
        }
    }
}

#[must_use]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[doc(alias = "GstValidateActionReturn")]
#[repr(i32)]
pub enum ActionReturn {
    #[doc(alias = "GST_VALIDATE_EXECUTE_ACTION_ERROR")]
    Error = ffi::GST_VALIDATE_EXECUTE_ACTION_ERROR,
    #[doc(alias = "GST_VALIDATE_EXECUTE_ACTION_OK")]
    Ok = ffi::GST_VALIDATE_EXECUTE_ACTION_OK,
    #[doc(alias = "GST_VALIDATE_EXECUTE_ACTION_ASYNC")]
    Async = ffi::GST_VALIDATE_EXECUTE_ACTION_ASYNC,
    #[doc(alias = "GST_VALIDATE_EXECUTE_ACTION_NON_BLOCKING")]
    NonBlocking = ffi::GST_VALIDATE_EXECUTE_ACTION_NON_BLOCKING,
    #[doc(alias = "GST_VALIDATE_EXECUTE_ACTION_ERROR_REPORTED")]
    ErrorReported = ffi::GST_VALIDATE_EXECUTE_ACTION_ERROR_REPORTED,
    #[doc(alias = "GST_VALIDATE_EXECUTE_ACTION_IN_PROGRESS")]
    InProgress = ffi::GST_VALIDATE_EXECUTE_ACTION_IN_PROGRESS,
    #[doc(alias = "GST_VALIDATE_EXECUTE_ACTION_NONE")]
    None = ffi::GST_VALIDATE_EXECUTE_ACTION_NONE,
    #[doc(alias = "GST_VALIDATE_EXECUTE_ACTION_DONE")]
    Done = ffi::GST_VALIDATE_EXECUTE_ACTION_DONE,
}

#[doc(hidden)]
impl IntoGlib for ActionReturn {
    type GlibType = ffi::GstValidateActionReturn;

    #[inline]
    fn into_glib(self) -> ffi::GstValidateActionReturn {
        self as ffi::GstValidateActionReturn
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstValidateActionReturn> for ActionReturn {
    #[inline]
    unsafe fn from_glib(value: ffi::GstValidateActionReturn) -> Self {
        skip_assert_initialized!();

        if !(ffi::GST_VALIDATE_EXECUTE_ACTION_ERROR..=ffi::GST_VALIDATE_EXECUTE_ACTION_DONE)
            .contains(&value)
        {
            ActionReturn::Error
        } else {
            std::mem::transmute::<i32, ActionReturn>(value)
        }
    }
}

impl TryFromGlib<ffi::GstValidateActionReturn> for ActionSuccess {
    type Error = ActionError;

    #[inline]
    unsafe fn try_from_glib(
        val: ffi::GstValidateActionReturn,
    ) -> Result<ActionSuccess, ActionError> {
        skip_assert_initialized!();
        ActionReturn::from_glib(val).into_result()
    }
}

impl ActionReturn {
    #[inline]
    pub fn into_result(self) -> Result<ActionSuccess, ActionError> {
        match self {
            Self::Error | Self::ErrorReported => {
                Err(ActionError::Error("Execution failed".to_string()))
            }
            _ => Ok(unsafe { std::mem::transmute::<ActionReturn, ActionSuccess>(self) }),
        }
    }

    #[inline]
    pub fn from_error(v: ActionError) -> Self {
        skip_assert_initialized!();

        match v {
            ActionError::Error(_) => Self::Error,
        }
    }

    #[inline]
    pub fn from_ok(v: ActionSuccess) -> Self {
        skip_assert_initialized!();
        unsafe { std::mem::transmute::<ActionSuccess, ActionReturn>(v) }
    }
}
