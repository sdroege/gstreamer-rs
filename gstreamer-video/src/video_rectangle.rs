// Copyright (C) 2017 Philippe Normand <philn@igalia.com~
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct VideoRectangle {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl VideoRectangle {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        VideoRectangle {
            x, y, w, h
        }
    }
}

pub fn center_video_rectangle(src: VideoRectangle, dst: VideoRectangle, scale: bool) -> VideoRectangle {
    let mut result = ffi::GstVideoRectangle { x: 0, y: 0, w: 0, h: 0 };
    let src_rect = ffi::GstVideoRectangle { x: src.x, y: src.y, w: src.w, h: src.h };
    let dst_rect = ffi::GstVideoRectangle { x: dst.x, y: dst.y, w: dst.w, h: dst.h };
    unsafe {
        ffi::gst_video_sink_center_rect(src_rect, dst_rect, &mut result, scale as i32);
    }
    VideoRectangle::new(result.x, result.y, result.w, result.h)
}
