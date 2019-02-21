// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib;
use glib::prelude::*;
use glib::translate::*;
use glib::value;
use glib_ffi;
use gobject_ffi;
use std::cmp;
use std::fmt;
use std::mem;
use std::ptr;

#[derive(Clone)]
pub struct VideoTimeCodeInterval(ffi::GstVideoTimeCodeInterval);

impl VideoTimeCodeInterval {
    pub fn from_string(tc_inter_str: &str) -> Option<Self> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_video_time_code_interval_new_from_string(
                tc_inter_str.to_glib_none().0,
            ))
        }
    }

    pub fn new(hours: u32, minutes: u32, seconds: u32, frames: u32) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut v = mem::zeroed();
            ffi::gst_video_time_code_interval_init(&mut v, hours, minutes, seconds, frames);
            VideoTimeCodeInterval(v)
        }
    }

    pub fn get_hours(&self) -> u32 {
        self.0.hours
    }

    pub fn set_hours(&mut self, hours: u32) {
        self.0.hours = hours
    }

    pub fn get_minutes(&self) -> u32 {
        self.0.minutes
    }

    pub fn set_minutes(&mut self, minutes: u32) {
        assert!(minutes < 60);
        self.0.minutes = minutes
    }

    pub fn get_seconds(&self) -> u32 {
        self.0.seconds
    }

    pub fn set_seconds(&mut self, seconds: u32) {
        assert!(seconds < 60);
        self.0.seconds = seconds
    }

    pub fn get_frames(&self) -> u32 {
        self.0.frames
    }

    pub fn set_frames(&mut self, hours: u32) {
        self.0.frames = hours
    }
}

unsafe impl Send for VideoTimeCodeInterval {}
unsafe impl Sync for VideoTimeCodeInterval {}

impl PartialEq for VideoTimeCodeInterval {
    fn eq(&self, other: &Self) -> bool {
        self.0.hours == other.0.hours
            && self.0.minutes == other.0.hours
            && self.0.seconds == other.0.seconds
            && self.0.frames == other.0.frames
    }
}

impl Eq for VideoTimeCodeInterval {}

impl PartialOrd for VideoTimeCodeInterval {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VideoTimeCodeInterval {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0
            .hours
            .cmp(&other.0.hours)
            .then_with(|| self.0.minutes.cmp(&other.0.hours))
            .then_with(|| self.0.seconds.cmp(&other.0.seconds))
            .then_with(|| self.0.frames.cmp(&other.0.frames))
    }
}

impl fmt::Debug for VideoTimeCodeInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("VideoTimeCodeInterval")
            .field("hours", &self.0.hours)
            .field("minutes", &self.0.minutes)
            .field("seconds", &self.0.seconds)
            .field("frames", &self.0.frames)
            .finish()
    }
}

impl fmt::Display for VideoTimeCodeInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{:02}:{:02}:{:02}:{:02}",
            self.0.hours, self.0.minutes, self.0.seconds, self.0.frames
        )
    }
}

#[doc(hidden)]
impl GlibPtrDefault for VideoTimeCodeInterval {
    type GlibType = *mut ffi::GstVideoTimeCodeInterval;
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GstVideoTimeCodeInterval> for VideoTimeCodeInterval {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GstVideoTimeCodeInterval, Self> {
        Stash(&self.0 as *const _, self)
    }

    #[inline]
    fn to_glib_full(&self) -> *const ffi::GstVideoTimeCodeInterval {
        unsafe { ffi::gst_video_time_code_interval_copy(&self.0 as *const _) }
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GstVideoTimeCodeInterval> for VideoTimeCodeInterval {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GstVideoTimeCodeInterval, Self> {
        let ptr = &mut self.0 as *mut _;
        StashMut(ptr, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::GstVideoTimeCodeInterval> for VideoTimeCodeInterval {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstVideoTimeCodeInterval) -> Self {
        assert!(!ptr.is_null());
        VideoTimeCodeInterval(ptr::read(ptr))
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::GstVideoTimeCodeInterval> for VideoTimeCodeInterval {
    #[inline]
    unsafe fn from_glib_none(ptr: *const ffi::GstVideoTimeCodeInterval) -> Self {
        assert!(!ptr.is_null());
        VideoTimeCodeInterval(ptr::read(ptr))
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::GstVideoTimeCodeInterval> for VideoTimeCodeInterval {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GstVideoTimeCodeInterval) -> Self {
        assert!(!ptr.is_null());
        let res = VideoTimeCodeInterval(ptr::read(ptr));
        ffi::gst_video_time_code_interval_free(ptr);

        res
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::GstVideoTimeCodeInterval> for VideoTimeCodeInterval {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::GstVideoTimeCodeInterval) -> Self {
        assert!(!ptr.is_null());
        VideoTimeCodeInterval(ptr::read(ptr))
    }
}

impl StaticType for VideoTimeCodeInterval {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_time_code_interval_get_type()) }
    }
}

#[doc(hidden)]
impl<'a> value::FromValueOptional<'a> for VideoTimeCodeInterval {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Option::<VideoTimeCodeInterval>::from_glib_full(gobject_ffi::g_value_dup_boxed(
            value.to_glib_none().0,
        )
            as *mut ffi::GstVideoTimeCodeInterval)
    }
}

#[doc(hidden)]
impl value::SetValue for VideoTimeCodeInterval {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            ToGlibPtr::<*const ffi::GstVideoTimeCodeInterval>::to_glib_none(this).0
                as glib_ffi::gpointer,
        )
    }
}

#[doc(hidden)]
impl value::SetValueOptional for VideoTimeCodeInterval {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            ToGlibPtr::<*const ffi::GstVideoTimeCodeInterval>::to_glib_none(&this).0
                as glib_ffi::gpointer,
        )
    }
}
