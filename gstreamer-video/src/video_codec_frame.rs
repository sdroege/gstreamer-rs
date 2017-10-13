// Copyright (C) 2017 Thibault Saunier <tsaunier@gnome.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;
use gst;
use gst::miniobject::MiniObject;

glib_wrapper! {
    pub struct VideoCodecFrame(Shared<ffi::GstVideoCodecFrame>);

    match fn {
        ref => |ptr| ffi::gst_video_codec_frame_ref(ptr),
        unref => |ptr| ffi::gst_video_codec_frame_unref(ptr),
        get_type => || ffi::gst_video_codec_frame_get_type(),
    }
}

impl VideoCodecFrame {
    pub fn flags(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).flags }
    }

    pub fn system_frame_number(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).system_frame_number }
    }

    pub fn decode_frame_number(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).decode_frame_number}
    }

    pub fn presentation_frame_number(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).presentation_frame_number}
    }

    pub fn dts(&self) -> gst::ClockTime {
        unsafe { from_glib((*self.to_glib_none().0).dts) }
    }

    pub fn pts(&self) -> gst::ClockTime {
        unsafe { from_glib((*self.to_glib_none().0).pts) }
    }

    pub fn duration(&self) -> gst::ClockTime {
        unsafe { from_glib((*self.to_glib_none().0).duration) }
    }

    pub fn distance_from_sync(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).distance_from_sync}
    }

    pub fn input_buffer(&self) -> &gst::BufferRef {
        unsafe { gst::BufferRef::from_ptr((*self.to_glib_none().0).input_buffer) }
    }

    pub fn output_buffer(&self) -> &gst::BufferRef {
        unsafe { gst::BufferRef::from_ptr((*self.to_glib_none().0).output_buffer) }
    }

    pub fn set_output_buffer(&self, output_buffer: &gst::BufferRef) {
        unsafe {(*self.to_glib_none().0).output_buffer = output_buffer.as_mut_ptr(); }
    }

    pub fn deadline(&self) -> gst::ClockTime {
        unsafe { from_glib((*self.to_glib_none().0).deadline) }
    }
}

unsafe impl Send for VideoCodecFrame {}
unsafe impl Sync for VideoCodecFrame {}
