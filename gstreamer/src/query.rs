// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use miniobject::*;
use structure::*;
use GenericFormattedValue;

use std::ptr;
use std::mem;
use std::fmt;
use std::ffi::CStr;
use std::ops::Deref;

use glib;
use glib::translate::{from_glib, from_glib_full, ToGlib, ToGlibPtr};

#[repr(C)]
pub struct QueryRef(ffi::GstQuery);

unsafe impl Send for QueryRef {}
unsafe impl Sync for QueryRef {}

pub type Query = GstRc<QueryRef>;

unsafe impl MiniObject for QueryRef {
    type GstType = ffi::GstQuery;
}

impl GstRc<QueryRef> {
    pub fn new_position(fmt: ::Format) -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_query_new_position(fmt.to_glib())) }
    }

    pub fn new_duration(fmt: ::Format) -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_query_new_duration(fmt.to_glib())) }
    }

    pub fn new_latency() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_query_new_latency()) }
    }

    pub fn new_seeking(fmt: ::Format) -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_query_new_seeking(fmt.to_glib())) }
    }

    pub fn new_segment(fmt: ::Format) -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_query_new_segment(fmt.to_glib())) }
    }

    pub fn new_convert<V: Into<GenericFormattedValue>>(value: V, dest_fmt: ::Format) -> Self {
        assert_initialized_main_thread!();
        let value = value.into();
        unsafe {
            from_glib_full(ffi::gst_query_new_convert(
                value.get_format().to_glib(),
                value.get_value(),
                dest_fmt.to_glib(),
            ))
        }
    }

    pub fn new_formats() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_query_new_formats()) }
    }

    pub fn new_buffering(fmt: ::Format) -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_query_new_buffering(fmt.to_glib())) }
    }

    pub fn new_custom(structure: ::Structure) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_query_new_custom(
                ffi::GST_QUERY_CUSTOM,
                structure.into_ptr(),
            ))
        }
    }

    pub fn new_uri() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_query_new_uri()) }
    }

    pub fn new_scheduling() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_query_new_scheduling()) }
    }

    pub fn new_accept_caps(caps: &::Caps) -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_query_new_accept_caps(caps.as_mut_ptr())) }
    }

    pub fn new_caps<'a, P: Into<Option<&'a ::Caps>>>(filter: P) -> Self {
        assert_initialized_main_thread!();
        let filter = filter.into();
        unsafe { from_glib_full(ffi::gst_query_new_caps(filter.to_glib_none().0)) }
    }

    pub fn new_drain() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_query_new_drain()) }
    }

    pub fn new_context(context_type: &str) -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_query_new_context(context_type.to_glib_none().0)) }
    }
}

impl QueryRef {
    pub fn get_structure(&self) -> Option<&StructureRef> {
        unsafe {
            let structure = ffi::gst_query_get_structure(self.as_mut_ptr());
            if structure.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(structure))
            }
        }
    }

    pub fn get_mut_structure(&mut self) -> &mut StructureRef {
        unsafe {
            let structure = ffi::gst_query_writable_structure(self.as_mut_ptr());
            StructureRef::from_glib_borrow_mut(structure)
        }
    }

    pub fn is_downstream(&self) -> bool {
        unsafe {
            ((*self.as_ptr()).type_ as u32) & (ffi::GST_QUERY_TYPE_DOWNSTREAM.bits()) != 0
        }
    }

    pub fn is_upstream(&self) -> bool {
        unsafe {
            ((*self.as_ptr()).type_ as u32) & (ffi::GST_QUERY_TYPE_UPSTREAM.bits()) != 0
        }
    }

    pub fn is_serialized(&self) -> bool {
        unsafe {
            ((*self.as_ptr()).type_ as u32) & (ffi::GST_QUERY_TYPE_SERIALIZED.bits()) != 0
        }
    }

    pub fn view(&self) -> QueryView<&Self> {
        let type_ = unsafe { (*self.as_ptr()).type_ };

        match type_ {
            ffi::GST_QUERY_POSITION => QueryView::Position(Position(self)),
            ffi::GST_QUERY_DURATION => QueryView::Duration(Duration(self)),
            ffi::GST_QUERY_LATENCY => QueryView::Latency(Latency(self)),
            ffi::GST_QUERY_JITTER => QueryView::Jitter(Jitter(self)),
            ffi::GST_QUERY_RATE => QueryView::Rate(Rate(self)),
            ffi::GST_QUERY_SEEKING => QueryView::Seeking(Seeking(self)),
            ffi::GST_QUERY_SEGMENT => QueryView::Segment(Segment(self)),
            ffi::GST_QUERY_CONVERT => QueryView::Convert(Convert(self)),
            ffi::GST_QUERY_FORMATS => QueryView::Formats(Formats(self)),
            ffi::GST_QUERY_BUFFERING => QueryView::Buffering(Buffering(self)),
            ffi::GST_QUERY_CUSTOM => QueryView::Custom(Custom(self)),
            ffi::GST_QUERY_URI => QueryView::Uri(Uri(self)),
            ffi::GST_QUERY_ALLOCATION => QueryView::Allocation(Allocation(self)),
            ffi::GST_QUERY_SCHEDULING => QueryView::Scheduling(Scheduling(self)),
            ffi::GST_QUERY_ACCEPT_CAPS => QueryView::AcceptCaps(AcceptCaps(self)),
            ffi::GST_QUERY_CAPS => QueryView::Caps(Caps(self)),
            ffi::GST_QUERY_DRAIN => QueryView::Drain(Drain(self)),
            ffi::GST_QUERY_CONTEXT => QueryView::Context(Context(self)),
            _ => QueryView::Other(Other(self)),
        }
    }

    pub fn view_mut(&mut self) -> QueryView<&mut Self> {
        unsafe { mem::transmute(self.view()) }
    }
}

impl glib::types::StaticType for QueryRef {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_query_get_type()) }
    }
}

impl fmt::Debug for QueryRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Query")
            .field("type", &unsafe {
                let type_ = ffi::gst_query_type_get_name((*self.as_ptr()).type_);
                CStr::from_ptr(type_).to_str().unwrap()
            })
            .field("structure", &self.get_structure())
            .finish()
    }
}

impl ToOwned for QueryRef {
    type Owned = GstRc<QueryRef>;

    fn to_owned(&self) -> GstRc<QueryRef> {
        unsafe {
            from_glib_full(ffi::gst_mini_object_copy(self.as_ptr() as *const _)
                as *mut _)
        }
    }
}

pub enum QueryView<T> {
    Position(Position<T>),
    Duration(Duration<T>),
    Latency(Latency<T>),
    Jitter(Jitter<T>),
    Rate(Rate<T>),
    Seeking(Seeking<T>),
    Segment(Segment<T>),
    Convert(Convert<T>),
    Formats(Formats<T>),
    Buffering(Buffering<T>),
    Custom(Custom<T>),
    Uri(Uri<T>),
    Allocation(Allocation<T>),
    Scheduling(Scheduling<T>),
    AcceptCaps(AcceptCaps<T>),
    Caps(Caps<T>),
    Drain(Drain<T>),
    Context(Context<T>),
    Other(Other<T>),
    __NonExhaustive,
}

macro_rules! declare_concrete_query(
    ($name:ident, $param:ident) => {
        pub struct $name<$param>($param);

        impl<'a> $name<&'a QueryRef> {
            pub fn get_query(&self) -> &QueryRef {
                self.0
            }
        }

        impl<'a> Deref for $name<&'a QueryRef> {
            type Target = QueryRef;

            fn deref(&self) -> &Self::Target {
                self.0
            }
        }

        impl<'a> $name<&'a mut QueryRef> {
            pub fn get_mut_query(&mut self) -> &mut QueryRef {
                self.0
            }
        }

        impl<'a> Deref for $name<&'a mut QueryRef> {
            type Target = $name<&'a QueryRef>;

            fn deref(&self) -> &Self::Target {
                unsafe {
                    mem::transmute(self)
                }
            }
        }
    }
);

declare_concrete_query!(Position, T);
impl<'a> Position<&'a QueryRef> {
    pub fn get_result(&self) -> GenericFormattedValue {
        unsafe {
            let mut fmt = mem::uninitialized();
            let mut pos = mem::uninitialized();

            ffi::gst_query_parse_position(self.as_mut_ptr(), &mut fmt, &mut pos);

            GenericFormattedValue::new(from_glib(fmt), pos)
        }
    }

    pub fn get_format(&self) -> ::Format {
        unsafe {
            let mut fmt = mem::uninitialized();

            ffi::gst_query_parse_position(self.as_mut_ptr(), &mut fmt, ptr::null_mut());

            from_glib(fmt)
        }
    }
}

impl<'a> Position<&'a mut QueryRef> {
    pub fn set<V: Into<GenericFormattedValue>>(&mut self, pos: V) {
        let pos = pos.into();
        assert_eq!(pos.get_format(), self.get_format());
        unsafe {
            ffi::gst_query_set_position(
                self.as_mut_ptr(),
                pos.get_format().to_glib(),
                pos.get_value(),
            );
        }
    }
}

declare_concrete_query!(Duration, T);
impl<'a> Duration<&'a QueryRef> {
    pub fn get_result(&self) -> GenericFormattedValue {
        unsafe {
            let mut fmt = mem::uninitialized();
            let mut pos = mem::uninitialized();

            ffi::gst_query_parse_duration(self.as_mut_ptr(), &mut fmt, &mut pos);

            GenericFormattedValue::new(from_glib(fmt), pos)
        }
    }

    pub fn get_format(&self) -> ::Format {
        unsafe {
            let mut fmt = mem::uninitialized();

            ffi::gst_query_parse_duration(self.as_mut_ptr(), &mut fmt, ptr::null_mut());

            from_glib(fmt)
        }
    }
}

impl<'a> Duration<&'a mut QueryRef> {
    pub fn set<V: Into<GenericFormattedValue>>(&mut self, dur: V) {
        let dur = dur.into();
        assert_eq!(dur.get_format(), self.get_format());
        unsafe {
            ffi::gst_query_set_duration(
                self.as_mut_ptr(),
                dur.get_format().to_glib(),
                dur.get_value(),
            );
        }
    }
}

declare_concrete_query!(Latency, T);
impl<'a> Latency<&'a QueryRef> {
    pub fn get_result(&self) -> (bool, ::ClockTime, ::ClockTime) {
        unsafe {
            let mut live = mem::uninitialized();
            let mut min = mem::uninitialized();
            let mut max = mem::uninitialized();

            ffi::gst_query_parse_latency(self.as_mut_ptr(), &mut live, &mut min, &mut max);

            (from_glib(live), from_glib(min), from_glib(max))
        }
    }
}

impl<'a> Latency<&'a mut QueryRef> {
    pub fn set(&mut self, live: bool, min: ::ClockTime, max: ::ClockTime) {
        unsafe {
            ffi::gst_query_set_latency(
                self.as_mut_ptr(),
                live.to_glib(),
                min.to_glib(),
                max.to_glib(),
            );
        }
    }
}

declare_concrete_query!(Jitter, T);
declare_concrete_query!(Rate, T);

declare_concrete_query!(Seeking, T);
impl<'a> Seeking<&'a QueryRef> {
    pub fn get_result(&self) -> (bool, GenericFormattedValue, GenericFormattedValue) {
        unsafe {
            let mut fmt = mem::uninitialized();
            let mut seekable = mem::uninitialized();
            let mut start = mem::uninitialized();
            let mut end = mem::uninitialized();
            ffi::gst_query_parse_seeking(
                self.as_mut_ptr(),
                &mut fmt,
                &mut seekable,
                &mut start,
                &mut end,
            );

            (
                from_glib(seekable),
                GenericFormattedValue::new(from_glib(fmt), start),
                GenericFormattedValue::new(from_glib(fmt), end),
            )
        }
    }

    pub fn get_format(&self) -> ::Format {
        unsafe {
            let mut fmt = mem::uninitialized();
            ffi::gst_query_parse_seeking(
                self.as_mut_ptr(),
                &mut fmt,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );

            from_glib(fmt)
        }
    }
}

impl<'a> Seeking<&'a mut QueryRef> {
    pub fn set<V: Into<GenericFormattedValue>>(&mut self, seekable: bool, start: V, end: V) {
        let start = start.into();
        let end = end.into();

        assert_eq!(self.get_format(), start.get_format());
        assert_eq!(start.get_format(), end.get_format());

        unsafe {
            ffi::gst_query_set_seeking(
                self.as_mut_ptr(),
                start.get_format().to_glib(),
                seekable.to_glib(),
                start.get_value(),
                end.get_value(),
            );
        }
    }
}

declare_concrete_query!(Segment, T);
impl<'a> Segment<&'a QueryRef> {
    pub fn get_result(&self) -> (f64, GenericFormattedValue, GenericFormattedValue) {
        unsafe {
            let mut rate = mem::uninitialized();
            let mut fmt = mem::uninitialized();
            let mut start = mem::uninitialized();
            let mut stop = mem::uninitialized();

            ffi::gst_query_parse_segment(
                self.as_mut_ptr(),
                &mut rate,
                &mut fmt,
                &mut start,
                &mut stop,
            );
            (
                rate,
                GenericFormattedValue::new(from_glib(fmt), start),
                GenericFormattedValue::new(from_glib(fmt), stop),
            )
        }
    }

    pub fn get_format(&self) -> ::Format {
        unsafe {
            let mut fmt = mem::uninitialized();

            ffi::gst_query_parse_segment(
                self.as_mut_ptr(),
                ptr::null_mut(),
                &mut fmt,
                ptr::null_mut(),
                ptr::null_mut(),
            );
            from_glib(fmt)
        }
    }
}

impl<'a> Segment<&'a mut QueryRef> {
    pub fn set<V: Into<GenericFormattedValue>>(&mut self, rate: f64, start: V, stop: V) {
        let start = start.into();
        let stop = stop.into();

        assert_eq!(start.get_format(), stop.get_format());

        unsafe {
            ffi::gst_query_set_segment(
                self.as_mut_ptr(),
                rate,
                start.get_format().to_glib(),
                start.get_value(),
                stop.get_value(),
            );
        }
    }
}

declare_concrete_query!(Convert, T);
impl<'a> Convert<&'a QueryRef> {
    pub fn get_result(&self) -> (GenericFormattedValue, GenericFormattedValue) {
        unsafe {
            let mut src_fmt = mem::uninitialized();
            let mut src = mem::uninitialized();
            let mut dest_fmt = mem::uninitialized();
            let mut dest = mem::uninitialized();

            ffi::gst_query_parse_convert(
                self.as_mut_ptr(),
                &mut src_fmt,
                &mut src,
                &mut dest_fmt,
                &mut dest,
            );
            (
                GenericFormattedValue::new(from_glib(src_fmt), src),
                GenericFormattedValue::new(from_glib(dest_fmt), dest),
            )
        }
    }

    pub fn get(&self) -> (GenericFormattedValue, ::Format) {
        unsafe {
            let mut src_fmt = mem::uninitialized();
            let mut src = mem::uninitialized();
            let mut dest_fmt = mem::uninitialized();

            ffi::gst_query_parse_convert(
                self.as_mut_ptr(),
                &mut src_fmt,
                &mut src,
                &mut dest_fmt,
                ptr::null_mut(),
            );
            (
                GenericFormattedValue::new(from_glib(src_fmt), src),
                from_glib(dest_fmt),
            )
        }
    }
}

impl<'a> Convert<&'a mut QueryRef> {
    pub fn set<V: Into<GenericFormattedValue>>(&mut self, src: V, dest: V) {
        let src = src.into();
        let dest = dest.into();

        unsafe {
            ffi::gst_query_set_convert(
                self.as_mut_ptr(),
                src.get_format().to_glib(),
                src.get_value(),
                dest.get_format().to_glib(),
                dest.get_value(),
            );
        }
    }
}

declare_concrete_query!(Formats, T);
impl<'a> Formats<&'a QueryRef> {
    pub fn get_result(&self) -> Vec<::Format> {
        unsafe {
            let mut n = mem::uninitialized();
            ffi::gst_query_parse_n_formats(self.as_mut_ptr(), &mut n);
            let mut res = Vec::with_capacity(n as usize);

            for i in 0..n {
                let mut fmt = mem::uninitialized();
                ffi::gst_query_parse_nth_format(self.as_mut_ptr(), i, &mut fmt);
                res.push(from_glib(fmt));
            }

            res
        }
    }
}

impl<'a> Formats<&'a mut QueryRef> {
    pub fn set(&mut self, formats: &[::Format]) {
        unsafe {
            let v: Vec<_> = formats.iter().map(|f| f.to_glib()).collect();
            ffi::gst_query_set_formatsv(self.as_mut_ptr(), v.len() as i32, v.as_ptr() as *mut _);
        }
    }
}

declare_concrete_query!(Buffering, T);
impl<'a> Buffering<&'a QueryRef> {
    pub fn get_format(&self) -> ::Format {
        unsafe {
            let mut fmt = mem::uninitialized();

            ffi::gst_query_parse_buffering_range(
                self.as_mut_ptr(),
                &mut fmt,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );

            from_glib(fmt)
        }
    }

    pub fn get_percent(&self) -> (bool, i32) {
        unsafe {
            let mut busy = mem::uninitialized();
            let mut percent = mem::uninitialized();

            ffi::gst_query_parse_buffering_percent(self.as_mut_ptr(), &mut busy, &mut percent);

            (from_glib(busy), percent)
        }
    }

    pub fn get_range(&self) -> (GenericFormattedValue, GenericFormattedValue, i64) {
        unsafe {
            let mut fmt = mem::uninitialized();
            let mut start = mem::uninitialized();
            let mut stop = mem::uninitialized();
            let mut estimated_total = mem::uninitialized();

            ffi::gst_query_parse_buffering_range(
                self.as_mut_ptr(),
                &mut fmt,
                &mut start,
                &mut stop,
                &mut estimated_total,
            );
            (
                GenericFormattedValue::new(from_glib(fmt), start),
                GenericFormattedValue::new(from_glib(fmt), stop),
                estimated_total,
            )
        }
    }

    pub fn get_stats(&self) -> (::BufferingMode, i32, i32, i64) {
        unsafe {
            let mut mode = mem::uninitialized();
            let mut avg_in = mem::uninitialized();
            let mut avg_out = mem::uninitialized();
            let mut buffering_left = mem::uninitialized();

            ffi::gst_query_parse_buffering_stats(
                self.as_mut_ptr(),
                &mut mode,
                &mut avg_in,
                &mut avg_out,
                &mut buffering_left,
            );

            (from_glib(mode), avg_in, avg_out, buffering_left)
        }
    }

    pub fn get_ranges(&self) -> Vec<(GenericFormattedValue, GenericFormattedValue)> {
        unsafe {
            let mut fmt = mem::uninitialized();
            ffi::gst_query_parse_buffering_range(
                self.as_mut_ptr(),
                &mut fmt,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );
            let fmt = from_glib(fmt);

            let n = ffi::gst_query_get_n_buffering_ranges(self.as_mut_ptr());
            let mut res = Vec::with_capacity(n as usize);
            for i in 0..n {
                let mut start = mem::uninitialized();
                let mut stop = mem::uninitialized();
                let s: bool = from_glib(ffi::gst_query_parse_nth_buffering_range(
                    self.as_mut_ptr(),
                    i,
                    &mut start,
                    &mut stop,
                ));
                if s {
                    res.push((
                        GenericFormattedValue::new(fmt, start),
                        GenericFormattedValue::new(fmt, stop),
                    ));
                }
            }

            res
        }
    }
}

impl<'a> Buffering<&'a mut QueryRef> {
    pub fn set_percent(&mut self, busy: bool, percent: i32) {
        unsafe {
            ffi::gst_query_set_buffering_percent(self.as_mut_ptr(), busy.to_glib(), percent);
        }
    }

    pub fn set_range<V: Into<GenericFormattedValue>>(
        &mut self,
        start: V,
        stop: V,
        estimated_total: i64,
    ) {
        let start = start.into();
        let stop = stop.into();

        assert_eq!(self.get_format(), start.get_format());
        assert_eq!(start.get_format(), stop.get_format());

        unsafe {
            ffi::gst_query_set_buffering_range(
                self.as_mut_ptr(),
                start.get_format().to_glib(),
                start.get_value(),
                stop.get_value(),
                estimated_total,
            );
        }
    }

    pub fn set_stats(
        &mut self,
        mode: ::BufferingMode,
        avg_in: i32,
        avg_out: i32,
        buffering_left: i64,
    ) {
        skip_assert_initialized!();
        unsafe {
            ffi::gst_query_set_buffering_stats(
                self.as_mut_ptr(),
                mode.to_glib(),
                avg_in,
                avg_out,
                buffering_left,
            );
        }
    }

    pub fn add_buffering_ranges<V: Into<GenericFormattedValue> + Copy>(
        &mut self,
        ranges: &[(V, V)],
    ) {
        unsafe {
            let fmt = self.get_format();

            for &(start, stop) in ranges {
                let start = start.into();
                let stop = stop.into();
                assert_eq!(start.get_format(), fmt);
                assert_eq!(stop.get_format(), fmt);
                ffi::gst_query_add_buffering_range(
                    self.as_mut_ptr(),
                    start.get_value(),
                    stop.get_value(),
                );
            }
        }
    }
}

declare_concrete_query!(Custom, T);

declare_concrete_query!(Uri, T);
impl<'a> Uri<&'a QueryRef> {
    pub fn get_uri(&self) -> Option<String> {
        unsafe {
            let mut uri = ptr::null_mut();
            ffi::gst_query_parse_uri(self.as_mut_ptr(), &mut uri);
            from_glib_full(uri)
        }
    }

    pub fn get_redirection(&self) -> (Option<String>, bool) {
        unsafe {
            let mut uri = ptr::null_mut();
            ffi::gst_query_parse_uri_redirection(self.as_mut_ptr(), &mut uri);
            let mut permanent = mem::uninitialized();
            ffi::gst_query_parse_uri_redirection_permanent(self.as_mut_ptr(), &mut permanent);

            (from_glib_full(uri), from_glib(permanent))
        }
    }
}

impl<'a> Uri<&'a mut QueryRef> {
    pub fn set_uri<'b, T: Into<&'b str>>(&mut self, uri: T) {
        let uri = uri.into();
        unsafe {
            ffi::gst_query_set_uri(self.as_mut_ptr(), uri.to_glib_none().0);
        }
    }

    pub fn set_redirection<'b, T: Into<&'b str>>(&mut self, uri: T, permanent: bool) {
        let uri = uri.into();
        unsafe {
            ffi::gst_query_set_uri_redirection(self.as_mut_ptr(), uri.to_glib_none().0);
            ffi::gst_query_set_uri_redirection_permanent(self.as_mut_ptr(), permanent.to_glib());
        }
    }
}

// TODO
declare_concrete_query!(Allocation, T);

declare_concrete_query!(Scheduling, T);
impl<'a> Scheduling<&'a QueryRef> {
    pub fn has_scheduling_mode(&self, mode: ::PadMode) -> bool {
        unsafe {
            from_glib(ffi::gst_query_has_scheduling_mode(
                self.as_mut_ptr(),
                mode.to_glib(),
            ))
        }
    }

    pub fn has_scheduling_mode_with_flags(
        &self,
        mode: ::PadMode,
        flags: ::SchedulingFlags,
    ) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gst_query_has_scheduling_mode_with_flags(
                self.as_mut_ptr(),
                mode.to_glib(),
                flags.to_glib(),
            ))
        }
    }

    pub fn get_scheduling_modes(&self) -> Vec<::PadMode> {
        unsafe {
            let n = ffi::gst_query_get_n_scheduling_modes(self.as_mut_ptr());
            let mut res = Vec::with_capacity(n as usize);
            for i in 0..n {
                res.push(from_glib(ffi::gst_query_parse_nth_scheduling_mode(
                    self.as_mut_ptr(),
                    i,
                )));
            }

            res
        }
    }

    pub fn get_result(&self) -> (::SchedulingFlags, i32, i32, i32) {
        unsafe {
            let mut flags = mem::uninitialized();
            let mut minsize = mem::uninitialized();
            let mut maxsize = mem::uninitialized();
            let mut align = mem::uninitialized();

            ffi::gst_query_parse_scheduling(
                self.as_mut_ptr(),
                &mut flags,
                &mut minsize,
                &mut maxsize,
                &mut align,
            );

            (from_glib(flags), minsize, maxsize, align)
        }
    }
}

impl<'a> Scheduling<&'a mut QueryRef> {
    pub fn add_scheduling_modes(&mut self, modes: &[::PadMode]) {
        unsafe {
            for mode in modes {
                ffi::gst_query_add_scheduling_mode(self.as_mut_ptr(), mode.to_glib());
            }
        }
    }

    pub fn set(&mut self, flags: ::SchedulingFlags, minsize: i32, maxsize: i32, align: i32) {
        unsafe {
            ffi::gst_query_set_scheduling(
                self.as_mut_ptr(),
                flags.to_glib(),
                minsize,
                maxsize,
                align,
            );
        }
    }
}

declare_concrete_query!(AcceptCaps, T);
impl<'a> AcceptCaps<&'a QueryRef> {
    pub fn get_caps(&self) -> &::CapsRef {
        unsafe {
            let mut caps = ptr::null_mut();
            ffi::gst_query_parse_accept_caps(self.as_mut_ptr(), &mut caps);
            ::CapsRef::from_ptr(caps)
        }
    }

    pub fn get_result(&self) -> bool {
        unsafe {
            let mut accepted = mem::uninitialized();
            ffi::gst_query_parse_accept_caps_result(self.as_mut_ptr(), &mut accepted);
            from_glib(accepted)
        }
    }
}

impl<'a> AcceptCaps<&'a mut QueryRef> {
    pub fn set_result(&mut self, accepted: bool) {
        unsafe {
            ffi::gst_query_set_accept_caps_result(self.as_mut_ptr(), accepted.to_glib());
        }
    }
}

declare_concrete_query!(Caps, T);
impl<'a> Caps<&'a QueryRef> {
    pub fn get_filter(&self) -> Option<&::CapsRef> {
        unsafe {
            let mut caps = ptr::null_mut();
            ffi::gst_query_parse_caps(self.as_mut_ptr(), &mut caps);
            if caps.is_null() {
                None
            } else {
                Some(::CapsRef::from_ptr(caps))
            }
        }
    }

    pub fn get_result(&self) -> Option<&::CapsRef> {
        unsafe {
            let mut caps = ptr::null_mut();
            ffi::gst_query_parse_caps_result(self.as_mut_ptr(), &mut caps);
            if caps.is_null() {
                None
            } else {
                Some(::CapsRef::from_ptr(caps))
            }
        }
    }
}

impl<'a> Caps<&'a mut QueryRef> {
    pub fn set_result(&mut self, caps: &::Caps) {
        unsafe {
            ffi::gst_query_set_caps_result(self.as_mut_ptr(), caps.as_mut_ptr());
        }
    }
}

declare_concrete_query!(Drain, T);

declare_concrete_query!(Context, T);
impl<'a> Context<&'a QueryRef> {
    pub fn get_context(&self) -> Option<&::ContextRef> {
        unsafe {
            let mut context = ptr::null_mut();
            ffi::gst_query_parse_context(self.as_mut_ptr(), &mut context);
            if context.is_null() {
                None
            } else {
                Some(::ContextRef::from_ptr(context))
            }
        }
    }

    pub fn get_context_type(&self) -> &str {
        unsafe {
            let mut context_type = ptr::null();
            ffi::gst_query_parse_context_type(self.as_mut_ptr(), &mut context_type);
            CStr::from_ptr(context_type).to_str().unwrap()
        }
    }
}

impl<'a> Context<&'a mut QueryRef> {
    pub fn set_context(&mut self, context: &::Context) {
        unsafe {
            ffi::gst_query_set_context(self.as_mut_ptr(), context.as_mut_ptr());
        }
    }
}

declare_concrete_query!(Other, T);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_writability() {
        ::init().unwrap();

        let mut q = Query::new_position(::Format::Time);

        match q.view() {
            QueryView::Position(ref p) => {
                let fmt = p.get_format();
                assert_eq!(fmt, ::Format::Time);
                assert!(!p.is_serialized());
            }
            _ => (),
        }

        match q.get_mut().unwrap().view_mut() {
            QueryView::Position(ref mut p) => {
                let pos = p.get_result();
                assert_eq!(pos.try_into_time(), Ok(::CLOCK_TIME_NONE));
                p.set(2 * ::SECOND);
            }
            _ => (),
        }

        match q.view() {
            QueryView::Position(ref p) => {
                let pos = p.get_result();
                assert_eq!(pos.try_into_time(), Ok(2 * ::SECOND));
            }
            _ => (),
        }
    }
}
