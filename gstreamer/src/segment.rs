// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Format;
use SeekFlags;
use SeekType;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use glib;
use std::mem;
use std::ptr;

pub struct Segment(ffi::GstSegment);

impl Segment {
    pub fn new() -> Segment {
        assert_initialized_main_thread!();
        unsafe { Self::uninitialized() }
    }

    pub fn clip(&self, format: Format, start: u64, stop: u64) -> Option<(u64, u64)> {
        unsafe {
            let mut clip_start = mem::uninitialized();
            let mut clip_stop = mem::uninitialized();
            let ret = from_glib(ffi::gst_segment_clip(
                self.to_glib_none().0,
                format.to_glib(),
                start,
                stop,
                &mut clip_start,
                &mut clip_stop,
            ));
            if ret {
                Some((clip_start, clip_stop))
            } else {
                None
            }
        }
    }

    pub fn copy_into(&self, dest: &mut Segment) {
        unsafe {
            ffi::gst_segment_copy_into(self.to_glib_none().0, dest.to_glib_none_mut().0);
        }
    }

    pub fn do_seek(
        &mut self,
        rate: f64,
        format: Format,
        flags: SeekFlags,
        start_type: SeekType,
        start: u64,
        stop_type: SeekType,
        stop: u64,
    ) -> Option<bool> {
        unsafe {
            let mut update = mem::uninitialized();
            let ret = from_glib(ffi::gst_segment_do_seek(
                self.to_glib_none_mut().0,
                rate,
                format.to_glib(),
                flags.to_glib(),
                start_type.to_glib(),
                start,
                stop_type.to_glib(),
                stop,
                &mut update,
            ));
            if ret {
                Some(from_glib(update))
            } else {
                None
            }
        }
    }

    pub fn init(&mut self, format: Format) {
        unsafe {
            ffi::gst_segment_init(self.to_glib_none_mut().0, format.to_glib());
        }
    }

    fn is_equal(&self, s1: &Segment) -> bool {
        unsafe {
            from_glib(ffi::gst_segment_is_equal(
                self.to_glib_none().0,
                s1.to_glib_none().0,
            ))
        }
    }

    pub fn offset_running_time(&mut self, format: Format, offset: i64) -> bool {
        unsafe {
            from_glib(ffi::gst_segment_offset_running_time(
                self.to_glib_none_mut().0,
                format.to_glib(),
                offset,
            ))
        }
    }

    pub fn position_from_running_time(&self, format: Format, running_time: u64) -> u64 {
        unsafe {
            ffi::gst_segment_position_from_running_time(
                self.to_glib_none().0,
                format.to_glib(),
                running_time,
            )
        }
    }

    pub fn position_from_running_time_full(&self, format: Format, running_time: u64) -> (i32, u64) {
        unsafe {
            let mut position = mem::uninitialized();
            let ret = ffi::gst_segment_position_from_running_time_full(
                self.to_glib_none().0,
                format.to_glib(),
                running_time,
                &mut position,
            );
            (ret, position)
        }
    }

    pub fn position_from_stream_time(&self, format: Format, stream_time: u64) -> u64 {
        unsafe {
            ffi::gst_segment_position_from_stream_time(
                self.to_glib_none().0,
                format.to_glib(),
                stream_time,
            )
        }
    }

    pub fn position_from_stream_time_full(&self, format: Format, stream_time: u64) -> (i32, u64) {
        unsafe {
            let mut position = mem::uninitialized();
            let ret = ffi::gst_segment_position_from_stream_time_full(
                self.to_glib_none().0,
                format.to_glib(),
                stream_time,
                &mut position,
            );
            (ret, position)
        }
    }

    pub fn set_running_time(&mut self, format: Format, running_time: u64) -> bool {
        unsafe {
            from_glib(ffi::gst_segment_set_running_time(
                self.to_glib_none_mut().0,
                format.to_glib(),
                running_time,
            ))
        }
    }

    pub fn to_position(&self, format: Format, running_time: u64) -> u64 {
        unsafe {
            ffi::gst_segment_to_position(self.to_glib_none().0, format.to_glib(), running_time)
        }
    }

    pub fn to_running_time(&self, format: Format, position: u64) -> u64 {
        unsafe {
            ffi::gst_segment_to_running_time(self.to_glib_none().0, format.to_glib(), position)
        }
    }

    pub fn to_running_time_full(&self, format: Format, position: u64) -> (i32, u64) {
        unsafe {
            let mut running_time = mem::uninitialized();
            let ret = ffi::gst_segment_to_running_time_full(
                self.to_glib_none().0,
                format.to_glib(),
                position,
                &mut running_time,
            );
            (ret, running_time)
        }
    }

    pub fn to_stream_time(&self, format: Format, position: u64) -> u64 {
        unsafe {
            ffi::gst_segment_to_stream_time(self.to_glib_none().0, format.to_glib(), position)
        }
    }

    pub fn to_stream_time_full(&self, format: Format, position: u64) -> (i32, u64) {
        unsafe {
            let mut stream_time = mem::uninitialized();
            let ret = ffi::gst_segment_to_stream_time_full(
                self.to_glib_none().0,
                format.to_glib(),
                position,
                &mut stream_time,
            );
            (ret, stream_time)
        }
    }
}

impl PartialEq for Segment {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}

impl Eq for Segment {}

unsafe impl Send for Segment {}

impl Clone for Segment {
    fn clone(&self) -> Self {
        unsafe { Segment(ptr::read(&self.0)) }
    }
}

impl glib::types::StaticType for Segment {
    fn static_type() -> glib::types::Type {
        unsafe { glib::translate::from_glib(ffi::gst_segment_get_type()) }
    }
}

#[doc(hidden)]
impl<'a> glib::value::FromValueOptional<'a> for Segment {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Option::<Segment>::from_glib_full(gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as
            *mut ffi::GstSegment)
    }
}

#[doc(hidden)]
impl glib::value::SetValue for Segment {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstSegment>::to_glib_none(this).0 as
                glib_ffi::gpointer,
        )
    }
}

#[doc(hidden)]
impl glib::value::SetValueOptional for Segment {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstSegment>::to_glib_none(&this).0 as
                glib_ffi::gpointer,
        )
    }
}

#[doc(hidden)]
impl glib::translate::Uninitialized for Segment {
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl glib::translate::GlibPtrDefault for Segment {
    type GlibType = *mut ffi::GstSegment;
}

#[doc(hidden)]
impl<'a> glib::translate::ToGlibPtr<'a, *const ffi::GstSegment> for Segment {
    type Storage = &'a Segment;

    fn to_glib_none(&'a self) -> glib::translate::Stash<'a, *const ffi::GstSegment, Self> {
        glib::translate::Stash(&self.0, self)
    }

    fn to_glib_full(&self) -> *const ffi::GstSegment {
        unimplemented!()
    }
}

#[doc(hidden)]
impl<'a> glib::translate::ToGlibPtrMut<'a, *mut ffi::GstSegment> for Segment {
    type Storage = &'a mut Segment;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> glib::translate::StashMut<'a, *mut ffi::GstSegment, Self> {
        glib::translate::StashMut(&mut self.0, self)
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*mut ffi::GstSegment> for Segment {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstSegment) -> Self {
        Segment(ptr::read(ptr))
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrFull<*mut ffi::GstSegment> for Segment {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GstSegment) -> Self {
        let segment = from_glib_none(ptr);
        glib_ffi::g_free(ptr as *mut _);
        segment
    }
}
