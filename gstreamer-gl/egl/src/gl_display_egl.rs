// Copyright (C) 2018 Víctor Jáquez <vjaquez@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::{ffi::gpointer, translate::*};
use gst_gl::GLDisplayType;
use libc::uintptr_t;

use crate::GLDisplayEGL;

impl GLDisplayEGL {
    #[doc(alias = "gst_gl_display_egl_new_with_egl_display")]
    #[doc(alias = "new_with_egl_display")]
    pub unsafe fn with_egl_display(
        display: uintptr_t,
    ) -> Result<GLDisplayEGL, glib::error::BoolError> {
        from_glib_full::<_, Option<GLDisplayEGL>>(ffi::gst_gl_display_egl_new_with_egl_display(
            display as gpointer,
        ))
        .ok_or_else(|| glib::bool_error!("Failed to create new EGL GL display"))
    }

    #[doc(alias = "gst_gl_display_egl_get_from_native")]
    #[doc(alias = "get_from_native")]
    pub unsafe fn from_native(display_type: GLDisplayType, display: uintptr_t) -> gpointer {
        ffi::gst_gl_display_egl_get_from_native(display_type.into_glib(), display)
    }
}
