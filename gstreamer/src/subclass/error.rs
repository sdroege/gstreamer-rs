// Copyright (C) 2016-2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use thiserror::Error;

use ErrorMessage;
use FlowReturn;

#[macro_export]
macro_rules! gst_panic_to_error(
    ($element:expr, $panicked:expr, $ret:expr, $code:block) => {{
        use std::panic::{self, AssertUnwindSafe};
        use std::sync::atomic::Ordering;
        use $crate::ElementExtManual;

        #[allow(clippy::unused_unit)]
        {
            if $panicked.load(Ordering::Relaxed) {
                $element.post_error_message($crate::gst_error_msg!($crate::LibraryError::Failed, ["Panicked"]));
                $ret
            } else {
                let result = panic::catch_unwind(AssertUnwindSafe(|| $code));

                match result {
                    Ok(result) => result,
                    Err(err) => {
                        $panicked.store(true, Ordering::Relaxed);
                        if let Some(cause) = err.downcast_ref::<&str>() {
                            $element.post_error_message($crate::gst_error_msg!($crate::LibraryError::Failed, ["Panicked: {}", cause]));
                        } else if let Some(cause) = err.downcast_ref::<String>() {
                            $element.post_error_message($crate::gst_error_msg!($crate::LibraryError::Failed, ["Panicked: {}", cause]));
                        } else {
                            $element.post_error_message($crate::gst_error_msg!($crate::LibraryError::Failed, ["Panicked"]));
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
