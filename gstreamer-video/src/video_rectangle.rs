// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::IntoGlib;
use std::mem;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct VideoRectangle {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl VideoRectangle {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        skip_assert_initialized!();
        Self { x, y, w, h }
    }
}

pub fn center_video_rectangle(
    src: &VideoRectangle,
    dst: &VideoRectangle,
    scale: bool,
) -> VideoRectangle {
    skip_assert_initialized!();
    let mut result = ffi::GstVideoRectangle {
        x: 0,
        y: 0,
        w: 0,
        h: 0,
    };
    let src_rect = ffi::GstVideoRectangle {
        x: src.x,
        y: src.y,
        w: src.w,
        h: src.h,
    };
    let dst_rect = ffi::GstVideoRectangle {
        x: dst.x,
        y: dst.y,
        w: dst.w,
        h: dst.h,
    };
    unsafe {
        ffi::gst_video_sink_center_rect(src_rect, dst_rect, &mut result, scale.into_glib());
    }
    VideoRectangle::new(result.x, result.y, result.w, result.h)
}

#[doc(hidden)]
impl glib::translate::Uninitialized for VideoRectangle {
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}
