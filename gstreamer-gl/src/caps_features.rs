// Copyright (C) 2018 Víctor Jáquez <vjaquez@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use gst::CapsFeatures;
use std::ffi::CStr;

lazy_static! {
    pub static ref CAPS_FEATURE_MEMORY_GL_MEMORY: &'static str = unsafe {
        CStr::from_ptr(ffi::GST_CAPS_FEATURE_MEMORY_GL_MEMORY)
            .to_str()
            .unwrap()
    };
    pub static ref CAPS_FEATURES_MEMORY_GL_MEMORY: CapsFeatures =
        CapsFeatures::new(&[*CAPS_FEATURE_MEMORY_GL_MEMORY]);
}
