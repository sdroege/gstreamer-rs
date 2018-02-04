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
pub struct QueryRef(ffi::GstQuery);

unsafe impl Send for QueryRef {}
unsafe impl Sync for QueryRef {}

pub type Query = GstRc<QueryRef>;

unsafe impl MiniObject for QueryRef {
    type GstType = ffi::GstQuery;
}

impl GstRc<QueryRef> {
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

impl From<QueryView> for Query {
    fn from(view: QueryView) -> Self {
        match view {
            QueryView::Position(position) => position.into(),
            QueryView::Duration(duration) => duration.into(),
            QueryView::Latency(latency) => latency.into(),
            QueryView::Jitter(jitter) => jitter.into(),
            QueryView::Rate(rate) => rate.into(),
            QueryView::Seeking(seeking) => seeking.into(),
            QueryView::Segment(segment) => segment.into(),
            QueryView::Convert(convert) => convert.into(),
            QueryView::Formats(formats) => formats.into(),
            QueryView::Buffering(buffering) => buffering.into(),
            QueryView::Custom(custom) => custom.into(),
            QueryView::Uri(uri) => uri.into(),
            QueryView::Allocation(allocation) => allocation.into(),
            QueryView::Scheduling(scheduling) => scheduling.into(),
            QueryView::AcceptCaps(accept_caps) => accept_caps.into(),
            QueryView::Caps(caps) => caps.into(),
            QueryView::Drain(drain) => drain.into(),
            QueryView::Context(context) => context.into(),
            QueryView::Other(other) => other.into(),
            _ => unimplemented!("Converting QueryView into Query for unknown query type"),
        }
    }
}

impl<'a> From<&'a QueryView> for &'a Query {
    fn from(view: &'a QueryView) -> Self {
        match *view {
            QueryView::Position(ref position) => position.into(),
            QueryView::Duration(ref duration) => duration.into(),
            QueryView::Latency(ref latency) => latency.into(),
            QueryView::Jitter(ref jitter) => jitter.into(),
            QueryView::Rate(ref rate) => rate.into(),
            QueryView::Seeking(ref seeking) => seeking.into(),
            QueryView::Segment(ref segment) => segment.into(),
            QueryView::Convert(ref convert) => convert.into(),
            QueryView::Formats(ref formats) => formats.into(),
            QueryView::Buffering(ref buffering) => buffering.into(),
            QueryView::Custom(ref custom) => custom.into(),
            QueryView::Uri(ref uri) => uri.into(),
            QueryView::Allocation(ref allocation) => allocation.into(),
            QueryView::Scheduling(ref scheduling) => scheduling.into(),
            QueryView::AcceptCaps(ref accept_caps) => accept_caps.into(),
            QueryView::Caps(ref caps) => caps.into(),
            QueryView::Drain(ref drain) => drain.into(),
            QueryView::Context(ref context) => context.into(),
            QueryView::Other(ref other) => other.into(),
            _ => unimplemented!("Converting QueryView into Query for unknown query type"),
        }
    }
}

impl<'a> From<&'a mut QueryView> for &'a Query {
    fn from(view: &'a mut QueryView) -> Self {
        match *view {
            QueryView::Position(ref position) => position.into(),
            QueryView::Duration(ref duration) => duration.into(),
            QueryView::Latency(ref latency) => latency.into(),
            QueryView::Jitter(ref jitter) => jitter.into(),
            QueryView::Rate(ref rate) => rate.into(),
            QueryView::Seeking(ref seeking) => seeking.into(),
            QueryView::Segment(ref segment) => segment.into(),
            QueryView::Convert(ref convert) => convert.into(),
            QueryView::Formats(ref formats) => formats.into(),
            QueryView::Buffering(ref buffering) => buffering.into(),
            QueryView::Custom(ref custom) => custom.into(),
            QueryView::Uri(ref uri) => uri.into(),
            QueryView::Allocation(ref allocation) => allocation.into(),
            QueryView::Scheduling(ref scheduling) => scheduling.into(),
            QueryView::AcceptCaps(ref accept_caps) => accept_caps.into(),
            QueryView::Caps(ref caps) => caps.into(),
            QueryView::Drain(ref drain) => drain.into(),
            QueryView::Context(ref context) => context.into(),
            QueryView::Other(ref other) => other.into(),
            _ => unimplemented!("Converting QueryView into Query for unknown query type"),
        }
    }
}

impl<'a> From<&'a mut QueryView> for &'a mut Query {
    fn from(view: &'a mut QueryView) -> Self {
        match *view {
            QueryView::Position(ref mut position) => position.into(),
            QueryView::Duration(ref mut duration) => duration.into(),
            QueryView::Latency(ref mut latency) => latency.into(),
            QueryView::Jitter(ref mut jitter) => jitter.into(),
            QueryView::Rate(ref mut rate) => rate.into(),
            QueryView::Seeking(ref mut seeking) => seeking.into(),
            QueryView::Segment(ref mut segment) => segment.into(),
            QueryView::Convert(ref mut convert) => convert.into(),
            QueryView::Formats(ref mut formats) => formats.into(),
            QueryView::Buffering(ref mut buffering) => buffering.into(),
            QueryView::Custom(ref mut custom) => custom.into(),
            QueryView::Uri(ref mut uri) => uri.into(),
            QueryView::Allocation(ref mut allocation) => allocation.into(),
            QueryView::Scheduling(ref mut scheduling) => scheduling.into(),
            QueryView::AcceptCaps(ref mut accept_caps) => accept_caps.into(),
            QueryView::Caps(ref mut caps) => caps.into(),
            QueryView::Drain(ref mut drain) => drain.into(),
            QueryView::Context(ref mut context) => context.into(),
            QueryView::Other(ref mut other) => other.into(),
            _ => unimplemented!("Converting QueryView into Query for unknown query type"),
        }
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

impl From<Query> for QueryView {
    fn from(query: Query) -> Self {
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

macro_rules! declare_concrete_query(
    ($name:ident) => {
        #[derive(Debug)]
        pub struct $name(Query);

        impl $name {
            pub unsafe fn into_ptr(self) -> *mut ffi::GstQuery {
                self.0.into_ptr()
            }
        }

        impl From<$name> for QueryView {
            fn from(concrete: $name) -> QueryView {
                QueryView::$name($name(concrete.0))
            }
        }

        impl From<$name> for Query {
            fn from(concrete: $name) -> Query {
                concrete.0
            }
        }

        impl<'a> From<&'a $name> for &'a Query {
            fn from(concrete: &'a $name) -> &'a Query {
                &concrete.0
            }
        }

        impl<'a> From<&'a mut $name> for &'a Query {
            fn from(concrete: &'a mut $name) -> &'a Query {
                &concrete.0
            }
        }

        impl<'a> From<&'a mut $name> for &'a mut Query {
            fn from(concrete: &'a mut $name) -> &'a mut Query {
                &mut concrete.0
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
                &mut self.0
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
        ::init().unwrap();

        let mut p = Query::new_position(::Format::Time);

        let fmt = p.get_format();
        assert_eq!(fmt, ::Format::Time);

        // deref
        assert!(!p.is_serialized());

        let pos = p.get_result();
        assert_eq!(pos.try_into_time(), Ok(::CLOCK_TIME_NONE));
        p.set(2 * ::SECOND);

        let pos = p.get_result();
        assert_eq!(pos.try_into_time(), Ok(2 * ::SECOND));
    }

    #[test]
    fn test_generic_ref() {
        fn check_ref(q: &Query) {
            assert!(q.is_writable());
            unsafe { assert!(!q.as_mut_ptr().is_null()); }
        }

        let p = Query::new_position(::Format::Time);
        check_ref(&p);

        let fmt = p.get_format();
        assert_eq!(fmt, ::Format::Time);
    }

    #[test]
    fn test_view_from_generic() {
        fn check_generic(q: Query) {
            match q.into() {
                QueryView::Position(mut p) => {
                    let pos = p.get_result();
                    assert_eq!(pos.try_into_time(), Ok(2 * ::SECOND));
                    p.set(3 * ::SECOND);
                    let pos = p.get_result();
                    assert_eq!(pos.try_into_time(), Ok(3 * ::SECOND));
               }
                _ => panic!("Wrong concrete Query in QueryView"),
            }
        }

        let mut p = Query::new_position(::Format::Time);
        p.set(2 * ::SECOND);

        check_generic(p.into());
    }

    #[test]
    fn test_view_from_concrete() {
        fn check_view(view: &mut QueryView) {
            use glib::ToSendValue;
            match *view {
                QueryView::Position(ref mut p) => {
                    let pos = p.get_result();
                    assert_eq!(pos.try_into_time(), Ok(2 * ::SECOND));
                    p.set(3 * ::SECOND);
               }
                _ => panic!("Wrong concrete Query in QueryView"),
            }

            {
                // ref mut QueryView to generic Query as ref
                let query: &Query = view.into();
                assert_eq!((query.0).type_, ffi::GST_QUERY_POSITION);
            }

            {
                // ref mut QueryView to generic Query as ref mut
                let query: &mut Query = view.into();
                {
                    let structure = query.get_mut().unwrap().get_mut_structure();
                    structure.set_value("test", true.to_send_value());
                }
                unsafe { assert!(!query.as_mut_ptr().is_null()); }
            }
        }

        // Concrete Position
        let mut p = Query::new_position(::Format::Time);
        p.set(2 * ::SECOND);

        // Postion to QueryView
        let mut view: QueryView = p.into();
        check_view(&mut view);

        match view {
            QueryView::Position(ref p) => {
                let pos = p.get_result();
                assert_eq!(pos.try_into_time(), Ok(3 * ::SECOND));
           }
            _ => panic!("Wrong concrete Query in QueryView"),
        }

        // QueryView to generic Query
        let query: &Query = &view.into();
        assert_eq!((query.0).type_, ffi::GST_QUERY_POSITION);

        // structure was updated in function check_view
        let structure = query.get_structure().unwrap();
        assert!(structure.has_field("test"));
    }

    #[test]
    fn test_concrete_to_ffi() {
        let p = Query::new_position(::Format::Time);
        unsafe { assert!(!p.into_ptr().is_null()); }
    }
}
