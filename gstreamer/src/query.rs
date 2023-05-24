// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    borrow::{Borrow, BorrowMut},
    ffi::CStr,
    fmt, mem,
    ops::{Deref, DerefMut},
    ptr,
};

use glib::{object::IsA, translate::*};

use crate::{
    format::{CompatibleFormattedValue, FormattedValue, GenericFormattedValue},
    structure::*,
};

mini_object_wrapper!(Query, QueryRef, ffi::GstQuery, || {
    ffi::gst_query_get_type()
});

impl QueryRef {
    #[doc(alias = "get_structure")]
    #[doc(alias = "gst_query_get_structure")]
    #[inline]
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
    #[doc(alias = "gst_query_writable_structure")]
    #[inline]
    pub fn structure_mut(&mut self) -> &mut StructureRef {
        unsafe {
            let structure = ffi::gst_query_writable_structure(self.as_mut_ptr());
            StructureRef::from_glib_borrow_mut(structure)
        }
    }

    #[doc(alias = "GST_QUERY_IS_DOWNSTREAM")]
    #[inline]
    pub fn is_downstream(&self) -> bool {
        unsafe { ((*self.as_ptr()).type_ as u32) & ffi::GST_QUERY_TYPE_DOWNSTREAM != 0 }
    }

    #[doc(alias = "GST_QUERY_IS_UPSTREAM")]
    #[inline]
    pub fn is_upstream(&self) -> bool {
        unsafe { ((*self.as_ptr()).type_ as u32) & ffi::GST_QUERY_TYPE_UPSTREAM != 0 }
    }

    #[doc(alias = "GST_QUERY_IS_SERIALIZED")]
    #[inline]
    pub fn is_serialized(&self) -> bool {
        unsafe { ((*self.as_ptr()).type_ as u32) & ffi::GST_QUERY_TYPE_SERIALIZED != 0 }
    }

    pub fn view(&self) -> QueryView {
        unsafe {
            let type_ = (*self.as_ptr()).type_;

            match type_ {
                ffi::GST_QUERY_POSITION => Position::view(self),
                ffi::GST_QUERY_DURATION => Duration::view(self),
                ffi::GST_QUERY_LATENCY => Latency::view(self),
                ffi::GST_QUERY_SEEKING => Seeking::view(self),
                ffi::GST_QUERY_SEGMENT => Segment::view(self),
                ffi::GST_QUERY_CONVERT => Convert::view(self),
                ffi::GST_QUERY_FORMATS => Formats::view(self),
                ffi::GST_QUERY_BUFFERING => Buffering::view(self),
                ffi::GST_QUERY_CUSTOM => Custom::view(self),
                ffi::GST_QUERY_URI => Uri::view(self),
                ffi::GST_QUERY_ALLOCATION => Allocation::view(self),
                ffi::GST_QUERY_SCHEDULING => Scheduling::view(self),
                ffi::GST_QUERY_ACCEPT_CAPS => AcceptCaps::view(self),
                ffi::GST_QUERY_CAPS => Caps::view(self),
                ffi::GST_QUERY_DRAIN => Drain::view(self),
                ffi::GST_QUERY_CONTEXT => Context::view(self),
                #[cfg(feature = "v1_16")]
                ffi::GST_QUERY_BITRATE => Bitrate::view(self),
                #[cfg(feature = "v1_22")]
                ffi::GST_QUERY_SELECTABLE => Selectable::view(self),
                _ => Other::view(self),
            }
        }
    }

    pub fn view_mut(&mut self) -> QueryViewMut {
        unsafe {
            let type_ = (*self.as_ptr()).type_;

            match type_ {
                ffi::GST_QUERY_POSITION => Position::view_mut(self),
                ffi::GST_QUERY_DURATION => Duration::view_mut(self),
                ffi::GST_QUERY_LATENCY => Latency::view_mut(self),
                ffi::GST_QUERY_SEEKING => Seeking::view_mut(self),
                ffi::GST_QUERY_SEGMENT => Segment::view_mut(self),
                ffi::GST_QUERY_CONVERT => Convert::view_mut(self),
                ffi::GST_QUERY_FORMATS => Formats::view_mut(self),
                ffi::GST_QUERY_BUFFERING => Buffering::view_mut(self),
                ffi::GST_QUERY_CUSTOM => Custom::view_mut(self),
                ffi::GST_QUERY_URI => Uri::view_mut(self),
                ffi::GST_QUERY_ALLOCATION => Allocation::view_mut(self),
                ffi::GST_QUERY_SCHEDULING => Scheduling::view_mut(self),
                ffi::GST_QUERY_ACCEPT_CAPS => AcceptCaps::view_mut(self),
                ffi::GST_QUERY_CAPS => Caps::view_mut(self),
                ffi::GST_QUERY_DRAIN => Drain::view_mut(self),
                ffi::GST_QUERY_CONTEXT => Context::view_mut(self),
                #[cfg(feature = "v1_16")]
                ffi::GST_QUERY_BITRATE => Bitrate::view_mut(self),
                #[cfg(feature = "v1_22")]
                ffi::GST_QUERY_SELECTABLE => Selectable::view_mut(self),
                _ => Other::view_mut(self),
            }
        }
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
            .field("ptr", &self.as_ptr())
            .field("type", &unsafe {
                let type_ = ffi::gst_query_type_get_name((*self.as_ptr()).type_);
                CStr::from_ptr(type_).to_str().unwrap()
            })
            .field("structure", &self.structure())
            .finish()
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum QueryView<'a> {
    Position(&'a Position),
    Duration(&'a Duration),
    Latency(&'a Latency),
    Seeking(&'a Seeking),
    Segment(&'a Segment),
    Convert(&'a Convert),
    Formats(&'a Formats),
    Buffering(&'a Buffering),
    Custom(&'a Custom),
    Uri(&'a Uri),
    Allocation(&'a Allocation),
    Scheduling(&'a Scheduling),
    AcceptCaps(&'a AcceptCaps),
    Caps(&'a Caps),
    Drain(&'a Drain),
    Context(&'a Context),
    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    Bitrate(&'a Bitrate),
    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    Selectable(&'a Selectable),
    Other(&'a Other),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum QueryViewMut<'a> {
    Position(&'a mut Position),
    Duration(&'a mut Duration),
    Latency(&'a mut Latency),
    Seeking(&'a mut Seeking),
    Segment(&'a mut Segment),
    Convert(&'a mut Convert),
    Formats(&'a mut Formats),
    Buffering(&'a mut Buffering),
    Custom(&'a mut Custom),
    Uri(&'a mut Uri),
    Allocation(&'a mut Allocation),
    Scheduling(&'a mut Scheduling),
    AcceptCaps(&'a mut AcceptCaps),
    Caps(&'a mut Caps),
    Drain(&'a mut Drain),
    Context(&'a mut Context),
    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    Bitrate(&'a mut Bitrate),
    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    Selectable(&'a mut Selectable),
    Other(&'a mut Other),
}

macro_rules! declare_concrete_query(
    ($name:ident, $param:ident) => {
        #[repr(transparent)]
        pub struct $name<$param = QueryRef>($param);

        impl $name {
            #[inline]
            pub fn query(&self) -> &QueryRef {
                unsafe { &*(self as *const Self as *const QueryRef) }
            }

            #[inline]
            pub fn query_mut(&mut self) -> &mut QueryRef {
                unsafe { &mut *(self as *mut Self as *mut QueryRef) }
            }

            #[inline]
            unsafe fn view(query: &QueryRef) -> QueryView<'_> {
                let query = &*(query as *const QueryRef as *const Self);
                QueryView::$name(query)
            }

            #[inline]
            unsafe fn view_mut(query: &mut QueryRef) -> QueryViewMut<'_> {
                let query = &mut *(query as *mut QueryRef as *mut Self);
                QueryViewMut::$name(query)
            }
        }

        impl Deref for $name {
            type Target = QueryRef;

            #[inline]
            fn deref(&self) -> &Self::Target {
                self.query()
            }
        }

        impl DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.query_mut()
            }
        }

        impl ToOwned for $name {
            type Owned = $name<Query>;

            #[inline]
            fn to_owned(&self) -> Self::Owned {
                $name::<Query>(self.copy())
            }
        }

        impl $name<Query> {
            #[inline]
            pub fn get_mut(&mut self) -> Option<&mut $name> {
                self.0
                    .get_mut()
                    .map(|query| unsafe { &mut *(query as *mut QueryRef as *mut $name) })
            }
        }

        impl Deref for $name<Query> {
            type Target = $name;

            #[inline]
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self.0.as_ptr() as *const Self::Target) }
            }
        }

        impl DerefMut for $name<Query> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                debug_assert!(self.0.is_writable());
                unsafe { &mut *(self.0.as_mut_ptr() as *mut Self::Target) }
            }
        }

        impl Borrow<$name> for $name<Query> {
            #[inline]
            fn borrow(&self) -> &$name {
                &*self
            }
        }

        impl BorrowMut<$name> for $name<Query> {
            #[inline]
            fn borrow_mut(&mut self) -> &mut $name {
                &mut *self
            }
        }

        impl From<$name<Query>> for Query {
            #[inline]
            fn from(concrete: $name<Query>) -> Self {
                skip_assert_initialized!();
                concrete.0
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

impl Position {
    #[doc(alias = "get_result")]
    #[doc(alias = "gst_query_parse_position")]
    pub fn result(&self) -> GenericFormattedValue {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();
            let mut pos = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_position(self.as_mut_ptr(), fmt.as_mut_ptr(), pos.as_mut_ptr());

            GenericFormattedValue::new(from_glib(fmt.assume_init()), pos.assume_init())
        }
    }

    #[doc(alias = "get_format")]
    #[doc(alias = "gst_query_parse_position")]
    pub fn format(&self) -> crate::Format {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_position(self.as_mut_ptr(), fmt.as_mut_ptr(), ptr::null_mut());

            from_glib(fmt.assume_init())
        }
    }

    #[doc(alias = "gst_query_set_position")]
    pub fn set(&mut self, pos: impl FormattedValue) {
        assert_eq!(pos.format(), self.format());
        unsafe {
            ffi::gst_query_set_position(
                self.as_mut_ptr(),
                pos.format().into_glib(),
                pos.into_raw_value(),
            );
        }
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Position")
            .field("structure", &self.query().structure())
            .field("result", &self.result())
            .field("format", &self.format())
            .finish()
    }
}

impl std::fmt::Debug for Position<Query> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Position::<QueryRef>::fmt(self, f)
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

impl Duration {
    #[doc(alias = "get_result")]
    #[doc(alias = "gst_query_parse_duration")]
    pub fn result(&self) -> GenericFormattedValue {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();
            let mut pos = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_duration(self.as_mut_ptr(), fmt.as_mut_ptr(), pos.as_mut_ptr());

            GenericFormattedValue::new(from_glib(fmt.assume_init()), pos.assume_init())
        }
    }

    #[doc(alias = "get_format")]
    #[doc(alias = "gst_query_parse_duration")]
    pub fn format(&self) -> crate::Format {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_duration(self.as_mut_ptr(), fmt.as_mut_ptr(), ptr::null_mut());

            from_glib(fmt.assume_init())
        }
    }

    #[doc(alias = "gst_query_set_duration")]
    pub fn set(&mut self, dur: impl FormattedValue) {
        assert_eq!(dur.format(), self.format());
        unsafe {
            ffi::gst_query_set_duration(
                self.as_mut_ptr(),
                dur.format().into_glib(),
                dur.into_raw_value(),
            );
        }
    }
}

impl std::fmt::Debug for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Duration")
            .field("structure", &self.query().structure())
            .field("result", &self.result())
            .field("format", &self.format())
            .finish()
    }
}

impl std::fmt::Debug for Duration<Query> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Duration::<QueryRef>::fmt(self, f)
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

impl Latency {
    #[doc(alias = "get_result")]
    #[doc(alias = "gst_query_parse_latency")]
    pub fn result(&self) -> (bool, crate::ClockTime, Option<crate::ClockTime>) {
        unsafe {
            let mut live = mem::MaybeUninit::uninit();
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_latency(
                self.as_mut_ptr(),
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

    #[doc(alias = "gst_query_set_latency")]
    pub fn set(
        &mut self,
        live: bool,
        min: crate::ClockTime,
        max: impl Into<Option<crate::ClockTime>>,
    ) {
        unsafe {
            ffi::gst_query_set_latency(
                self.as_mut_ptr(),
                live.into_glib(),
                min.into_glib(),
                max.into().into_glib(),
            );
        }
    }
}

impl std::fmt::Debug for Latency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Latency")
            .field("structure", &self.query().structure())
            .field("result", &self.result())
            .finish()
    }
}

impl std::fmt::Debug for Latency<Query> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Latency::<QueryRef>::fmt(self, f)
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

impl Seeking {
    #[doc(alias = "get_result")]
    #[doc(alias = "gst_query_parse_seeking")]
    pub fn result(&self) -> (bool, GenericFormattedValue, GenericFormattedValue) {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();
            let mut seekable = mem::MaybeUninit::uninit();
            let mut start = mem::MaybeUninit::uninit();
            let mut end = mem::MaybeUninit::uninit();
            ffi::gst_query_parse_seeking(
                self.as_mut_ptr(),
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
    #[doc(alias = "gst_query_parse_seeking")]
    pub fn format(&self) -> crate::Format {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();
            ffi::gst_query_parse_seeking(
                self.as_mut_ptr(),
                fmt.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );

            from_glib(fmt.assume_init())
        }
    }

    #[doc(alias = "gst_query_set_seeking")]
    pub fn set<V: FormattedValue>(
        &mut self,
        seekable: bool,
        start: V,
        end: impl CompatibleFormattedValue<V>,
    ) {
        assert_eq!(self.format(), start.format());
        let end = end.try_into_checked(start).unwrap();

        unsafe {
            ffi::gst_query_set_seeking(
                self.as_mut_ptr(),
                start.format().into_glib(),
                seekable.into_glib(),
                start.into_raw_value(),
                end.into_raw_value(),
            );
        }
    }
}

impl std::fmt::Debug for Seeking {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Seeking")
            .field("structure", &self.query().structure())
            .field("result", &self.result())
            .field("format", &self.format())
            .finish()
    }
}

impl std::fmt::Debug for Seeking<Query> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Seeking::<QueryRef>::fmt(self, f)
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

impl Segment {
    #[doc(alias = "get_result")]
    #[doc(alias = "gst_query_parse_segment")]
    pub fn result(&self) -> (f64, GenericFormattedValue, GenericFormattedValue) {
        unsafe {
            let mut rate = mem::MaybeUninit::uninit();
            let mut fmt = mem::MaybeUninit::uninit();
            let mut start = mem::MaybeUninit::uninit();
            let mut stop = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_segment(
                self.as_mut_ptr(),
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
    #[doc(alias = "gst_query_parse_segment")]
    pub fn format(&self) -> crate::Format {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_segment(
                self.as_mut_ptr(),
                ptr::null_mut(),
                fmt.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
            );
            from_glib(fmt.assume_init())
        }
    }

    #[doc(alias = "gst_query_set_segment")]
    pub fn set<V: FormattedValue>(
        &mut self,
        rate: f64,
        start: V,
        stop: impl CompatibleFormattedValue<V>,
    ) {
        let stop = stop.try_into_checked(start).unwrap();

        unsafe {
            ffi::gst_query_set_segment(
                self.as_mut_ptr(),
                rate,
                start.format().into_glib(),
                start.into_raw_value(),
                stop.into_raw_value(),
            );
        }
    }
}

impl std::fmt::Debug for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Segment")
            .field("structure", &self.query().structure())
            .field("result", &self.result())
            .field("format", &self.format())
            .finish()
    }
}

impl std::fmt::Debug for Segment<Query> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Segment::<QueryRef>::fmt(self, f)
    }
}

declare_concrete_query!(Convert, T);
impl Convert<Query> {
    #[doc(alias = "gst_query_new_convert")]
    pub fn new(value: impl FormattedValue, dest_fmt: crate::Format) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Self(from_glib_full(ffi::gst_query_new_convert(
                value.format().into_glib(),
                value.into_raw_value(),
                dest_fmt.into_glib(),
            )))
        }
    }
}

impl Convert {
    #[doc(alias = "get_result")]
    #[doc(alias = "gst_query_parse_convert")]
    pub fn result(&self) -> (GenericFormattedValue, GenericFormattedValue) {
        unsafe {
            let mut src_fmt = mem::MaybeUninit::uninit();
            let mut src = mem::MaybeUninit::uninit();
            let mut dest_fmt = mem::MaybeUninit::uninit();
            let mut dest = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_convert(
                self.as_mut_ptr(),
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

    #[doc(alias = "gst_query_parse_convert")]
    pub fn get(&self) -> (GenericFormattedValue, crate::Format) {
        unsafe {
            let mut src_fmt = mem::MaybeUninit::uninit();
            let mut src = mem::MaybeUninit::uninit();
            let mut dest_fmt = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_convert(
                self.as_mut_ptr(),
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

    #[doc(alias = "gst_query_set_convert")]
    pub fn set(&mut self, src: impl FormattedValue, dest: impl FormattedValue) {
        unsafe {
            ffi::gst_query_set_convert(
                self.as_mut_ptr(),
                src.format().into_glib(),
                src.into_raw_value(),
                dest.format().into_glib(),
                dest.into_raw_value(),
            );
        }
    }
}

impl std::fmt::Debug for Convert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (source, dest) = self.result();

        f.debug_struct("Convert")
            .field("structure", &self.query().structure())
            .field("source", &source)
            .field("dest", &dest)
            .finish()
    }
}

impl std::fmt::Debug for Convert<Query> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Convert::<QueryRef>::fmt(self, f)
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

impl Formats {
    #[doc(alias = "get_result")]
    #[doc(alias = "gst_query_parse_n_formats")]
    #[doc(alias = "gst_query_parse_nth_format")]
    pub fn result(&self) -> Vec<crate::Format> {
        unsafe {
            let mut n = mem::MaybeUninit::uninit();
            ffi::gst_query_parse_n_formats(self.as_mut_ptr(), n.as_mut_ptr());
            let n = n.assume_init();
            let mut res = Vec::with_capacity(n as usize);

            for i in 0..n {
                let mut fmt = mem::MaybeUninit::uninit();
                ffi::gst_query_parse_nth_format(self.as_mut_ptr(), i, fmt.as_mut_ptr());
                res.push(from_glib(fmt.assume_init()));
            }

            res
        }
    }

    #[doc(alias = "gst_query_set_formats")]
    #[doc(alias = "gst_query_set_formatsv")]
    pub fn set(&mut self, formats: &[crate::Format]) {
        unsafe {
            let v: Vec<_> = formats.iter().map(|f| f.into_glib()).collect();
            ffi::gst_query_set_formatsv(self.as_mut_ptr(), v.len() as i32, v.as_ptr() as *mut _);
        }
    }
}

impl std::fmt::Debug for Formats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Formats")
            .field("structure", &self.query().structure())
            .field("result", &self.result())
            .finish()
    }
}

impl std::fmt::Debug for Formats<Query> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Formats::<QueryRef>::fmt(self, f)
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

impl Buffering {
    #[doc(alias = "get_format")]
    #[doc(alias = "gst_query_parse_buffering_range")]
    pub fn format(&self) -> crate::Format {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_buffering_range(
                self.as_mut_ptr(),
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
                self.as_mut_ptr(),
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
                self.as_mut_ptr(),
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
                self.as_mut_ptr(),
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
    #[doc(alias = "gst_query_parse_nth_buffering_range")]
    pub fn ranges(&self) -> Vec<(GenericFormattedValue, GenericFormattedValue)> {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();
            ffi::gst_query_parse_buffering_range(
                self.as_mut_ptr(),
                fmt.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );
            let fmt = from_glib(fmt.assume_init());

            let n = ffi::gst_query_get_n_buffering_ranges(self.as_mut_ptr());
            let mut res = Vec::with_capacity(n as usize);
            for i in 0..n {
                let mut start = mem::MaybeUninit::uninit();
                let mut stop = mem::MaybeUninit::uninit();
                let s: bool = from_glib(ffi::gst_query_parse_nth_buffering_range(
                    self.as_mut_ptr(),
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

    #[doc(alias = "gst_query_set_buffering_percent")]
    pub fn set_percent(&mut self, busy: bool, percent: i32) {
        unsafe {
            ffi::gst_query_set_buffering_percent(self.as_mut_ptr(), busy.into_glib(), percent);
        }
    }

    #[doc(alias = "gst_query_set_buffering_range")]
    pub fn set_range<V: FormattedValue>(
        &mut self,
        start: V,
        stop: impl CompatibleFormattedValue<V>,
        estimated_total: i64,
    ) {
        assert_eq!(self.format(), start.format());
        let stop = stop.try_into_checked(start).unwrap();

        unsafe {
            ffi::gst_query_set_buffering_range(
                self.as_mut_ptr(),
                start.format().into_glib(),
                start.into_raw_value(),
                stop.into_raw_value(),
                estimated_total,
            );
        }
    }

    #[doc(alias = "gst_query_set_buffering_stats")]
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
                self.as_mut_ptr(),
                mode.into_glib(),
                avg_in,
                avg_out,
                buffering_left,
            );
        }
    }

    #[doc(alias = "gst_query_add_buffering_range")]
    pub fn add_buffering_ranges<V: FormattedValue, U: CompatibleFormattedValue<V> + Copy>(
        &mut self,
        ranges: &[(V, U)],
    ) {
        unsafe {
            let fmt = self.format();

            for &(start, stop) in ranges {
                assert_eq!(start.format(), fmt);
                let stop = stop.try_into_checked(start).unwrap();
                ffi::gst_query_add_buffering_range(
                    self.as_mut_ptr(),
                    start.into_raw_value(),
                    stop.into_raw_value(),
                );
            }
        }
    }
}

impl std::fmt::Debug for Buffering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Buffering")
            .field("structure", &self.query().structure())
            .field("format", &self.format())
            .field("percent", &self.percent())
            .field("range", &self.range())
            .finish()
    }
}

impl std::fmt::Debug for Buffering<Query> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Buffering::<QueryRef>::fmt(self, f)
    }
}

declare_concrete_query!(Custom, T);
impl Custom<Query> {
    #[doc(alias = "gst_query_new_custom")]
    pub fn new(structure: crate::Structure) -> Self {
        skip_assert_initialized!();
        unsafe {
            Self(from_glib_full(ffi::gst_query_new_custom(
                ffi::GST_QUERY_CUSTOM,
                structure.into_glib_ptr(),
            )))
        }
    }
}

impl std::fmt::Debug for Custom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Custom")
            .field("structure", &self.query().structure())
            .finish()
    }
}

impl std::fmt::Debug for Custom<Query> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Custom::<QueryRef>::fmt(self, f)
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

impl Uri {
    #[doc(alias = "get_uri")]
    #[doc(alias = "gst_query_parse_uri")]
    pub fn uri(&self) -> Option<glib::GString> {
        unsafe {
            let mut uri = ptr::null_mut();
            ffi::gst_query_parse_uri(self.as_mut_ptr(), &mut uri);
            from_glib_full(uri)
        }
    }

    #[doc(alias = "get_redirection")]
    #[doc(alias = "gst_query_parse_uri_redirection")]
    #[doc(alias = "gst_query_parse_uri_redirection_permanent")]
    pub fn redirection(&self) -> (Option<glib::GString>, bool) {
        unsafe {
            let mut uri = ptr::null_mut();
            ffi::gst_query_parse_uri_redirection(self.as_mut_ptr(), &mut uri);
            let mut permanent = mem::MaybeUninit::uninit();
            ffi::gst_query_parse_uri_redirection_permanent(
                self.as_mut_ptr(),
                permanent.as_mut_ptr(),
            );

            (from_glib_full(uri), from_glib(permanent.assume_init()))
        }
    }

    #[doc(alias = "gst_query_set_uri")]
    pub fn set_uri<'a, T>(&mut self, uri: impl Into<Option<&'a T>>)
    where
        T: 'a + AsRef<str> + ?Sized,
    {
        unsafe {
            ffi::gst_query_set_uri(
                self.as_mut_ptr(),
                uri.into().map(AsRef::as_ref).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_query_set_uri_redirection")]
    #[doc(alias = "gst_query_set_uri_redirection_permanent")]
    pub fn set_redirection<'a, T>(&mut self, uri: impl Into<Option<&'a T>>, permanent: bool)
    where
        T: 'a + AsRef<str> + ?Sized,
    {
        unsafe {
            ffi::gst_query_set_uri_redirection(
                self.as_mut_ptr(),
                uri.into().map(AsRef::as_ref).to_glib_none().0,
            );
            ffi::gst_query_set_uri_redirection_permanent(
                self.0.as_mut_ptr(),
                permanent.into_glib(),
            );
        }
    }
}

impl std::fmt::Debug for Uri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (redirection, permanent) = self.redirection();
        f.debug_struct("Uri")
            .field("structure", &self.query().structure())
            .field("uri", &self.uri())
            .field("redirection", &redirection)
            .field("redirection-permanent", &permanent)
            .finish()
    }
}

impl std::fmt::Debug for Uri<Query> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Uri::<QueryRef>::fmt(self, f)
    }
}

declare_concrete_query!(Allocation, T);
impl Allocation<Query> {
    #[doc(alias = "gst_query_new_allocation")]
    pub fn new(caps: Option<&crate::Caps>, need_pool: bool) -> Self {
        skip_assert_initialized!();
        unsafe {
            Self(from_glib_full(ffi::gst_query_new_allocation(
                caps.map(|caps| caps.as_mut_ptr())
                    .unwrap_or(ptr::null_mut()),
                need_pool.into_glib(),
            )))
        }
    }
}

impl Allocation {
    #[doc(alias = "gst_query_parse_allocation")]
    pub fn get(&self) -> (Option<&crate::CapsRef>, bool) {
        unsafe {
            let mut caps = ptr::null_mut();
            let mut need_pool = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_allocation(self.as_mut_ptr(), &mut caps, need_pool.as_mut_ptr());
            (
                if caps.is_null() {
                    None
                } else {
                    Some(crate::CapsRef::from_ptr(caps))
                },
                from_glib(need_pool.assume_init()),
            )
        }
    }

    #[doc(alias = "gst_query_parse_allocation")]
    pub fn get_owned(&self) -> (Option<crate::Caps>, bool) {
        unsafe {
            let (caps, need_pool) = self.get();
            (caps.map(|caps| from_glib_none(caps.as_ptr())), need_pool)
        }
    }

    #[doc(alias = "gst_allocation_params")]
    #[doc(alias = "gst_query_get_n_allocation_params")]
    #[doc(alias = "gst_query_parse_nth_allocation_param")]
    pub fn allocation_params(&self) -> Vec<(Option<crate::Allocator>, crate::AllocationParams)> {
        unsafe {
            let n = ffi::gst_query_get_n_allocation_params(self.as_mut_ptr());
            let mut params = Vec::with_capacity(n as usize);
            for i in 0..n {
                let mut allocator = ptr::null_mut();
                let mut p = mem::MaybeUninit::uninit();
                ffi::gst_query_parse_nth_allocation_param(
                    self.as_mut_ptr(),
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
    #[doc(alias = "gst_query_parse_nth_allocation_pool")]
    pub fn allocation_pools(&self) -> Vec<(Option<crate::BufferPool>, u32, u32, u32)> {
        unsafe {
            let n = ffi::gst_query_get_n_allocation_pools(self.as_mut_ptr());
            let mut pools = Vec::with_capacity(n as usize);
            for i in 0..n {
                let mut pool = ptr::null_mut();
                let mut size = mem::MaybeUninit::uninit();
                let mut min_buffers = mem::MaybeUninit::uninit();
                let mut max_buffers = mem::MaybeUninit::uninit();

                ffi::gst_query_parse_nth_allocation_pool(
                    self.0.as_mut_ptr(),
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
    #[doc(alias = "gst_query_parse_nth_allocation_meta")]
    pub fn allocation_metas(&self) -> Vec<(glib::Type, Option<&crate::StructureRef>)> {
        unsafe {
            let n = ffi::gst_query_get_n_allocation_metas(self.0.as_mut_ptr());
            let mut metas = Vec::with_capacity(n as usize);
            for i in 0..n {
                let mut structure = ptr::null();

                let api =
                    ffi::gst_query_parse_nth_allocation_meta(self.as_mut_ptr(), i, &mut structure);
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
                self.as_mut_ptr(),
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
                self.as_mut_ptr(),
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
            let n = ffi::gst_query_get_n_allocation_pools(self.as_mut_ptr());
            assert!(idx < n);
            ffi::gst_query_set_nth_allocation_pool(
                self.as_mut_ptr(),
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
            let n = ffi::gst_query_get_n_allocation_pools(self.as_mut_ptr());
            assert!(idx < n);
            ffi::gst_query_remove_nth_allocation_pool(self.as_mut_ptr(), idx);
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
                self.as_mut_ptr(),
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
            let n = ffi::gst_query_get_n_allocation_params(self.as_mut_ptr());
            assert!(idx < n);
            ffi::gst_query_set_nth_allocation_param(
                self.as_mut_ptr(),
                idx,
                allocator.to_glib_none().0 as *mut ffi::GstAllocator,
                params.as_ptr(),
            );
        }
    }

    #[doc(alias = "gst_query_remove_nth_allocation_param")]
    pub fn remove_nth_allocation_param(&mut self, idx: u32) {
        unsafe {
            let n = ffi::gst_query_get_n_allocation_params(self.as_mut_ptr());
            assert!(idx < n);
            ffi::gst_query_remove_nth_allocation_param(self.as_mut_ptr(), idx);
        }
    }

    #[doc(alias = "gst_query_add_allocation_meta")]
    pub fn add_allocation_meta<U: crate::MetaAPI>(
        &mut self,
        structure: Option<&crate::StructureRef>,
    ) {
        unsafe {
            ffi::gst_query_add_allocation_meta(
                self.as_mut_ptr(),
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
            let n = ffi::gst_query_get_n_allocation_metas(self.as_mut_ptr());
            assert!(idx < n);
            ffi::gst_query_remove_nth_allocation_meta(self.as_mut_ptr(), idx);
        }
    }
}

impl std::fmt::Debug for Allocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (caps, need_pool) = self.get();
        f.debug_struct("Allocation")
            .field("structure", &self.query().structure())
            .field("caps", &caps)
            .field("need-pool", &need_pool)
            .field("allocation-params", &self.allocation_params())
            .field("allocation-pools", &self.allocation_pools())
            .field("allocation-metas", &self.allocation_metas())
            .finish()
    }
}

impl std::fmt::Debug for Allocation<Query> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Allocation::<QueryRef>::fmt(self, f)
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

impl Scheduling {
    #[doc(alias = "gst_query_has_scheduling_mode")]
    pub fn has_scheduling_mode(&self, mode: crate::PadMode) -> bool {
        unsafe {
            from_glib(ffi::gst_query_has_scheduling_mode(
                self.as_mut_ptr(),
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
                self.as_mut_ptr(),
                mode.into_glib(),
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_scheduling_modes")]
    #[doc(alias = "gst_query_get_n_scheduling_modes")]
    pub fn scheduling_modes(&self) -> Vec<crate::PadMode> {
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

    #[doc(alias = "get_result")]
    #[doc(alias = "gst_query_parse_scheduling")]
    pub fn result(&self) -> (crate::SchedulingFlags, i32, i32, i32) {
        unsafe {
            let mut flags = mem::MaybeUninit::uninit();
            let mut minsize = mem::MaybeUninit::uninit();
            let mut maxsize = mem::MaybeUninit::uninit();
            let mut align = mem::MaybeUninit::uninit();

            ffi::gst_query_parse_scheduling(
                self.as_mut_ptr(),
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

    #[doc(alias = "gst_query_add_scheduling_mode")]
    pub fn add_scheduling_modes(&mut self, modes: &[crate::PadMode]) {
        unsafe {
            for mode in modes {
                ffi::gst_query_add_scheduling_mode(self.as_mut_ptr(), mode.into_glib());
            }
        }
    }

    #[doc(alias = "gst_query_set_scheduling")]
    pub fn set(&mut self, flags: crate::SchedulingFlags, minsize: i32, maxsize: i32, align: i32) {
        unsafe {
            ffi::gst_query_set_scheduling(
                self.as_mut_ptr(),
                flags.into_glib(),
                minsize,
                maxsize,
                align,
            );
        }
    }
}

impl std::fmt::Debug for Scheduling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Scheduling")
            .field("structure", &self.query().structure())
            .field("result", &self.result())
            .field("scheduling-modes", &self.scheduling_modes())
            .finish()
    }
}

impl std::fmt::Debug for Scheduling<Query> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Scheduling::<QueryRef>::fmt(self, f)
    }
}

declare_concrete_query!(AcceptCaps, T);
impl AcceptCaps<Query> {
    #[doc(alias = "gst_query_new_accept_caps")]
    pub fn new(caps: &crate::Caps) -> Self {
        skip_assert_initialized!();
        unsafe {
            Self(from_glib_full(ffi::gst_query_new_accept_caps(
                caps.as_mut_ptr(),
            )))
        }
    }
}

impl AcceptCaps {
    #[doc(alias = "get_caps")]
    #[doc(alias = "gst_query_parse_accept_caps")]
    pub fn caps(&self) -> &crate::CapsRef {
        unsafe {
            let mut caps = ptr::null_mut();
            ffi::gst_query_parse_accept_caps(self.as_mut_ptr(), &mut caps);
            crate::CapsRef::from_ptr(caps)
        }
    }

    #[doc(alias = "get_caps_owned")]
    #[doc(alias = "gst_query_parse_accept_caps")]
    pub fn caps_owned(&self) -> crate::Caps {
        unsafe { from_glib_none(self.caps().as_ptr()) }
    }

    #[doc(alias = "get_result")]
    #[doc(alias = "gst_query_parse_accept_caps_result")]
    pub fn result(&self) -> bool {
        unsafe {
            let mut accepted = mem::MaybeUninit::uninit();
            ffi::gst_query_parse_accept_caps_result(self.as_mut_ptr(), accepted.as_mut_ptr());
            from_glib(accepted.assume_init())
        }
    }

    #[doc(alias = "gst_query_set_accept_caps_result")]
    pub fn set_result(&mut self, accepted: bool) {
        unsafe {
            ffi::gst_query_set_accept_caps_result(self.as_mut_ptr(), accepted.into_glib());
        }
    }
}

impl std::fmt::Debug for AcceptCaps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AcceptCaps")
            .field("structure", &self.query().structure())
            .field("result", &self.result())
            .field("caps", &self.caps())
            .finish()
    }
}

impl std::fmt::Debug for AcceptCaps<Query> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        AcceptCaps::<QueryRef>::fmt(self, f)
    }
}

declare_concrete_query!(Caps, T);
impl Caps<Query> {
    #[doc(alias = "gst_query_new_caps")]
    pub fn new(filter: Option<&crate::Caps>) -> Self {
        skip_assert_initialized!();
        unsafe {
            Self(from_glib_full(ffi::gst_query_new_caps(
                filter.to_glib_none().0,
            )))
        }
    }
}

impl Caps {
    #[doc(alias = "get_filter")]
    #[doc(alias = "gst_query_parse_caps")]
    pub fn filter(&self) -> Option<&crate::CapsRef> {
        unsafe {
            let mut caps = ptr::null_mut();
            ffi::gst_query_parse_caps(self.as_mut_ptr(), &mut caps);
            if caps.is_null() {
                None
            } else {
                Some(crate::CapsRef::from_ptr(caps))
            }
        }
    }

    #[doc(alias = "get_filter_owned")]
    #[doc(alias = "gst_query_parse_caps")]
    pub fn filter_owned(&self) -> Option<crate::Caps> {
        unsafe { self.filter().map(|caps| from_glib_none(caps.as_ptr())) }
    }

    #[doc(alias = "get_result")]
    #[doc(alias = "gst_query_parse_caps_result")]
    pub fn result(&self) -> Option<&crate::CapsRef> {
        unsafe {
            let mut caps = ptr::null_mut();
            ffi::gst_query_parse_caps_result(self.as_mut_ptr(), &mut caps);
            if caps.is_null() {
                None
            } else {
                Some(crate::CapsRef::from_ptr(caps))
            }
        }
    }

    #[doc(alias = "get_result_owned")]
    #[doc(alias = "gst_query_parse_caps_result")]
    pub fn result_owned(&self) -> Option<crate::Caps> {
        unsafe { self.result().map(|caps| from_glib_none(caps.as_ptr())) }
    }

    #[doc(alias = "gst_query_set_caps_result")]
    pub fn set_result<'a>(&mut self, caps: impl Into<Option<&'a crate::Caps>>) {
        unsafe {
            ffi::gst_query_set_caps_result(
                self.as_mut_ptr(),
                caps.into()
                    .map(|caps| caps.as_mut_ptr())
                    .unwrap_or(ptr::null_mut()),
            );
        }
    }
}

impl std::fmt::Debug for Caps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Caps")
            .field("structure", &self.query().structure())
            .field("result", &self.result())
            .field("filter", &self.filter())
            .finish()
    }
}

impl std::fmt::Debug for Caps<Query> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Caps::<QueryRef>::fmt(self, f)
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

impl std::fmt::Debug for Drain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Drain")
            .field("structure", &self.query().structure())
            .finish()
    }
}

impl std::fmt::Debug for Drain<Query> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Drain::<QueryRef>::fmt(self, f)
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

impl Context {
    #[doc(alias = "get_context")]
    #[doc(alias = "gst_query_parse_context")]
    pub fn context(&self) -> Option<&crate::ContextRef> {
        unsafe {
            let mut context = ptr::null_mut();
            ffi::gst_query_parse_context(self.as_mut_ptr(), &mut context);
            if context.is_null() {
                None
            } else {
                Some(crate::ContextRef::from_ptr(context))
            }
        }
    }

    #[doc(alias = "get_context_owned")]
    #[doc(alias = "gst_query_parse_context")]
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
            ffi::gst_query_parse_context_type(self.as_mut_ptr(), &mut context_type);
            CStr::from_ptr(context_type).to_str().unwrap()
        }
    }

    #[doc(alias = "gst_query_set_context")]
    pub fn set_context<'a>(&mut self, context: impl Into<Option<&'a crate::Context>>) {
        unsafe {
            ffi::gst_query_set_context(
                self.as_mut_ptr(),
                context
                    .into()
                    .map(|context| context.as_mut_ptr())
                    .unwrap_or(ptr::null_mut()),
            );
        }
    }
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Context")
            .field("structure", &self.query().structure())
            .field("context", &self.context())
            .field("context-type", &self.context_type())
            .finish()
    }
}

impl std::fmt::Debug for Context<Query> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Context::<QueryRef>::fmt(self, f)
    }
}

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
declare_concrete_query!(Bitrate, T);

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
impl Bitrate<Query> {
    #[doc(alias = "gst_query_new_bitrate")]
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { Self(from_glib_full(ffi::gst_query_new_bitrate())) }
    }
}

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
impl Default for Bitrate<Query> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
impl Bitrate {
    #[doc(alias = "get_bitrate")]
    #[doc(alias = "gst_query_parse_bitrate")]
    pub fn bitrate(&self) -> u32 {
        unsafe {
            let mut bitrate = mem::MaybeUninit::uninit();
            ffi::gst_query_parse_bitrate(self.as_mut_ptr(), bitrate.as_mut_ptr());
            bitrate.assume_init()
        }
    }

    #[doc(alias = "gst_query_set_bitrate")]
    pub fn set_bitrate(&mut self, bitrate: u32) {
        unsafe {
            ffi::gst_query_set_bitrate(self.as_mut_ptr(), bitrate);
        }
    }
}

#[cfg(feature = "v1_16")]
impl std::fmt::Debug for Bitrate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bitrate")
            .field("structure", &self.query().structure())
            .field("bitrate", &self.bitrate())
            .finish()
    }
}

#[cfg(feature = "v1_16")]
impl std::fmt::Debug for Bitrate<Query> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Bitrate::<QueryRef>::fmt(self, f)
    }
}

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
declare_concrete_query!(Selectable, T);

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
impl Selectable<Query> {
    #[doc(alias = "gst_query_new_selectable")]
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { Self(from_glib_full(ffi::gst_query_new_selectable())) }
    }
}

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
impl Default for Selectable<Query> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
impl Selectable {
    #[doc(alias = "get_selectable")]
    #[doc(alias = "gst_query_parse_selectable")]
    pub fn selectable(&self) -> bool {
        unsafe {
            let mut selectable = mem::MaybeUninit::uninit();
            ffi::gst_query_parse_selectable(self.as_mut_ptr(), selectable.as_mut_ptr());
            from_glib(selectable.assume_init())
        }
    }

    #[doc(alias = "gst_query_set_selectable")]
    pub fn set_selectable(&mut self, selectable: bool) {
        unsafe {
            ffi::gst_query_set_selectable(self.as_mut_ptr(), selectable.into_glib());
        }
    }
}

#[cfg(feature = "v1_22")]
impl std::fmt::Debug for Selectable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Selectable")
            .field("structure", &self.query().structure())
            .field("selectable", &self.selectable())
            .finish()
    }
}

#[cfg(feature = "v1_22")]
impl std::fmt::Debug for Selectable<Query> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Selectable::<QueryRef>::fmt(self, f)
    }
}

declare_concrete_query!(Other, T);

impl std::fmt::Debug for Other {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Other")
            .field("structure", &self.query().structure())
            .finish()
    }
}

impl std::fmt::Debug for Other<Query> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Other::<QueryRef>::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClockTime;

    #[test]
    fn test_writability() {
        crate::init().unwrap();

        fn check_mut(query: &mut QueryRef) {
            skip_assert_initialized!();
            match query.view_mut() {
                QueryViewMut::Position(p) => {
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
                QueryView::Position(p) => {
                    let pos = p.result();
                    assert_eq!(pos.try_into(), Ok(Some(3 * ClockTime::SECOND)));
                    assert!(!p.as_mut_ptr().is_null());
                }
                _ => panic!("Wrong concrete Query in Query"),
            }
        }

        let mut p = Position::new(crate::Format::Time);
        let pos = p.result();
        assert_eq!(pos.try_into(), Ok(ClockTime::NONE));

        p.structure_mut().set("check_mut", true);

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
        if let QueryViewMut::Duration(d) = query.view_mut() {
            d.set(Some(2 * ClockTime::SECOND));
        }

        if let QueryView::Duration(d) = query.view() {
            let duration = d.result();
            assert_eq!(duration.try_into(), Ok(Some(2 * ClockTime::SECOND)));
        }
    }

    #[test]
    fn test_concrete_to_sys() {
        crate::init().unwrap();

        let p = Position::new(crate::Format::Time);
        assert!(!p.as_mut_ptr().is_null());
    }

    #[test]
    fn allocation_need_pool() {
        crate::init().unwrap();

        let mut a = Allocation::new(Some(&crate::Caps::new_empty_simple("foo/bar")), true);
        let pool = crate::BufferPool::new();
        a.add_allocation_pool(Some(&pool), 1024, 1, 4);
    }

    #[test]
    fn allocation_do_not_need_pool() {
        crate::init().unwrap();

        let mut a = Allocation::new(Some(&crate::Caps::new_empty_simple("foo/bar")), false);
        a.add_allocation_pool(crate::BufferPool::NONE, 1024, 1, 4);

        // cannot infer type of the type parameter `T` declared on the enum `Option`
        //a.add_allocation_pool(None, 1024, 1, 4);

        // This would be possible if we moved the `crate::BufferPool`
        // as a generic argument instead of using current arg type:
        // - `pool: Option<&impl IsA<crate::BufferPool>>`
        //a.add_allocation_pool::<crate::BufferPool>(None, 1024, 1, 4);
    }

    #[test]
    fn set_uri() {
        crate::init().unwrap();

        let mut uri_q = Uri::new();
        uri_q.set_uri("https://test.org");
        uri_q.set_uri(&String::from("https://test.org"));

        uri_q.set_uri(Some("https://test.org"));
        uri_q.set_uri(Some(&String::from("https://test.org")));

        // FIXME: this is commented out for now due to an inconsistent
        //        assertion in `GStreamer` which results in critical logs.
        /*
        let none: Option<&str> = None;
        uri_q.set_uri(none);

        let none: Option<String> = None;
        uri_q.set_uri(none.as_ref());

        uri_q.set_uri::<str>(None);
        */
    }
}
