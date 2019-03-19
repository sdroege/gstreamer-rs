// Copyright (C) 2018 Víctor Jáquez <vjaquez@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_gl_sys;
use std::ffi::CStr;

lazy_static! {
    pub static ref GL_DISPLAY_CONTEXT_TYPE: &'static str = unsafe {
        CStr::from_ptr(gst_gl_sys::GST_GL_DISPLAY_CONTEXT_TYPE)
            .to_str()
            .unwrap()
    };
}
