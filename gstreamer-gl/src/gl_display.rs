// Copyright (C) 2018 Víctor Jáquez <vjaquez@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib::translate::*;
use gst::{Context, MiniObject};
use std::ffi::CStr;
use std::mem;
use GLDisplay;

pub trait ContextGLExt {
    fn get_gl_display(&self) -> Option<GLDisplay>;
    fn set_gl_display(&self, display: &GLDisplay);
}

impl ContextGLExt for Context {
    fn get_gl_display(&self) -> Option<GLDisplay> {
        unsafe {
            let mut display = mem::uninitialized();
            if from_glib(ffi::gst_context_get_gl_display(
                self.as_mut_ptr(),
                &mut display,
            )) {
                Some(from_glib_full(display))
            } else {
                None
            }
        }
    }

    fn set_gl_display(&self, display: &GLDisplay) {
        unsafe {
            ffi::gst_context_set_gl_display(self.as_mut_ptr(), display.to_glib_none().0);
        }
    }
}

lazy_static! {
    pub static ref GL_DISPLAY_CONTEXT_TYPE: &'static str = unsafe {
        CStr::from_ptr(ffi::GST_GL_DISPLAY_CONTEXT_TYPE)
            .to_str()
            .unwrap()
    };
}
