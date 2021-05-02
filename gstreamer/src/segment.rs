// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Format;
use crate::FormattedValue;
use crate::GenericFormattedValue;
use crate::SeekFlags;
use crate::SeekType;
use glib::translate::*;
use glib::StaticType;
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ptr;

pub type Segment = FormattedSegment<GenericFormattedValue>;
#[repr(transparent)]
pub struct FormattedSegment<T: FormattedValue>(ffi::GstSegment, PhantomData<T>);

impl Segment {
    pub fn reset_with_format(&mut self, format: Format) {
        unsafe {
            ffi::gst_segment_init(self.to_glib_none_mut().0, format.into_glib());
        }
    }

    pub fn set_format(&mut self, format: Format) {
        self.0.format = format.into_glib();
    }

    pub fn downcast<T: FormattedValue>(self) -> Result<FormattedSegment<T>, Self> {
        if T::default_format() == Format::Undefined || T::default_format() == self.format() {
            Ok(FormattedSegment(self.0, PhantomData))
        } else {
            Err(self)
        }
    }

    pub fn downcast_ref<T: FormattedValue>(&self) -> Option<&FormattedSegment<T>> {
        if T::default_format() == Format::Undefined || T::default_format() == self.format() {
            Some(unsafe {
                &*(self as *const FormattedSegment<GenericFormattedValue>
                    as *const FormattedSegment<T>)
            })
        } else {
            None
        }
    }

    pub fn downcast_mut<T: FormattedValue>(&mut self) -> Option<&mut FormattedSegment<T>> {
        if T::default_format() == Format::Undefined || T::default_format() == self.format() {
            Some(unsafe {
                &mut *(self as *mut FormattedSegment<GenericFormattedValue>
                    as *mut FormattedSegment<T>)
            })
        } else {
            None
        }
    }
}

impl<T: FormattedValue> FormattedSegment<T> {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        let segment = unsafe {
            let mut segment = mem::MaybeUninit::zeroed();
            ffi::gst_segment_init(segment.as_mut_ptr(), T::default_format().into_glib());
            segment.assume_init()
        };
        FormattedSegment(segment, PhantomData)
    }

    pub fn upcast(self) -> Segment {
        FormattedSegment(self.0, PhantomData)
    }

    pub fn upcast_ref(&self) -> &Segment {
        unsafe {
            &*(self as *const FormattedSegment<T> as *const FormattedSegment<GenericFormattedValue>)
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            ffi::gst_segment_init(&mut self.0, T::default_format().into_glib());
        }
    }

    pub fn clip<V: Into<T>>(&self, start: V, stop: V) -> Option<(T, T)> {
        let start = start.into();
        let stop = stop.into();

        if T::default_format() == Format::Undefined {
            assert_eq!(self.format(), start.format());
            assert_eq!(self.format(), stop.format());
        }

        unsafe {
            let mut clip_start = mem::MaybeUninit::uninit();
            let mut clip_stop = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_segment_clip(
                &self.0,
                start.format().into_glib(),
                start.to_raw_value() as u64,
                stop.to_raw_value() as u64,
                clip_start.as_mut_ptr(),
                clip_stop.as_mut_ptr(),
            ));
            if ret {
                Some((
                    T::from_raw(self.format(), clip_start.assume_init() as i64),
                    T::from_raw(self.format(), clip_stop.assume_init() as i64),
                ))
            } else {
                None
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
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

        if T::default_format() == Format::Undefined {
            assert_eq!(self.format(), start.format());
            assert_eq!(self.format(), stop.format());
        }

        unsafe {
            let mut update = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_segment_do_seek(
                &mut self.0,
                rate,
                self.format().into_glib(),
                flags.into_glib(),
                start_type.into_glib(),
                start.to_raw_value() as u64,
                stop_type.into_glib(),
                stop.to_raw_value() as u64,
                update.as_mut_ptr(),
            ));
            if ret {
                Some(from_glib(update.assume_init()))
            } else {
                None
            }
        }
    }

    pub fn offset_running_time(&mut self, offset: i64) -> Result<(), glib::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_segment_offset_running_time(
                    &mut self.0,
                    self.format().into_glib(),
                    offset,
                ),
                "Offset is not in the segment"
            )
        }
    }

    pub fn position_from_running_time<V: Into<T>>(&self, running_time: V) -> T {
        let running_time = running_time.into();

        if T::default_format() == Format::Undefined {
            assert_eq!(self.format(), running_time.format());
        }

        unsafe {
            T::from_raw(
                self.format(),
                ffi::gst_segment_position_from_running_time(
                    &self.0,
                    self.format().into_glib(),
                    running_time.to_raw_value() as u64,
                ) as i64,
            )
        }
    }

    pub fn position_from_running_time_full<V: Into<T>>(&self, running_time: V) -> (i32, T) {
        let running_time = running_time.into();

        if T::default_format() == Format::Undefined {
            assert_eq!(self.format(), running_time.format());
        }

        unsafe {
            let mut position = mem::MaybeUninit::uninit();
            let ret = ffi::gst_segment_position_from_running_time_full(
                &self.0,
                self.format().into_glib(),
                running_time.to_raw_value() as u64,
                position.as_mut_ptr(),
            );
            (
                ret,
                T::from_raw(self.format(), position.assume_init() as i64),
            )
        }
    }

    pub fn position_from_stream_time<V: Into<T>>(&self, stream_time: V) -> T {
        let stream_time = stream_time.into();

        if T::default_format() == Format::Undefined {
            assert_eq!(self.format(), stream_time.format());
        }

        unsafe {
            T::from_raw(
                self.format(),
                ffi::gst_segment_position_from_stream_time(
                    &self.0,
                    self.format().into_glib(),
                    stream_time.to_raw_value() as u64,
                ) as i64,
            )
        }
    }

    pub fn position_from_stream_time_full<V: Into<T>>(&self, stream_time: V) -> (i32, T) {
        let stream_time = stream_time.into();

        if T::default_format() == Format::Undefined {
            assert_eq!(self.format(), stream_time.format());
        }

        unsafe {
            let mut position = mem::MaybeUninit::uninit();
            let ret = ffi::gst_segment_position_from_stream_time_full(
                &self.0,
                self.format().into_glib(),
                stream_time.to_raw_value() as u64,
                position.as_mut_ptr(),
            );
            (
                ret,
                T::from_raw(self.format(), position.assume_init() as i64),
            )
        }
    }

    pub fn set_running_time<V: Into<T>>(&mut self, running_time: V) -> Result<(), glib::BoolError> {
        let running_time = running_time.into();

        if T::default_format() == Format::Undefined {
            assert_eq!(self.format(), running_time.format());
        }

        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_segment_set_running_time(
                    &mut self.0,
                    self.format().into_glib(),
                    running_time.to_raw_value() as u64,
                ),
                "Running time is not in the segment"
            )
        }
    }

    pub fn to_running_time<V: Into<T>>(&self, position: V) -> T {
        let position = position.into();

        if T::default_format() == Format::Undefined {
            assert_eq!(self.format(), position.format());
        }

        unsafe {
            T::from_raw(
                self.format(),
                ffi::gst_segment_to_running_time(
                    &self.0,
                    self.format().into_glib(),
                    position.to_raw_value() as u64,
                ) as i64,
            )
        }
    }

    pub fn to_running_time_full<V: Into<T>>(&self, position: V) -> (i32, T) {
        let position = position.into();

        if T::default_format() == Format::Undefined {
            assert_eq!(self.format(), position.format());
        }

        unsafe {
            let mut running_time = mem::MaybeUninit::uninit();
            let ret = ffi::gst_segment_to_running_time_full(
                &self.0,
                self.format().into_glib(),
                position.to_raw_value() as u64,
                running_time.as_mut_ptr(),
            );
            (
                ret,
                T::from_raw(self.format(), running_time.assume_init() as i64),
            )
        }
    }

    pub fn to_stream_time<V: Into<T>>(&self, position: V) -> T {
        let position = position.into();

        if T::default_format() == Format::Undefined {
            assert_eq!(self.format(), position.format());
        }

        unsafe {
            T::from_raw(
                self.format(),
                ffi::gst_segment_to_stream_time(
                    &self.0,
                    self.format().into_glib(),
                    position.to_raw_value() as u64,
                ) as i64,
            )
        }
    }

    pub fn to_stream_time_full<V: Into<T>>(&self, position: V) -> (i32, T) {
        let position = position.into();

        if T::default_format() == Format::Undefined {
            assert_eq!(self.format(), position.format());
        }

        unsafe {
            let mut stream_time = mem::MaybeUninit::uninit();
            let ret = ffi::gst_segment_to_stream_time_full(
                &self.0,
                self.format().into_glib(),
                position.to_raw_value() as u64,
                stream_time.as_mut_ptr(),
            );
            (
                ret,
                T::from_raw(self.format(), stream_time.assume_init() as i64),
            )
        }
    }

    #[doc(alias = "get_flags")]
    pub fn flags(&self) -> crate::SegmentFlags {
        unsafe { from_glib(self.0.flags) }
    }

    pub fn set_flags(&mut self, flags: crate::SegmentFlags) {
        self.0.flags = flags.into_glib();
    }

    #[doc(alias = "get_rate")]
    pub fn rate(&self) -> f64 {
        self.0.rate
    }

    #[allow(clippy::float_cmp)]
    pub fn set_rate(&mut self, rate: f64) {
        assert_ne!(rate, 0.0);
        self.0.rate = rate;
    }

    #[doc(alias = "get_applied_rate")]
    pub fn applied_rate(&self) -> f64 {
        self.0.applied_rate
    }

    #[allow(clippy::float_cmp)]
    pub fn set_applied_rate(&mut self, applied_rate: f64) {
        assert_ne!(applied_rate, 0.0);
        self.0.applied_rate = applied_rate;
    }

    #[doc(alias = "get_format")]
    pub fn format(&self) -> Format {
        unsafe { from_glib(self.0.format) }
    }

    #[doc(alias = "get_base")]
    pub fn base(&self) -> T {
        unsafe { T::from_raw(self.format(), self.0.base as i64) }
    }

    pub fn set_base<V: Into<T>>(&mut self, base: V) {
        let base = base.into();

        if T::default_format() == Format::Undefined {
            assert_eq!(self.format(), base.format());
        }

        self.0.base = unsafe { base.to_raw_value() } as u64;
    }

    #[doc(alias = "get_offset")]
    pub fn offset(&self) -> T {
        unsafe { T::from_raw(self.format(), self.0.offset as i64) }
    }

    pub fn set_offset<V: Into<T>>(&mut self, offset: V) {
        let offset = offset.into();

        if T::default_format() == Format::Undefined {
            assert_eq!(self.format(), offset.format());
        }

        self.0.offset = unsafe { offset.to_raw_value() } as u64;
    }

    #[doc(alias = "get_start")]
    pub fn start(&self) -> T {
        unsafe { T::from_raw(self.format(), self.0.start as i64) }
    }

    pub fn set_start<V: Into<T>>(&mut self, start: V) {
        let start = start.into();

        if T::default_format() == Format::Undefined {
            assert_eq!(self.format(), start.format());
        }

        self.0.start = unsafe { start.to_raw_value() } as u64;
    }

    #[doc(alias = "get_stop")]
    pub fn stop(&self) -> T {
        unsafe { T::from_raw(self.format(), self.0.stop as i64) }
    }

    pub fn set_stop<V: Into<T>>(&mut self, stop: V) {
        let stop = stop.into();

        if T::default_format() == Format::Undefined {
            assert_eq!(self.format(), stop.format());
        }

        self.0.stop = unsafe { stop.to_raw_value() } as u64;
    }

    #[doc(alias = "get_time")]
    pub fn time(&self) -> T {
        unsafe { T::from_raw(self.format(), self.0.time as i64) }
    }

    pub fn set_time<V: Into<T>>(&mut self, time: V) {
        let time = time.into();

        if T::default_format() == Format::Undefined {
            assert_eq!(self.format(), time.format());
        }

        self.0.time = unsafe { time.to_raw_value() } as u64;
    }

    #[doc(alias = "get_position")]
    pub fn position(&self) -> T {
        unsafe { T::from_raw(self.format(), self.0.position as i64) }
    }

    pub fn set_position<V: Into<T>>(&mut self, position: V) {
        let position = position.into();

        if T::default_format() == Format::Undefined {
            assert_eq!(self.format(), position.format());
        }

        self.0.position = unsafe { position.to_raw_value() } as u64;
    }

    #[doc(alias = "get_duration")]
    pub fn duration(&self) -> T {
        unsafe { T::from_raw(self.format(), self.0.duration as i64) }
    }

    pub fn set_duration<V: Into<T>>(&mut self, duration: V) {
        let duration = duration.into();

        if T::default_format() == Format::Undefined {
            assert_eq!(self.format(), duration.format());
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
unsafe impl<T: FormattedValue> Sync for FormattedSegment<T> {}

impl<T: FormattedValue> Clone for FormattedSegment<T> {
    fn clone(&self) -> Self {
        unsafe { FormattedSegment(ptr::read(&self.0), PhantomData) }
    }
}

impl<T: FormattedValue> AsRef<Segment> for FormattedSegment<T> {
    fn as_ref(&self) -> &Segment {
        unsafe {
            &*(self as *const FormattedSegment<T> as *const FormattedSegment<GenericFormattedValue>)
        }
    }
}

impl<T: FormattedValue> fmt::Debug for FormattedSegment<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let segment = self.as_ref();
        match segment.format() {
            Format::Undefined => f
                .debug_struct("Segment")
                .field("format", &Format::Undefined)
                .finish(),
            Format::Time => {
                let segment = segment.downcast_ref::<crate::ClockTime>().unwrap();
                f.debug_struct("Segment")
                    .field("format", &Format::Time)
                    .field("start", &segment.start().to_string())
                    .field("offset", &segment.offset().to_string())
                    .field("stop", &segment.stop().to_string())
                    .field("rate", &segment.rate())
                    .field("applied_rate", &segment.applied_rate())
                    .field("flags", &segment.flags())
                    .field("time", &segment.time().to_string())
                    .field("base", &segment.base().to_string())
                    .field("position", &segment.position().to_string())
                    .field("duration", &segment.duration().to_string())
                    .finish()
            }
            _ => f
                .debug_struct("Segment")
                .field("format", &segment.format())
                .field("start", &segment.start())
                .field("offset", &segment.offset())
                .field("stop", &segment.stop())
                .field("rate", &segment.rate())
                .field("applied_rate", &segment.applied_rate())
                .field("flags", &segment.flags())
                .field("time", &segment.time())
                .field("base", &segment.base())
                .field("position", &segment.position())
                .field("duration", &segment.duration())
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

impl glib::value::ValueType for Segment {
    type Type = Self;
}

#[doc(hidden)]
unsafe impl<'a> glib::value::FromValue<'a> for Segment {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib_none(
            glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *mut ffi::GstSegment
        )
    }
}

#[doc(hidden)]
impl<T: FormattedValue> glib::value::ToValue for FormattedSegment<T> {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Segment>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                self.to_glib_none().0 as *mut _,
            )
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[doc(hidden)]
impl<T: FormattedValue> glib::value::ToValueOptional for FormattedSegment<T> {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<Segment>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                s.to_glib_none().0 as *mut _,
            )
        }
        value
    }
}
#[doc(hidden)]
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
    unsafe fn from_glib_borrow(ptr: *mut ffi::GstSegment) -> Borrowed<Self> {
        Borrowed::new(FormattedSegment(ptr::read(ptr), PhantomData))
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrFull<*mut ffi::GstSegment> for Segment {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GstSegment) -> Self {
        let segment = from_glib_none(ptr);
        glib::ffi::g_free(ptr as *mut _);
        segment
    }
}
