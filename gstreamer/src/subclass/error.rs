// Take a look at the license at the top of the repository in the LICENSE file.

use thiserror::Error;

use crate::{prelude::ElementExt, ErrorMessage, FlowReturn};

#[doc(hidden)]
#[inline(never)]
pub fn post_panic_error_message(
    element: &crate::Element,
    src: &crate::Object,
    panic: Option<Box<dyn std::any::Any + Send + 'static>>,
) {
    let cause = panic.as_ref().and_then(|err| {
        err.downcast_ref::<&str>()
            .copied()
            .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
    });

    let msg = if let Some(cause) = cause {
        crate::message::Error::builder(crate::LibraryError::Failed, &format!("Panicked: {}", cause))
            .src(src)
            .build()
    } else {
        crate::message::Error::builder(crate::LibraryError::Failed, "Panicked")
            .src(src)
            .build()
    };

    let _ = element.post_message(msg);
}

#[macro_export]
macro_rules! panic_to_error(
    ($imp:expr, $ret:expr, $code:block) => {{
        #[allow(clippy::unused_unit)]
        {
            let panicked = $imp.panicked();
            let element = $crate::glib::subclass::types::ObjectSubclassExt::obj($imp);
            let element = unsafe { $crate::glib::Cast::unsafe_cast_ref::<$crate::Element>(element.as_ref()) };
            if panicked.load(std::sync::atomic::Ordering::Relaxed) {
                $crate::subclass::post_panic_error_message(
                    element,
                    $crate::glib::Cast::upcast_ref::<$crate::Object>(element),
                    None,
                );
                $ret
            } else {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $code));

                match result {
                    Ok(result) => result,
                    Err(err) => {
                        panicked.store(true, std::sync::atomic::Ordering::Relaxed);
                        $crate::subclass::post_panic_error_message(
                            element,
                            $crate::glib::Cast::upcast_ref::<$crate::Object>(element),
                            Some(err),
                        );
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
