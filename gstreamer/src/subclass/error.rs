// Take a look at the license at the top of the repository in the LICENSE file.

use thiserror::Error;

use crate::ErrorMessage;
use crate::FlowReturn;

#[macro_export]
macro_rules! panic_to_error(
    ($imp:expr, $ret:expr, $code:block) => {{
        use std::panic::{self, AssertUnwindSafe};
        use std::sync::atomic::Ordering;

        #[allow(clippy::unused_unit)]
        {
            if $imp.panicked().load(Ordering::Relaxed) {
                $imp.post_error_message($crate::error_msg!($crate::LibraryError::Failed, ["Panicked"]));
                $ret
            } else {
                let result = panic::catch_unwind(AssertUnwindSafe(|| $code));

                match result {
                    Ok(result) => result,
                    Err(err) => {
                        $imp.panicked().store(true, Ordering::Relaxed);
                        if let Some(cause) = err.downcast_ref::<&str>() {
                            $imp.post_error_message($crate::error_msg!($crate::LibraryError::Failed, ["Panicked: {}", cause]));
                        } else if let Some(cause) = err.downcast_ref::<String>() {
                            $imp.post_error_message($crate::error_msg!($crate::LibraryError::Failed, ["Panicked: {}", cause]));
                        } else {
                            $imp.post_error_message($crate::error_msg!($crate::LibraryError::Failed, ["Panicked"]));
                        }
                        $ret
                    }
                }
            }
        }
    }};
);

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum FlowError {
    #[error("Flushing")]
    Flushing,
    #[error("Eos")]
    Eos,
    #[error("Not Negotiated")]
    NotNegotiated(ErrorMessage),
    #[error("Error")]
    Error(ErrorMessage),
}

impl From<FlowError> for FlowReturn {
    fn from(err: FlowError) -> Self {
        FlowReturn::from(&err)
    }
}

impl<'a> From<&'a FlowError> for FlowReturn {
    fn from(err: &FlowError) -> FlowReturn {
        match *err {
            FlowError::Flushing => FlowReturn::Flushing,
            FlowError::Eos => FlowReturn::Eos,
            FlowError::NotNegotiated(..) => FlowReturn::NotNegotiated,
            FlowError::Error(..) => FlowReturn::Error,
        }
    }
}
