// Take a look at the license at the top of the repository in the LICENSE file.

use crate::structure::*;
use crate::GenericFormattedValue;

use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;

use glib::object::IsA;
use glib::translate::*;

mini_object_wrapper!(Query, QueryRef, ffi::GstQuery, || {
    ffi::gst_query_get_type()
});

impl QueryRef {
    #[doc(alias = "get_structure")]
    #[doc(alias = "gst_query_get_structure")]
    pub fn structure(&self) -> Option<&StructureRef> {
        unsafe {
            let structure = ffi::gst_query_get_structure(self.as_mut_ptr());
            if structure.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(structure))
            }
        }
    }

    #[doc(alias = "get_mut_structure")]
    pub fn structure_mut(&mut self) -> &mut StructureRef {
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
            .field("structure", &self.structure())
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
            pub fn query(&self) -> &QueryRef {
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
            pub fn query_mut(&mut self) -> &mut QueryRef {
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
    #[doc(alias = "gst_query_new_position")]
    pub fn new(fmt: crate::Format) -> Self {
        assert_initialized_main_thread!();
        unsafe { Self(from_glib_full(ffi::gst_query_new_position(fmt.into_glib()))) }
    }
}

impl<T: AsPtr> Position<T> {
    #[doc(alias = "get_result")]
    pub fn result(&self) -> GenericFormattedValue {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();
            let mut pos = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_position(self.0.as_ptr(), fmt.as_mut_ptr(), pos.as_mut_ptr());

            GenericFormattedValue::new(from_glib(fmt.assume_init()), pos.assume_init())
        }
    }

    #[doc(alias = "get_format")]
    pub fn format(&self) -> crate::Format {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_position(self.0.as_ptr(), fmt.as_mut_ptr(), ptr::null_mut());

            from_glib(fmt.assume_init())
        }
    }
}

impl<T: AsMutPtr> Position<T> {
    #[doc(alias = "gst_query_set_position")]
    pub fn set<V: Into<GenericFormattedValue>>(&mut self, pos: V) {
        let pos = pos.into();
        assert_eq!(pos.format(), self.format());
        unsafe {
            ffi::gst_query_set_position(self.0.as_mut_ptr(), pos.format().into_glib(), pos.value());
        }
    }
}

declare_concrete_query!(Duration, T);
impl Duration<Query> {
    #[doc(alias = "gst_query_new_duration")]
    pub fn new(fmt: crate::Format) -> Self {
        assert_initialized_main_thread!();
        unsafe { Self(from_glib_full(ffi::gst_query_new_duration(fmt.into_glib()))) }
    }
}

impl<T: AsPtr> Duration<T> {
    #[doc(alias = "get_result")]
    pub fn result(&self) -> GenericFormattedValue {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();
            let mut pos = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_duration(self.0.as_ptr(), fmt.as_mut_ptr(), pos.as_mut_ptr());

            GenericFormattedValue::new(from_glib(fmt.assume_init()), pos.assume_init())
        }
    }

    #[doc(alias = "get_format")]
    pub fn format(&self) -> crate::Format {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_duration(self.0.as_ptr(), fmt.as_mut_ptr(), ptr::null_mut());

            from_glib(fmt.assume_init())
        }
    }
}

impl<T: AsMutPtr> Duration<T> {
    #[doc(alias = "gst_query_set_duration")]
    pub fn set<V: Into<GenericFormattedValue>>(&mut self, dur: V) {
        let dur = dur.into();
        assert_eq!(dur.format(), self.format());
        unsafe {
            ffi::gst_query_set_duration(self.0.as_mut_ptr(), dur.format().into_glib(), dur.value());
        }
    }
}

declare_concrete_query!(Latency, T);
impl Latency<Query> {
    #[doc(alias = "gst_query_new_latency")]
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
    #[doc(alias = "get_result")]
    pub fn result(&self) -> (bool, crate::ClockTime, Option<crate::ClockTime>) {
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
                try_from_glib(min.assume_init()).expect("undefined min latency"),
                from_glib(max.assume_init()),
            )
        }
    }
}

impl<T: AsMutPtr> Latency<T> {
    #[doc(alias = "gst_query_set_latency")]
    pub fn set(
        &mut self,
        live: bool,
        min: crate::ClockTime,
        max: impl Into<Option<crate::ClockTime>>,
    ) {
        unsafe {
            ffi::gst_query_set_latency(
                self.0.as_mut_ptr(),
                live.into_glib(),
                min.into_glib(),
                max.into().into_glib(),
            );
        }
    }
}

declare_concrete_query!(Seeking, T);
impl Seeking<Query> {
    #[doc(alias = "gst_query_new_seeking")]
    pub fn new(fmt: crate::Format) -> Self {
        assert_initialized_main_thread!();
        unsafe { Self(from_glib_full(ffi::gst_query_new_seeking(fmt.into_glib()))) }
    }
}

impl<T: AsPtr> Seeking<T> {
    #[doc(alias = "get_result")]
    pub fn result(&self) -> (bool, GenericFormattedValue, GenericFormattedValue) {
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

    #[doc(alias = "get_format")]
    pub fn format(&self) -> crate::Format {
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
    #[doc(alias = "gst_query_set_seeking")]
    pub fn set<V: Into<GenericFormattedValue>>(&mut self, seekable: bool, start: V, end: V) {
        let start = start.into();
        let end = end.into();

        assert_eq!(self.format(), start.format());
        assert_eq!(start.format(), end.format());

        unsafe {
            ffi::gst_query_set_seeking(
                self.0.as_mut_ptr(),
                start.format().into_glib(),
                seekable.into_glib(),
                start.value(),
                end.value(),
            );
        }
    }
}

declare_concrete_query!(Segment, T);
impl Segment<Query> {
    #[doc(alias = "gst_query_new_segment")]
    pub fn new(fmt: crate::Format) -> Self {
        assert_initialized_main_thread!();
        unsafe { Self(from_glib_full(ffi::gst_query_new_segment(fmt.into_glib()))) }
    }
}

impl<T: AsPtr> Segment<T> {
    #[doc(alias = "get_result")]
    pub fn result(&self) -> (f64, GenericFormattedValue, GenericFormattedValue) {
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

    #[doc(alias = "get_format")]
    pub fn format(&self) -> crate::Format {
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
    #[doc(alias = "gst_query_set_segment")]
    pub fn set<V: Into<GenericFormattedValue>>(&mut self, rate: f64, start: V, stop: V) {
        let start = start.into();
        let stop = stop.into();

        assert_eq!(start.format(), stop.format());

        unsafe {
            ffi::gst_query_set_segment(
                self.0.as_mut_ptr(),
                rate,
                start.format().into_glib(),
                start.value(),
                stop.value(),
            );
        }
    }
}

declare_concrete_query!(Convert, T);
impl Convert<Query> {
    #[doc(alias = "gst_query_new_convert")]
    pub fn new<V: Into<GenericFormattedValue>>(value: V, dest_fmt: crate::Format) -> Self {
        assert_initialized_main_thread!();
        let value = value.into();
        unsafe {
            Self(from_glib_full(ffi::gst_query_new_convert(
                value.format().into_glib(),
                value.value(),
                dest_fmt.into_glib(),
            )))
        }
    }
}

impl<T: AsPtr> Convert<T> {
    #[doc(alias = "get_result")]
    pub fn result(&self) -> (GenericFormattedValue, GenericFormattedValue) {
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
    #[doc(alias = "gst_query_set_convert")]
    pub fn set<V: Into<GenericFormattedValue>>(&mut self, src: V, dest: V) {
        let src = src.into();
        let dest = dest.into();

        unsafe {
            ffi::gst_query_set_convert(
                self.0.as_mut_ptr(),
                src.format().into_glib(),
                src.value(),
                dest.format().into_glib(),
                dest.value(),
            );
        }
    }
}

declare_concrete_query!(Formats, T);
impl Formats<Query> {
    #[doc(alias = "gst_query_new_formats")]
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
    #[doc(alias = "get_result")]
    pub fn result(&self) -> Vec<crate::Format> {
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
    #[doc(alias = "gst_query_set_formatsv")]
    pub fn set(&mut self, formats: &[crate::Format]) {
        unsafe {
            let v: Vec<_> = formats.iter().map(|f| f.into_glib()).collect();
            ffi::gst_query_set_formatsv(self.0.as_mut_ptr(), v.len() as i32, v.as_ptr() as *mut _);
        }
    }
}

declare_concrete_query!(Buffering, T);
impl Buffering<Query> {
    #[doc(alias = "gst_query_new_buffering")]
    pub fn new(fmt: crate::Format) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Self(from_glib_full(ffi::gst_query_new_buffering(
                fmt.into_glib(),
            )))
        }
    }
}

impl<T: AsPtr> Buffering<T> {
    #[doc(alias = "get_format")]
    pub fn format(&self) -> crate::Format {
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

    #[doc(alias = "get_percent")]
    #[doc(alias = "gst_query_parse_buffering_percent")]
    pub fn percent(&self) -> (bool, i32) {
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

    #[doc(alias = "get_range")]
    #[doc(alias = "gst_query_parse_buffering_range")]
    pub fn range(&self) -> (GenericFormattedValue, GenericFormattedValue, i64) {
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

    #[doc(alias = "get_stats")]
    #[doc(alias = "gst_query_parse_buffering_stats")]
    pub fn stats(&self) -> (crate::BufferingMode, i32, i32, i64) {
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

    #[doc(alias = "get_ranges")]
    #[doc(alias = "gst_query_get_n_buffering_ranges")]
    pub fn ranges(&self) -> Vec<(GenericFormattedValue, GenericFormattedValue)> {
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
            ffi::gst_query_set_buffering_percent(self.0.as_mut_ptr(), busy.into_glib(), percent);
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

        assert_eq!(self.format(), start.format());
        assert_eq!(start.format(), stop.format());

        unsafe {
            ffi::gst_query_set_buffering_range(
                self.0.as_mut_ptr(),
                start.format().into_glib(),
                start.value(),
                stop.value(),
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
                mode.into_glib(),
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
            let fmt = self.format();

            for &(start, stop) in ranges {
                let start = start.into();
                let stop = stop.into();
                assert_eq!(start.format(), fmt);
                assert_eq!(stop.format(), fmt);
                ffi::gst_query_add_buffering_range(
                    self.0.as_mut_ptr(),
                    start.value(),
                    stop.value(),
                );
            }
        }
    }
}

declare_concrete_query!(Custom, T);
impl Custom<Query> {
    #[doc(alias = "gst_query_new_custom")]
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
    #[doc(alias = "gst_query_new_uri")]
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
    #[doc(alias = "get_uri")]
    #[doc(alias = "gst_query_parse_uri")]
    pub fn uri(&self) -> Option<String> {
        unsafe {
            let mut uri = ptr::null_mut();
            ffi::gst_query_parse_uri(self.0.as_ptr(), &mut uri);
            from_glib_full(uri)
        }
    }

    #[doc(alias = "get_redirection")]
    #[doc(alias = "gst_query_parse_uri_redirection")]
    #[doc(alias = "gst_query_parse_uri_redirection_permanent")]
    pub fn redirection(&self) -> (Option<String>, bool) {
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
    #[doc(alias = "gst_query_set_uri")]
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
            ffi::gst_query_set_uri_redirection_permanent(
                self.0.as_mut_ptr(),
                permanent.into_glib(),
            );
        }
    }
}

declare_concrete_query!(Allocation, T);
impl Allocation<Query> {
    #[doc(alias = "gst_query_new_allocation")]
    pub fn new(caps: &crate::Caps, need_pool: bool) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Self(from_glib_full(ffi::gst_query_new_allocation(
                caps.as_mut_ptr(),
                need_pool.into_glib(),
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

    #[doc(alias = "gst_allocation_params")]
    #[doc(alias = "gst_query_get_n_allocation_params")]
    pub fn allocation_params(&self) -> Vec<(Option<crate::Allocator>, crate::AllocationParams)> {
        unsafe {
            let n = ffi::gst_query_get_n_allocation_params(self.0.as_ptr());
            let mut params = Vec::with_capacity(n as usize);
            for i in 0..n {
                let mut allocator = ptr::null_mut();
                let mut p = mem::MaybeUninit::uninit();
                ffi::gst_query_parse_nth_allocation_param(
                    self.0.as_ptr(),
                    i,
                    &mut allocator,
                    p.as_mut_ptr(),
                );
                params.push((from_glib_full(allocator), from_glib(p.assume_init())));
            }

            params
        }
    }

    #[doc(alias = "get_allocation_pools")]
    #[doc(alias = "gst_query_get_n_allocation_pools")]
    pub fn allocation_pools(&self) -> Vec<(Option<crate::BufferPool>, u32, u32, u32)> {
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

    #[doc(alias = "get_allocation_metas")]
    #[doc(alias = "gst_query_get_n_allocation_metas")]
    pub fn allocation_metas(&self) -> Vec<(glib::Type, Option<&crate::StructureRef>)> {
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

    #[doc(alias = "gst_query_find_allocation_meta")]
    pub fn find_allocation_meta<U: crate::MetaAPI>(&self) -> Option<u32> {
        unsafe {
            let mut idx = mem::MaybeUninit::uninit();
            if ffi::gst_query_find_allocation_meta(
                self.0.as_ptr(),
                U::meta_api().into_glib(),
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
    #[doc(alias = "gst_query_add_allocation_pool")]
    pub fn add_allocation_pool(
        &mut self,
        pool: Option<&impl IsA<crate::BufferPool>>,
        size: u32,
        min_buffers: u32,
        max_buffers: u32,
    ) {
        unsafe {
            ffi::gst_query_add_allocation_pool(
                self.0.as_mut_ptr(),
                pool.to_glib_none().0 as *mut ffi::GstBufferPool,
                size,
                min_buffers,
                max_buffers,
            );
        }
    }

    #[doc(alias = "gst_query_set_nth_allocation_pool")]
    pub fn set_nth_allocation_pool(
        &mut self,
        idx: u32,
        pool: Option<&impl IsA<crate::BufferPool>>,
        size: u32,
        min_buffers: u32,
        max_buffers: u32,
    ) {
        unsafe {
            let n = ffi::gst_query_get_n_allocation_pools(self.0.as_ptr());
            assert!(idx < n);
            ffi::gst_query_set_nth_allocation_pool(
                self.0.as_mut_ptr(),
                idx,
                pool.to_glib_none().0 as *mut ffi::GstBufferPool,
                size,
                min_buffers,
                max_buffers,
            );
        }
    }

    #[doc(alias = "gst_query_remove_nth_allocation_pool")]
    pub fn remove_nth_allocation_pool(&mut self, idx: u32) {
        unsafe {
            let n = ffi::gst_query_get_n_allocation_pools(self.0.as_ptr());
            assert!(idx < n);
            ffi::gst_query_remove_nth_allocation_pool(self.0.as_mut_ptr(), idx);
        }
    }

    #[doc(alias = "gst_query_add_allocation_param")]
    pub fn add_allocation_param(
        &mut self,
        allocator: Option<&impl IsA<crate::Allocator>>,
        params: crate::AllocationParams,
    ) {
        unsafe {
            ffi::gst_query_add_allocation_param(
                self.0.as_mut_ptr(),
                allocator.to_glib_none().0 as *mut ffi::GstAllocator,
                params.as_ptr(),
            );
        }
    }

    #[doc(alias = "gst_query_set_nth_allocation_param")]
    pub fn set_nth_allocation_param(
        &mut self,
        idx: u32,
        allocator: Option<&impl IsA<crate::Allocator>>,
        params: crate::AllocationParams,
    ) {
        unsafe {
            let n = ffi::gst_query_get_n_allocation_params(self.0.as_ptr());
            assert!(idx < n);
            ffi::gst_query_set_nth_allocation_param(
                self.0.as_mut_ptr(),
                idx,
                allocator.to_glib_none().0 as *mut ffi::GstAllocator,
                params.as_ptr(),
            );
        }
    }

    #[doc(alias = "gst_query_remove_nth_allocation_param")]
    pub fn remove_nth_allocation_param(&mut self, idx: u32) {
        unsafe {
            let n = ffi::gst_query_get_n_allocation_params(self.0.as_ptr());
            assert!(idx < n);
            ffi::gst_query_remove_nth_allocation_param(self.0.as_mut_ptr(), idx);
        }
    }

    #[doc(alias = "gst_query_add_allocation_meta")]
    pub fn add_allocation_meta<U: crate::MetaAPI>(
        &mut self,
        structure: Option<&crate::StructureRef>,
    ) {
        unsafe {
            ffi::gst_query_add_allocation_meta(
                self.0.as_mut_ptr(),
                U::meta_api().into_glib(),
                if let Some(structure) = structure {
                    structure.as_ptr()
                } else {
                    ptr::null()
                },
            );
        }
    }

    #[doc(alias = "gst_query_remove_nth_allocation_meta")]
    pub fn remove_nth_allocation_meta(&mut self, idx: u32) {
        unsafe {
            let n = ffi::gst_query_get_n_allocation_metas(self.0.as_ptr());
            assert!(idx < n);
            ffi::gst_query_remove_nth_allocation_meta(self.0.as_mut_ptr(), idx);
        }
    }
}

declare_concrete_query!(Scheduling, T);
impl Scheduling<Query> {
    #[doc(alias = "gst_query_new_scheduling")]
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
    #[doc(alias = "gst_query_has_scheduling_mode")]
    pub fn has_scheduling_mode(&self, mode: crate::PadMode) -> bool {
        unsafe {
            from_glib(ffi::gst_query_has_scheduling_mode(
                self.0.as_ptr(),
                mode.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_query_has_scheduling_mode_with_flags")]
    pub fn has_scheduling_mode_with_flags(
        &self,
        mode: crate::PadMode,
        flags: crate::SchedulingFlags,
    ) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gst_query_has_scheduling_mode_with_flags(
                self.0.as_ptr(),
                mode.into_glib(),
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_scheduling_modes")]
    #[doc(alias = "gst_query_get_n_scheduling_modes")]
    pub fn scheduling_modes(&self) -> Vec<crate::PadMode> {
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

    #[doc(alias = "get_result")]
    pub fn result(&self) -> (crate::SchedulingFlags, i32, i32, i32) {
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
                ffi::gst_query_add_scheduling_mode(self.0.as_mut_ptr(), mode.into_glib());
            }
        }
    }

    #[doc(alias = "gst_query_set_scheduling")]
    pub fn set(&mut self, flags: crate::SchedulingFlags, minsize: i32, maxsize: i32, align: i32) {
        unsafe {
            ffi::gst_query_set_scheduling(
                self.0.as_mut_ptr(),
                flags.into_glib(),
                minsize,
                maxsize,
                align,
            );
        }
    }
}

declare_concrete_query!(AcceptCaps, T);
impl AcceptCaps<Query> {
    #[doc(alias = "gst_query_new_accept_caps")]
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
    #[doc(alias = "get_caps")]
    #[doc(alias = "gst_query_parse_accept_caps")]
    pub fn caps(&self) -> &crate::CapsRef {
        unsafe {
            let mut caps = ptr::null_mut();
            ffi::gst_query_parse_accept_caps(self.0.as_ptr(), &mut caps);
            crate::CapsRef::from_ptr(caps)
        }
    }

    #[doc(alias = "get_caps_owned")]
    pub fn caps_owned(&self) -> crate::Caps {
        unsafe { from_glib_none(self.caps().as_ptr()) }
    }

    #[doc(alias = "get_result")]
    #[doc(alias = "gst_query_parse_accept_caps_result")]
    pub fn result(&self) -> bool {
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
            ffi::gst_query_set_accept_caps_result(self.0.as_mut_ptr(), accepted.into_glib());
        }
    }
}

declare_concrete_query!(Caps, T);
impl Caps<Query> {
    #[doc(alias = "gst_query_new_caps")]
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
    #[doc(alias = "get_filter")]
    pub fn filter(&self) -> Option<&crate::CapsRef> {
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

    #[doc(alias = "get_filter_owned")]
    pub fn filter_owned(&self) -> Option<crate::Caps> {
        unsafe { self.filter().map(|caps| from_glib_none(caps.as_ptr())) }
    }

    #[doc(alias = "get_result")]
    #[doc(alias = "gst_query_parse_caps_result")]
    pub fn result(&self) -> Option<&crate::CapsRef> {
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

    #[doc(alias = "get_result_owned")]
    pub fn result_owned(&self) -> Option<crate::Caps> {
        unsafe { self.result().map(|caps| from_glib_none(caps.as_ptr())) }
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
    #[doc(alias = "gst_query_new_drain")]
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
    #[doc(alias = "gst_query_new_context")]
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
    #[doc(alias = "get_context")]
    #[doc(alias = "gst_query_parse_context")]
    pub fn context(&self) -> Option<&crate::ContextRef> {
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

    #[doc(alias = "get_context_owned")]
    pub fn context_owned(&self) -> Option<crate::Context> {
        unsafe {
            self.context()
                .map(|context| from_glib_none(context.as_ptr()))
        }
    }

    #[doc(alias = "get_context_type")]
    #[doc(alias = "gst_query_parse_context_type")]
    pub fn context_type(&self) -> &str {
        unsafe {
            let mut context_type = ptr::null();
            ffi::gst_query_parse_context_type(self.0.as_ptr(), &mut context_type);
            CStr::from_ptr(context_type).to_str().unwrap()
        }
    }
}

impl<T: AsMutPtr> Context<T> {
    #[doc(alias = "gst_query_set_context")]
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
    #[doc(alias = "gst_query_new_bitrate")]
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
    #[doc(alias = "get_bitrate")]
    #[doc(alias = "gst_query_parse_bitrate")]
    pub fn bitrate(&self) -> u32 {
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
    #[doc(alias = "gst_query_set_bitrate")]
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
    use crate::ClockTime;
    use std::convert::TryInto;

    #[test]
    fn test_writability() {
        crate::init().unwrap();

        fn check_mut(query: &mut QueryRef) {
            skip_assert_initialized!();
            match query.view_mut() {
                QueryView::Position(ref mut p) => {
                    let pos = p.result();
                    assert_eq!(pos.try_into(), Ok(ClockTime::NONE));
                    p.set(Some(3 * ClockTime::SECOND));
                    let pos = p.result();
                    assert_eq!(pos.try_into(), Ok(Some(3 * ClockTime::SECOND)));
                }
                _ => panic!("Wrong concrete Query in Query"),
            }
        }

        fn check_ref(query: &QueryRef) {
            skip_assert_initialized!();
            match query.view() {
                QueryView::Position(ref p) => {
                    let pos = p.result();
                    assert_eq!(pos.try_into(), Ok(Some(3 * ClockTime::SECOND)));
                    unsafe {
                        assert!(!p.as_mut_ptr().is_null());
                    }
                }
                _ => panic!("Wrong concrete Query in Query"),
            }
        }

        let mut p = Position::new(crate::Format::Time);
        let pos = p.result();
        assert_eq!(pos.try_into(), Ok(ClockTime::NONE));

        p.structure_mut().set("check_mut", &true);

        // deref
        assert!(!p.is_serialized());

        {
            check_mut(&mut p);

            let structure = p.structure();
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
            d.set(Some(2 * ClockTime::SECOND));
        }

        if let QueryView::Duration(d) = &query.view() {
            let duration = d.result();
            assert_eq!(duration.try_into(), Ok(Some(2 * ClockTime::SECOND)));
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
