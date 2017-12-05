// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Format;
use FormatValue;
use SeekFlags;
use SeekType;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use glib;
use std::mem;
use std::ptr;
use std::fmt;

pub struct Segment(ffi::GstSegment);

impl Segment {
    pub fn new() -> Segment {
        assert_initialized_main_thread!();
        unsafe { Self::uninitialized() }
    }

    pub fn clip<V: Into<FormatValue>>(
        &self,
        start: V,
        stop: V,
    ) -> Option<(FormatValue, FormatValue)> {
        let start = start.into();
        let stop = stop.into();
        assert_eq!(self.get_format(), start.to_format());
        assert_eq!(start.to_format(), stop.to_format());
        unsafe {
            let mut clip_start = mem::uninitialized();
            let mut clip_stop = mem::uninitialized();
            let ret = from_glib(ffi::gst_segment_clip(
                self.to_glib_none().0,
                start.to_format().to_glib(),
                start.to_value() as u64,
                stop.to_value() as u64,
                &mut clip_start,
                &mut clip_stop,
            ));
            if ret {
                Some((
                    FormatValue::new(self.get_format(), clip_start as i64),
                    FormatValue::new(self.get_format(), clip_stop as i64),
                ))
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

    #[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
    pub fn do_seek<V: Into<FormatValue>>(
        &mut self,
        rate: f64,
        flags: SeekFlags,
        start_type: SeekType,
        start: V,
        stop_type: SeekType,
        stop: V,
    ) -> Option<bool> {
        skip_assert_initialized!();
        let start = start.into();
        let stop = stop.into();
        assert_eq!(self.get_format(), start.to_format());
        assert_eq!(start.to_format(), stop.to_format());
        unsafe {
            let mut update = mem::uninitialized();
            let ret = from_glib(ffi::gst_segment_do_seek(
                self.to_glib_none_mut().0,
                rate,
                self.get_format().to_glib(),
                flags.to_glib(),
                start_type.to_glib(),
                start.to_value() as u64,
                stop_type.to_glib(),
                stop.to_value() as u64,
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

    pub fn position_from_running_time<V: Into<FormatValue>>(&self, running_time: V) -> FormatValue {
        let running_time = running_time.into();
        assert_eq!(self.get_format(), running_time.to_format());
        unsafe {
            FormatValue::new(
                self.get_format(),
                ffi::gst_segment_position_from_running_time(
                    self.to_glib_none().0,
                    self.get_format().to_glib(),
                    running_time.to_value() as u64,
                ) as i64,
            )
        }
    }

    pub fn position_from_running_time_full<V: Into<FormatValue>>(
        &self,
        running_time: V,
    ) -> (i32, FormatValue) {
        let running_time = running_time.into();
        assert_eq!(self.get_format(), running_time.to_format());
        unsafe {
            let mut position = mem::uninitialized();
            let ret = ffi::gst_segment_position_from_running_time_full(
                self.to_glib_none().0,
                self.get_format().to_glib(),
                running_time.to_value() as u64,
                &mut position,
            );
            (ret, FormatValue::new(self.get_format(), position as i64))
        }
    }

    pub fn position_from_stream_time<V: Into<FormatValue>>(&self, stream_time: V) -> FormatValue {
        let stream_time = stream_time.into();
        assert_eq!(self.get_format(), stream_time.to_format());
        unsafe {
            FormatValue::new(
                self.get_format(),
                ffi::gst_segment_position_from_stream_time(
                    self.to_glib_none().0,
                    self.get_format().to_glib(),
                    stream_time.to_value() as u64,
                ) as i64,
            )
        }
    }

    pub fn position_from_stream_time_full<V: Into<FormatValue>>(
        &self,
        stream_time: V,
    ) -> (i32, FormatValue) {
        let stream_time = stream_time.into();
        assert_eq!(self.get_format(), stream_time.to_format());
        unsafe {
            let mut position = mem::uninitialized();
            let ret = ffi::gst_segment_position_from_stream_time_full(
                self.to_glib_none().0,
                self.get_format().to_glib(),
                stream_time.to_value() as u64,
                &mut position,
            );
            (ret, FormatValue::new(self.get_format(), position as i64))
        }
    }

    pub fn set_running_time<V: Into<FormatValue>>(&mut self, running_time: V) -> bool {
        let running_time = running_time.into();
        assert_eq!(self.get_format(), running_time.to_format());
        unsafe {
            from_glib(ffi::gst_segment_set_running_time(
                self.to_glib_none_mut().0,
                self.get_format().to_glib(),
                running_time.to_value() as u64,
            ))
        }
    }

    pub fn to_position<V: Into<FormatValue>>(&self, running_time: V) -> FormatValue {
        let running_time = running_time.into();
        assert_eq!(self.get_format(), running_time.to_format());
        unsafe {
            FormatValue::new(
                self.get_format(),
                ffi::gst_segment_to_position(
                    self.to_glib_none().0,
                    self.get_format().to_glib(),
                    running_time.to_value() as u64,
                ) as i64,
            )
        }
    }

    pub fn to_running_time<V: Into<FormatValue>>(&self, position: V) -> FormatValue {
        let position = position.into();
        assert_eq!(self.get_format(), position.to_format());
        unsafe {
            FormatValue::new(
                self.get_format(),
                ffi::gst_segment_to_running_time(
                    self.to_glib_none().0,
                    self.get_format().to_glib(),
                    position.to_value() as u64,
                ) as i64,
            )
        }
    }

    pub fn to_running_time_full<V: Into<FormatValue>>(&self, position: V) -> (i32, FormatValue) {
        let position = position.into();
        assert_eq!(self.get_format(), position.to_format());
        unsafe {
            let mut running_time = mem::uninitialized();
            let ret = ffi::gst_segment_to_running_time_full(
                self.to_glib_none().0,
                self.get_format().to_glib(),
                position.to_value() as u64,
                &mut running_time,
            );
            (
                ret,
                FormatValue::new(self.get_format(), running_time as i64),
            )
        }
    }

    pub fn to_stream_time<V: Into<FormatValue>>(&self, position: V) -> FormatValue {
        let position = position.into();
        assert_eq!(self.get_format(), position.to_format());
        unsafe {
            FormatValue::new(
                self.get_format(),
                ffi::gst_segment_to_stream_time(
                    self.to_glib_none().0,
                    self.get_format().to_glib(),
                    position.to_value() as u64,
                ) as i64,
            )
        }
    }

    pub fn to_stream_time_full<V: Into<FormatValue>>(&self, position: V) -> (i32, FormatValue) {
        let position = position.into();
        assert_eq!(self.get_format(), position.to_format());
        unsafe {
            let mut stream_time = mem::uninitialized();
            let ret = ffi::gst_segment_to_stream_time_full(
                self.to_glib_none().0,
                self.get_format().to_glib(),
                position.to_value() as u64,
                &mut stream_time,
            );
            (ret, FormatValue::new(self.get_format(), stream_time as i64))
        }
    }

    pub fn get_flags(&self) -> ::SegmentFlags {
        from_glib(self.0.flags)
    }

    pub fn set_flags(&mut self, flags: ::SegmentFlags) {
        self.0.flags = flags.to_glib();
    }

    pub fn get_rate(&self) -> f64 {
        self.0.rate
    }

    pub fn set_rate(&mut self, rate: f64) {
        self.0.rate = rate;
    }

    pub fn get_applied_rate(&self) -> f64 {
        self.0.applied_rate
    }

    pub fn set_applied_rate(&mut self, applied_rate: f64) {
        self.0.applied_rate = applied_rate;
    }

    pub fn get_format(&self) -> Format {
        from_glib(self.0.format)
    }

    pub fn set_format(&mut self, format: Format) {
        self.0.format = format.to_glib();
    }

    pub fn get_base(&self) -> FormatValue {
        FormatValue::new(self.get_format(), self.0.base as i64)
    }

    pub fn set_base<V: Into<FormatValue>>(&mut self, base: V) {
        let base = base.into();
        assert_eq!(self.get_format(), base.to_format());
        self.0.base = base.to_value() as u64;
    }

    pub fn get_offset(&self) -> FormatValue {
        FormatValue::new(self.get_format(), self.0.offset as i64)
    }

    pub fn set_offset<V: Into<FormatValue>>(&mut self, offset: V) {
        let offset = offset.into();
        assert_eq!(self.get_format(), offset.to_format());
        self.0.offset = offset.to_value() as u64;
    }

    pub fn get_start(&self) -> FormatValue {
        FormatValue::new(self.get_format(), self.0.start as i64)
    }

    pub fn set_start<V: Into<FormatValue>>(&mut self, start: V) {
        let start = start.into();
        assert_eq!(self.get_format(), start.to_format());
        self.0.start = start.to_value() as u64;
    }

    pub fn get_stop(&self) -> FormatValue {
        FormatValue::new(self.get_format(), self.0.stop as i64)
    }

    pub fn set_stop<V: Into<FormatValue>>(&mut self, stop: V) {
        let stop = stop.into();
        assert_eq!(self.get_format(), stop.to_format());
        self.0.stop = stop.to_value() as u64;
    }

    pub fn get_time(&self) -> FormatValue {
        FormatValue::new(self.get_format(), self.0.time as i64)
    }

    pub fn set_time<V: Into<FormatValue>>(&mut self, time: V) {
        let time = time.into();
        assert_eq!(self.get_format(), time.to_format());
        self.0.time = time.to_value() as u64;
    }

    pub fn get_position(&self) -> FormatValue {
        FormatValue::new(self.get_format(), self.0.position as i64)
    }

    pub fn set_position<V: Into<FormatValue>>(&mut self, position: V) {
        let position = position.into();
        assert_eq!(self.get_format(), position.to_format());
        self.0.position = position.to_value() as u64;
    }

    pub fn get_duration(&self) -> FormatValue {
        FormatValue::new(self.get_format(), self.0.duration as i64)
    }

    pub fn set_duration<V: Into<FormatValue>>(&mut self, duration: V) {
        let duration = duration.into();
        assert_eq!(self.get_format(), duration.to_format());
        self.0.duration = duration.to_value() as u64;
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

impl fmt::Debug for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.get_format() {
            Format::Undefined => f.debug_struct("Segment")
                .field("format", &Format::Undefined)
                .finish(),
            Format::Time => f.debug_struct("Segment")
                .field("format", &Format::Time)
                .field(
                    "start",
                    &self.get_start().try_to_time().unwrap().to_string(),
                )
                .field(
                    "offset",
                    &self.get_offset().try_to_time().unwrap().to_string(),
                )
                .field("stop", &self.get_stop().try_to_time().unwrap().to_string())
                .field("rate", &self.get_rate())
                .field("applied_rate", &self.get_applied_rate())
                .field("flags", &self.get_flags())
                .field("time", &self.get_time().try_to_time().unwrap().to_string())
                .field("base", &self.get_base().try_to_time().unwrap().to_string())
                .field(
                    "position",
                    &self.get_position().try_to_time().unwrap().to_string(),
                )
                .field(
                    "duration",
                    &self.get_duration().try_to_time().unwrap().to_string(),
                )
                .finish(),
            _ => f.debug_struct("Segment")
                .field("format", &self.get_format())
                .field("start", &self.get_start())
                .field("offset", &self.get_offset())
                .field("stop", &self.get_stop())
                .field("rate", &self.get_rate())
                .field("applied_rate", &self.get_applied_rate())
                .field("flags", &self.get_flags())
                .field("time", &self.get_time())
                .field("base", &self.get_base())
                .field("position", &self.get_position())
                .field("duration", &self.get_duration())
                .finish(),
        }
    }
}

impl glib::types::StaticType for Segment {
    fn static_type() -> glib::types::Type {
        unsafe { glib::translate::from_glib(ffi::gst_segment_get_type()) }
    }
}

impl Default for Segment {
    fn default() -> Self {
        Self::new()
    }
}

#[doc(hidden)]
impl<'a> glib::value::FromValueOptional<'a> for Segment {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Option::<Segment>::from_glib_none(gobject_ffi::g_value_get_boxed(value.to_glib_none().0)
            as *mut ffi::GstSegment)
    }
}

#[doc(hidden)]
impl glib::value::SetValue for Segment {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstSegment>::to_glib_none(this).0
                as glib_ffi::gpointer,
        )
    }
}

#[doc(hidden)]
impl glib::value::SetValueOptional for Segment {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstSegment>::to_glib_none(&this).0
                as glib_ffi::gpointer,
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
impl glib::translate::FromGlibPtrNone<*const ffi::GstSegment> for Segment {
    #[inline]
    unsafe fn from_glib_none(ptr: *const ffi::GstSegment) -> Self {
        Segment(ptr::read(ptr))
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
impl glib::translate::FromGlibPtrBorrow<*mut ffi::GstSegment> for Segment {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::GstSegment) -> Self {
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
