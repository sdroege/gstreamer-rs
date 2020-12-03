// Copyright (C) 2019 Guillaume Desmottes <guillaume.desmottes@collabora.com>
// Copyright (C) 2019 Vivia Nikolaidou <vivia@ahiru.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::mem;

use glib::translate::*;

use once_cell::sync::Lazy;

pub static BUFFER_POOL_OPTION_VIDEO_AFFINE_TRANSFORMATION_META: Lazy<&'static str> =
    Lazy::new(|| unsafe {
        CStr::from_ptr(gst_video_sys::GST_BUFFER_POOL_OPTION_VIDEO_AFFINE_TRANSFORMATION_META)
            .to_str()
            .unwrap()
    });
pub static BUFFER_POOL_OPTION_VIDEO_ALIGNMENT: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(gst_video_sys::GST_BUFFER_POOL_OPTION_VIDEO_ALIGNMENT)
        .to_str()
        .unwrap()
});
pub static BUFFER_POOL_OPTION_VIDEO_GL_TEXTURE_UPLOAD_META: Lazy<&'static str> =
    Lazy::new(|| unsafe {
        CStr::from_ptr(gst_video_sys::GST_BUFFER_POOL_OPTION_VIDEO_GL_TEXTURE_UPLOAD_META)
            .to_str()
            .unwrap()
    });
pub static BUFFER_POOL_OPTION_VIDEO_META: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(gst_video_sys::GST_BUFFER_POOL_OPTION_VIDEO_META)
        .to_str()
        .unwrap()
});

#[derive(Debug, Clone)]
pub struct VideoAlignment(pub(crate) gst_video_sys::GstVideoAlignment);

impl VideoAlignment {
    pub fn get_padding_top(&self) -> u32 {
        self.0.padding_top
    }
    pub fn get_padding_bottom(&self) -> u32 {
        self.0.padding_bottom
    }
    pub fn get_padding_left(&self) -> u32 {
        self.0.padding_left
    }
    pub fn get_padding_right(&self) -> u32 {
        self.0.padding_right
    }
    pub fn get_stride_align(&self) -> &[u32; gst_video_sys::GST_VIDEO_MAX_PLANES as usize] {
        &self.0.stride_align
    }

    pub fn new(
        padding_top: u32,
        padding_bottom: u32,
        padding_left: u32,
        padding_right: u32,
        stride_align: &[u32; gst_video_sys::GST_VIDEO_MAX_PLANES as usize],
    ) -> Self {
        assert_initialized_main_thread!();

        let videoalignment = unsafe {
            let mut videoalignment: gst_video_sys::GstVideoAlignment = mem::zeroed();

            videoalignment.padding_top = padding_top;
            videoalignment.padding_bottom = padding_bottom;
            videoalignment.padding_left = padding_left;
            videoalignment.padding_right = padding_right;
            videoalignment.stride_align = *stride_align;

            videoalignment
        };

        VideoAlignment(videoalignment)
    }
}

impl PartialEq for VideoAlignment {
    fn eq(&self, other: &VideoAlignment) -> bool {
        self.get_padding_top() == other.get_padding_top()
            && self.get_padding_bottom() == other.get_padding_bottom()
            && self.get_padding_left() == other.get_padding_left()
            && self.get_padding_right() == other.get_padding_right()
            && self.get_stride_align() == other.get_stride_align()
    }
}

impl Eq for VideoAlignment {}

pub trait VideoBufferPoolConfig {
    fn get_video_alignment(&self) -> Option<VideoAlignment>;

    fn set_video_alignment(&mut self, align: &VideoAlignment);
}

impl VideoBufferPoolConfig for gst::BufferPoolConfig {
    fn get_video_alignment(&self) -> Option<VideoAlignment> {
        unsafe {
            let mut alignment = mem::MaybeUninit::zeroed();
            let ret = from_glib(gst_video_sys::gst_buffer_pool_config_get_video_alignment(
                self.as_ref().as_mut_ptr(),
                alignment.as_mut_ptr(),
            ));
            if ret {
                Some(VideoAlignment(alignment.assume_init()))
            } else {
                None
            }
        }
    }

    fn set_video_alignment(&mut self, align: &VideoAlignment) {
        unsafe {
            gst_video_sys::gst_buffer_pool_config_set_video_alignment(
                self.as_mut().as_mut_ptr(),
                &align.0 as *const _ as *mut _,
            )
        }
    }
}
