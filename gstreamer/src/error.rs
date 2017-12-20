// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error;
use std::fmt;

use glib_ffi;

#[macro_export]
macro_rules! gst_error_msg(
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorMessage {
    pub(crate) error_domain: glib_ffi::GQuark,
    pub(crate) error_code: i32,
    pub(crate) message: Option<String>,
    pub(crate) debug: Option<String>,
    pub(crate) filename: &'static str,
    pub(crate) function: &'static str,
    pub(crate) line: u32,
}

impl ErrorMessage {
    pub fn new<
        'a,
        'b,
        T: ::MessageErrorDomain,
        U: Into<Option<&'a str>>,
        V: Into<Option<&'b str>>,
    >(
        error: &T,
        message: U,
        debug: V,
        filename: &'static str,
        function: &'static str,
        line: u32,
    ) -> ErrorMessage {
        let domain = T::domain();
        let code = error.code();
        let message = message.into();
        let debug = debug.into();

        ErrorMessage {
            error_domain: domain,
            error_code: code,
            message: message.map(String::from),
            debug: debug.map(String::from),
            filename: filename,
            function: function,
            line: line,
        }
    }
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "Error {:?} ({:?}) at {}:{}",
            self.message, self.debug, self.filename, self.line
        )
    }
}

impl Error for ErrorMessage {
    fn description(&self) -> &str {
        "ErrorMessage"
    }
}
