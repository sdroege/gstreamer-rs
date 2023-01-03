// Copyright (C) 2019 Víctor Jáquez <vjaquez@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::{ffi::gpointer, translate::*};
use libc::uintptr_t;

use crate::GLDisplayWayland;

impl GLDisplayWayland {
    pub unsafe fn with_display(
        display: uintptr_t,
    ) -> Result<GLDisplayWayland, glib::error::BoolError> {
        from_glib_full::<_, Option<GLDisplayWayland>>(ffi::gst_gl_display_wayland_new_with_display(
            display as gpointer,
        ))
        .ok_or_else(|| glib::bool_error!("Failed to create new Wayland GL display"))
    }
}
