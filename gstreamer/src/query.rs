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
use std::ops::{Deref, DerefMut};

use glib;
use glib::translate::{from_glib, from_glib_full, ToGlib, ToGlibPtr};

#[repr(C)]
pub struct Query(ffi::GstQuery);

unsafe impl Send for Query {}
unsafe impl Sync for Query {}

pub type QueryRc = GstRc<Query>;

unsafe impl MiniObject for Query {
    type GstType = ffi::GstQuery;
}

impl Query {
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
}

impl Query {
    pub fn new_position(fmt: ::Format) -> Position {
        assert_initialized_main_thread!();
        Position(
            unsafe { from_glib_full(ffi::gst_query_new_position(fmt.to_glib())) }
        )
    }

    pub fn new_duration(fmt: ::Format) -> Duration {
        assert_initialized_main_thread!();
        Duration(
            unsafe { from_glib_full(ffi::gst_query_new_duration(fmt.to_glib())) }
        )
    }

    pub fn new_latency() -> Latency {
        assert_initialized_main_thread!();
        Latency(
            unsafe { from_glib_full(ffi::gst_query_new_latency()) }
        )
    }

    pub fn new_seeking(fmt: ::Format) -> Seeking {
        assert_initialized_main_thread!();
        Seeking(
            unsafe { from_glib_full(ffi::gst_query_new_seeking(fmt.to_glib())) }
        )
    }

    pub fn new_segment(fmt: ::Format) -> Segment {
        assert_initialized_main_thread!();
        Segment(
            unsafe { from_glib_full(ffi::gst_query_new_segment(fmt.to_glib())) }
        )
    }

    pub fn new_convert<V: Into<GenericFormattedValue>>(value: V, dest_fmt: ::Format) -> Convert {
        assert_initialized_main_thread!();
        let value = value.into();
        Convert(
            unsafe {
                from_glib_full(ffi::gst_query_new_convert(
                    value.get_format().to_glib(),
                    value.get_value(),
                    dest_fmt.to_glib(),
                ))
            }
        )
    }

    pub fn new_formats() -> Formats {
        assert_initialized_main_thread!();
        Formats(
            unsafe { from_glib_full(ffi::gst_query_new_formats()) }
        )
    }

    pub fn new_buffering(fmt: ::Format) -> Buffering {
        assert_initialized_main_thread!();
        Buffering(
            unsafe { from_glib_full(ffi::gst_query_new_buffering(fmt.to_glib())) }
        )
    }

    pub fn new_custom(structure: ::Structure) -> Custom {
        assert_initialized_main_thread!();
        Custom(
            unsafe {
                from_glib_full(ffi::gst_query_new_custom(
                    ffi::GST_QUERY_CUSTOM,
                    structure.into_ptr(),
                ))
            }
        )
    }

    pub fn new_uri() -> Uri {
        assert_initialized_main_thread!();
        Uri(
            unsafe { from_glib_full(ffi::gst_query_new_uri()) }
        )
    }

    pub fn new_scheduling() -> Scheduling {
        assert_initialized_main_thread!();
        Scheduling(
            unsafe { from_glib_full(ffi::gst_query_new_scheduling()) }
        )
    }

    pub fn new_accept_caps(caps: &::Caps) -> AcceptCaps {
        assert_initialized_main_thread!();
        AcceptCaps(
            unsafe { from_glib_full(ffi::gst_query_new_accept_caps(caps.as_mut_ptr())) }
        )
    }

    pub fn new_caps<'a, P: Into<Option<&'a ::Caps>>>(filter: P) -> Caps {
        assert_initialized_main_thread!();
        let filter = filter.into();
        Caps(
            unsafe { from_glib_full(ffi::gst_query_new_caps(filter.to_glib_none().0)) }
        )
    }

    pub fn new_drain() -> Drain {
        assert_initialized_main_thread!();
        Drain(
            unsafe { from_glib_full(ffi::gst_query_new_drain()) }
        )
    }

    pub fn new_context(context_type: &str) -> Context {
        assert_initialized_main_thread!();
        Context(
            unsafe { from_glib_full(ffi::gst_query_new_context(context_type.to_glib_none().0)) }
        )
    }
}

impl glib::types::StaticType for Query {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_query_get_type()) }
    }
}

impl fmt::Debug for Query {
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

impl From<QueryRc> for QueryWrapper {
    fn from(query: QueryRc) -> Self {
        QueryWrapper(query.into())
    }
}

impl From<QueryRc> for QueryView {
    fn from(query: QueryRc) -> Self {
        let type_ = (query.0).type_;

        match type_ {
            ffi::GST_QUERY_POSITION => QueryView::Position(Position(query)),
            ffi::GST_QUERY_DURATION => QueryView::Duration(Duration(query)),
            ffi::GST_QUERY_LATENCY => QueryView::Latency(Latency(query)),
            ffi::GST_QUERY_JITTER => QueryView::Jitter(Jitter(query)),
            ffi::GST_QUERY_RATE => QueryView::Rate(Rate(query)),
            ffi::GST_QUERY_SEEKING => QueryView::Seeking(Seeking(query)),
            ffi::GST_QUERY_SEGMENT => QueryView::Segment(Segment(query)),
            ffi::GST_QUERY_CONVERT => QueryView::Convert(Convert(query)),
            ffi::GST_QUERY_FORMATS => QueryView::Formats(Formats(query)),
            ffi::GST_QUERY_BUFFERING => QueryView::Buffering(Buffering(query)),
            ffi::GST_QUERY_CUSTOM => QueryView::Custom(Custom(query)),
            ffi::GST_QUERY_URI => QueryView::Uri(Uri(query)),
            ffi::GST_QUERY_ALLOCATION => QueryView::Allocation(Allocation(query)),
            ffi::GST_QUERY_SCHEDULING => QueryView::Scheduling(Scheduling(query)),
            ffi::GST_QUERY_ACCEPT_CAPS => QueryView::AcceptCaps(AcceptCaps(query)),
            ffi::GST_QUERY_CAPS => QueryView::Caps(Caps(query)),
            ffi::GST_QUERY_DRAIN => QueryView::Drain(Drain(query)),
            ffi::GST_QUERY_CONTEXT => QueryView::Context(Context(query)),
            _ => QueryView::Other(Other(query)),
        }
    }
}

#[derive(Debug)]
pub enum QueryView {
    Position(Position),
    Duration(Duration),
    Latency(Latency),
    Jitter(Jitter),
    Rate(Rate),
    Seeking(Seeking),
    Segment(Segment),
    Convert(Convert),
    Formats(Formats),
    Buffering(Buffering),
    Custom(Custom),
    Uri(Uri),
    Allocation(Allocation),
    Scheduling(Scheduling),
    AcceptCaps(AcceptCaps),
    Caps(Caps),
    Drain(Drain),
    Context(Context),
    Other(Other),
    __NonExhaustive,
}

impl Deref for QueryView {
    type Target = Query;

    fn deref(&self) -> &Self::Target {
        match *self {
            QueryView::Position(ref position) => position,
            QueryView::Duration(ref duration) => duration,
            QueryView::Latency(ref latency) => latency,
            QueryView::Jitter(ref jitter) => jitter,
            QueryView::Rate(ref rate) => rate,
            QueryView::Seeking(ref seeking) => seeking,
            QueryView::Segment(ref segment) => segment,
            QueryView::Convert(ref convert) => convert,
            QueryView::Formats(ref formats) => formats,
            QueryView::Buffering(ref buffering) => buffering,
            QueryView::Custom(ref custom) => custom,
            QueryView::Uri(ref uri) => uri,
            QueryView::Allocation(ref allocation) => allocation,
            QueryView::Scheduling(ref scheduling) => scheduling,
            QueryView::AcceptCaps(ref accept_caps) => accept_caps,
            QueryView::Caps(ref caps) => caps,
            QueryView::Drain(ref drain) => drain,
            QueryView::Context(ref context) => context,
            QueryView::Other(ref other) => other,
            _ => unimplemented!("Converting QueryView into Query for unknown query type"),
        }
    }
}

// Bypass `QueryRc`: if `QueryView` is a `&mut`, this means that
// `Query` can also be accessed as `&mut` thanks to the controls
// enforced by `QueryWrapper`
impl DerefMut for QueryView {
    fn deref_mut(&mut self) -> &mut Query {
        match *self {
            QueryView::Position(ref mut position) => position,
            QueryView::Duration(ref mut duration) => duration,
            QueryView::Latency(ref mut latency) => latency,
            QueryView::Jitter(ref mut jitter) => jitter,
            QueryView::Rate(ref mut rate) => rate,
            QueryView::Seeking(ref mut seeking) => seeking,
            QueryView::Segment(ref mut segment) => segment,
            QueryView::Convert(ref mut convert) => convert,
            QueryView::Formats(ref mut formats) => formats,
            QueryView::Buffering(ref mut buffering) => buffering,
            QueryView::Custom(ref mut custom) => custom,
            QueryView::Uri(ref mut uri) => uri,
            QueryView::Allocation(ref mut allocation) => allocation,
            QueryView::Scheduling(ref mut scheduling) => scheduling,
            QueryView::AcceptCaps(ref mut accept_caps) => accept_caps,
            QueryView::Caps(ref mut caps) => caps,
            QueryView::Drain(ref mut drain) => drain,
            QueryView::Context(ref mut context) => context,
            QueryView::Other(ref mut other) => other,
            _ => unimplemented!("Converting QueryView into Query for unknown query type"),
        }
    }
}

impl<'a> From<&'a QueryView> for &'a QueryRc {
    fn from(view: &'a QueryView) -> Self {
        match *view {
            QueryView::Position(ref position) => &position.0,
            QueryView::Duration(ref duration) => &duration.0,
            QueryView::Latency(ref latency) => &latency.0,
            QueryView::Jitter(ref jitter) => &jitter.0,
            QueryView::Rate(ref rate) => &rate.0,
            QueryView::Seeking(ref seeking) => &seeking.0,
            QueryView::Segment(ref segment) => &segment.0,
            QueryView::Convert(ref convert) => &convert.0,
            QueryView::Formats(ref formats) => &formats.0,
            QueryView::Buffering(ref buffering) => &buffering.0,
            QueryView::Custom(ref custom) => &custom.0,
            QueryView::Uri(ref uri) => &uri.0,
            QueryView::Allocation(ref allocation) => &allocation.0,
            QueryView::Scheduling(ref scheduling) => &scheduling.0,
            QueryView::AcceptCaps(ref accept_caps) => &accept_caps.0,
            QueryView::Caps(ref caps) => &caps.0,
            QueryView::Drain(ref drain) => &drain.0,
            QueryView::Context(ref context) => &context.0,
            QueryView::Other(ref other) => &other.0,
            _ => unimplemented!("Converting QueryView into Query for unknown query type"),
        }
    }
}

impl From<QueryView> for QueryWrapper {
    fn from(query_view: QueryView) -> Self {
        QueryWrapper(query_view)
    }
}

pub struct QueryWrapper(QueryView);

impl QueryWrapper {
    pub fn get_view<'a>(&'a self) -> &'a QueryView {
        &self.0
    }

    pub fn is_writable(&mut self) -> bool {
        let query: &QueryRc = (&self.0).into();
        query.is_writable()
    }

    pub fn try_view_mut<'a>(&'a mut self) -> Option<&'a mut QueryView> {
        if self.is_writable() {
            Some(&mut self.0)
        } else {
            None
        }
    }

    pub fn try_query_mut<'a>(&'a mut self) -> Option<&'a mut Query> {
        if self.is_writable() {
            Some(&mut self.0)
        } else {
            None
        }
    }
}

impl Deref for QueryWrapper {
    type Target = QueryView;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Don't implement `DerefMut` for `QueryWrapper`
// It is not safe to `deref_mut` a `QueryWrapper` to a mutable `QueryView`
// Use `try_view_mut` or `try_query_mut` which performs the appropriate checks

pub trait QueryImpl<T> {
    fn downcast_ref<'a>(&'a self) -> Option<&'a T>;
    fn downcast_mut<'a>(&'a mut self) -> Option<&'a mut T>;
}

macro_rules! declare_concrete_query(
    ($name:ident) => {
        #[derive(Debug)]
        pub struct $name(QueryRc);

        impl $name {
            pub unsafe fn into_ptr(self) -> *mut ffi::GstQuery {
                self.0.into_ptr()
            }
        }

        impl QueryImpl<$name> for QueryWrapper {
            fn downcast_ref<'a>(&'a self) -> Option<&'a $name> {
                match self.0 {
                    QueryView::$name(ref concrete) => Some(concrete),
                    _ => None,
                }
            }

            fn downcast_mut<'a>(&'a mut self) -> Option<&'a mut $name> {
                if self.is_writable() {
                    match self.0 {
                        QueryView::$name(ref mut concrete) => Some(concrete),
                        _ => None,
                    }
                } else {
                    None
                }
            }
        }

        impl From<$name> for QueryWrapper {
            fn from(concrete: $name) -> QueryWrapper {
                QueryWrapper(QueryView::$name($name(concrete.0)))
            }
        }

        impl<'a> From<&'a $name> for &'a QueryRc {
            fn from(concrete: &'a $name) -> Self {
                (&concrete.0).into()
            }
        }

        impl Deref for $name {
            type Target = Query;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Query {
                // Mutability is controlled by the borrow checker
                // and external ref_count on the inner Query
                // is checked by QueryWrapper
                // si we can safely use make_mut here
                self.0.make_mut()
            }
        }
    }
);

declare_concrete_query!(Position);
impl Position {
    pub fn get_result(&self) -> GenericFormattedValue {
        unsafe {
            let mut fmt = mem::uninitialized();
            let mut pos = mem::uninitialized();

            ffi::gst_query_parse_position(self.0.as_mut_ptr(), &mut fmt, &mut pos);

            GenericFormattedValue::new(from_glib(fmt), pos)
        }
    }

    pub fn get_format(&self) -> ::Format {
        unsafe {
            let mut fmt = mem::uninitialized();

            ffi::gst_query_parse_position(self.0.as_mut_ptr(), &mut fmt, ptr::null_mut());

            from_glib(fmt)
        }
    }

    pub fn set<V: Into<GenericFormattedValue>>(&mut self, pos: V) {
        let pos = pos.into();
        assert_eq!(pos.get_format(), self.get_format());
        unsafe {
            ffi::gst_query_set_position(
                self.0.as_mut_ptr(),
                pos.get_format().to_glib(),
                pos.get_value(),
            );
        }
    }
}

declare_concrete_query!(Duration);
impl Duration {
    pub fn get_result(&self) -> GenericFormattedValue {
        unsafe {
            let mut fmt = mem::uninitialized();
            let mut pos = mem::uninitialized();

            ffi::gst_query_parse_duration(self.0.as_mut_ptr(), &mut fmt, &mut pos);

            GenericFormattedValue::new(from_glib(fmt), pos)
        }
    }

    pub fn get_format(&self) -> ::Format {
        unsafe {
            let mut fmt = mem::uninitialized();

            ffi::gst_query_parse_duration(self.0.as_mut_ptr(), &mut fmt, ptr::null_mut());

            from_glib(fmt)
        }
    }

    pub fn set<V: Into<GenericFormattedValue>>(&mut self, dur: V) {
        let dur = dur.into();
        assert_eq!(dur.get_format(), self.get_format());
        unsafe {
            ffi::gst_query_set_duration(
                self.0.as_mut_ptr(),
                dur.get_format().to_glib(),
                dur.get_value(),
            );
        }
    }
}

declare_concrete_query!(Latency);
impl Latency {
    pub fn get_result(&self) -> (bool, ::ClockTime, ::ClockTime) {
        unsafe {
            let mut live = mem::uninitialized();
            let mut min = mem::uninitialized();
            let mut max = mem::uninitialized();

            ffi::gst_query_parse_latency(self.0.as_mut_ptr(), &mut live, &mut min, &mut max);

            (from_glib(live), from_glib(min), from_glib(max))
        }
    }

    pub fn set(&mut self, live: bool, min: ::ClockTime, max: ::ClockTime) {
        unsafe {
            ffi::gst_query_set_latency(
                self.0.as_mut_ptr(),
                live.to_glib(),
                min.to_glib(),
                max.to_glib(),
            );
        }
    }
}

declare_concrete_query!(Jitter);
declare_concrete_query!(Rate);

declare_concrete_query!(Seeking);
impl Seeking {
    pub fn get_result(&self) -> (bool, GenericFormattedValue, GenericFormattedValue) {
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
                self.0.as_mut_ptr(),
                &mut fmt,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );

            from_glib(fmt)
        }
    }

    pub fn set<V: Into<GenericFormattedValue>>(&mut self, seekable: bool, start: V, end: V) {
        let start = start.into();
        let end = end.into();

        assert_eq!(self.get_format(), start.get_format());
        assert_eq!(start.get_format(), end.get_format());

        unsafe {
            ffi::gst_query_set_seeking(
                self.0.as_mut_ptr(),
                start.get_format().to_glib(),
                seekable.to_glib(),
                start.get_value(),
                end.get_value(),
            );
        }
    }
}

declare_concrete_query!(Segment);
impl Segment {
    pub fn get_result(&self) -> (f64, GenericFormattedValue, GenericFormattedValue) {
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
                self.0.as_mut_ptr(),
                ptr::null_mut(),
                &mut fmt,
                ptr::null_mut(),
                ptr::null_mut(),
            );
            from_glib(fmt)
        }
    }

    pub fn set<V: Into<GenericFormattedValue>>(&mut self, rate: f64, start: V, stop: V) {
        let start = start.into();
        let stop = stop.into();

        assert_eq!(start.get_format(), stop.get_format());

        unsafe {
            ffi::gst_query_set_segment(
                self.0.as_mut_ptr(),
                rate,
                start.get_format().to_glib(),
                start.get_value(),
                stop.get_value(),
            );
        }
    }
}

declare_concrete_query!(Convert);
impl Convert {
    pub fn get_result(&self) -> (GenericFormattedValue, GenericFormattedValue) {
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
                self.0.as_mut_ptr(),
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

    pub fn set<V: Into<GenericFormattedValue>>(&mut self, src: V, dest: V) {
        let src = src.into();
        let dest = dest.into();

        unsafe {
            ffi::gst_query_set_convert(
                self.0.as_mut_ptr(),
                src.get_format().to_glib(),
                src.get_value(),
                dest.get_format().to_glib(),
                dest.get_value(),
            );
        }
    }
}

declare_concrete_query!(Formats);
impl Formats {
    pub fn get_result(&self) -> Vec<::Format> {
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

    pub fn set(&mut self, formats: &[::Format]) {
        unsafe {
            let v: Vec<_> = formats.iter().map(|f| f.to_glib()).collect();
            ffi::gst_query_set_formatsv(self.0.as_mut_ptr(), v.len() as i32, v.as_ptr() as *mut _);
        }
    }
}

declare_concrete_query!(Buffering);
impl Buffering {
    pub fn get_format(&self) -> ::Format {
        unsafe {
            let mut fmt = mem::uninitialized();

            ffi::gst_query_parse_buffering_range(
                self.0.as_mut_ptr(),
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

            ffi::gst_query_parse_buffering_percent(self.0.as_mut_ptr(), &mut busy, &mut percent);

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
                self.0.as_mut_ptr(),
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
                self.0.as_mut_ptr(),
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
                self.0.as_mut_ptr(),
                &mut fmt,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );
            let fmt = from_glib(fmt);

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
                    res.push((
                        GenericFormattedValue::new(fmt, start),
                        GenericFormattedValue::new(fmt, stop),
                    ));
                }
            }

            res
        }
    }

    pub fn set_percent(&mut self, busy: bool, percent: i32) {
        unsafe {
            ffi::gst_query_set_buffering_percent(self.0.as_mut_ptr(), busy.to_glib(), percent);
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
                self.0.as_mut_ptr(),
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
                self.0.as_mut_ptr(),
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
                    self.0.as_mut_ptr(),
                    start.get_value(),
                    stop.get_value(),
                );
            }
        }
    }
}

declare_concrete_query!(Custom);

declare_concrete_query!(Uri);
impl Uri {
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
}

// TODO
declare_concrete_query!(Allocation);

declare_concrete_query!(Scheduling);
impl Scheduling {
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
        skip_assert_initialized!();
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

    pub fn get_result(&self) -> (::SchedulingFlags, i32, i32, i32) {
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
}

declare_concrete_query!(AcceptCaps);
impl AcceptCaps {
    pub fn get_caps(&self) -> &::CapsRef {
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

    pub fn set_result(&mut self, accepted: bool) {
        unsafe {
            ffi::gst_query_set_accept_caps_result(self.0.as_mut_ptr(), accepted.to_glib());
        }
    }
}

declare_concrete_query!(Caps);
impl Caps {
    pub fn get_filter(&self) -> Option<&::CapsRef> {
        unsafe {
            let mut caps = ptr::null_mut();
            ffi::gst_query_parse_caps(self.0.as_mut_ptr(), &mut caps);
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
            ffi::gst_query_parse_caps_result(self.0.as_mut_ptr(), &mut caps);
            if caps.is_null() {
                None
            } else {
                Some(::CapsRef::from_ptr(caps))
            }
        }
    }

    pub fn set_result(&mut self, caps: &::Caps) {
        unsafe {
            ffi::gst_query_set_caps_result(self.0.as_mut_ptr(), caps.as_mut_ptr());
        }
    }
}

declare_concrete_query!(Drain);

declare_concrete_query!(Context);
impl Context {
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

    pub fn set_context(&mut self, context: &::Context) {
        unsafe {
            ffi::gst_query_set_context(self.0.as_mut_ptr(), context.as_mut_ptr());
        }
    }
}

declare_concrete_query!(Other);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_writability() {
        fn check_mut_view(view: &mut QueryView) {
            match *view {
                QueryView::Position(ref mut p) => {
                    let pos = p.get_result();
                    assert_eq!(pos.try_into_time(), Ok(2 * ::SECOND));
                    p.set(3 * ::SECOND);
                    let pos = p.get_result();
                    assert_eq!(pos.try_into_time(), Ok(3 * ::SECOND));

                    p.get_mut_structure()
                        .set("check_mut", &true);
                }
                _ => panic!("Wrong concrete Query in Query"),
            }
        }

        fn check_ref_view(view: &QueryView) {
            match *view {
                QueryView::Position(ref p) => {
                    let pos = p.get_result();
                    assert_eq!(pos.try_into_time(), Ok(3 * ::SECOND));
                    unsafe { assert!(!p.as_mut_ptr().is_null()); }

                    p.get_structure()
                        .unwrap()
                        .has_field("check_mut");
                }
                _ => panic!("Wrong concrete Query in Query"),
            }
        }

        let mut p = Query::new_position(::Format::Time);
        let fmt = p.get_format();
        assert_eq!(fmt, ::Format::Time);

        // deref
        assert!(!p.is_serialized());

        p.set(2 * ::SECOND);

        let mut wrapper: QueryWrapper = p.into();

        {
            let view = wrapper.try_view_mut().unwrap();
            check_mut_view(view);
        }

        // `QueryWrapper` can `deref` to `&QueryView`
        check_ref_view(&wrapper);

        // or use explicite function
        check_ref_view(wrapper.get_view());
    }

    #[test]
    fn test_deref() {
        fn check_mut(q: &mut Query) {
            unsafe { assert!(!q.as_mut_ptr().is_null()); }
        }

        fn check_ref(q: &Query) {
            unsafe { assert!(!q.as_mut_ptr().is_null()); }
        }

        let mut p = Query::new_position(::Format::Time);

        // Concrete `Position` can deref as `&mut Query` here
        // because its mutability is controlled by the borrow checker
        check_mut(&mut p);

        let mut wrapper: QueryWrapper = p.into();

        {
            // Attempt to get `Query` as `&mut` from `QueryWrapper`
            let query_ref = wrapper.try_query_mut();
            assert!(query_ref.is_some());
            let mut query_ref = query_ref.unwrap();
            check_mut(&mut query_ref);
        }

        {
            // Wrong concrete type
            let d: Option<&::query::Duration> = wrapper.downcast_ref();
            assert!(d.is_none());
        }

        {
            // Expected concrete type
            let p: Option<&::query::Position> = wrapper.downcast_ref();
            assert!(p.is_some());

            // Deref `QueryWrapper` as `&Query`
            check_ref(&wrapper);

            // cannot borrow `wrapper` as mutable because it is also borrowed as immutable
            //let p: Option<&mut ::query::Position> = wrapper.downcast_mut();
        }

        // Expected concrete type as `&mut`
        let p: Option<&mut ::query::Position> = wrapper.downcast_mut();
        assert!(p.is_some());
        let mut p = p.unwrap();

        check_mut(&mut p);

        // cannot borrow `wrapper` as immutable because it is also borrowed as mutable
        //check_ref(&wrapper);

        let fmt = p.get_format();
        assert_eq!(fmt, ::Format::Time);
    }

    #[test]
    fn test_concrete_to_ffi() {
        let p = Query::new_position(::Format::Time);
        unsafe { assert!(!p.into_ptr().is_null()); }
    }
}
