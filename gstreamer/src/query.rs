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

use std::ptr;
use std::mem;
use std::ffi::CStr;
use std::ops::Deref;

use glib;
use glib::translate::{from_glib, from_glib_full, ToGlib, ToGlibPtr};

#[repr(C)]
pub struct QueryRef(ffi::GstQuery);

pub type Query = GstRc<QueryRef>;

unsafe impl MiniObject for QueryRef {
    type GstType = ffi::GstQuery;
}

impl Query {
    pub fn new_position(fmt: ::Format) -> Self {
        unsafe { from_glib_full(ffi::gst_query_new_position(fmt.to_glib())) }
    }

    pub fn new_duration(fmt: ::Format) -> Self {
        unsafe { from_glib_full(ffi::gst_query_new_duration(fmt.to_glib())) }
    }

    pub fn new_latency() -> Self {
        unsafe { from_glib_full(ffi::gst_query_new_latency()) }
    }

    pub fn new_seeking(fmt: ::Format) -> Self {
        unsafe { from_glib_full(ffi::gst_query_new_seeking(fmt.to_glib())) }
    }

    pub fn new_segment(fmt: ::Format) -> Self {
        unsafe { from_glib_full(ffi::gst_query_new_segment(fmt.to_glib())) }
    }

    pub fn new_convert(src_fmt: ::Format, value: i64, dest_fmt: ::Format) -> Self {
        unsafe {
            from_glib_full(ffi::gst_query_new_convert(
                src_fmt.to_glib(),
                value,
                dest_fmt.to_glib(),
            ))
        }
    }

    pub fn new_formats() -> Self {
        unsafe { from_glib_full(ffi::gst_query_new_formats()) }
    }

    pub fn new_buffering(fmt: ::Format) -> Self {
        unsafe { from_glib_full(ffi::gst_query_new_buffering(fmt.to_glib())) }
    }

    pub fn new_custom(structure: ::Structure) -> Self {
        unsafe {
            from_glib_full(ffi::gst_query_new_custom(
                ffi::GST_QUERY_CUSTOM,
                structure.into_ptr(),
            ))
        }
    }

    pub fn new_uri() -> Self {
        unsafe { from_glib_full(ffi::gst_query_new_uri()) }
    }

    pub fn new_scheduling() -> Self {
        unsafe { from_glib_full(ffi::gst_query_new_scheduling()) }
    }

    pub fn new_accept_caps(caps: &::Caps) -> Self {
        unsafe { from_glib_full(ffi::gst_query_new_accept_caps(caps.as_mut_ptr())) }
    }

    pub fn new_caps(filter: &::Caps) -> Self {
        unsafe { from_glib_full(ffi::gst_query_new_caps(filter.as_mut_ptr())) }
    }

    pub fn new_drain() -> Self {
        unsafe { from_glib_full(ffi::gst_query_new_drain()) }
    }

    pub fn new_context(context_type: &str) -> Self {
        unsafe { from_glib_full(ffi::gst_query_new_context(context_type.to_glib_none().0)) }
    }
}

impl QueryRef {
    pub fn get_structure(&self) -> &StructureRef {
        unsafe {
            let structure = ffi::gst_query_get_structure(self.as_mut_ptr());
            StructureRef::from_glib_borrow(structure)
        }
    }

    pub fn get_mut_structure(&mut self) -> &mut StructureRef {
        unsafe {
            let structure = ffi::gst_query_writable_structure(self.as_mut_ptr());
            StructureRef::from_glib_borrow_mut(structure)
        }
    }

    pub fn is_downstream(&self) -> bool {
        unsafe { ((*self.as_ptr()).type_ as u32) & (ffi::GST_QUERY_TYPE_DOWNSTREAM.bits()) != 0 }
    }

    pub fn is_upstream(&self) -> bool {
        unsafe { ((*self.as_ptr()).type_ as u32) & (ffi::GST_QUERY_TYPE_UPSTREAM.bits()) != 0 }
    }

    pub fn is_serialized(&self) -> bool {
        unsafe { ((*self.as_ptr()).type_ as u32) & (ffi::GST_QUERY_TYPE_SERIALIZED.bits()) != 0 }
    }

    pub fn view(&self) -> QueryView<&Self> {
        let type_ = unsafe { (*self.as_ptr()).type_ };

        if type_ == ffi::GST_QUERY_POSITION {
            QueryView::Position(Position(self))
        } else if type_ == ffi::GST_QUERY_DURATION {
            QueryView::Duration(Duration(self))
        } else if type_ == ffi::GST_QUERY_LATENCY {
            QueryView::Latency(Latency(self))
        } else if type_ == ffi::GST_QUERY_JITTER {
            QueryView::Jitter(Jitter(self))
        } else if type_ == ffi::GST_QUERY_RATE {
            QueryView::Rate(Rate(self))
        } else if type_ == ffi::GST_QUERY_SEEKING {
            QueryView::Seeking(Seeking(self))
        } else if type_ == ffi::GST_QUERY_SEGMENT {
            QueryView::Segment(Segment(self))
        } else if type_ == ffi::GST_QUERY_CONVERT {
            QueryView::Convert(Convert(self))
        } else if type_ == ffi::GST_QUERY_FORMATS {
            QueryView::Formats(Formats(self))
        } else if type_ == ffi::GST_QUERY_BUFFERING {
            QueryView::Buffering(Buffering(self))
        } else if type_ == ffi::GST_QUERY_CUSTOM {
            QueryView::Custom(Custom(self))
        } else if type_ == ffi::GST_QUERY_URI {
            QueryView::Uri(Uri(self))
        } else if type_ == ffi::GST_QUERY_ALLOCATION {
            QueryView::Allocation(Allocation(self))
        } else if type_ == ffi::GST_QUERY_SCHEDULING {
            QueryView::Scheduling(Scheduling(self))
        } else if type_ == ffi::GST_QUERY_ACCEPT_CAPS {
            QueryView::AcceptCaps(AcceptCaps(self))
        } else if type_ == ffi::GST_QUERY_CAPS {
            QueryView::Caps(Caps(self))
        } else if type_ == ffi::GST_QUERY_DRAIN {
            QueryView::Drain(Drain(self))
        } else if type_ == ffi::GST_QUERY_CONTEXT {
            QueryView::Context(Context(self))
        } else {
            QueryView::Other(Other(self))
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

pub struct Position<T>(T);
impl<'a> Position<&'a QueryRef> {
    pub fn get(&self) -> (::Format, i64) {
        unsafe {
            let mut fmt = mem::uninitialized();
            let mut pos = mem::uninitialized();

            ffi::gst_query_parse_position(self.0.as_mut_ptr(), &mut fmt, &mut pos);

            (from_glib(fmt), pos)
        }
    }

    pub fn get_query(&self) -> &QueryRef {
        self.0
    }
}

impl<'a> Position<&'a mut QueryRef> {
    pub fn set(&mut self, fmt: ::Format, pos: i64) {
        unsafe {
            ffi::gst_query_set_position(self.0.as_mut_ptr(), fmt.to_glib(), pos);
        }
    }

    pub fn get_mut_query(&mut self) -> &mut QueryRef {
        self.0
    }
}

pub struct Duration<T>(T);
impl<'a> Duration<&'a QueryRef> {
    pub fn get(&self) -> (::Format, i64) {
        unsafe {
            let mut fmt = mem::uninitialized();
            let mut pos = mem::uninitialized();

            ffi::gst_query_parse_duration(self.0.as_mut_ptr(), &mut fmt, &mut pos);

            (from_glib(fmt), pos)
        }
    }

    pub fn get_query(&self) -> &QueryRef {
        self.0
    }
}

impl<'a> Duration<&'a mut QueryRef> {
    pub fn set(&mut self, fmt: ::Format, pos: i64) {
        unsafe {
            ffi::gst_query_set_duration(self.0.as_mut_ptr(), fmt.to_glib(), pos);
        }
    }

    pub fn get_mut_query(&mut self) -> &mut QueryRef {
        self.0
    }
}

pub struct Latency<T>(T);
impl<'a> Latency<&'a QueryRef> {
    pub fn get(&self) -> (bool, u64, u64) {
        unsafe {
            let mut live = mem::uninitialized();
            let mut min = mem::uninitialized();
            let mut max = mem::uninitialized();

            ffi::gst_query_parse_latency(self.0.as_mut_ptr(), &mut live, &mut min, &mut max);

            (from_glib(live), min, max)
        }
    }

    pub fn get_query(&self) -> &QueryRef {
        self.0
    }
}

impl<'a> Latency<&'a mut QueryRef> {
    pub fn set(&mut self, live: bool, min: u64, max: u64) {
        unsafe {
            ffi::gst_query_set_latency(self.0.as_mut_ptr(), live.to_glib(), min, max);
        }
    }

    pub fn get_mut_query(&mut self) -> &mut QueryRef {
        self.0
    }
}

pub struct Jitter<T>(T);
impl<'a> Jitter<&'a QueryRef> {
    pub fn get_query(&self) -> &QueryRef {
        self.0
    }
}

impl<'a> Jitter<&'a mut QueryRef> {
    pub fn get_mut_query(&mut self) -> &mut QueryRef {
        self.0
    }
}

pub struct Rate<T>(T);
impl<'a> Rate<&'a QueryRef> {
    pub fn get_query(&self) -> &QueryRef {
        self.0
    }
}

impl<'a> Rate<&'a mut QueryRef> {
    pub fn get_mut_query(&mut self) -> &mut QueryRef {
        self.0
    }
}

pub struct Seeking<T>(T);
impl<'a> Seeking<&'a QueryRef> {
    pub fn get(&self) -> (::Format, bool, i64, i64) {
        unsafe {
            let mut fmt = mem::uninitialized();
            let mut seekable = mem::uninitialized();
            let mut start = mem::uninitialized();
            let mut end = mem::uninitialized();
            ffi::gst_query_parse_seeking(
                self.0.as_mut_ptr(),
                &mut fmt,
                &mut seekable,
                &mut start,
                &mut end,
            );

            (from_glib(fmt), from_glib(seekable), start, end)
        }
    }

    pub fn get_query(&self) -> &QueryRef {
        self.0
    }
}

impl<'a> Seeking<&'a mut QueryRef> {
    pub fn set(&mut self, fmt: ::Format, seekable: bool, start: i64, end: i64) {
        unsafe {
            ffi::gst_query_set_seeking(
                self.0.as_mut_ptr(),
                fmt.to_glib(),
                seekable.to_glib(),
                start,
                end,
            );
        }
    }

    pub fn get_mut_query(&mut self) -> &mut QueryRef {
        self.0
    }
}

pub struct Segment<T>(T);
impl<'a> Segment<&'a QueryRef> {
    pub fn get(&self) -> (f64, ::Format, i64, i64) {
        unsafe {
            let mut rate = mem::uninitialized();
            let mut fmt = mem::uninitialized();
            let mut start = mem::uninitialized();
            let mut stop = mem::uninitialized();

            ffi::gst_query_parse_segment(
                self.0.as_mut_ptr(),
                &mut rate,
                &mut fmt,
                &mut start,
                &mut stop,
            );
            (rate, from_glib(fmt), start, stop)
        }
    }

    pub fn get_query(&self) -> &QueryRef {
        self.0
    }
}

impl<'a> Segment<&'a mut QueryRef> {
    pub fn set(&mut self, rate: f64, fmt: ::Format, start: i64, stop: i64) {
        unsafe {
            ffi::gst_query_set_segment(self.0.as_mut_ptr(), rate, fmt.to_glib(), start, stop);
        }
    }

    pub fn get_mut_query(&mut self) -> &mut QueryRef {
        self.0
    }
}

pub struct Convert<T>(T);
impl<'a> Convert<&'a QueryRef> {
    pub fn get(&self) -> (::Format, i64, ::Format, i64) {
        unsafe {
            let mut src_fmt = mem::uninitialized();
            let mut src = mem::uninitialized();
            let mut dest_fmt = mem::uninitialized();
            let mut dest = mem::uninitialized();

            ffi::gst_query_parse_convert(
                self.0.as_mut_ptr(),
                &mut src_fmt,
                &mut src,
                &mut dest_fmt,
                &mut dest,
            );
            (from_glib(src_fmt), src, from_glib(dest_fmt), dest)
        }
    }

    pub fn get_query(&self) -> &QueryRef {
        self.0
    }
}

impl<'a> Convert<&'a mut QueryRef> {
    pub fn set(&mut self, src_fmt: ::Format, src: i64, dest_fmt: ::Format, dest: i64) {
        unsafe {
            ffi::gst_query_set_convert(
                self.0.as_mut_ptr(),
                src_fmt.to_glib(),
                src,
                dest_fmt.to_glib(),
                dest,
            );
        }
    }

    pub fn get_mut_query(&mut self) -> &mut QueryRef {
        self.0
    }
}

pub struct Formats<T>(T);
impl<'a> Formats<&'a QueryRef> {
    pub fn get(&self) -> Vec<::Format> {
        unsafe {
            let mut n = mem::uninitialized();
            ffi::gst_query_parse_n_formats(self.0.as_mut_ptr(), &mut n);
            let mut res = Vec::with_capacity(n as usize);

            for i in 0..n {
                let mut fmt = mem::uninitialized();
                ffi::gst_query_parse_nth_format(self.0.as_mut_ptr(), i, &mut fmt);
                res.push(from_glib(fmt));
            }

            res
        }
    }

    pub fn get_query(&self) -> &QueryRef {
        self.0
    }
}

impl<'a> Formats<&'a mut QueryRef> {
    pub fn set(&mut self, formats: &[::Format]) {
        unsafe {
            let v: Vec<_> = formats.iter().map(|f| f.to_glib()).collect();
            ffi::gst_query_set_formatsv(self.0.as_mut_ptr(), v.len() as i32, v.as_ptr() as *mut _);
        }
    }

    pub fn get_mut_query(&mut self) -> &mut QueryRef {
        self.0
    }
}

pub struct Buffering<T>(T);
impl<'a> Buffering<&'a QueryRef> {
    pub fn get_percent(&self) -> (bool, i32) {
        unsafe {
            let mut busy = mem::uninitialized();
            let mut percent = mem::uninitialized();

            ffi::gst_query_parse_buffering_percent(self.0.as_mut_ptr(), &mut busy, &mut percent);

            (from_glib(busy), percent)
        }
    }

    pub fn get_range(&self) -> (::Format, i64, i64, i64) {
        unsafe {
            let mut fmt = mem::uninitialized();
            let mut start = mem::uninitialized();
            let mut stop = mem::uninitialized();
            let mut estimated_total = mem::uninitialized();

            ffi::gst_query_parse_buffering_range(
                self.0.as_mut_ptr(),
                &mut fmt,
                &mut start,
                &mut stop,
                &mut estimated_total,
            );
            (from_glib(fmt), start, stop, estimated_total)
        }
    }

    pub fn get_stats(&self) -> (::BufferingMode, i32, i32, i64) {
        unsafe {
            let mut mode = mem::uninitialized();
            let mut avg_in = mem::uninitialized();
            let mut avg_out = mem::uninitialized();
            let mut buffering_left = mem::uninitialized();

            ffi::gst_query_parse_buffering_stats(
                self.0.as_mut_ptr(),
                &mut mode,
                &mut avg_in,
                &mut avg_out,
                &mut buffering_left,
            );

            (from_glib(mode), avg_in, avg_out, buffering_left)
        }
    }

    pub fn get_ranges(&self) -> Vec<(i64, i64)> {
        unsafe {
            let n = ffi::gst_query_get_n_buffering_ranges(self.0.as_mut_ptr());
            let mut res = Vec::with_capacity(n as usize);
            for i in 0..n {
                let mut start = mem::uninitialized();
                let mut stop = mem::uninitialized();
                let s: bool = from_glib(ffi::gst_query_parse_nth_buffering_range(
                    self.0.as_mut_ptr(),
                    i,
                    &mut start,
                    &mut stop,
                ));
                if s {
                    res.push((start, stop));
                }
            }

            res
        }
    }

    pub fn get_query(&self) -> &QueryRef {
        self.0
    }
}

impl<'a> Buffering<&'a mut QueryRef> {
    pub fn set_percent(&mut self, busy: bool, percent: i32) {
        unsafe {
            ffi::gst_query_set_buffering_percent(self.0.as_mut_ptr(), busy.to_glib(), percent);
        }
    }

    pub fn set_range(&mut self, fmt: ::Format, start: i64, stop: i64, estimated_total: i64) {
        unsafe {
            ffi::gst_query_set_buffering_range(
                self.0.as_mut_ptr(),
                fmt.to_glib(),
                start,
                stop,
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
        unsafe {
            ffi::gst_query_set_buffering_stats(
                self.0.as_mut_ptr(),
                mode.to_glib(),
                avg_in,
                avg_out,
                buffering_left,
            );
        }
    }

    pub fn add_buffering_ranges(&mut self, ranges: &[(i64, i64)]) {
        unsafe {
            for &(start, stop) in ranges {
                ffi::gst_query_add_buffering_range(self.0.as_mut_ptr(), start, stop);
            }
        }
    }

    pub fn get_mut_query(&mut self) -> &mut QueryRef {
        self.0
    }
}

pub struct Custom<T>(T);
impl<'a> Custom<&'a QueryRef> {
    pub fn get_query(&self) -> &QueryRef {
        self.0
    }
}

impl<'a> Custom<&'a mut QueryRef> {
    pub fn get_mut_query(&mut self) -> &mut QueryRef {
        self.0
    }
}

pub struct Uri<T>(T);
impl<'a> Uri<&'a QueryRef> {
    pub fn get_uri(&self) -> Option<String> {
        unsafe {
            let mut uri = ptr::null_mut();
            ffi::gst_query_parse_uri(self.0.as_mut_ptr(), &mut uri);
            from_glib_full(uri)
        }
    }

    pub fn get_redirection(&self) -> (Option<String>, bool) {
        unsafe {
            let mut uri = ptr::null_mut();
            ffi::gst_query_parse_uri_redirection(self.0.as_mut_ptr(), &mut uri);
            let mut permanent = mem::uninitialized();
            ffi::gst_query_parse_uri_redirection_permanent(self.0.as_mut_ptr(), &mut permanent);

            (from_glib_full(uri), from_glib(permanent))
        }
    }

    pub fn get_query(&self) -> &QueryRef {
        self.0
    }
}

impl<'a> Uri<&'a mut QueryRef> {
    pub fn set_uri<'b, T: Into<&'b str>>(&mut self, uri: T) {
        let uri = uri.into();
        unsafe {
            ffi::gst_query_set_uri(self.0.as_mut_ptr(), uri.to_glib_none().0);
        }
    }

    pub fn set_redirection<'b, T: Into<&'b str>>(&mut self, uri: T, permanent: bool) {
        let uri = uri.into();
        unsafe {
            ffi::gst_query_set_uri_redirection(self.0.as_mut_ptr(), uri.to_glib_none().0);
            ffi::gst_query_set_uri_redirection_permanent(self.0.as_mut_ptr(), permanent.to_glib());
        }
    }

    pub fn get_mut_query(&mut self) -> &mut QueryRef {
        self.0
    }
}

// TODO
pub struct Allocation<T>(T);
impl<'a> Allocation<&'a QueryRef> {
    pub fn get_query(&self) -> &QueryRef {
        self.0
    }
}

impl<'a> Allocation<&'a mut QueryRef> {
    pub fn get_mut_query(&mut self) -> &mut QueryRef {
        self.0
    }
}

pub struct Scheduling<T>(T);
impl<'a> Scheduling<&'a QueryRef> {
    pub fn has_scheduling_mode(&self, mode: ::PadMode) -> bool {
        unsafe {
            from_glib(ffi::gst_query_has_scheduling_mode(
                self.0.as_mut_ptr(),
                mode.to_glib(),
            ))
        }
    }

    pub fn has_scheduling_mode_with_flags(
        &self,
        mode: ::PadMode,
        flags: ::SchedulingFlags,
    ) -> bool {
        unsafe {
            from_glib(ffi::gst_query_has_scheduling_mode_with_flags(
                self.0.as_mut_ptr(),
                mode.to_glib(),
                flags.to_glib(),
            ))
        }
    }

    pub fn get_scheduling_modes(&self) -> Vec<::PadMode> {
        unsafe {
            let n = ffi::gst_query_get_n_scheduling_modes(self.0.as_mut_ptr());
            let mut res = Vec::with_capacity(n as usize);
            for i in 0..n {
                res.push(from_glib(ffi::gst_query_parse_nth_scheduling_mode(
                    self.0.as_mut_ptr(),
                    i,
                )));
            }

            res
        }
    }

    pub fn get(&self) -> (::SchedulingFlags, i32, i32, i32) {
        unsafe {
            let mut flags = mem::uninitialized();
            let mut minsize = mem::uninitialized();
            let mut maxsize = mem::uninitialized();
            let mut align = mem::uninitialized();

            ffi::gst_query_parse_scheduling(
                self.0.as_mut_ptr(),
                &mut flags,
                &mut minsize,
                &mut maxsize,
                &mut align,
            );

            (from_glib(flags), minsize, maxsize, align)
        }
    }

    pub fn get_query(&self) -> &QueryRef {
        self.0
    }
}

impl<'a> Scheduling<&'a mut QueryRef> {
    pub fn add_scheduling_modes(&mut self, modes: &[::PadMode]) {
        unsafe {
            for mode in modes {
                ffi::gst_query_add_scheduling_mode(self.0.as_mut_ptr(), mode.to_glib());
            }
        }
    }

    pub fn set(&mut self, flags: ::SchedulingFlags, minsize: i32, maxsize: i32, align: i32) {
        unsafe {
            ffi::gst_query_set_scheduling(
                self.0.as_mut_ptr(),
                flags.to_glib(),
                minsize,
                maxsize,
                align,
            );
        }
    }

    pub fn get_mut_query(&mut self) -> &mut QueryRef {
        self.0
    }
}

pub struct AcceptCaps<T>(T);
impl<'a> AcceptCaps<&'a QueryRef> {
    pub fn get(&self) -> &::CapsRef {
        unsafe {
            let mut caps = ptr::null_mut();
            ffi::gst_query_parse_accept_caps(self.0.as_mut_ptr(), &mut caps);
            ::CapsRef::from_ptr(caps)
        }
    }

    pub fn get_result(&self) -> bool {
        unsafe {
            let mut accepted = mem::uninitialized();
            ffi::gst_query_parse_accept_caps_result(self.0.as_mut_ptr(), &mut accepted);
            from_glib(accepted)
        }
    }

    pub fn get_query(&self) -> &QueryRef {
        self.0
    }
}

impl<'a> AcceptCaps<&'a mut QueryRef> {
    pub fn set_result(&mut self, accepted: bool) {
        unsafe {
            ffi::gst_query_set_accept_caps_result(self.0.as_mut_ptr(), accepted.to_glib());
        }
    }

    pub fn get_mut_query(&mut self) -> &mut QueryRef {
        self.0
    }
}

pub struct Caps<T>(T);
impl<'a> Caps<&'a QueryRef> {
    pub fn get_filter(&self) -> &::CapsRef {
        unsafe {
            let mut caps = ptr::null_mut();
            ffi::gst_query_parse_caps(self.0.as_mut_ptr(), &mut caps);
            ::CapsRef::from_ptr(caps)
        }
    }

    pub fn get_result(&self) -> &::CapsRef {
        unsafe {
            let mut caps = ptr::null_mut();
            ffi::gst_query_parse_caps_result(self.0.as_mut_ptr(), &mut caps);
            ::CapsRef::from_ptr(caps)
        }
    }

    pub fn get_query(&self) -> &QueryRef {
        self.0
    }
}

impl<'a> Caps<&'a mut QueryRef> {
    pub fn set_result(&mut self, caps: &::Caps) {
        unsafe {
            ffi::gst_query_set_caps_result(self.0.as_mut_ptr(), caps.as_mut_ptr());
        }
    }

    pub fn get_mut_query(&mut self) -> &mut QueryRef {
        self.0
    }
}

pub struct Drain<T>(T);
impl<'a> Drain<&'a QueryRef> {
    pub fn get_query(&self) -> &QueryRef {
        self.0
    }
}

impl<'a> Drain<&'a mut QueryRef> {
    pub fn get_mut_query(&mut self) -> &mut QueryRef {
        self.0
    }
}

pub struct Context<T>(T);
impl<'a> Context<&'a QueryRef> {
    pub fn get_context(&self) -> Option<&::ContextRef> {
        unsafe {
            let mut context = ptr::null_mut();
            ffi::gst_query_parse_context(self.0.as_mut_ptr(), &mut context);
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
            ffi::gst_query_parse_context_type(self.0.as_mut_ptr(), &mut context_type);
            CStr::from_ptr(context_type).to_str().unwrap()
        }
    }

    pub fn get_query(&self) -> &QueryRef {
        self.0
    }
}

impl<'a> Context<&'a mut QueryRef> {
    pub fn set_context(&mut self, context: &::Context) {
        unsafe {
            ffi::gst_query_set_context(self.0.as_mut_ptr(), context.as_mut_ptr());
        }
    }

    pub fn get_mut_query(&mut self) -> &mut QueryRef {
        self.0
    }
}

pub struct Other<T>(T);
impl<'a> Other<&'a QueryRef> {
    pub fn get_query(&self) -> &QueryRef {
        self.0
    }
}

impl<'a> Other<&'a mut QueryRef> {
    pub fn get_mut_query(&mut self) -> &mut QueryRef {
        self.0
    }
}

macro_rules! impl_deref_view(
    ($name:ident) => {
        impl<'a> Deref for $name<&'a mut QueryRef> {
            type Target = $name<&'a QueryRef>;

            fn deref(&self) -> &Self::Target {
                unsafe {
                    mem::transmute(self)
                }
            }
        }
    };
);

impl_deref_view!(Position);
impl_deref_view!(Duration);
impl_deref_view!(Latency);
impl_deref_view!(Jitter);
impl_deref_view!(Rate);
impl_deref_view!(Seeking);
impl_deref_view!(Segment);
impl_deref_view!(Convert);
impl_deref_view!(Formats);
impl_deref_view!(Buffering);
impl_deref_view!(Custom);
impl_deref_view!(Uri);
impl_deref_view!(Allocation);
impl_deref_view!(Scheduling);
impl_deref_view!(AcceptCaps);
impl_deref_view!(Caps);
impl_deref_view!(Drain);
impl_deref_view!(Context);
impl_deref_view!(Other);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_writability() {
        ::init().unwrap();

        let mut q = Query::new_position(::Format::Time);

        match q.view() {
            QueryView::Position(ref p) => {
                let (fmt, pos) = p.get();
                assert_eq!(fmt, ::Format::Time);
                assert_eq!(pos, -1);
            }
            _ => (),
        }

        match q.get_mut().unwrap().view_mut() {
            QueryView::Position(ref mut p) => {
                let (fmt, pos) = p.get();
                assert_eq!(fmt, ::Format::Time);
                assert_eq!(pos, -1);
                p.set(::Format::Time, 2);
            }
            _ => (),
        }

        match q.view() {
            QueryView::Position(ref p) => {
                let (fmt, pos) = p.get();
                assert_eq!(fmt, ::Format::Time);
                assert_eq!(pos, 2);
            }
            _ => (),
        }
    }
}
