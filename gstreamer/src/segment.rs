// Take a look at the license at the top of the repository in the LICENSE file.

use crate::format::{
    CompatibleFormattedValue, FormattedValue, FormattedValueFullRange, FormattedValueIntrinsic,
    FormattedValueNoneBuilder, NoneSignedBuilder, UnsignedIntoSigned,
};
use crate::Format;
use crate::GenericFormattedValue;
use crate::SeekFlags;
use crate::SeekType;
use glib::translate::*;
use glib::StaticType;
use std::fmt;
use std::marker::PhantomData;
use std::mem;

pub type Segment = FormattedSegment<GenericFormattedValue>;

glib::wrapper! {
    #[doc(alias = "GstSegment")]
    pub struct FormattedSegment<T: FormattedValueIntrinsic>(BoxedInline<ffi::GstSegment>);

    match fn {
        copy => |ptr| ffi::gst_segment_copy(ptr),
        free => |ptr| ffi::gst_segment_free(ptr),
        init => |_ptr| (),
        copy_into => |dest, src| { *dest = *src; },
        clear => |_ptr| (),
    }
}

impl Segment {
    pub fn reset_with_format(&mut self, format: Format) {
        unsafe {
            ffi::gst_segment_init(self.to_glib_none_mut().0, format.into_glib());
        }
    }

    pub fn set_format(&mut self, format: Format) {
        self.inner.format = format.into_glib();
    }

    pub fn downcast<T: FormattedValueIntrinsic>(self) -> Result<FormattedSegment<T>, Self> {
        if T::default_format() == Format::Undefined || T::default_format() == self.format() {
            Ok(FormattedSegment {
                inner: self.inner,
                phantom: PhantomData,
            })
        } else {
            Err(self)
        }
    }

    pub fn downcast_ref<T: FormattedValueIntrinsic>(&self) -> Option<&FormattedSegment<T>> {
        if T::default_format() == Format::Undefined || T::default_format() == self.format() {
            Some(unsafe {
                &*(self as *const FormattedSegment<GenericFormattedValue>
                    as *const FormattedSegment<T>)
            })
        } else {
            None
        }
    }

    pub fn downcast_mut<T: FormattedValueIntrinsic>(&mut self) -> Option<&mut FormattedSegment<T>> {
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

impl<T: FormattedValueIntrinsic> FormattedSegment<T> {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        let segment = unsafe {
            let mut segment = mem::MaybeUninit::zeroed();
            ffi::gst_segment_init(segment.as_mut_ptr(), T::default_format().into_glib());
            segment.assume_init()
        };
        FormattedSegment {
            inner: segment,
            phantom: PhantomData,
        }
    }

    pub fn upcast(self) -> Segment {
        FormattedSegment {
            inner: self.inner,
            phantom: PhantomData,
        }
    }

    pub fn upcast_ref(&self) -> &Segment {
        unsafe {
            &*(self as *const FormattedSegment<T> as *const FormattedSegment<GenericFormattedValue>)
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            ffi::gst_segment_init(&mut self.inner, T::default_format().into_glib());
        }
    }

    #[doc(alias = "gst_segment_clip")]
    pub fn clip(
        &self,
        start: impl CompatibleFormattedValue<T>,
        stop: impl CompatibleFormattedValue<T>,
    ) -> Option<(T::FullRange, T::FullRange)> {
        let start = start.try_into_checked_explicit(self.format()).unwrap();
        let stop = stop.try_into_checked_explicit(self.format()).unwrap();

        unsafe {
            let mut clip_start = mem::MaybeUninit::uninit();
            let mut clip_stop = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_segment_clip(
                &self.inner,
                start.format().into_glib(),
                start.into_raw_value() as u64,
                stop.into_raw_value() as u64,
                clip_start.as_mut_ptr(),
                clip_stop.as_mut_ptr(),
            ));
            if ret {
                Some((
                    T::FullRange::from_raw(self.format(), clip_start.assume_init() as i64),
                    T::FullRange::from_raw(self.format(), clip_stop.assume_init() as i64),
                ))
            } else {
                None
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    #[doc(alias = "gst_segment_do_seek")]
    pub fn do_seek(
        &mut self,
        rate: f64,
        flags: SeekFlags,
        start_type: SeekType,
        start: impl CompatibleFormattedValue<T>,
        stop_type: SeekType,
        stop: impl CompatibleFormattedValue<T>,
    ) -> Option<bool> {
        skip_assert_initialized!();
        let start = start.try_into_checked_explicit(self.format()).unwrap();
        let stop = stop.try_into_checked_explicit(self.format()).unwrap();

        unsafe {
            let mut update = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_segment_do_seek(
                &mut self.inner,
                rate,
                self.format().into_glib(),
                flags.into_glib(),
                start_type.into_glib(),
                start.into_raw_value() as u64,
                stop_type.into_glib(),
                stop.into_raw_value() as u64,
                update.as_mut_ptr(),
            ));
            if ret {
                Some(from_glib(update.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_segment_offset_running_time")]
    pub fn offset_running_time(&mut self, offset: i64) -> Result<(), glib::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_segment_offset_running_time(
                    &mut self.inner,
                    self.format().into_glib(),
                    offset,
                ),
                "Offset is not in the segment"
            )
        }
    }

    #[doc(alias = "gst_segment_set_running_time")]
    pub fn set_running_time(
        &mut self,
        running_time: impl CompatibleFormattedValue<T>,
    ) -> Result<(), glib::BoolError> {
        let running_time = running_time
            .try_into_checked_explicit(self.format())
            .unwrap();

        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_segment_set_running_time(
                    &mut self.inner,
                    self.format().into_glib(),
                    running_time.into_raw_value() as u64,
                ),
                "Running time is not in the segment"
            )
        }
    }

    #[doc(alias = "get_flags")]
    pub fn flags(&self) -> crate::SegmentFlags {
        unsafe { from_glib(self.inner.flags) }
    }

    pub fn set_flags(&mut self, flags: crate::SegmentFlags) {
        self.inner.flags = flags.into_glib();
    }

    #[doc(alias = "get_rate")]
    pub fn rate(&self) -> f64 {
        self.inner.rate
    }

    #[allow(clippy::float_cmp)]
    pub fn set_rate(&mut self, rate: f64) {
        assert_ne!(rate, 0.0);
        self.inner.rate = rate;
    }

    #[doc(alias = "get_applied_rate")]
    pub fn applied_rate(&self) -> f64 {
        self.inner.applied_rate
    }

    #[allow(clippy::float_cmp)]
    pub fn set_applied_rate(&mut self, applied_rate: f64) {
        assert_ne!(applied_rate, 0.0);
        self.inner.applied_rate = applied_rate;
    }

    #[doc(alias = "get_format")]
    pub fn format(&self) -> Format {
        unsafe { from_glib(self.inner.format) }
    }

    #[doc(alias = "get_base")]
    pub fn base(&self) -> T::FullRange {
        unsafe { T::FullRange::from_raw(self.format(), self.inner.base as i64) }
    }

    pub fn set_base(&mut self, base: impl CompatibleFormattedValue<T>) {
        let base = base.try_into_checked_explicit(self.format()).unwrap();
        self.inner.base = unsafe { base.into_raw_value() } as u64;
    }

    #[doc(alias = "get_offset")]
    pub fn offset(&self) -> T::FullRange {
        unsafe { T::FullRange::from_raw(self.format(), self.inner.offset as i64) }
    }

    pub fn set_offset(&mut self, offset: impl CompatibleFormattedValue<T>) {
        let offset = offset.try_into_checked_explicit(self.format()).unwrap();
        self.inner.offset = unsafe { offset.into_raw_value() } as u64;
    }

    #[doc(alias = "get_start")]
    pub fn start(&self) -> T::FullRange {
        unsafe { T::FullRange::from_raw(self.format(), self.inner.start as i64) }
    }

    pub fn set_start(&mut self, start: impl CompatibleFormattedValue<T>) {
        let start = start.try_into_checked_explicit(self.format()).unwrap();
        self.inner.start = unsafe { start.into_raw_value() } as u64;
    }

    #[doc(alias = "get_stop")]
    pub fn stop(&self) -> T::FullRange {
        unsafe { T::FullRange::from_raw(self.format(), self.inner.stop as i64) }
    }

    pub fn set_stop(&mut self, stop: impl CompatibleFormattedValue<T>) {
        let stop = stop.try_into_checked_explicit(self.format()).unwrap();
        self.inner.stop = unsafe { stop.into_raw_value() } as u64;
    }

    #[doc(alias = "get_time")]
    pub fn time(&self) -> T::FullRange {
        unsafe { T::FullRange::from_raw(self.format(), self.inner.time as i64) }
    }

    pub fn set_time(&mut self, time: impl CompatibleFormattedValue<T>) {
        let time = time.try_into_checked_explicit(self.format()).unwrap();
        self.inner.time = unsafe { time.into_raw_value() } as u64;
    }

    #[doc(alias = "get_position")]
    pub fn position(&self) -> T::FullRange {
        unsafe { T::FullRange::from_raw(self.format(), self.inner.position as i64) }
    }

    pub fn set_position(&mut self, position: impl CompatibleFormattedValue<T>) {
        let position = position.try_into_checked_explicit(self.format()).unwrap();
        self.inner.position = unsafe { position.into_raw_value() } as u64;
    }

    #[doc(alias = "get_duration")]
    pub fn duration(&self) -> T::FullRange {
        unsafe { T::FullRange::from_raw(self.format(), self.inner.duration as i64) }
    }

    pub fn set_duration(&mut self, duration: impl CompatibleFormattedValue<T>) {
        let duration = duration.try_into_checked_explicit(self.format()).unwrap();
        self.inner.duration = unsafe { duration.into_raw_value() } as u64;
    }
}

impl<T: FormattedValueIntrinsic> PartialEq for FormattedSegment<T> {
    #[inline]
    #[doc(alias = "gst_segment_is_equal")]
    fn eq(&self, other: &Self) -> bool {
        unsafe { from_glib(ffi::gst_segment_is_equal(&self.inner, &other.inner)) }
    }
}

impl<T> FormattedSegment<T>
where
    T: FormattedValueIntrinsic,
    T::FullRange: FormattedValueNoneBuilder,
{
    #[doc(alias = "gst_segment_position_from_running_time")]
    pub fn position_from_running_time(
        &self,
        running_time: impl CompatibleFormattedValue<T>,
    ) -> T::FullRange {
        let running_time = running_time
            .try_into_checked_explicit(self.format())
            .unwrap();
        if running_time.is_none() {
            return T::FullRange::none_for_format(self.format());
        }

        unsafe {
            T::FullRange::from_raw(
                self.format(),
                ffi::gst_segment_position_from_running_time(
                    &self.inner,
                    self.format().into_glib(),
                    running_time.into_raw_value() as u64,
                ) as i64,
            )
        }
    }

    #[doc(alias = "gst_segment_position_from_stream_time")]
    pub fn position_from_stream_time(
        &self,
        stream_time: impl CompatibleFormattedValue<T>,
    ) -> T::FullRange {
        let stream_time = stream_time
            .try_into_checked_explicit(self.format())
            .unwrap();
        if stream_time.is_none() {
            return T::FullRange::none_for_format(self.format());
        }

        unsafe {
            T::FullRange::from_raw(
                self.format(),
                ffi::gst_segment_position_from_stream_time(
                    &self.inner,
                    self.format().into_glib(),
                    stream_time.into_raw_value() as u64,
                ) as i64,
            )
        }
    }

    #[doc(alias = "gst_segment_to_running_time")]
    pub fn to_running_time(&self, position: impl CompatibleFormattedValue<T>) -> T::FullRange {
        let position = position.try_into_checked_explicit(self.format()).unwrap();
        if position.is_none() {
            return T::FullRange::none_for_format(self.format());
        }

        unsafe {
            T::FullRange::from_raw(
                self.format(),
                ffi::gst_segment_to_running_time(
                    &self.inner,
                    self.format().into_glib(),
                    position.into_raw_value() as u64,
                ) as i64,
            )
        }
    }

    #[doc(alias = "gst_segment_to_stream_time")]
    pub fn to_stream_time(&self, position: impl CompatibleFormattedValue<T>) -> T::FullRange {
        let position = position.try_into_checked_explicit(self.format()).unwrap();
        if position.is_none() {
            return T::FullRange::none_for_format(self.format());
        }

        unsafe {
            T::FullRange::from_raw(
                self.format(),
                ffi::gst_segment_to_stream_time(
                    &self.inner,
                    self.format().into_glib(),
                    position.into_raw_value() as u64,
                ) as i64,
            )
        }
    }
}

impl<T> FormattedSegment<T>
where
    T: FormattedValueIntrinsic,
    T::FullRange: UnsignedIntoSigned,
    T::FullRange: NoneSignedBuilder<Signed = <T::FullRange as UnsignedIntoSigned>::Signed>,
{
    #[doc(alias = "gst_segment_position_from_running_time_full")]
    pub fn position_from_running_time_full(
        &self,
        running_time: impl CompatibleFormattedValue<T>,
    ) -> <T::FullRange as UnsignedIntoSigned>::Signed {
        let running_time = running_time
            .try_into_checked_explicit(self.format())
            .unwrap();
        if running_time.is_none() {
            return T::FullRange::none_signed_for_format(self.format());
        }

        unsafe {
            let mut position = mem::MaybeUninit::uninit();
            let sign = ffi::gst_segment_position_from_running_time_full(
                &self.inner,
                self.format().into_glib(),
                running_time.into_raw_value() as u64,
                position.as_mut_ptr(),
            );

            T::FullRange::from_raw(self.format(), position.assume_init() as i64).into_signed(sign)
        }
    }

    #[doc(alias = "gst_segment_position_from_stream_time_full")]
    pub fn position_from_stream_time_full(
        &self,
        stream_time: impl CompatibleFormattedValue<T>,
    ) -> <T::FullRange as UnsignedIntoSigned>::Signed {
        let stream_time = stream_time
            .try_into_checked_explicit(self.format())
            .unwrap();
        if stream_time.is_none() {
            return T::FullRange::none_signed_for_format(self.format());
        }

        unsafe {
            let mut position = mem::MaybeUninit::uninit();
            let sign = ffi::gst_segment_position_from_stream_time_full(
                &self.inner,
                self.format().into_glib(),
                stream_time.into_raw_value() as u64,
                position.as_mut_ptr(),
            );

            T::FullRange::from_raw(self.format(), position.assume_init() as i64).into_signed(sign)
        }
    }

    #[doc(alias = "gst_segment_to_running_time_full")]
    pub fn to_running_time_full(
        &self,
        position: impl CompatibleFormattedValue<T>,
    ) -> <T::FullRange as UnsignedIntoSigned>::Signed {
        let position = position.try_into_checked_explicit(self.format()).unwrap();
        if position.is_none() {
            return T::FullRange::none_signed_for_format(self.format());
        }

        unsafe {
            let mut running_time = mem::MaybeUninit::uninit();
            let sign = ffi::gst_segment_to_running_time_full(
                &self.inner,
                self.format().into_glib(),
                position.into_raw_value() as u64,
                running_time.as_mut_ptr(),
            );

            T::FullRange::from_raw(self.format(), running_time.assume_init() as i64)
                .into_signed(sign)
        }
    }

    #[doc(alias = "gst_segment_to_stream_time_full")]
    pub fn to_stream_time_full(
        &self,
        position: impl CompatibleFormattedValue<T>,
    ) -> <T::FullRange as UnsignedIntoSigned>::Signed {
        let position = position.try_into_checked_explicit(self.format()).unwrap();
        if position.is_none() {
            return T::FullRange::none_signed_for_format(self.format());
        }

        unsafe {
            let mut stream_time = mem::MaybeUninit::uninit();
            let sign = ffi::gst_segment_to_stream_time_full(
                &self.inner,
                self.format().into_glib(),
                position.into_raw_value() as u64,
                stream_time.as_mut_ptr(),
            );

            T::FullRange::from_raw(self.format(), stream_time.assume_init() as i64)
                .into_signed(sign)
        }
    }
}

impl<T: FormattedValueIntrinsic> Eq for FormattedSegment<T> {}

unsafe impl<T: FormattedValueIntrinsic> Send for FormattedSegment<T> {}
unsafe impl<T: FormattedValueIntrinsic> Sync for FormattedSegment<T> {}

impl<T: FormattedValueIntrinsic> AsRef<Segment> for FormattedSegment<T> {
    fn as_ref(&self) -> &Segment {
        unsafe {
            &*(self as *const FormattedSegment<T> as *const FormattedSegment<GenericFormattedValue>)
        }
    }
}

impl<T: FormattedValueIntrinsic> fmt::Debug for FormattedSegment<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::utils::Displayable;

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
                    .field("start", &segment.start().display())
                    .field("offset", &segment.offset().display())
                    .field("stop", &segment.stop().display())
                    .field("rate", &segment.rate())
                    .field("applied_rate", &segment.applied_rate())
                    .field("flags", &segment.flags())
                    .field("time", &segment.time().display())
                    .field("base", &segment.base().display())
                    .field("position", &segment.position().display())
                    .field("duration", &segment.duration().display())
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

impl<T: FormattedValueIntrinsic> Default for FormattedSegment<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: FormattedValueIntrinsic> glib::types::StaticType for FormattedSegment<T> {
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
unsafe impl<'a> glib::value::FromValue<'a> for &'a Segment {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        Segment::from_glib_ptr_borrow(
            glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *const ffi::GstSegment
        )
    }
}

#[doc(hidden)]
impl<T: FormattedValueIntrinsic> glib::value::ToValue for FormattedSegment<T> {
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
impl<T: FormattedValueIntrinsic> glib::value::ToValueOptional for FormattedSegment<T> {
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

impl<T: FormattedValueIntrinsic> From<FormattedSegment<T>> for glib::Value {
    fn from(v: FormattedSegment<T>) -> glib::Value {
        skip_assert_initialized!();
        glib::value::ToValue::to_value(&v)
    }
}
