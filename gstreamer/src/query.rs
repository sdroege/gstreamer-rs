// Take a look at the license at the top of the repository in the LICENSE file.

use crate::structure::*;
use crate::GenericFormattedValue;

use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;

use glib::translate::*;

gst_define_mini_object_wrapper!(Query, QueryRef, ffi::GstQuery, || {
    ffi::gst_query_get_type()
});

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
        unsafe { ((*self.as_ptr()).type_ as u32) & ffi::GST_QUERY_TYPE_DOWNSTREAM != 0 }
    }

    pub fn is_upstream(&self) -> bool {
        unsafe { ((*self.as_ptr()).type_ as u32) & ffi::GST_QUERY_TYPE_UPSTREAM != 0 }
    }

    pub fn is_serialized(&self) -> bool {
        unsafe { ((*self.as_ptr()).type_ as u32) & ffi::GST_QUERY_TYPE_SERIALIZED != 0 }
    }

    pub fn view(&self) -> QueryView<&Self> {
        let type_ = unsafe { (*self.as_ptr()).type_ };

        match type_ {
            ffi::GST_QUERY_POSITION => QueryView::Position(Position(self)),
            ffi::GST_QUERY_DURATION => QueryView::Duration(Duration(self)),
            ffi::GST_QUERY_LATENCY => QueryView::Latency(Latency(self)),
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
            ffi::GST_QUERY_BITRATE => QueryView::Bitrate(Bitrate(self)),
            _ => QueryView::Other(Other(self)),
        }
    }

    pub fn view_mut(&mut self) -> QueryView<&mut Self> {
        unsafe { mem::transmute(self.view()) }
    }
}

impl fmt::Debug for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        QueryRef::fmt(self, f)
    }
}

impl fmt::Debug for QueryRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Query")
            .field("ptr", unsafe { &self.as_ptr() })
            .field("type", &unsafe {
                let type_ = ffi::gst_query_type_get_name((*self.as_ptr()).type_);
                CStr::from_ptr(type_).to_str().unwrap()
            })
            .field("structure", &self.get_structure())
            .finish()
    }
}

pub unsafe trait AsPtr {
    unsafe fn as_ptr(&self) -> *mut ffi::GstQuery;
}

pub unsafe trait AsMutPtr: AsPtr {
    unsafe fn as_mut_ptr(&self) -> *mut ffi::GstQuery;
}

unsafe impl AsPtr for Query {
    unsafe fn as_ptr(&self) -> *mut ffi::GstQuery {
        QueryRef::as_ptr(self) as *mut ffi::GstQuery
    }
}

unsafe impl AsMutPtr for Query {
    unsafe fn as_mut_ptr(&self) -> *mut ffi::GstQuery {
        QueryRef::as_ptr(self) as *mut ffi::GstQuery
    }
}

unsafe impl<'a> AsPtr for &'a QueryRef {
    unsafe fn as_ptr(&self) -> *mut ffi::GstQuery {
        QueryRef::as_ptr(self) as *mut ffi::GstQuery
    }
}

unsafe impl<'a> AsPtr for &'a mut QueryRef {
    unsafe fn as_ptr(&self) -> *mut ffi::GstQuery {
        QueryRef::as_ptr(self) as *mut ffi::GstQuery
    }
}

unsafe impl<'a> AsMutPtr for &'a mut QueryRef {
    unsafe fn as_mut_ptr(&self) -> *mut ffi::GstQuery {
        QueryRef::as_ptr(self) as *mut ffi::GstQuery
    }
}

#[derive(Debug)]
pub enum QueryView<T> {
    Position(Position<T>),
    Duration(Duration<T>),
    Latency(Latency<T>),
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
    Bitrate(Bitrate<T>),
    Other(Other<T>),
    __NonExhaustive,
}

macro_rules! declare_concrete_query(
    ($name:ident, $param:ident) => {
        #[derive(Debug)]
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

        impl<'a> Deref for $name<&'a mut QueryRef> {
            type Target = $name<&'a QueryRef>;

            fn deref(&self) -> &Self::Target {
                unsafe {
                    &*(self as *const $name<&'a mut QueryRef> as *const $name<&'a QueryRef>)
                }
            }
        }

        impl<'a> $name<&'a mut QueryRef> {
            pub fn get_mut_query(&mut self) -> &mut QueryRef {
                self.0
            }
        }

        impl Deref for $name<Query> {
            type Target = QueryRef;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name<Query> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.0.get_mut().unwrap()
            }
        }

        impl From<$name<Query>> for Query {
            fn from(concrete: $name<Query>) -> Self {
                skip_assert_initialized!();
                unsafe { from_glib_none(concrete.0.as_mut_ptr()) }
            }
        }
    }
);

declare_concrete_query!(Position, T);
impl Position<Query> {
    pub fn new(fmt: crate::Format) -> Self {
        assert_initialized_main_thread!();
        unsafe { Self(from_glib_full(ffi::gst_query_new_position(fmt.to_glib()))) }
    }
}

impl<T: AsPtr> Position<T> {
    pub fn get_result(&self) -> GenericFormattedValue {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();
            let mut pos = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_position(self.0.as_ptr(), fmt.as_mut_ptr(), pos.as_mut_ptr());

            GenericFormattedValue::new(from_glib(fmt.assume_init()), pos.assume_init())
        }
    }

    pub fn get_format(&self) -> crate::Format {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_position(self.0.as_ptr(), fmt.as_mut_ptr(), ptr::null_mut());

            from_glib(fmt.assume_init())
        }
    }
}

impl<T: AsMutPtr> Position<T> {
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

declare_concrete_query!(Duration, T);
impl Duration<Query> {
    pub fn new(fmt: crate::Format) -> Self {
        assert_initialized_main_thread!();
        unsafe { Self(from_glib_full(ffi::gst_query_new_duration(fmt.to_glib()))) }
    }
}

impl<T: AsPtr> Duration<T> {
    pub fn get_result(&self) -> GenericFormattedValue {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();
            let mut pos = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_duration(self.0.as_ptr(), fmt.as_mut_ptr(), pos.as_mut_ptr());

            GenericFormattedValue::new(from_glib(fmt.assume_init()), pos.assume_init())
        }
    }

    pub fn get_format(&self) -> crate::Format {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_duration(self.0.as_ptr(), fmt.as_mut_ptr(), ptr::null_mut());

            from_glib(fmt.assume_init())
        }
    }
}

impl<T: AsMutPtr> Duration<T> {
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

declare_concrete_query!(Latency, T);
impl Latency<Query> {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { Self(from_glib_full(ffi::gst_query_new_latency())) }
    }
}

impl Default for Latency<Query> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: AsPtr> Latency<T> {
    pub fn get_result(&self) -> (bool, crate::ClockTime, crate::ClockTime) {
        unsafe {
            let mut live = mem::MaybeUninit::uninit();
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_latency(
                self.0.as_ptr(),
                live.as_mut_ptr(),
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );

            (
                from_glib(live.assume_init()),
                from_glib(min.assume_init()),
                from_glib(max.assume_init()),
            )
        }
    }
}

impl<T: AsMutPtr> Latency<T> {
    pub fn set(&mut self, live: bool, min: crate::ClockTime, max: crate::ClockTime) {
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

declare_concrete_query!(Seeking, T);
impl Seeking<Query> {
    pub fn new(fmt: crate::Format) -> Self {
        assert_initialized_main_thread!();
        unsafe { Self(from_glib_full(ffi::gst_query_new_seeking(fmt.to_glib()))) }
    }
}

impl<T: AsPtr> Seeking<T> {
    pub fn get_result(&self) -> (bool, GenericFormattedValue, GenericFormattedValue) {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();
            let mut seekable = mem::MaybeUninit::uninit();
            let mut start = mem::MaybeUninit::uninit();
            let mut end = mem::MaybeUninit::uninit();
            ffi::gst_query_parse_seeking(
                self.0.as_ptr(),
                fmt.as_mut_ptr(),
                seekable.as_mut_ptr(),
                start.as_mut_ptr(),
                end.as_mut_ptr(),
            );

            (
                from_glib(seekable.assume_init()),
                GenericFormattedValue::new(from_glib(fmt.assume_init()), start.assume_init()),
                GenericFormattedValue::new(from_glib(fmt.assume_init()), end.assume_init()),
            )
        }
    }

    pub fn get_format(&self) -> crate::Format {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();
            ffi::gst_query_parse_seeking(
                self.0.as_ptr(),
                fmt.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );

            from_glib(fmt.assume_init())
        }
    }
}

impl<T: AsMutPtr> Seeking<T> {
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

declare_concrete_query!(Segment, T);
impl Segment<Query> {
    pub fn new(fmt: crate::Format) -> Self {
        assert_initialized_main_thread!();
        unsafe { Self(from_glib_full(ffi::gst_query_new_segment(fmt.to_glib()))) }
    }
}

impl<T: AsPtr> Segment<T> {
    pub fn get_result(&self) -> (f64, GenericFormattedValue, GenericFormattedValue) {
        unsafe {
            let mut rate = mem::MaybeUninit::uninit();
            let mut fmt = mem::MaybeUninit::uninit();
            let mut start = mem::MaybeUninit::uninit();
            let mut stop = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_segment(
                self.0.as_ptr(),
                rate.as_mut_ptr(),
                fmt.as_mut_ptr(),
                start.as_mut_ptr(),
                stop.as_mut_ptr(),
            );
            (
                rate.assume_init(),
                GenericFormattedValue::new(from_glib(fmt.assume_init()), start.assume_init()),
                GenericFormattedValue::new(from_glib(fmt.assume_init()), stop.assume_init()),
            )
        }
    }

    pub fn get_format(&self) -> crate::Format {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_segment(
                self.0.as_ptr(),
                ptr::null_mut(),
                fmt.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
            );
            from_glib(fmt.assume_init())
        }
    }
}

impl<T: AsMutPtr> Segment<T> {
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

declare_concrete_query!(Convert, T);
impl Convert<Query> {
    pub fn new<V: Into<GenericFormattedValue>>(value: V, dest_fmt: crate::Format) -> Self {
        assert_initialized_main_thread!();
        let value = value.into();
        unsafe {
            Self(from_glib_full(ffi::gst_query_new_convert(
                value.get_format().to_glib(),
                value.get_value(),
                dest_fmt.to_glib(),
            )))
        }
    }
}

impl<T: AsPtr> Convert<T> {
    pub fn get_result(&self) -> (GenericFormattedValue, GenericFormattedValue) {
        unsafe {
            let mut src_fmt = mem::MaybeUninit::uninit();
            let mut src = mem::MaybeUninit::uninit();
            let mut dest_fmt = mem::MaybeUninit::uninit();
            let mut dest = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_convert(
                self.0.as_ptr(),
                src_fmt.as_mut_ptr(),
                src.as_mut_ptr(),
                dest_fmt.as_mut_ptr(),
                dest.as_mut_ptr(),
            );
            (
                GenericFormattedValue::new(from_glib(src_fmt.assume_init()), src.assume_init()),
                GenericFormattedValue::new(from_glib(dest_fmt.assume_init()), dest.assume_init()),
            )
        }
    }

    pub fn get(&self) -> (GenericFormattedValue, crate::Format) {
        unsafe {
            let mut src_fmt = mem::MaybeUninit::uninit();
            let mut src = mem::MaybeUninit::uninit();
            let mut dest_fmt = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_convert(
                self.0.as_ptr(),
                src_fmt.as_mut_ptr(),
                src.as_mut_ptr(),
                dest_fmt.as_mut_ptr(),
                ptr::null_mut(),
            );
            (
                GenericFormattedValue::new(from_glib(src_fmt.assume_init()), src.assume_init()),
                from_glib(dest_fmt.assume_init()),
            )
        }
    }
}

impl<T: AsMutPtr> Convert<T> {
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

declare_concrete_query!(Formats, T);
impl Formats<Query> {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { Self(from_glib_full(ffi::gst_query_new_formats())) }
    }
}

impl Default for Formats<Query> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: AsPtr> Formats<T> {
    pub fn get_result(&self) -> Vec<crate::Format> {
        unsafe {
            let mut n = mem::MaybeUninit::uninit();
            ffi::gst_query_parse_n_formats(self.0.as_ptr(), n.as_mut_ptr());
            let n = n.assume_init();
            let mut res = Vec::with_capacity(n as usize);

            for i in 0..n {
                let mut fmt = mem::MaybeUninit::uninit();
                ffi::gst_query_parse_nth_format(self.0.as_ptr(), i, fmt.as_mut_ptr());
                res.push(from_glib(fmt.assume_init()));
            }

            res
        }
    }
}

impl<T: AsMutPtr> Formats<T> {
    pub fn set(&mut self, formats: &[crate::Format]) {
        unsafe {
            let v: Vec<_> = formats.iter().map(|f| f.to_glib()).collect();
            ffi::gst_query_set_formatsv(self.0.as_mut_ptr(), v.len() as i32, v.as_ptr() as *mut _);
        }
    }
}

declare_concrete_query!(Buffering, T);
impl Buffering<Query> {
    pub fn new(fmt: crate::Format) -> Self {
        assert_initialized_main_thread!();
        unsafe { Self(from_glib_full(ffi::gst_query_new_buffering(fmt.to_glib()))) }
    }
}

impl<T: AsPtr> Buffering<T> {
    pub fn get_format(&self) -> crate::Format {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_buffering_range(
                self.0.as_ptr(),
                fmt.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );

            from_glib(fmt.assume_init())
        }
    }

    pub fn get_percent(&self) -> (bool, i32) {
        unsafe {
            let mut busy = mem::MaybeUninit::uninit();
            let mut percent = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_buffering_percent(
                self.0.as_ptr(),
                busy.as_mut_ptr(),
                percent.as_mut_ptr(),
            );

            (from_glib(busy.assume_init()), percent.assume_init())
        }
    }

    pub fn get_range(&self) -> (GenericFormattedValue, GenericFormattedValue, i64) {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();
            let mut start = mem::MaybeUninit::uninit();
            let mut stop = mem::MaybeUninit::uninit();
            let mut estimated_total = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_buffering_range(
                self.0.as_ptr(),
                fmt.as_mut_ptr(),
                start.as_mut_ptr(),
                stop.as_mut_ptr(),
                estimated_total.as_mut_ptr(),
            );
            (
                GenericFormattedValue::new(from_glib(fmt.assume_init()), start.assume_init()),
                GenericFormattedValue::new(from_glib(fmt.assume_init()), stop.assume_init()),
                estimated_total.assume_init(),
            )
        }
    }

    pub fn get_stats(&self) -> (crate::BufferingMode, i32, i32, i64) {
        unsafe {
            let mut mode = mem::MaybeUninit::uninit();
            let mut avg_in = mem::MaybeUninit::uninit();
            let mut avg_out = mem::MaybeUninit::uninit();
            let mut buffering_left = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_buffering_stats(
                self.0.as_ptr(),
                mode.as_mut_ptr(),
                avg_in.as_mut_ptr(),
                avg_out.as_mut_ptr(),
                buffering_left.as_mut_ptr(),
            );

            (
                from_glib(mode.assume_init()),
                avg_in.assume_init(),
                avg_out.assume_init(),
                buffering_left.assume_init(),
            )
        }
    }

    pub fn get_ranges(&self) -> Vec<(GenericFormattedValue, GenericFormattedValue)> {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();
            ffi::gst_query_parse_buffering_range(
                self.0.as_ptr(),
                fmt.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );
            let fmt = from_glib(fmt.assume_init());

            let n = ffi::gst_query_get_n_buffering_ranges(self.0.as_ptr());
            let mut res = Vec::with_capacity(n as usize);
            for i in 0..n {
                let mut start = mem::MaybeUninit::uninit();
                let mut stop = mem::MaybeUninit::uninit();
                let s: bool = from_glib(ffi::gst_query_parse_nth_buffering_range(
                    self.0.as_ptr(),
                    i,
                    start.as_mut_ptr(),
                    stop.as_mut_ptr(),
                ));
                if s {
                    res.push((
                        GenericFormattedValue::new(fmt, start.assume_init()),
                        GenericFormattedValue::new(fmt, stop.assume_init()),
                    ));
                }
            }

            res
        }
    }
}

impl<T: AsMutPtr> Buffering<T> {
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
        mode: crate::BufferingMode,
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

declare_concrete_query!(Custom, T);
impl Custom<Query> {
    pub fn new(structure: crate::Structure) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Self(from_glib_full(ffi::gst_query_new_custom(
                ffi::GST_QUERY_CUSTOM,
                structure.into_ptr(),
            )))
        }
    }
}

declare_concrete_query!(Uri, T);
impl Uri<Query> {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { Self(from_glib_full(ffi::gst_query_new_uri())) }
    }
}

impl Default for Uri<Query> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: AsPtr> Uri<T> {
    pub fn get_uri(&self) -> Option<String> {
        unsafe {
            let mut uri = ptr::null_mut();
            ffi::gst_query_parse_uri(self.0.as_ptr(), &mut uri);
            from_glib_full(uri)
        }
    }

    pub fn get_redirection(&self) -> (Option<String>, bool) {
        unsafe {
            let mut uri = ptr::null_mut();
            ffi::gst_query_parse_uri_redirection(self.0.as_ptr(), &mut uri);
            let mut permanent = mem::MaybeUninit::uninit();
            ffi::gst_query_parse_uri_redirection_permanent(self.0.as_ptr(), permanent.as_mut_ptr());

            (from_glib_full(uri), from_glib(permanent.assume_init()))
        }
    }
}

impl<T: AsMutPtr> Uri<T> {
    pub fn set_uri<'b, U: Into<&'b str>>(&mut self, uri: U) {
        let uri = uri.into();
        unsafe {
            ffi::gst_query_set_uri(self.0.as_mut_ptr(), uri.to_glib_none().0);
        }
    }

    pub fn set_redirection<'b, U: Into<&'b str>>(&mut self, uri: U, permanent: bool) {
        let uri = uri.into();
        unsafe {
            ffi::gst_query_set_uri_redirection(self.0.as_mut_ptr(), uri.to_glib_none().0);
            ffi::gst_query_set_uri_redirection_permanent(self.0.as_mut_ptr(), permanent.to_glib());
        }
    }
}

declare_concrete_query!(Allocation, T);
impl Allocation<Query> {
    pub fn new(caps: &crate::Caps, need_pool: bool) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Self(from_glib_full(ffi::gst_query_new_allocation(
                caps.as_mut_ptr(),
                need_pool.to_glib(),
            )))
        }
    }
}

impl<T: AsPtr> Allocation<T> {
    pub fn get(&self) -> (&crate::CapsRef, bool) {
        unsafe {
            let mut caps = ptr::null_mut();
            let mut need_pool = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_allocation(self.0.as_ptr(), &mut caps, need_pool.as_mut_ptr());
            (
                crate::CapsRef::from_ptr(caps),
                from_glib(need_pool.assume_init()),
            )
        }
    }

    pub fn get_owned(&self) -> (crate::Caps, bool) {
        unsafe {
            let (caps, need_pool) = self.get();
            (from_glib_none(caps.as_ptr()), need_pool)
        }
    }

    pub fn get_allocation_pools(&self) -> Vec<(Option<crate::BufferPool>, u32, u32, u32)> {
        unsafe {
            let n = ffi::gst_query_get_n_allocation_pools(self.0.as_ptr());
            let mut pools = Vec::with_capacity(n as usize);
            for i in 0..n {
                let mut pool = ptr::null_mut();
                let mut size = mem::MaybeUninit::uninit();
                let mut min_buffers = mem::MaybeUninit::uninit();
                let mut max_buffers = mem::MaybeUninit::uninit();

                ffi::gst_query_parse_nth_allocation_pool(
                    self.0.as_ptr(),
                    i,
                    &mut pool,
                    size.as_mut_ptr(),
                    min_buffers.as_mut_ptr(),
                    max_buffers.as_mut_ptr(),
                );
                pools.push((
                    from_glib_full(pool),
                    size.assume_init(),
                    min_buffers.assume_init(),
                    max_buffers.assume_init(),
                ));
            }

            pools
        }
    }

    pub fn get_allocation_metas(&self) -> Vec<(glib::Type, Option<&crate::StructureRef>)> {
        unsafe {
            let n = ffi::gst_query_get_n_allocation_metas(self.0.as_ptr());
            let mut metas = Vec::with_capacity(n as usize);
            for i in 0..n {
                let mut structure = ptr::null();

                let api =
                    ffi::gst_query_parse_nth_allocation_meta(self.0.as_ptr(), i, &mut structure);
                metas.push((
                    from_glib(api),
                    if structure.is_null() {
                        None
                    } else {
                        Some(crate::StructureRef::from_glib_borrow(structure))
                    },
                ));
            }

            metas
        }
    }

    pub fn find_allocation_meta<U: crate::MetaAPI>(&self) -> Option<u32> {
        unsafe {
            let mut idx = mem::MaybeUninit::uninit();
            if ffi::gst_query_find_allocation_meta(
                self.0.as_ptr(),
                U::get_meta_api().to_glib(),
                idx.as_mut_ptr(),
            ) != glib::ffi::GFALSE
            {
                Some(idx.assume_init())
            } else {
                None
            }
        }
    }
}

impl<T: AsMutPtr> Allocation<T> {
    pub fn add_allocation_pool(
        &mut self,
        pool: Option<&crate::BufferPool>,
        size: u32,
        min_buffers: u32,
        max_buffers: u32,
    ) {
        unsafe {
            ffi::gst_query_add_allocation_pool(
                self.0.as_mut_ptr(),
                pool.to_glib_none().0,
                size,
                min_buffers,
                max_buffers,
            );
        }
    }

    pub fn set_nth_allocation_pool(
        &mut self,
        idx: u32,
        pool: Option<&crate::BufferPool>,
        size: u32,
        min_buffers: u32,
        max_buffers: u32,
    ) {
        unsafe {
            ffi::gst_query_set_nth_allocation_pool(
                self.0.as_mut_ptr(),
                idx,
                pool.to_glib_none().0,
                size,
                min_buffers,
                max_buffers,
            );
        }
    }

    pub fn remove_nth_allocation_pool(&mut self, idx: u32) {
        unsafe {
            ffi::gst_query_remove_nth_allocation_pool(self.0.as_mut_ptr(), idx);
        }
    }

    pub fn add_allocation_meta<U: crate::MetaAPI>(
        &mut self,
        structure: Option<&crate::StructureRef>,
    ) {
        unsafe {
            ffi::gst_query_add_allocation_meta(
                self.0.as_mut_ptr(),
                U::get_meta_api().to_glib(),
                if let Some(structure) = structure {
                    structure.as_ptr()
                } else {
                    ptr::null()
                },
            );
        }
    }

    pub fn remove_nth_allocation_meta(&mut self, idx: u32) {
        unsafe {
            ffi::gst_query_remove_nth_allocation_meta(self.0.as_mut_ptr(), idx);
        }
    }
}

declare_concrete_query!(Scheduling, T);
impl Scheduling<Query> {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { Self(from_glib_full(ffi::gst_query_new_scheduling())) }
    }
}

impl Default for Scheduling<Query> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: AsPtr> Scheduling<T> {
    pub fn has_scheduling_mode(&self, mode: crate::PadMode) -> bool {
        unsafe {
            from_glib(ffi::gst_query_has_scheduling_mode(
                self.0.as_ptr(),
                mode.to_glib(),
            ))
        }
    }

    pub fn has_scheduling_mode_with_flags(
        &self,
        mode: crate::PadMode,
        flags: crate::SchedulingFlags,
    ) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gst_query_has_scheduling_mode_with_flags(
                self.0.as_ptr(),
                mode.to_glib(),
                flags.to_glib(),
            ))
        }
    }

    pub fn get_scheduling_modes(&self) -> Vec<crate::PadMode> {
        unsafe {
            let n = ffi::gst_query_get_n_scheduling_modes(self.0.as_ptr());
            let mut res = Vec::with_capacity(n as usize);
            for i in 0..n {
                res.push(from_glib(ffi::gst_query_parse_nth_scheduling_mode(
                    self.0.as_ptr(),
                    i,
                )));
            }

            res
        }
    }

    pub fn get_result(&self) -> (crate::SchedulingFlags, i32, i32, i32) {
        unsafe {
            let mut flags = mem::MaybeUninit::uninit();
            let mut minsize = mem::MaybeUninit::uninit();
            let mut maxsize = mem::MaybeUninit::uninit();
            let mut align = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_scheduling(
                self.0.as_ptr(),
                flags.as_mut_ptr(),
                minsize.as_mut_ptr(),
                maxsize.as_mut_ptr(),
                align.as_mut_ptr(),
            );

            (
                from_glib(flags.assume_init()),
                minsize.assume_init(),
                maxsize.assume_init(),
                align.assume_init(),
            )
        }
    }
}

impl<T: AsMutPtr> Scheduling<T> {
    pub fn add_scheduling_modes(&mut self, modes: &[crate::PadMode]) {
        unsafe {
            for mode in modes {
                ffi::gst_query_add_scheduling_mode(self.0.as_mut_ptr(), mode.to_glib());
            }
        }
    }

    pub fn set(&mut self, flags: crate::SchedulingFlags, minsize: i32, maxsize: i32, align: i32) {
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

declare_concrete_query!(AcceptCaps, T);
impl AcceptCaps<Query> {
    pub fn new(caps: &crate::Caps) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Self(from_glib_full(ffi::gst_query_new_accept_caps(
                caps.as_mut_ptr(),
            )))
        }
    }
}

impl<T: AsPtr> AcceptCaps<T> {
    pub fn get_caps(&self) -> &crate::CapsRef {
        unsafe {
            let mut caps = ptr::null_mut();
            ffi::gst_query_parse_accept_caps(self.0.as_ptr(), &mut caps);
            crate::CapsRef::from_ptr(caps)
        }
    }

    pub fn get_caps_owned(&self) -> crate::Caps {
        unsafe { from_glib_none(self.get_caps().as_ptr()) }
    }

    pub fn get_result(&self) -> bool {
        unsafe {
            let mut accepted = mem::MaybeUninit::uninit();
            ffi::gst_query_parse_accept_caps_result(self.0.as_ptr(), accepted.as_mut_ptr());
            from_glib(accepted.assume_init())
        }
    }
}

impl<T: AsMutPtr> AcceptCaps<T> {
    pub fn set_result(&mut self, accepted: bool) {
        unsafe {
            ffi::gst_query_set_accept_caps_result(self.0.as_mut_ptr(), accepted.to_glib());
        }
    }
}

declare_concrete_query!(Caps, T);
impl Caps<Query> {
    pub fn new(filter: Option<&crate::Caps>) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Self(from_glib_full(ffi::gst_query_new_caps(
                filter.to_glib_none().0,
            )))
        }
    }
}

impl<T: AsPtr> Caps<T> {
    pub fn get_filter(&self) -> Option<&crate::CapsRef> {
        unsafe {
            let mut caps = ptr::null_mut();
            ffi::gst_query_parse_caps(self.0.as_ptr(), &mut caps);
            if caps.is_null() {
                None
            } else {
                Some(crate::CapsRef::from_ptr(caps))
            }
        }
    }

    pub fn get_filter_owned(&self) -> Option<crate::Caps> {
        unsafe { self.get_filter().map(|caps| from_glib_none(caps.as_ptr())) }
    }

    pub fn get_result(&self) -> Option<&crate::CapsRef> {
        unsafe {
            let mut caps = ptr::null_mut();
            ffi::gst_query_parse_caps_result(self.0.as_ptr(), &mut caps);
            if caps.is_null() {
                None
            } else {
                Some(crate::CapsRef::from_ptr(caps))
            }
        }
    }

    pub fn get_result_owned(&self) -> Option<crate::Caps> {
        unsafe { self.get_result().map(|caps| from_glib_none(caps.as_ptr())) }
    }
}

impl<T: AsMutPtr> Caps<T> {
    pub fn set_result(&mut self, caps: &crate::Caps) {
        unsafe {
            ffi::gst_query_set_caps_result(self.0.as_mut_ptr(), caps.as_mut_ptr());
        }
    }
}

declare_concrete_query!(Drain, T);
impl Drain<Query> {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { Self(from_glib_full(ffi::gst_query_new_drain())) }
    }
}

impl Default for Drain<Query> {
    fn default() -> Self {
        Self::new()
    }
}

declare_concrete_query!(Context, T);
impl Context<Query> {
    pub fn new(context_type: &str) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Self(from_glib_full(ffi::gst_query_new_context(
                context_type.to_glib_none().0,
            )))
        }
    }
}

impl<T: AsPtr> Context<T> {
    pub fn get_context(&self) -> Option<&crate::ContextRef> {
        unsafe {
            let mut context = ptr::null_mut();
            ffi::gst_query_parse_context(self.0.as_ptr(), &mut context);
            if context.is_null() {
                None
            } else {
                Some(crate::ContextRef::from_ptr(context))
            }
        }
    }

    pub fn get_context_owned(&self) -> Option<crate::Context> {
        unsafe {
            self.get_context()
                .map(|context| from_glib_none(context.as_ptr()))
        }
    }

    pub fn get_context_type(&self) -> &str {
        unsafe {
            let mut context_type = ptr::null();
            ffi::gst_query_parse_context_type(self.0.as_ptr(), &mut context_type);
            CStr::from_ptr(context_type).to_str().unwrap()
        }
    }
}

impl<T: AsMutPtr> Context<T> {
    pub fn set_context(&mut self, context: &crate::Context) {
        unsafe {
            ffi::gst_query_set_context(self.0.as_mut_ptr(), context.as_mut_ptr());
        }
    }
}

declare_concrete_query!(Bitrate, T);

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
impl Bitrate<Query> {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { Self(from_glib_full(ffi::gst_query_new_bitrate())) }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
impl Default for Bitrate<Query> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: AsPtr> Bitrate<T> {
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn get_bitrate(&self) -> u32 {
        unsafe {
            let mut bitrate = mem::MaybeUninit::uninit();
            ffi::gst_query_parse_bitrate(self.0.as_ptr(), bitrate.as_mut_ptr());
            bitrate.assume_init()
        }
    }
}

impl<T: AsMutPtr> Bitrate<T> {
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn set_bitrate(&mut self, bitrate: u32) {
        unsafe {
            ffi::gst_query_set_bitrate(self.0.as_mut_ptr(), bitrate);
        }
    }
}

declare_concrete_query!(Other, T);

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_writability() {
        crate::init().unwrap();

        fn check_mut(query: &mut QueryRef) {
            skip_assert_initialized!();
            match query.view_mut() {
                QueryView::Position(ref mut p) => {
                    let pos = p.get_result();
                    assert_eq!(pos.try_into(), Ok(crate::CLOCK_TIME_NONE));
                    p.set(3 * crate::SECOND);
                    let pos = p.get_result();
                    assert_eq!(pos.try_into(), Ok(3 * crate::SECOND));
                }
                _ => panic!("Wrong concrete Query in Query"),
            }
        }

        fn check_ref(query: &QueryRef) {
            skip_assert_initialized!();
            match query.view() {
                QueryView::Position(ref p) => {
                    let pos = p.get_result();
                    assert_eq!(pos.try_into(), Ok(3 * crate::SECOND));
                    unsafe {
                        assert!(!p.as_mut_ptr().is_null());
                    }
                }
                _ => panic!("Wrong concrete Query in Query"),
            }
        }

        let mut p = Position::new(crate::Format::Time);
        let pos = p.get_result();
        assert_eq!(pos.try_into(), Ok(crate::CLOCK_TIME_NONE));

        p.get_mut_structure().set("check_mut", &true);

        // deref
        assert!(!p.is_serialized());

        {
            check_mut(&mut p);

            let structure = p.get_structure();
            structure.unwrap().has_field("check_mut");

            // Expected: cannot borrow `p` as mutable because it is also borrowed as immutable
            //check_mut(&mut p);
        }

        check_ref(&p);
    }

    #[test]
    fn test_into_query() {
        crate::init().unwrap();
        let d = Duration::new(crate::Format::Time);

        let mut query: Query = d.into();
        assert!(query.is_writable());

        let query = query.make_mut();
        if let QueryView::Duration(d) = &mut query.view_mut() {
            d.set(2 * crate::SECOND);
        }

        if let QueryView::Duration(d) = &query.view() {
            let duration = d.get_result();
            assert_eq!(duration.try_into(), Ok(2 * crate::SECOND));
        }
    }

    #[test]
    fn test_concrete_to_sys() {
        crate::init().unwrap();

        let p = Position::new(crate::Format::Time);
        unsafe {
            assert!(!p.as_mut_ptr().is_null());
        }
    }
}
