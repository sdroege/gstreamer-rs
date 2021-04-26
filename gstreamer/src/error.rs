// Take a look at the license at the top of the repository in the LICENSE file.

use thiserror::Error;

use glib::prelude::*;

#[macro_export]
macro_rules! error_msg(
// Plain strings
    ($err:expr, ($msg:expr), [$dbg:expr]) =>  {
        $crate::ErrorMessage::new(&$err, Some($msg),
                          Some($dbg),
                          file!(), module_path!(), line!())
    };
    ($err:expr, ($msg:expr)) => {
        $crate::ErrorMessage::new(&$err, Some($msg),
                          None,
                          file!(), module_path!(), line!())
    };
    ($err:expr, [$dbg:expr]) => {
        $crate::ErrorMessage::new(&$err, None,
                          Some($dbg),
                          file!(), module_path!(), line!())
    };

// Format strings
    ($err:expr, ($($msg:tt)*), [$($dbg:tt)*]) =>  { {
        $crate::ErrorMessage::new(&$err, Some(format!($($msg)*).as_ref()),
                          Some(format!($($dbg)*).as_ref()),
                          file!(), module_path!(), line!())
    }};
    ($err:expr, ($($msg:tt)*)) =>  { {
        $crate::ErrorMessage::new(&$err, Some(format!($($msg)*).as_ref()),
                          None,
                          file!(), module_path!(), line!())
    }};

    ($err:expr, [$($dbg:tt)*]) =>  { {
        $crate::ErrorMessage::new(&$err, None,
                          Some(format!($($dbg)*).as_ref()),
                          file!(), module_path!(), line!())
    }};
);

#[derive(Clone, Debug, PartialEq, Eq, Error)]
#[error("Error {:?} ({:?}) at {}:{}", .message, .debug, .filename, .line)]
pub struct ErrorMessage {
    pub(crate) error_domain: glib::Quark,
    pub(crate) error_code: i32,
    pub(crate) message: Option<String>,
    pub(crate) debug: Option<String>,
    pub(crate) filename: &'static str,
    pub(crate) function: &'static str,
    pub(crate) line: u32,
}

impl ErrorMessage {
    pub fn new<T: crate::MessageErrorDomain>(
        error: &T,
        message: Option<&str>,
        debug: Option<&str>,
        filename: &'static str,
        function: &'static str,
        line: u32,
    ) -> ErrorMessage {
        assert_initialized_main_thread!();
        let error_domain = T::domain();
        let error_code = error.code();

        ErrorMessage {
            error_domain,
            error_code,
            message: message.map(String::from),
            debug: debug.map(String::from),
            filename,
            function,
            line,
        }
    }
}

#[macro_export]
macro_rules! loggable_error(
// Plain strings
    ($cat:expr, $msg:expr) => {
        $crate::LoggableError::new($cat.clone(), $crate::glib::bool_error!($msg))
    };

// Format strings
    ($cat:expr, $($msg:tt)*) =>  { {
        $crate::LoggableError::new($cat.clone(), $crate::glib::bool_error!($($msg)*))
    }};
);

#[macro_export]
macro_rules! result_from_gboolean(
// Plain strings
    ($ffi_bool:expr, $cat:expr, $msg:expr) =>  {
        $crate::glib::result_from_gboolean!($ffi_bool, $msg)
            .map_err(|bool_err| $crate::LoggableError::new($cat.clone(), bool_err))
    };

// Format strings
    ($ffi_bool:expr, $cat:expr, $($msg:tt)*) =>  { {
        $crate::glib::result_from_gboolean!($ffi_bool, $($msg)*)
            .map_err(|bool_err| $crate::LoggableError::new($cat.clone(), bool_err))
    }};
);

#[derive(Debug, Clone, Error)]
#[error("Error {:?}: {:?} at {}:{}", .category.name(), .bool_error.message, .bool_error.filename, .bool_error.line)]
pub struct LoggableError {
    category: crate::DebugCategory,
    bool_error: glib::BoolError,
}

impl LoggableError {
    pub fn new(category: crate::DebugCategory, bool_error: glib::BoolError) -> LoggableError {
        assert_initialized_main_thread!();
        LoggableError {
            category,
            bool_error,
        }
    }

    pub fn log(&self) {
        self.category.log(
            None as Option<&glib::Object>,
            crate::DebugLevel::Error,
            self.bool_error.filename,
            self.bool_error.function,
            self.bool_error.line,
            format_args!("{}", self.bool_error.message),
        );
    }

    pub fn log_with_object<O: IsA<glib::Object>>(&self, obj: &O) {
        self.category.log(
            Some(obj),
            crate::DebugLevel::Error,
            self.bool_error.filename,
            self.bool_error.function,
            self.bool_error.line,
            format_args!("{}", self.bool_error.message),
        );
    }

    pub fn category(&self) -> crate::DebugCategory {
        self.category
    }
}

impl From<glib::BoolError> for LoggableError {
    fn from(bool_error: glib::BoolError) -> Self {
        skip_assert_initialized!();
        LoggableError {
            category: *crate::CAT_RUST,
            bool_error,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_message() {
        crate::init().unwrap();

        let e = ErrorMessage::new(
            &crate::CoreError::Failed,
            Some("message"),
            Some("debug"),
            "filename",
            "function",
            7,
        );
        assert_eq!(
            format!("{}", e),
            "Error Some(\"message\") (Some(\"debug\")) at filename:7"
        );
    }

    #[test]
    fn logabble_error() {
        crate::init().unwrap();

        let e: LoggableError = glib::BoolError::new("msg", "filename", "function", 7).into();
        assert_eq!(
            format!("{}", e),
            "Error \"GST_RUST\": \"msg\" at filename:7"
        );
    }
}
