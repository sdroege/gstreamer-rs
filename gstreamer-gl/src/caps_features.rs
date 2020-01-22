// Copyright (C) 2018 Víctor Jáquez <vjaquez@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst::CapsFeatures;
use gst_gl_sys;
use std::ffi::CStr;

use once_cell::sync::Lazy;

pub static CAPS_FEATURE_MEMORY_GL_MEMORY: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(gst_gl_sys::GST_CAPS_FEATURE_MEMORY_GL_MEMORY)
        .to_str()
        .unwrap()
});
pub static CAPS_FEATURES_MEMORY_GL_MEMORY: Lazy<CapsFeatures> =
    Lazy::new(|| CapsFeatures::new(&[*CAPS_FEATURE_MEMORY_GL_MEMORY]));
