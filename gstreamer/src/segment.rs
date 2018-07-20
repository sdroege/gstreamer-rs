// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ptr;
use Format;
use FormattedValue;
use GenericFormattedValue;
use SeekFlags;
use SeekType;

pub type Segment = FormattedSegment<GenericFormattedValue>;
#[repr(C)]
pub struct FormattedSegment<T: FormattedValue>(ffi::GstSegment, PhantomData<T>);

impl Segment {
    pub fn reset_with_format(&mut self, format: Format) {
        unsafe {
            ffi::gst_segment_init(self.to_glib_none_mut().0, format.to_glib());
        }
    }

    pub fn set_format(&mut self, format: Format) {
        self.0.format = format.to_glib();
    }

    pub fn downcast<T: FormattedValue>(self) -> Result<FormattedSegment<T>, Self> {
        if T::get_default_format() == Format::Undefined
            || T::get_default_format() == self.get_format()
        {
            Ok(FormattedSegment(self.0, PhantomData))
        } else {
            Err(self)
        }
    }

    pub fn downcast_ref<T: FormattedValue>(&self) -> Option<&FormattedSegment<T>> {
        if T::get_default_format() == Format::Undefined
            || T::get_default_format() == self.get_format()
        {
            Some(unsafe { &*(self as *const FormattedSegment<GenericFormattedValue> as *const FormattedSegment<T>) })
        } else {
            None
        }
    }

    pub fn downcast_mut<T: FormattedValue>(&mut self) -> Option<&mut FormattedSegment<T>> {
        if T::get_default_format() == Format::Undefined
            || T::get_default_format() == self.get_format()
        {
            Some(unsafe { &mut *(self as *mut FormattedSegment<GenericFormattedValue> as *mut FormattedSegment<T>) })
        } else {
            None
        }
    }
}

impl<T: FormattedValue> FormattedSegment<T> {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        let segment = unsafe {
            let mut segment = mem::zeroed();
            ffi::gst_segment_init(&mut segment, T::get_default_format().to_glib());
            segment
        };
        FormattedSegment(segment, PhantomData)
    }

    pub fn upcast(self) -> Segment {
        FormattedSegment(self.0, PhantomData)
    }

    pub fn upcast_ref(&self) -> &Segment {
        unsafe { &*(self as *const FormattedSegment<T> as *const FormattedSegment<GenericFormattedValue>) }
    }

    pub fn reset(&mut self) {
        unsafe {
            ffi::gst_segment_init(&mut self.0, T::get_default_format().to_glib());
        }
    }

    pub fn clip<V: Into<T>>(&self, start: V, stop: V) -> Option<(T, T)> {
        let start = start.into();
        let stop = stop.into();

        if T::get_default_format() == Format::Undefined {
            assert_eq!(self.get_format(), start.get_format());
            assert_eq!(self.get_format(), stop.get_format());
        }

        unsafe {
            let mut clip_start = mem::uninitialized();
            let mut clip_stop = mem::uninitialized();
            let ret = from_glib(ffi::gst_segment_clip(
                &self.0,
                start.get_format().to_glib(),
                start.to_raw_value() as u64,
                stop.to_raw_value() as u64,
                &mut clip_start,
                &mut clip_stop,
            ));
            if ret {
                Some((
                    T::from_raw(self.get_format(), clip_start as i64),
                    T::from_raw(self.get_format(), clip_stop as i64),
                ))
            } else {
                None
            }
        }
    }

    #[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
    pub fn do_seek<V: Into<T>>(
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

        if T::get_default_format() == Format::Undefined {
            assert_eq!(self.get_format(), start.get_format());
            assert_eq!(self.get_format(), stop.get_format());
        }

        unsafe {
            let mut update = mem::uninitialized();
            let ret = from_glib(ffi::gst_segment_do_seek(
                &mut self.0,
                rate,
                self.get_format().to_glib(),
                flags.to_glib(),
                start_type.to_glib(),
                start.to_raw_value() as u64,
                stop_type.to_glib(),
                stop.to_raw_value() as u64,
                &mut update,
            ));
            if ret {
                Some(from_glib(update))
            } else {
                None
            }
        }
    }

    pub fn offset_running_time(&mut self, offset: i64) -> bool {
        unsafe {
            from_glib(ffi::gst_segment_offset_running_time(
                &mut self.0,
                self.get_format().to_glib(),
                offset,
            ))
        }
    }

    pub fn position_from_running_time<V: Into<T>>(&self, running_time: V) -> T {
        let running_time = running_time.into();

        if T::get_default_format() == Format::Undefined {
            assert_eq!(self.get_format(), running_time.get_format());
        }

        unsafe {
            T::from_raw(
                self.get_format(),
                ffi::gst_segment_position_from_running_time(
                    &self.0,
                    self.get_format().to_glib(),
                    running_time.to_raw_value() as u64,
                ) as i64,
            )
        }
    }

    pub fn position_from_running_time_full<V: Into<T>>(&self, running_time: V) -> (i32, T) {
        let running_time = running_time.into();

        if T::get_default_format() == Format::Undefined {
            assert_eq!(self.get_format(), running_time.get_format());
        }

        unsafe {
            let mut position = mem::uninitialized();
            let ret = ffi::gst_segment_position_from_running_time_full(
                &self.0,
                self.get_format().to_glib(),
                running_time.to_raw_value() as u64,
                &mut position,
            );
            (ret, T::from_raw(self.get_format(), position as i64))
        }
    }

    pub fn position_from_stream_time<V: Into<T>>(&self, stream_time: V) -> T {
        let stream_time = stream_time.into();

        if T::get_default_format() == Format::Undefined {
            assert_eq!(self.get_format(), stream_time.get_format());
        }

        unsafe {
            T::from_raw(
                self.get_format(),
                ffi::gst_segment_position_from_stream_time(
                    &self.0,
                    self.get_format().to_glib(),
                    stream_time.to_raw_value() as u64,
                ) as i64,
            )
        }
    }

    pub fn position_from_stream_time_full<V: Into<T>>(&self, stream_time: V) -> (i32, T) {
        let stream_time = stream_time.into();

        if T::get_default_format() == Format::Undefined {
            assert_eq!(self.get_format(), stream_time.get_format());
        }

        unsafe {
            let mut position = mem::uninitialized();
            let ret = ffi::gst_segment_position_from_stream_time_full(
                &self.0,
                self.get_format().to_glib(),
                stream_time.to_raw_value() as u64,
                &mut position,
            );
            (ret, T::from_raw(self.get_format(), position as i64))
        }
    }

    pub fn set_running_time<V: Into<T>>(&mut self, running_time: V) -> bool {
        let running_time = running_time.into();

        if T::get_default_format() == Format::Undefined {
            assert_eq!(self.get_format(), running_time.get_format());
        }

        unsafe {
            from_glib(ffi::gst_segment_set_running_time(
                &mut self.0,
                self.get_format().to_glib(),
                running_time.to_raw_value() as u64,
            ))
        }
    }

    pub fn to_running_time<V: Into<T>>(&self, position: V) -> T {
        let position = position.into();

        if T::get_default_format() == Format::Undefined {
            assert_eq!(self.get_format(), position.get_format());
        }

        unsafe {
            T::from_raw(
                self.get_format(),
                ffi::gst_segment_to_running_time(
                    &self.0,
                    self.get_format().to_glib(),
                    position.to_raw_value() as u64,
                ) as i64,
            )
        }
    }

    pub fn to_running_time_full<V: Into<T>>(&self, position: V) -> (i32, T) {
        let position = position.into();

        if T::get_default_format() == Format::Undefined {
            assert_eq!(self.get_format(), position.get_format());
        }

        unsafe {
            let mut running_time = mem::uninitialized();
            let ret = ffi::gst_segment_to_running_time_full(
                &self.0,
                self.get_format().to_glib(),
                position.to_raw_value() as u64,
                &mut running_time,
            );
            (ret, T::from_raw(self.get_format(), running_time as i64))
        }
    }

    pub fn to_stream_time<V: Into<T>>(&self, position: V) -> T {
        let position = position.into();

        if T::get_default_format() == Format::Undefined {
            assert_eq!(self.get_format(), position.get_format());
        }

        unsafe {
            T::from_raw(
                self.get_format(),
                ffi::gst_segment_to_stream_time(
                    &self.0,
                    self.get_format().to_glib(),
                    position.to_raw_value() as u64,
                ) as i64,
            )
        }
    }

    pub fn to_stream_time_full<V: Into<T>>(&self, position: V) -> (i32, T) {
        let position = position.into();

        if T::get_default_format() == Format::Undefined {
            assert_eq!(self.get_format(), position.get_format());
        }

        unsafe {
            let mut stream_time = mem::uninitialized();
            let ret = ffi::gst_segment_to_stream_time_full(
                &self.0,
                self.get_format().to_glib(),
                position.to_raw_value() as u64,
                &mut stream_time,
            );
            (ret, T::from_raw(self.get_format(), stream_time as i64))
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

    #[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
    pub fn set_rate(&mut self, rate: f64) {
        assert_ne!(rate, 0.0);
        self.0.rate = rate;
    }

    pub fn get_applied_rate(&self) -> f64 {
        self.0.applied_rate
    }

    #[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
    pub fn set_applied_rate(&mut self, applied_rate: f64) {
        assert_ne!(applied_rate, 0.0);
        self.0.applied_rate = applied_rate;
    }

    pub fn get_format(&self) -> Format {
        from_glib(self.0.format)
    }

    pub fn get_base(&self) -> T {
        unsafe { T::from_raw(self.get_format(), self.0.base as i64) }
    }

    pub fn set_base<V: Into<T>>(&mut self, base: V) {
        let base = base.into();

        if T::get_default_format() == Format::Undefined {
            assert_eq!(self.get_format(), base.get_format());
        }

        self.0.base = unsafe { base.to_raw_value() } as u64;
    }

    pub fn get_offset(&self) -> T {
        unsafe { T::from_raw(self.get_format(), self.0.offset as i64) }
    }

    pub fn set_offset<V: Into<T>>(&mut self, offset: V) {
        let offset = offset.into();

        if T::get_default_format() == Format::Undefined {
            assert_eq!(self.get_format(), offset.get_format());
        }

        self.0.offset = unsafe { offset.to_raw_value() } as u64;
    }

    pub fn get_start(&self) -> T {
        unsafe { T::from_raw(self.get_format(), self.0.start as i64) }
    }

    pub fn set_start<V: Into<T>>(&mut self, start: V) {
        let start = start.into();

        if T::get_default_format() == Format::Undefined {
            assert_eq!(self.get_format(), start.get_format());
        }

        self.0.start = unsafe { start.to_raw_value() } as u64;
    }

    pub fn get_stop(&self) -> T {
        unsafe { T::from_raw(self.get_format(), self.0.stop as i64) }
    }

    pub fn set_stop<V: Into<T>>(&mut self, stop: V) {
        let stop = stop.into();

        if T::get_default_format() == Format::Undefined {
            assert_eq!(self.get_format(), stop.get_format());
        }

        self.0.stop = unsafe { stop.to_raw_value() } as u64;
    }

    pub fn get_time(&self) -> T {
        unsafe { T::from_raw(self.get_format(), self.0.time as i64) }
    }

    pub fn set_time<V: Into<T>>(&mut self, time: V) {
        let time = time.into();

        if T::get_default_format() == Format::Undefined {
            assert_eq!(self.get_format(), time.get_format());
        }

        self.0.time = unsafe { time.to_raw_value() } as u64;
    }

    pub fn get_position(&self) -> T {
        unsafe { T::from_raw(self.get_format(), self.0.position as i64) }
    }

    pub fn set_position<V: Into<T>>(&mut self, position: V) {
        let position = position.into();

        if T::get_default_format() == Format::Undefined {
            assert_eq!(self.get_format(), position.get_format());
        }

        self.0.position = unsafe { position.to_raw_value() } as u64;
    }

    pub fn get_duration(&self) -> T {
        unsafe { T::from_raw(self.get_format(), self.0.duration as i64) }
    }

    pub fn set_duration<V: Into<T>>(&mut self, duration: V) {
        let duration = duration.into();

        if T::get_default_format() == Format::Undefined {
            assert_eq!(self.get_format(), duration.get_format());
        }

        self.0.duration = unsafe { duration.to_raw_value() } as u64;
    }
}

impl<T: FormattedValue> PartialEq for FormattedSegment<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { from_glib(ffi::gst_segment_is_equal(&self.0, &other.0)) }
    }
}

impl<T: FormattedValue> Eq for FormattedSegment<T> {}

unsafe impl<T: FormattedValue> Send for FormattedSegment<T> {}

impl<T: FormattedValue> Clone for FormattedSegment<T> {
    fn clone(&self) -> Self {
        unsafe { FormattedSegment(ptr::read(&self.0), PhantomData) }
    }
}

impl<T: FormattedValue> AsRef<Segment> for FormattedSegment<T> {
    fn as_ref(&self) -> &Segment {
        unsafe { &*(self as *const FormattedSegment<T> as *const FormattedSegment<GenericFormattedValue>) }
    }
}

impl<T: FormattedValue> fmt::Debug for FormattedSegment<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let segment = self.as_ref();
        match segment.get_format() {
            Format::Undefined => f.debug_struct("Segment")
                .field("format", &Format::Undefined)
                .finish(),
            Format::Time => {
                let segment = segment.downcast_ref::<::ClockTime>().unwrap();
                f.debug_struct("Segment")
                    .field("format", &Format::Time)
                    .field("start", &segment.get_start().to_string())
                    .field("offset", &segment.get_offset().to_string())
                    .field("stop", &segment.get_stop().to_string())
                    .field("rate", &segment.get_rate())
                    .field("applied_rate", &segment.get_applied_rate())
                    .field("flags", &segment.get_flags())
                    .field("time", &segment.get_time().to_string())
                    .field("base", &segment.get_base().to_string())
                    .field("position", &segment.get_position().to_string())
                    .field("duration", &segment.get_duration().to_string())
                    .finish()
            }
            _ => f.debug_struct("Segment")
                .field("format", &segment.get_format())
                .field("start", &segment.get_start())
                .field("offset", &segment.get_offset())
                .field("stop", &segment.get_stop())
                .field("rate", &segment.get_rate())
                .field("applied_rate", &segment.get_applied_rate())
                .field("flags", &segment.get_flags())
                .field("time", &segment.get_time())
                .field("base", &segment.get_base())
                .field("position", &segment.get_position())
                .field("duration", &segment.get_duration())
                .finish(),
        }
    }
}

impl<T: FormattedValue> Default for FormattedSegment<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: FormattedValue> glib::types::StaticType for FormattedSegment<T> {
    fn static_type() -> glib::types::Type {
        unsafe { glib::translate::from_glib(ffi::gst_segment_get_type()) }
    }
}

#[doc(hidden)]
impl<'a> glib::value::FromValueOptional<'a> for Segment {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Option::<Segment>::from_glib_none(gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *mut ffi::GstSegment)
    }
}

#[doc(hidden)]
impl<T: FormattedValue> glib::value::SetValue for FormattedSegment<T> {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstSegment>::to_glib_none(this).0
                as glib_ffi::gpointer,
        )
    }
}

#[doc(hidden)]
impl<T: FormattedValue> glib::value::SetValueOptional for FormattedSegment<T> {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstSegment>::to_glib_none(&this).0
                as glib_ffi::gpointer,
        )
    }
}

#[doc(hidden)]
impl<T: FormattedValue> glib::translate::GlibPtrDefault for FormattedSegment<T> {
    type GlibType = *mut ffi::GstSegment;
}

#[doc(hidden)]
impl<'a, T: FormattedValue> glib::translate::ToGlibPtr<'a, *const ffi::GstSegment>
    for FormattedSegment<T>
{
    type Storage = &'a FormattedSegment<T>;

    fn to_glib_none(&'a self) -> glib::translate::Stash<'a, *const ffi::GstSegment, Self> {
        glib::translate::Stash(&self.0, self)
    }

    fn to_glib_full(&self) -> *const ffi::GstSegment {
        unimplemented!()
    }
}

#[doc(hidden)]
impl<'a, T: FormattedValue> glib::translate::ToGlibPtrMut<'a, *mut ffi::GstSegment>
    for FormattedSegment<T>
{
    type Storage = &'a mut FormattedSegment<T>;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> glib::translate::StashMut<'a, *mut ffi::GstSegment, Self> {
        glib::translate::StashMut(&mut self.0, self)
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*const ffi::GstSegment> for Segment {
    #[inline]
    unsafe fn from_glib_none(ptr: *const ffi::GstSegment) -> Self {
        FormattedSegment(ptr::read(ptr), PhantomData)
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*mut ffi::GstSegment> for Segment {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstSegment) -> Self {
        FormattedSegment(ptr::read(ptr), PhantomData)
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrBorrow<*mut ffi::GstSegment> for Segment {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::GstSegment) -> Self {
        FormattedSegment(ptr::read(ptr), PhantomData)
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
