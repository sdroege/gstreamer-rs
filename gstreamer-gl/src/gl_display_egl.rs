// Copyright (C) 2018 Víctor Jáquez <vjaquez@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use glib_sys::gpointer;
use gst_gl_sys;
use libc::uintptr_t;
use GLDisplayEGL;
use GLDisplayType;

impl GLDisplayEGL {
    pub unsafe fn with_egl_display(
        display: uintptr_t,
    ) -> Result<GLDisplayEGL, glib::error::BoolError> {
        let result = from_glib_full(gst_gl_sys::gst_gl_display_egl_new_with_egl_display(
            display as gpointer,
        ));
        match result {
            Some(d) => Ok(d),
            None => Err(glib_bool_error!("Failed to create new EGL GL display")),
        }
    }

    pub unsafe fn get_from_native(display_type: GLDisplayType, display: uintptr_t) -> gpointer {
        gst_gl_sys::gst_gl_display_egl_get_from_native(display_type.to_glib(), display)
    }
}
