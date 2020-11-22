// Copyright (C) 2018 Víctor Jáquez <vjaquez@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::GLDisplay;
use glib::translate::*;
use glib::IsA;
use gst::ContextRef;
use std::ptr;

pub trait ContextGLExt {
    fn get_gl_display(&self) -> Option<GLDisplay>;
    fn set_gl_display<T: IsA<GLDisplay>>(&self, display: &T);
}

impl ContextGLExt for ContextRef {
    fn get_gl_display(&self) -> Option<GLDisplay> {
        unsafe {
            let mut display = ptr::null_mut();
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

    fn set_gl_display<T: IsA<GLDisplay>>(&self, display: &T) {
        unsafe {
            ffi::gst_context_set_gl_display(self.as_mut_ptr(), display.as_ref().to_glib_none().0);
        }
    }
}
