// Copyright (C) 2018 Víctor Jáquez <vjaquez@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib::error::ErrorDomain;
use glib::translate::*;
use glib::Quark;
use GLContextError;
use GLWindowError;

impl ErrorDomain for GLContextError {
    fn domain() -> Quark {
        skip_assert_initialized!();
        unsafe { from_glib(ffi::gst_gl_context_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();
        match code {
            0 => Some(GLContextError::Failed),
            1 => Some(GLContextError::WrongConfig),
            2 => Some(GLContextError::WrongApi),
            3 => Some(GLContextError::OldLibs),
            4 => Some(GLContextError::CreateContext),
            5 => Some(GLContextError::ResourceUnavailable),
            _ => Some(GLContextError::Failed),
        }
    }
}

impl ErrorDomain for GLWindowError {
    fn domain() -> Quark {
        skip_assert_initialized!();
        unsafe { from_glib(ffi::gst_gl_window_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();
        match code {
            0 => Some(GLWindowError::Failed),
            1 => Some(GLWindowError::OldLibs),
            2 => Some(GLWindowError::ResourceUnavailable),
            _ => Some(GLWindowError::Failed),
        }
    }
}
