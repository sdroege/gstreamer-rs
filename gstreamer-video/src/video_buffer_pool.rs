// Copyright (C) 2019 Guillaume Desmottes <guillaume.desmottes@collabora.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;

lazy_static! {
    pub static ref BUFFER_POOL_OPTION_VIDEO_AFFINE_TRANSFORMATION_META: &'static str = unsafe {
        CStr::from_ptr(gst_video_sys::GST_BUFFER_POOL_OPTION_VIDEO_AFFINE_TRANSFORMATION_META)
            .to_str()
            .unwrap()
    };
    pub static ref BUFFER_POOL_OPTION_VIDEO_ALIGNMENT: &'static str = unsafe {
        CStr::from_ptr(gst_video_sys::GST_BUFFER_POOL_OPTION_VIDEO_ALIGNMENT)
            .to_str()
            .unwrap()
    };
    pub static ref BUFFER_POOL_OPTION_VIDEO_GL_TEXTURE_UPLOAD_META: &'static str = unsafe {
        CStr::from_ptr(gst_video_sys::GST_BUFFER_POOL_OPTION_VIDEO_GL_TEXTURE_UPLOAD_META)
            .to_str()
            .unwrap()
    };
    pub static ref BUFFER_POOL_OPTION_VIDEO_META: &'static str = unsafe {
        CStr::from_ptr(gst_video_sys::GST_BUFFER_POOL_OPTION_VIDEO_META)
            .to_str()
            .unwrap()
    };
}
