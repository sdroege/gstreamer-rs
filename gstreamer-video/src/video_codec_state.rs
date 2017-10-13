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
use gst_ffi;
use std::mem;
use std::ptr;

use gst;
use gst::miniobject::MiniObject;

use video_info::VideoInfo;

glib_wrapper! {
    pub struct VideoCodecState(Shared<ffi::GstVideoCodecState>);

    match fn {
        ref => |ptr| ffi::gst_video_codec_state_ref(ptr),
        unref => |ptr| ffi::gst_video_codec_state_unref(ptr),
        get_type => || ffi::gst_video_codec_state_get_type(),
    }
}

impl VideoCodecState {
    pub fn info(&self) -> VideoInfo {
        unsafe { VideoInfo(ptr::read(&((*self.to_glib_none().0).info))) }
    }

    //pub fn set_info(&self, info: VideoInfo) {
        //ptr::write(ptr::read(&((self.to_glib_none().0).info)), *(info.to_glib_none()));
    //}

    pub fn caps(&self) -> &gst::CapsRef {
        unsafe { gst::CapsRef::from_ptr((*self.to_glib_none().0).caps) }
    }

    pub fn set_caps(&self, caps: &gst::CapsRef) {
        unsafe {
            let prev = (*self.to_glib_none().0).caps;

            if !prev.is_null() {
                gst_ffi::gst_mini_object_unref(prev as *mut gst_ffi::GstMiniObject)
            }

            ptr::write(&mut (*self.to_glib_none().0).caps, mem::transmute(
                    gst_ffi::gst_mini_object_ref(caps.as_ptr() as *mut gst_ffi::GstMiniObject)));
        }
    }

    pub fn codec_data(&self) -> &gst::BufferRef {
        unsafe { gst::BufferRef::from_ptr((*self.to_glib_none().0).codec_data) }
    }

    pub fn set_codec_data(&self, codec_data: &mut gst::BufferRef) {
        unsafe {
            let prev = (*self.to_glib_none().0).codec_data;

            if !prev.is_null() {
                gst_ffi::gst_mini_object_unref(prev as *mut gst_ffi::GstMiniObject)
            }

            ptr::write(&mut (*self.to_glib_none().0).codec_data, mem::transmute(
                    gst_ffi::gst_mini_object_ref(codec_data.as_ptr() as *mut gst_ffi::GstMiniObject)));
        }
    }

    pub fn allocation_caps(&self) -> &gst::CapsRef {
        unsafe { gst::CapsRef::from_ptr((*self.to_glib_none().0).allocation_caps) }
    }

    pub fn set_allocation_caps(&self, allocation_caps: &mut gst::CapsRef) {
        unsafe {
            let prev = (*self.to_glib_none().0).allocation_caps;

            if !prev.is_null() {
                gst_ffi::gst_mini_object_unref(prev as *mut gst_ffi::GstMiniObject)
            }

            ptr::write(&mut (*self.to_glib_none().0).allocation_caps, mem::transmute(
                    gst_ffi::gst_mini_object_ref(allocation_caps.as_ptr() as *mut gst_ffi::GstMiniObject)));
        }
    }
}

unsafe impl Send for VideoCodecState {}
unsafe impl Sync for VideoCodecState {}
