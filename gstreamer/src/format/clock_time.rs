// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    fmt,
    io::{self, prelude::*},
    time::Duration,
};

use glib::{translate::*, StaticType};

use super::{
    Format, FormattedValue, FormattedValueError, FormattedValueFullRange, FormattedValueIntrinsic,
    FormattedValueNoneBuilder, GenericFormattedValue, Signed, SpecificFormattedValue,
    SpecificFormattedValueFullRange, SpecificFormattedValueIntrinsic,
};

const TRY_FROM_FLOAT_SECS_ERROR_MSG: &str =
    "can not convert float seconds to ClockTime: value is either negative, too big or NaN";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TryFromFloatSecsError;

impl fmt::Display for TryFromFloatSecsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(TRY_FROM_FLOAT_SECS_ERROR_MSG)
    }
}

impl std::error::Error for TryFromFloatSecsError {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub struct ClockTime(u64);

impl ClockTime {
    #[doc(alias = "GST_SECOND")]
    pub const SECOND: ClockTime = ClockTime(1_000_000_000);
    #[doc(alias = "GST_MSECOND")]
    pub const MSECOND: ClockTime = ClockTime(1_000_000);
    #[doc(alias = "GST_USECOND")]
    pub const USECOND: ClockTime = ClockTime(1_000);
    #[doc(alias = "GST_NSECOND")]
    pub const NSECOND: ClockTime = ClockTime(1);
    // checker-ignore-item
    pub const MAX: ClockTime = ClockTime(ffi::GST_CLOCK_TIME_NONE - 1);

    #[inline]
    pub const fn hours(self) -> u64 {
        self.0 / Self::SECOND.0 / 60 / 60
    }

    #[inline]
    pub const fn minutes(self) -> u64 {
        self.0 / Self::SECOND.0 / 60
    }

    #[inline]
    pub const fn seconds(self) -> u64 {
        self.0 / Self::SECOND.0
    }

    #[inline]
    pub fn seconds_f32(self) -> f32 {
        self.0 as f32 / Self::SECOND.0 as f32
    }

    #[inline]
    pub fn seconds_f64(self) -> f64 {
        self.0 as f64 / Self::SECOND.0 as f64
    }

    #[inline]
    pub const fn mseconds(self) -> u64 {
        self.0 / Self::MSECOND.0
    }

    #[inline]
    pub const fn useconds(self) -> u64 {
        self.0 / Self::USECOND.0
    }

    #[inline]
    pub const fn nseconds(self) -> u64 {
        self.0
    }

    // rustdoc-stripper-ignore-next
    /// Builds a new `ClockTime` which value is the given number of seconds.
    ///
    /// # Panics
    ///
    /// Panics if the resulting duration in nanoseconds exceeds the `u64` range.
    #[track_caller]
    #[inline]
    pub const fn from_seconds(seconds: u64) -> Self {
        skip_assert_initialized!();
        // `Option::expect` is not `const` as of rustc 1.63.0.
        ClockTime(match seconds.checked_mul(Self::SECOND.0) {
            Some(res) => res,
            None => panic!("Out of `ClockTime` range"),
        })
    }

    // rustdoc-stripper-ignore-next
    /// Builds a new `ClockTime` which value is the given number of seconds.
    ///
    /// Returns an error if seconds is negative, infinite or NaN, or
    /// the resulting duration in nanoseconds exceeds the `u64` range.
    #[inline]
    pub fn try_from_seconds_f32(seconds: f32) -> Result<Self, TryFromFloatSecsError> {
        skip_assert_initialized!();

        let dur = Duration::try_from_secs_f32(seconds).map_err(|_| TryFromFloatSecsError)?;
        ClockTime::try_from(dur).map_err(|_| TryFromFloatSecsError)
    }

    // rustdoc-stripper-ignore-next
    /// Builds a new `ClockTime` which value is the given number of seconds.
    ///
    /// # Panics
    ///
    /// Panics if seconds is negative, infinite or NaN, or the resulting duration
    /// in nanoseconds exceeds the `u64` range.
    #[track_caller]
    #[inline]
    pub fn from_seconds_f32(seconds: f32) -> Self {
        skip_assert_initialized!();

        Self::try_from_seconds_f32(seconds).expect(TRY_FROM_FLOAT_SECS_ERROR_MSG)
    }

    // rustdoc-stripper-ignore-next
    /// Builds a new `ClockTime` which value is the given number of seconds.
    ///
    /// Returns an error if seconds is negative, infinite or NaN, or
    /// the resulting duration in nanoseconds exceeds the `u64` range.
    #[inline]
    pub fn try_from_seconds_f64(seconds: f64) -> Result<Self, TryFromFloatSecsError> {
        skip_assert_initialized!();

        let dur = Duration::try_from_secs_f64(seconds).map_err(|_| TryFromFloatSecsError)?;
        ClockTime::try_from(dur).map_err(|_| TryFromFloatSecsError)
    }

    // rustdoc-stripper-ignore-next
    /// Builds a new `ClockTime` which value is the given number of seconds.
    ///
    /// # Panics
    ///
    /// Panics if seconds is negative, infinite or NaN, or the resulting duration
    /// in nanoseconds exceeds the `u64` range.
    #[track_caller]
    #[inline]
    pub fn from_seconds_f64(seconds: f64) -> Self {
        skip_assert_initialized!();

        Self::try_from_seconds_f64(seconds).expect(TRY_FROM_FLOAT_SECS_ERROR_MSG)
    }

    // rustdoc-stripper-ignore-next
    /// Builds a new `ClockTime` which value is the given number of milliseconds.
    ///
    /// # Panics
    ///
    /// Panics if the resulting duration in nanoseconds exceeds the `u64` range.
    #[track_caller]
    #[inline]
    pub const fn from_mseconds(mseconds: u64) -> Self {
        skip_assert_initialized!();
        // `Option::expect` is not `const` as of rustc 1.63.0.
        ClockTime(match mseconds.checked_mul(Self::MSECOND.0) {
            Some(res) => res,
            None => panic!("Out of `ClockTime` range"),
        })
    }

    // rustdoc-stripper-ignore-next
    /// Builds a new `ClockTime` which value is the given number of microseconds.
    ///
    /// # Panics
    ///
    /// Panics if the resulting duration in nanoseconds exceeds the `u64` range.
    #[track_caller]
    #[inline]
    pub const fn from_useconds(useconds: u64) -> Self {
        skip_assert_initialized!();
        // `Option::expect` is not `const` as of rustc 1.63.0.
        ClockTime(match useconds.checked_mul(Self::USECOND.0) {
            Some(res) => res,
            None => panic!("Out of `ClockTime` range"),
        })
    }

    // rustdoc-stripper-ignore-next
    /// Builds a new `ClockTime` which value is the given number of nanoseconds.
    ///
    /// # Panics
    ///
    /// Panics if the requested duration equals `GST_CLOCK_TIME_NONE`
    /// (`u64::MAX`).
    #[track_caller]
    #[inline]
    pub const fn from_nseconds(nseconds: u64) -> Self {
        skip_assert_initialized!();
        assert!(
            nseconds != ffi::GST_CLOCK_TIME_NONE,
            "Attempt to build a `ClockTime` with value `GST_CLOCK_TIME_NONE`",
        );
        ClockTime(nseconds * Self::NSECOND.0)
    }
}

impl Signed<ClockTime> {
    // rustdoc-stripper-ignore-next
    /// Returns the `self` in nanoseconds.
    #[inline]
    pub fn nseconds(self) -> Signed<u64> {
        match self {
            Signed::Positive(val) => Signed::Positive(val.nseconds()),
            Signed::Negative(val) => Signed::Negative(val.nseconds()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates new value from nanoseconds.
    #[inline]
    pub fn from_nseconds(val: Signed<u64>) -> Self {
        skip_assert_initialized!();
        match val {
            Signed::Positive(val) => Signed::Positive(ClockTime::from_nseconds(val)),
            Signed::Negative(val) => Signed::Negative(ClockTime::from_nseconds(val)),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the `self` in microseconds.
    #[inline]
    pub fn useconds(self) -> Signed<u64> {
        match self {
            Signed::Positive(val) => Signed::Positive(val.useconds()),
            Signed::Negative(val) => Signed::Negative(val.useconds()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates new value from microseconds.
    #[inline]
    pub fn from_useconds(val: Signed<u64>) -> Self {
        skip_assert_initialized!();
        match val {
            Signed::Positive(val) => Signed::Positive(ClockTime::from_useconds(val)),
            Signed::Negative(val) => Signed::Negative(ClockTime::from_useconds(val)),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the `self` in milliseconds.
    #[inline]
    pub fn mseconds(self) -> Signed<u64> {
        match self {
            Signed::Positive(val) => Signed::Positive(val.mseconds()),
            Signed::Negative(val) => Signed::Negative(val.mseconds()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates new value from milliseconds.
    #[inline]
    pub fn from_mseconds(val: Signed<u64>) -> Self {
        skip_assert_initialized!();
        match val {
            Signed::Positive(val) => Signed::Positive(ClockTime::from_mseconds(val)),
            Signed::Negative(val) => Signed::Negative(ClockTime::from_mseconds(val)),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the `self` in seconds.
    #[inline]
    pub fn seconds(self) -> Signed<u64> {
        match self {
            Signed::Positive(val) => Signed::Positive(val.seconds()),
            Signed::Negative(val) => Signed::Negative(val.seconds()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the `self` in f32 seconds.
    #[inline]
    pub fn seconds_f32(self) -> f32 {
        match self {
            Signed::Positive(val) => val.seconds_f32(),
            Signed::Negative(val) => -val.seconds_f32(),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the `self` in f64 seconds.
    #[inline]
    pub fn seconds_f64(self) -> f64 {
        match self {
            Signed::Positive(val) => val.seconds_f64(),
            Signed::Negative(val) => -val.seconds_f64(),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates new value from seconds.
    #[inline]
    pub fn from_seconds(val: Signed<u64>) -> Self {
        skip_assert_initialized!();
        match val {
            Signed::Positive(val) => Signed::Positive(ClockTime::from_seconds(val)),
            Signed::Negative(val) => Signed::Negative(ClockTime::from_seconds(val)),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Builds a new `Signed<ClockTime>` which value is the given number of seconds.
    ///
    /// Returns an error if seconds is infinite or NaN, or
    /// the resulting duration in nanoseconds exceeds the `u64` range.
    #[inline]
    pub fn try_from_seconds_f32(seconds: f32) -> Result<Self, TryFromFloatSecsError> {
        skip_assert_initialized!();

        ClockTime::try_from_seconds_f32(seconds.abs()).map(|ct| {
            if seconds.is_sign_positive() {
                Signed::Positive(ct)
            } else {
                Signed::Negative(ct)
            }
        })
    }

    // rustdoc-stripper-ignore-next
    /// Builds a new `Signed<ClockTime>` which value is the given number of seconds.
    ///
    /// # Panics
    ///
    /// Panics if seconds is infinite or NaN, or the resulting duration
    /// in nanoseconds exceeds the `u64` range.
    #[track_caller]
    #[inline]
    pub fn from_seconds_f32(seconds: f32) -> Self {
        skip_assert_initialized!();

        Self::try_from_seconds_f32(seconds).expect(TRY_FROM_FLOAT_SECS_ERROR_MSG)
    }

    // rustdoc-stripper-ignore-next
    /// Builds a new `Signed<ClockTime>` which value is the given number of seconds.
    ///
    /// Returns an error if seconds is infinite or NaN, or
    /// the resulting duration in nanoseconds exceeds the `u64` range.
    #[inline]
    pub fn try_from_seconds_f64(seconds: f64) -> Result<Self, TryFromFloatSecsError> {
        skip_assert_initialized!();

        ClockTime::try_from_seconds_f64(seconds.abs()).map(|ct| {
            if seconds.is_sign_positive() {
                Signed::Positive(ct)
            } else {
                Signed::Negative(ct)
            }
        })
    }

    // rustdoc-stripper-ignore-next
    /// Builds a new `Signed<ClockTime>` which value is the given number of seconds.
    ///
    /// # Panics
    ///
    /// Panics if seconds is infinite or NaN, or the resulting duration
    /// in nanoseconds exceeds the `u64` range.
    #[track_caller]
    #[inline]
    pub fn from_seconds_f64(seconds: f64) -> Self {
        skip_assert_initialized!();

        Self::try_from_seconds_f64(seconds).expect(TRY_FROM_FLOAT_SECS_ERROR_MSG)
    }
}

impl_format_value_traits!(ClockTime, Time, Time, u64);
option_glib_newtype_from_to!(ClockTime, ffi::GST_CLOCK_TIME_NONE);

// FIXME `functions in traits cannot be const` (rustc 1.64.0)
// rustdoc-stripper-ignore-next
/// `ClockTime` formatted value constructor trait.
pub trait TimeFormatConstructor {
    // rustdoc-stripper-ignore-next
    /// Builds a `ClockTime` formatted value from `self` interpreted as nano seconds.
    fn nseconds(self) -> ClockTime;

    // rustdoc-stripper-ignore-next
    /// Builds a `ClockTime` formatted value from `self` interpreted as micro seconds.
    fn useconds(self) -> ClockTime;

    // rustdoc-stripper-ignore-next
    /// Builds a `ClockTime` formatted value from `self` interpreted as milli seconds.
    fn mseconds(self) -> ClockTime;

    // rustdoc-stripper-ignore-next
    /// Builds a `ClockTime` formatted value from `self` interpreted as seconds.
    fn seconds(self) -> ClockTime;

    // rustdoc-stripper-ignore-next
    /// Builds a `ClockTime` formatted value from `self` interpreted as minutes.
    fn minutes(self) -> ClockTime;

    // rustdoc-stripper-ignore-next
    /// Builds a `ClockTime` formatted value from `self` interpreted as hours.
    fn hours(self) -> ClockTime;
}

impl TimeFormatConstructor for u64 {
    #[track_caller]
    #[inline]
    fn nseconds(self) -> ClockTime {
        ClockTime::from_nseconds(self)
    }

    #[track_caller]
    #[inline]
    fn useconds(self) -> ClockTime {
        ClockTime::from_useconds(self)
    }

    #[track_caller]
    #[inline]
    fn mseconds(self) -> ClockTime {
        ClockTime::from_mseconds(self)
    }

    #[track_caller]
    #[inline]
    fn seconds(self) -> ClockTime {
        ClockTime::from_seconds(self)
    }

    #[track_caller]
    #[inline]
    fn minutes(self) -> ClockTime {
        ClockTime::from_seconds(self * 60)
    }

    #[track_caller]
    #[inline]
    fn hours(self) -> ClockTime {
        ClockTime::from_seconds(self * 60 * 60)
    }
}

impl glib::value::ValueType for ClockTime {
    type Type = Self;
}

pub enum ClockTimeValueTypeOrNoneChecker {}

unsafe impl glib::value::ValueTypeChecker for ClockTimeValueTypeOrNoneChecker {
    type Error = glib::value::ValueTypeMismatchOrNoneError<glib::value::ValueTypeMismatchError>;

    #[inline]
    fn check(value: &glib::Value) -> Result<(), Self::Error> {
        skip_assert_initialized!();
        glib::value::GenericValueTypeChecker::<ClockTime>::check(value)?;

        let gct = unsafe { glib::gobject_ffi::g_value_get_uint64(value.to_glib_none().0) };
        if gct == ffi::GST_CLOCK_TIME_NONE {
            return Err(glib::value::ValueTypeMismatchOrNoneError::UnexpectedNone);
        }

        Ok(())
    }
}

unsafe impl<'a> glib::value::FromValue<'a> for ClockTime {
    type Checker = ClockTimeValueTypeOrNoneChecker;

    #[inline]
    unsafe fn from_value(value: &glib::Value) -> ClockTime {
        skip_assert_initialized!();
        ClockTime(glib::gobject_ffi::g_value_get_uint64(
            value.to_glib_none().0,
        ))
    }
}

impl glib::value::ToValue for ClockTime {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<ClockTime>();
        let gct = self.into_glib();
        if gct == ffi::GST_CLOCK_TIME_NONE {
            crate::warning!(
                crate::CAT_RUST,
                "converting a defined `ClockTime` with value `GST_CLOCK_TIME_NONE` to `Value`, this is probably not what you wanted.",
            );
        }
        unsafe { glib::gobject_ffi::g_value_set_uint64(value.to_glib_none_mut().0, gct) }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl glib::value::ToValueOptional for ClockTime {
    #[inline]
    fn to_value_optional(opt: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<ClockTime>();
        let inner = opt.map(|inner| inner.0).unwrap_or(ffi::GST_CLOCK_TIME_NONE);
        unsafe { glib::gobject_ffi::g_value_set_uint64(value.to_glib_none_mut().0, inner) };

        value
    }
}

impl From<ClockTime> for glib::Value {
    #[inline]
    fn from(v: ClockTime) -> glib::Value {
        glib::value::ToValue::to_value(&v)
    }
}

#[doc(hidden)]
impl glib::StaticType for ClockTime {
    #[inline]
    fn static_type() -> glib::Type {
        <u64 as glib::StaticType>::static_type()
    }
}

impl glib::HasParamSpec for ClockTime {
    type ParamSpec = glib::ParamSpecUInt64;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecUInt64Builder;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

#[derive(Debug)]
pub struct DurationError;

impl fmt::Display for DurationError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "out of range conversion from Duration attempted")
    }
}

impl std::error::Error for DurationError {}

impl TryFrom<Duration> for ClockTime {
    type Error = DurationError;

    #[inline]
    fn try_from(d: Duration) -> Result<Self, Self::Error> {
        skip_assert_initialized!();

        let nanos = d.as_nanos();

        // Note: `std::u64::MAX` is `ClockTime::NONE`.
        if nanos >= std::u64::MAX as u128 {
            return Err(DurationError);
        }

        Ok(ClockTime::from_nseconds(nanos as u64))
    }
}

impl From<ClockTime> for Duration {
    #[inline]
    fn from(t: ClockTime) -> Self {
        skip_assert_initialized!();

        Duration::from_nanos(t.nseconds())
    }
}

impl_common_ops_for_newtype_uint!(ClockTime, u64);
impl_signed_div_mul!(ClockTime, u64);
impl_signed_int_into_signed!(ClockTime, u64);

// rustdoc-stripper-ignore-next
/// Tell [`pad_clocktime`] what kind of time we're formatting
enum Sign {
    // rustdoc-stripper-ignore-next
    /// An undefined time (`None`)
    Undefined,

    // rustdoc-stripper-ignore-next
    /// A non-negative time (zero or greater)
    NonNegative,

    // For a future ClockTimeDiff formatting
    #[allow(dead_code)]
    // rustdoc-stripper-ignore-next
    /// A negative time (below zero)
    Negative,
}

// Derived from libcore `Formatter::pad_integral` (same APACHE v2 + MIT licenses)
//
// TODO: Would be useful for formatting ClockTimeDiff
// if it was a new type instead of an alias for i64
//
// rustdoc-stripper-ignore-next
/// Performs the correct padding for a clock time which has already been
/// emitted into a str, as by [`write_clocktime`]. The str should *not*
/// contain the sign; that will be added by this method.
fn pad_clocktime(f: &mut fmt::Formatter<'_>, sign: Sign, buf: &str) -> fmt::Result {
    skip_assert_initialized!();
    use std::fmt::{Alignment, Write};

    use self::Sign::*;

    // Start by determining how we're padding, gathering
    // settings from the Formatter and the Sign

    // Choose the fill character
    let sign_aware_zero_pad = f.sign_aware_zero_pad();
    let fill_char = match sign {
        Undefined if sign_aware_zero_pad => '-', // Zero-padding an undefined time
        _ if sign_aware_zero_pad => '0',         // Zero-padding a valid time
        _ => f.fill(),                           // Otherwise, pad with the user-chosen character
    };

    // Choose the sign character
    let sign_plus = f.sign_plus();
    let sign_char = match sign {
        Undefined if sign_plus => Some(fill_char), // User requested sign, time is undefined
        NonNegative if sign_plus => Some('+'),     // User requested sign, time is zero or above
        Negative => Some('-'),                     // Time is below zero
        _ => None,                                 // Otherwise, add no sign
    };

    // Our minimum width is the value's width, plus 1 for the sign if present
    let width = buf.len() + sign_char.map_or(0, |_| 1);

    // Subtract the minimum width from the requested width to get the padding,
    // taking care not to allow wrapping due to underflow
    let padding = f.width().unwrap_or(0).saturating_sub(width);

    // Split the required padding into the three possible parts
    let align = f.align().unwrap_or(Alignment::Right);
    let (pre_padding, zero_padding, post_padding) = match align {
        _ if sign_aware_zero_pad => (0, padding, 0), // Zero-padding: Pad between sign and value
        Alignment::Left => (0, 0, padding),          // Align left: Pad on the right side
        Alignment::Right => (padding, 0, 0),         // Align right: Pad on the left side

        // Align center: Split equally between left and right side
        // If the required padding is odd, the right side gets one more char
        Alignment::Center => (padding / 2, 0, (padding + 1) / 2),
    };

    // And now for the actual writing

    for _ in 0..pre_padding {
        f.write_char(fill_char)?; // Left padding
    }
    if let Some(c) = sign_char {
        f.write_char(c)?; // ------- Sign character
    }
    for _ in 0..zero_padding {
        f.write_char(fill_char)?; // Padding between sign and value
    }
    f.write_str(buf)?; // ---------- Value
    for _ in 0..post_padding {
        f.write_char(fill_char)?; // Right padding
    }

    Ok(())
}

// rustdoc-stripper-ignore-next
/// Writes an unpadded, signless clocktime string with the given precision
fn write_clocktime<W: io::Write>(
    mut writer: W,
    clocktime: Option<ClockTime>,
    precision: usize,
) -> io::Result<()> {
    skip_assert_initialized!();
    let precision = std::cmp::min(9, precision);

    if let Some(ns) = clocktime.map(ClockTime::nseconds) {
        // Split the time into parts
        let (s, ns) = num_integer::div_rem(ns, 1_000_000_000);
        let (m, s) = num_integer::div_rem(s, 60);
        let (h, m) = num_integer::div_rem(m, 60);

        // Write HH:MM:SS
        write!(writer, "{h}:{m:02}:{s:02}")?;

        if precision > 0 {
            // Format the nanoseconds into a stack-allocated string
            // The value is zero-padded so always 9 digits long
            let mut buf = [0u8; 9];
            write!(&mut buf[..], "{ns:09}").unwrap();
            let buf_str = std::str::from_utf8(&buf[..]).unwrap();

            // Write decimal point and a prefix of the nanoseconds for more precision
            write!(writer, ".{buf_str:.precision$}")?;
        }
    } else {
        // Undefined time

        // Write HH:MM:SS, but invalid
        write!(writer, "--:--:--")?;

        if precision > 0 {
            // Write decimal point and dashes for more precision
            write!(writer, ".{:->p$}", "", p = precision)?;
        }
    }

    Ok(())
}

fn fmt_opt_clock_time(ct: Option<ClockTime>, f: &mut fmt::Formatter) -> fmt::Result {
    skip_assert_initialized!();
    let precision = f.precision().unwrap_or(9);

    // What the maximum time (u64::MAX - 1) would format to
    const MAX_SIZE: usize = "5124095:34:33.709551614".len();

    // Write the unpadded clocktime value into a stack-allocated string
    let mut buf = [0u8; MAX_SIZE];
    let mut cursor = io::Cursor::new(&mut buf[..]);
    write_clocktime(&mut cursor, ct, precision).unwrap();
    let pos = cursor.position() as usize;
    let buf_str = std::str::from_utf8(&buf[..pos]).unwrap();

    let sign = if ct.is_some() {
        Sign::NonNegative
    } else {
        Sign::Undefined
    };

    pad_clocktime(f, sign, buf_str)
}

impl fmt::Display for ClockTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_opt_clock_time(Some(*self), f)
    }
}

impl fmt::Debug for ClockTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

pub struct DisplayableOptClockTime(Option<ClockTime>);

impl fmt::Display for DisplayableOptClockTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_opt_clock_time(self.0, f)
    }
}

impl fmt::Debug for DisplayableOptClockTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl crate::utils::Displayable for Option<ClockTime> {
    type DisplayImpl = DisplayableOptClockTime;

    fn display(self) -> DisplayableOptClockTime {
        DisplayableOptClockTime(self)
    }
}

impl crate::utils::Displayable for ClockTime {
    type DisplayImpl = ClockTime;

    fn display(self) -> ClockTime {
        self
    }
}

impl std::iter::Sum for ClockTime {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        skip_assert_initialized!();
        iter.fold(ClockTime::ZERO, |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use opt_ops::prelude::*;

    use super::*;
    use crate::format::{Signed, UnsignedIntoSigned};

    const CT_1: ClockTime = ClockTime::from_nseconds(1);
    const CT_2: ClockTime = ClockTime::from_nseconds(2);
    const CT_3: ClockTime = ClockTime::from_nseconds(3);
    const CT_10: ClockTime = ClockTime::from_nseconds(10);
    const CT_20: ClockTime = ClockTime::from_nseconds(20);
    const CT_30: ClockTime = ClockTime::from_nseconds(30);

    const P_CT_0: Signed<ClockTime> = Signed::Positive(ClockTime::ZERO);
    const P_CT_NONE: Option<Signed<ClockTime>> = None;
    const P_CT_1: Signed<ClockTime> = Signed::Positive(ClockTime::from_nseconds(1));
    const P_CT_2: Signed<ClockTime> = Signed::Positive(ClockTime::from_nseconds(2));
    const P_CT_3: Signed<ClockTime> = Signed::Positive(ClockTime::from_nseconds(3));
    const N_CT_1: Signed<ClockTime> = Signed::Negative(ClockTime::from_nseconds(1));
    const N_CT_2: Signed<ClockTime> = Signed::Negative(ClockTime::from_nseconds(2));
    const N_CT_3: Signed<ClockTime> = Signed::Negative(ClockTime::from_nseconds(3));

    #[test]
    fn opt_time_clock() {
        assert_eq!(CT_1.into_glib(), 1);
        assert_eq!(Some(CT_1).into_glib(), 1);
        assert_eq!(ClockTime::NONE.into_glib(), ffi::GST_CLOCK_TIME_NONE);

        let ct_1_from: ClockTime = unsafe { try_from_glib(1u64) }.unwrap();
        assert_eq!(ct_1_from, CT_1);

        let opt_ct_some: Option<ClockTime> = unsafe { from_glib(1u64) };
        assert_eq!(opt_ct_some, Some(CT_1));

        let ct_none: Option<ClockTime> = unsafe { from_glib(ffi::GST_CLOCK_TIME_NONE) };
        assert_eq!(ct_none, None);
    }

    #[test]
    #[allow(clippy::eq_op, clippy::op_ref)]
    fn ops() {
        assert_eq!(CT_10 + CT_20, CT_30);
        assert_eq!(CT_30 - CT_20, CT_10);
        assert_eq!(CT_30 - CT_30, ClockTime::ZERO);
        assert_eq!(CT_10 * 3, CT_30);
        assert_eq!(3 * CT_10, CT_30);
        assert_eq!(CT_20 / 2, CT_10);
        assert_eq!(CT_20 / CT_2, 10);
        assert_eq!(CT_30.nseconds(), 30);

        assert_eq!(P_CT_1 + P_CT_2, P_CT_3);
        assert_eq!(P_CT_3 + N_CT_2, P_CT_1);
        assert_eq!(P_CT_2 + N_CT_3, N_CT_1);
        assert_eq!(N_CT_3 + P_CT_1, N_CT_2);
        assert_eq!(N_CT_2 + P_CT_3, P_CT_1);
        assert_eq!(N_CT_2 + N_CT_1, N_CT_3);

        assert_eq!(CT_1 + P_CT_2, P_CT_3);
        assert_eq!(P_CT_1 + CT_2, P_CT_3);
        assert_eq!(CT_3 + N_CT_1, P_CT_2);
        assert_eq!(N_CT_1 + CT_2, P_CT_1);

        assert_eq!(P_CT_3 - P_CT_2, P_CT_1);
        assert_eq!(P_CT_2 - P_CT_3, N_CT_1);
        assert_eq!(P_CT_2 - N_CT_1, P_CT_3);
        assert_eq!(N_CT_2 - P_CT_1, N_CT_3);
        assert_eq!(N_CT_3 - N_CT_1, N_CT_2);

        assert_eq!(CT_3 - P_CT_2, P_CT_1);
        assert_eq!(P_CT_3 - CT_2, P_CT_1);
        assert_eq!(N_CT_2 - CT_1, N_CT_3);
        assert_eq!(CT_2 - N_CT_1, P_CT_3);

        assert_eq!(P_CT_1 * 2i64, P_CT_2);
        assert_eq!(P_CT_1 * -2i64, N_CT_2);
        assert_eq!(N_CT_1 * 2i64, N_CT_2);
        assert_eq!(N_CT_1 * -2i64, P_CT_2);

        assert_eq!(2i64 * P_CT_1, P_CT_2);
        assert_eq!(-2i64 * P_CT_1, N_CT_2);

        assert_eq!(P_CT_1 * 2u64, P_CT_2);
        assert_eq!(N_CT_1 * 2u64, N_CT_2);

        assert_eq!(P_CT_2 / 2i64, P_CT_1);
        assert_eq!(P_CT_2 / -2i64, N_CT_1);
        assert_eq!(N_CT_2 / 2i64, N_CT_1);
        assert_eq!(N_CT_2 / -2i64, P_CT_1);

        assert_eq!(P_CT_2 / N_CT_2, Signed::Negative(1));

        assert_eq!(P_CT_2 / 2u64, P_CT_1);
        assert_eq!(N_CT_2 / 2u64, N_CT_1);

        assert_eq!(P_CT_3 % 2i64, P_CT_1);
        assert_eq!(P_CT_3 % -2i64, P_CT_1);
        assert_eq!(N_CT_3 % 2i64, N_CT_1);
        assert_eq!(N_CT_3 % -2i64, N_CT_1);

        assert_eq!(N_CT_3 % N_CT_2, N_CT_1);

        assert_eq!(P_CT_3 % 2u64, P_CT_1);
        assert_eq!(N_CT_3 % 2u64, N_CT_1);
    }

    #[test]
    fn checked_ops() {
        assert_eq!(CT_1.checked_add(CT_1), Some(CT_2));
        assert_eq!(P_CT_1.checked_add(P_CT_2), Some(P_CT_3));
        assert_eq!(P_CT_3.checked_add(N_CT_2), Some(P_CT_1));
        assert_eq!(P_CT_2.checked_add(N_CT_3), Some(N_CT_1));
        assert_eq!(N_CT_3.checked_add(P_CT_1), Some(N_CT_2));
        assert_eq!(N_CT_2.checked_add(P_CT_3), Some(P_CT_1));
        assert_eq!(N_CT_2.checked_add(N_CT_1), Some(N_CT_3));

        assert_eq!(CT_1.opt_checked_add(CT_1), Ok(Some(CT_2)));
        assert_eq!(CT_1.opt_checked_add(Some(CT_1)), Ok(Some(CT_2)));
        assert_eq!(Some(CT_1).opt_checked_add(Some(CT_1)), Ok(Some(CT_2)));
        assert_eq!(CT_1.opt_checked_add(ClockTime::NONE), Ok(None));
        assert_eq!(Some(CT_1).opt_checked_add(ClockTime::NONE), Ok(None));

        assert_eq!(CT_1.opt_checked_add(P_CT_1), Ok(Some(P_CT_2)));
        assert_eq!(N_CT_3.opt_checked_add(CT_1), Ok(Some(N_CT_2)));

        assert!(ClockTime::MAX.checked_add(CT_1).is_none());
        assert_eq!(
            ClockTime::MAX.opt_checked_add(Some(CT_1)),
            Err(opt_ops::Error::Overflow)
        );

        assert_eq!(P_CT_1.opt_checked_add(P_CT_1), Ok(Some(P_CT_2)));
        assert_eq!(P_CT_1.opt_checked_add(Some(N_CT_2)), Ok(Some(N_CT_1)));
        assert_eq!(Some(P_CT_1).opt_checked_add(Some(P_CT_1)), Ok(Some(P_CT_2)));
        assert_eq!(P_CT_1.opt_checked_add(ClockTime::NONE), Ok(None));
        assert_eq!(Some(N_CT_1).opt_checked_add(ClockTime::NONE), Ok(None));

        assert_eq!(
            ClockTime::MAX.into_positive().opt_checked_add(Some(P_CT_1)),
            Err(opt_ops::Error::Overflow)
        );

        assert_eq!(CT_2.checked_sub(CT_1), Some(CT_1));
        assert_eq!(P_CT_3.checked_sub(P_CT_2), Some(P_CT_1));
        assert_eq!(P_CT_2.checked_sub(P_CT_3), Some(N_CT_1));
        assert_eq!(P_CT_2.checked_sub(N_CT_1), Some(P_CT_3));
        assert_eq!(N_CT_2.checked_sub(P_CT_1), Some(N_CT_3));
        assert_eq!(N_CT_3.checked_sub(N_CT_1), Some(N_CT_2));
        assert_eq!(N_CT_2.checked_sub(N_CT_3), Some(P_CT_1));

        assert_eq!(CT_2.opt_checked_sub(CT_1), Ok(Some(CT_1)));
        assert_eq!(CT_2.opt_checked_sub(Some(CT_1)), Ok(Some(CT_1)));
        assert_eq!(Some(CT_2).opt_checked_sub(CT_1), Ok(Some(CT_1)));
        assert_eq!(Some(CT_2).opt_checked_sub(Some(CT_1)), Ok(Some(CT_1)));
        assert_eq!(CT_2.opt_checked_sub(ClockTime::NONE), Ok(None));
        assert_eq!(Some(CT_2).opt_checked_sub(ClockTime::NONE), Ok(None));

        assert_eq!(P_CT_2.opt_checked_sub(CT_1), Ok(Some(P_CT_1)));
        assert_eq!(N_CT_2.opt_checked_sub(CT_1), Ok(Some(N_CT_3)));

        assert!(CT_1.checked_sub(CT_2).is_none());
        assert_eq!(
            Some(CT_1).opt_checked_sub(CT_2),
            Err(opt_ops::Error::Overflow)
        );

        assert_eq!(P_CT_2.opt_checked_sub(Some(N_CT_1)), Ok(Some(P_CT_3)));
        assert_eq!(Some(N_CT_2).opt_checked_sub(P_CT_1), Ok(Some(N_CT_3)));

        assert_eq!(CT_1.checked_mul(2), Some(CT_2));
        assert_eq!(Some(CT_1).opt_checked_mul(2), Ok(Some(CT_2)));
        assert_eq!(1u64.opt_checked_mul(Some(CT_2)), Ok(Some(CT_2)));
        assert_eq!(P_CT_1.checked_mul(2), Some(P_CT_2));
        assert_eq!(P_CT_1.checked_mul(-2), Some(N_CT_2));
        assert_eq!(N_CT_1.checked_mul(2), Some(N_CT_2));
        assert_eq!(N_CT_1.checked_mul(-2), Some(P_CT_2));

        assert_eq!(Some(P_CT_1).opt_checked_mul(-2i64), Ok(Some(N_CT_2)));
        assert_eq!(N_CT_1.opt_checked_mul(2u64), Ok(Some(N_CT_2)));

        assert_eq!((-2i64).opt_checked_mul(Some(P_CT_1)), Ok(Some(N_CT_2)));

        assert_eq!(P_CT_1.checked_mul_unsigned(2u64), Some(P_CT_2));
        assert_eq!(N_CT_1.checked_mul_unsigned(2u64), Some(N_CT_2));

        assert_eq!(CT_3.checked_div(3), Some(CT_1));
        assert_eq!(P_CT_3.checked_div(3), Some(P_CT_1));
        assert_eq!(P_CT_3.checked_div(-3), Some(N_CT_1));
        assert_eq!(N_CT_3.checked_div(3), Some(N_CT_1));
        assert_eq!(N_CT_3.checked_div(-3), Some(P_CT_1));

        assert_eq!(Some(CT_3).opt_checked_div(CT_3), Ok(Some(1)));

        assert_eq!(Some(P_CT_3).opt_checked_div(-3i64), Ok(Some(N_CT_1)));
        assert_eq!(N_CT_3.opt_checked_div(3u64), Ok(Some(N_CT_1)));

        assert_eq!(P_CT_3.checked_div_unsigned(3u64), Some(P_CT_1));
        assert_eq!(N_CT_3.checked_div_unsigned(3u64), Some(N_CT_1));
    }

    #[test]
    fn overflowing_ops() {
        assert_eq!(CT_1.overflowing_add(CT_2), (CT_3, false));
        assert_eq!(CT_1.opt_overflowing_add(Some(CT_2)), Some((CT_3, false)));
        assert_eq!(Some(CT_1).opt_overflowing_add(CT_2), Some((CT_3, false)));
        assert_eq!(
            Some(CT_1).opt_overflowing_add(Some(CT_2)),
            Some((CT_3, false))
        );

        assert_eq!(ClockTime::NONE.opt_overflowing_add(CT_2), None);
        assert_eq!(CT_1.opt_overflowing_add(ClockTime::NONE), None);

        assert_eq!(
            ClockTime::MAX.overflowing_add(CT_1),
            (ClockTime::ZERO, true)
        );
        assert_eq!(
            Some(ClockTime::MAX).opt_overflowing_add(Some(CT_1)),
            Some((ClockTime::ZERO, true)),
        );

        assert_eq!(CT_3.overflowing_sub(CT_2), (CT_1, false));
        assert_eq!(CT_3.opt_overflowing_sub(Some(CT_2)), Some((CT_1, false)));
        assert_eq!(Some(CT_3).opt_overflowing_sub(CT_2), Some((CT_1, false)));
        assert_eq!(
            Some(CT_3).opt_overflowing_sub(Some(CT_2)),
            Some((CT_1, false))
        );
        assert_eq!(
            Some(CT_3).opt_overflowing_sub(&Some(CT_2)),
            Some((CT_1, false))
        );
        assert_eq!(ClockTime::NONE.opt_overflowing_sub(CT_2), None);
        assert_eq!(CT_2.opt_overflowing_sub(ClockTime::NONE), None);

        assert_eq!(CT_1.overflowing_sub(CT_2), (ClockTime::MAX, true));
        assert_eq!(
            Some(CT_1).opt_overflowing_sub(CT_2),
            Some((ClockTime::MAX, true))
        );
    }

    #[test]
    fn saturating_ops() {
        let p_ct_max: Signed<ClockTime> = ClockTime::MAX.into_positive();
        let n_ct_max: Signed<ClockTime> = ClockTime::MAX.into_negative();

        assert_eq!(CT_1.saturating_add(CT_2), CT_3);
        assert_eq!(P_CT_1.saturating_add(P_CT_2), P_CT_3);
        assert_eq!(P_CT_2.saturating_add(N_CT_3), N_CT_1);
        assert_eq!(P_CT_3.saturating_add(N_CT_2), P_CT_1);
        assert_eq!(N_CT_3.saturating_add(P_CT_1), N_CT_2);
        assert_eq!(N_CT_2.saturating_add(P_CT_3), P_CT_1);
        assert_eq!(N_CT_2.saturating_add(N_CT_1), N_CT_3);

        assert_eq!(CT_1.opt_saturating_add(Some(CT_2)), Some(CT_3));
        assert_eq!(Some(CT_1).opt_saturating_add(Some(CT_2)), Some(CT_3));
        assert_eq!(Some(CT_1).opt_saturating_add(ClockTime::NONE), None);

        assert_eq!(P_CT_1.opt_saturating_add(Some(CT_2)), Some(P_CT_3));
        assert_eq!(Some(CT_1).opt_saturating_add(P_CT_2), Some(P_CT_3));

        assert_eq!(ClockTime::MAX.saturating_add(CT_1), ClockTime::MAX);
        assert_eq!(
            Some(ClockTime::MAX).opt_saturating_add(Some(CT_1)),
            Some(ClockTime::MAX)
        );
        assert_eq!(p_ct_max.saturating_add(P_CT_1), p_ct_max);

        assert_eq!(CT_3.saturating_sub(CT_2), CT_1);
        assert_eq!(P_CT_3.saturating_sub(P_CT_2), P_CT_1);
        assert_eq!(P_CT_2.saturating_sub(P_CT_3), N_CT_1);
        assert_eq!(P_CT_2.saturating_sub(N_CT_1), P_CT_3);
        assert_eq!(N_CT_2.saturating_sub(P_CT_1), N_CT_3);
        assert_eq!(N_CT_3.saturating_sub(N_CT_1), N_CT_2);
        assert_eq!(N_CT_2.saturating_sub(N_CT_3), P_CT_1);

        assert_eq!(CT_3.opt_saturating_sub(Some(CT_2)), Some(CT_1));
        assert_eq!(Some(CT_3).opt_saturating_sub(Some(CT_2)), Some(CT_1));
        assert_eq!(Some(CT_3).opt_saturating_sub(ClockTime::NONE), None);

        assert_eq!(P_CT_2.opt_saturating_sub(Some(CT_3)), Some(N_CT_1));
        assert_eq!(Some(CT_3).opt_saturating_sub(P_CT_2), Some(P_CT_1));

        assert!(CT_1.saturating_sub(CT_2).is_zero());
        assert_eq!(P_CT_1.saturating_sub(P_CT_2), N_CT_1);
        assert_eq!(
            Some(CT_1).opt_saturating_sub(Some(CT_2)),
            Some(ClockTime::ZERO)
        );

        assert_eq!(CT_1.saturating_mul(2), CT_2);
        assert_eq!(ClockTime::MAX.saturating_mul(2), ClockTime::MAX);

        assert_eq!(P_CT_1.saturating_mul(2), P_CT_2);
        assert_eq!(P_CT_1.saturating_mul(-2), N_CT_2);
        assert_eq!(N_CT_1.saturating_mul(2), N_CT_2);
        assert_eq!(N_CT_1.saturating_mul(-2), P_CT_2);

        assert_eq!(Some(N_CT_1).opt_saturating_mul(-2i64), Some(P_CT_2));
        assert_eq!((-2i64).opt_saturating_mul(Some(N_CT_1)), Some(P_CT_2));

        assert_eq!(P_CT_1.saturating_mul_unsigned(2u64), P_CT_2);
        assert_eq!(N_CT_1.saturating_mul_unsigned(2u64), N_CT_2);

        assert_eq!(p_ct_max.saturating_mul(2), p_ct_max);
        assert_eq!(n_ct_max.saturating_mul(2), n_ct_max);

        assert_eq!(Some(2i64).opt_saturating_mul(p_ct_max), Some(p_ct_max));
        assert_eq!(2u64.opt_saturating_mul(Some(n_ct_max)), Some(n_ct_max));

        assert_eq!(p_ct_max.saturating_mul_unsigned(2u64), p_ct_max);
        assert_eq!(n_ct_max.saturating_mul_unsigned(2u64), n_ct_max);
    }

    #[test]
    fn wrapping_ops() {
        assert_eq!(CT_1.wrapping_add(CT_2), CT_3);
        assert_eq!(CT_1.opt_wrapping_add(CT_2), Some(CT_3));
        assert_eq!(Some(CT_1).opt_wrapping_add(CT_2), Some(CT_3));
        assert_eq!(Some(CT_1).opt_wrapping_add(Some(CT_2)), Some(CT_3));
        assert_eq!(Some(CT_1).opt_wrapping_add(None), None);

        assert_eq!(ClockTime::MAX.wrapping_add(CT_1), ClockTime::ZERO);
        assert_eq!(
            Some(ClockTime::MAX).opt_wrapping_add(Some(CT_1)),
            Some(ClockTime::ZERO)
        );

        assert_eq!(CT_3.wrapping_sub(CT_2), CT_1);
        assert_eq!(CT_3.opt_wrapping_sub(CT_2), Some(CT_1));
        assert_eq!(Some(CT_3).opt_wrapping_sub(CT_2), Some(CT_1));
        assert_eq!(Some(CT_3).opt_wrapping_sub(Some(CT_2)), Some(CT_1));
        assert_eq!(Some(CT_3).opt_wrapping_sub(ClockTime::NONE), None);

        assert_eq!(CT_1.wrapping_sub(CT_2), ClockTime::MAX);
        assert_eq!(
            Some(CT_1).opt_wrapping_sub(Some(CT_2)),
            Some(ClockTime::MAX)
        );
    }

    #[test]
    fn mul_div_ops() {
        use muldiv::MulDiv;

        assert_eq!(CT_1.mul_div_floor(7, 3), Some(CT_2));

        assert_eq!(P_CT_1.mul_div_floor(7u64, 3), Some(P_CT_2));
        assert_eq!(P_CT_1.mul_div_floor(-7i64, 3), Some(N_CT_2));
        assert_eq!(P_CT_1.mul_div_floor(7i64, -3), Some(N_CT_2));
        assert_eq!(P_CT_1.mul_div_floor(-7i64, -3), Some(P_CT_2));

        assert_eq!(N_CT_1.mul_div_floor(7u64, 3), Some(N_CT_2));
        assert_eq!(N_CT_1.mul_div_floor(-7i64, 3), Some(P_CT_2));
        assert_eq!(N_CT_1.mul_div_floor(7i64, -3), Some(P_CT_2));
        assert_eq!(N_CT_1.mul_div_floor(-7i64, -3), Some(N_CT_2));

        assert_eq!(CT_1.mul_div_round(10, 3), Some(CT_3));
        assert_eq!(CT_1.mul_div_round(8, 3), Some(CT_3));

        assert_eq!(P_CT_1.mul_div_round(10u64, 3), Some(P_CT_3));
        assert_eq!(P_CT_1.mul_div_round(8u64, 3), Some(P_CT_3));
        assert_eq!(P_CT_1.mul_div_round(-10i64, 3), Some(N_CT_3));
        assert_eq!(P_CT_1.mul_div_round(-8i64, 3), Some(N_CT_3));
        assert_eq!(P_CT_1.mul_div_round(10i64, -3), Some(N_CT_3));
        assert_eq!(P_CT_1.mul_div_round(-10i64, -3), Some(P_CT_3));

        assert_eq!(N_CT_1.mul_div_round(10u64, 3), Some(N_CT_3));
        assert_eq!(N_CT_1.mul_div_round(-10i64, 3), Some(P_CT_3));
        assert_eq!(N_CT_1.mul_div_round(10i64, -3), Some(P_CT_3));
        assert_eq!(N_CT_1.mul_div_round(-10i64, -3), Some(N_CT_3));

        assert_eq!(CT_1.mul_div_ceil(7, 3), Some(CT_3));

        assert_eq!(P_CT_1.mul_div_ceil(7u64, 3), Some(P_CT_3));
        assert_eq!(P_CT_1.mul_div_ceil(-7i64, 3), Some(N_CT_3));
        assert_eq!(P_CT_1.mul_div_ceil(7i64, -3), Some(N_CT_3));
        assert_eq!(P_CT_1.mul_div_ceil(-7i64, -3), Some(P_CT_3));

        assert_eq!(N_CT_1.mul_div_ceil(7u64, 3), Some(N_CT_3));
        assert_eq!(N_CT_1.mul_div_ceil(-7i64, 3), Some(P_CT_3));
        assert_eq!(N_CT_1.mul_div_ceil(7i64, -3), Some(P_CT_3));
        assert_eq!(N_CT_1.mul_div_ceil(-7i64, -3), Some(N_CT_3));
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn comp() {
        assert!(ClockTime::ZERO < CT_2);
        assert!(Some(ClockTime::ZERO) < Some(CT_2));
        assert!(CT_2 < CT_3);
        assert!(Some(CT_2) < Some(CT_3));
        assert!(ClockTime::ZERO < CT_3);
        assert!(Some(ClockTime::ZERO) < Some(CT_3));

        assert_eq!(CT_2, CT_2);
        assert_ne!(CT_3, CT_2);

        assert!(ClockTime::ZERO.into_positive() < P_CT_1);
        assert!(ClockTime::ZERO.into_positive() > N_CT_1);
        assert!(P_CT_1 < P_CT_2);
        assert!(P_CT_1 > N_CT_2);
        assert!(N_CT_1 < P_CT_2);
        assert!(N_CT_3 < N_CT_2);

        assert!(P_CT_1 < CT_2);
        assert!(CT_1 < P_CT_2);
        assert!(N_CT_2 < CT_1);
        assert!(CT_1 > N_CT_2);

        assert_eq!(CT_2, P_CT_2);
        assert_ne!(N_CT_3, CT_3);

        assert_eq!(Some(CT_2).opt_lt(Some(CT_3)), Some(true));
        assert_eq!(Some(CT_3).opt_lt(CT_2), Some(false));
        assert_eq!(Some(CT_2).opt_le(Some(CT_3)), Some(true));
        assert_eq!(Some(CT_3).opt_le(CT_3), Some(true));

        assert_eq!(Some(P_CT_2).opt_lt(Some(P_CT_3)), Some(true));
        assert_eq!(Some(P_CT_3).opt_lt(P_CT_2), Some(false));
        assert_eq!(Some(P_CT_2).opt_le(Some(P_CT_3)), Some(true));
        assert_eq!(Some(P_CT_3).opt_le(P_CT_3), Some(true));

        assert_eq!(Some(P_CT_0).opt_lt(P_CT_NONE), None);
        assert_eq!(P_CT_NONE.opt_lt(P_CT_0), None);

        assert_eq!(Some(N_CT_3).opt_lt(Some(N_CT_2)), Some(true));
        assert_eq!(Some(N_CT_2).opt_lt(N_CT_3), Some(false));
        assert_eq!(Some(N_CT_3).opt_le(Some(N_CT_2)), Some(true));
        assert_eq!(Some(N_CT_3).opt_le(N_CT_3), Some(true));

        assert_eq!(Some(P_CT_2).opt_lt(N_CT_3), Some(false));
        assert_eq!(Some(N_CT_3).opt_lt(Some(P_CT_2)), Some(true));

        assert!(CT_3 > CT_2);
        assert!(Some(CT_3) > Some(CT_2));
        assert!(CT_2 > ClockTime::ZERO);
        assert!(Some(CT_2) > Some(ClockTime::ZERO));
        assert!(CT_3 > ClockTime::ZERO);
        assert!(Some(CT_3) > Some(ClockTime::ZERO));

        assert!(!(ClockTime::NONE > None));
        // This doesn't work due to the `PartialOrd` impl on `Option<T>`
        //assert_eq!(Some(ClockTime::ZERO) > ClockTime::ZERO, false);
        assert!(!(Some(ClockTime::ZERO) < ClockTime::NONE));
        assert_eq!(Some(CT_3).opt_gt(Some(CT_2)), Some(true));
        assert_eq!(Some(CT_3).opt_ge(Some(CT_2)), Some(true));
        assert_eq!(Some(CT_3).opt_ge(CT_3), Some(true));

        assert_eq!(Some(P_CT_3).opt_gt(Some(P_CT_2)), Some(true));
        assert_eq!(Some(P_CT_3).opt_ge(Some(P_CT_2)), Some(true));
        assert_eq!(Some(P_CT_3).opt_ge(P_CT_3), Some(true));

        assert_eq!(Some(P_CT_0).opt_gt(P_CT_NONE), None);
        assert_eq!(P_CT_NONE.opt_gt(P_CT_0), None);

        assert_eq!(Some(N_CT_3).opt_gt(Some(N_CT_2)), Some(false));
        assert_eq!(Some(N_CT_3).opt_ge(Some(N_CT_2)), Some(false));
        assert_eq!(Some(N_CT_3).opt_ge(N_CT_3), Some(true));

        assert_eq!(Some(P_CT_2).opt_gt(N_CT_3), Some(true));
        assert_eq!(Some(N_CT_3).opt_gt(Some(P_CT_2)), Some(false));

        assert!(!(ClockTime::NONE < None));
        assert!(!(ClockTime::NONE > None));

        // This doesn't work due to the `PartialOrd` impl on `Option<T>`
        //assert!(Some(ClockTime::ZERO) > ClockTime::NONE, false);
        // Use opt_gt instead.
        assert_eq!(Some(ClockTime::ZERO).opt_gt(ClockTime::NONE), None);
        assert_eq!(ClockTime::ZERO.opt_gt(ClockTime::NONE), None);
        assert_eq!(ClockTime::ZERO.opt_ge(ClockTime::NONE), None);
        assert_eq!(ClockTime::NONE.opt_gt(Some(ClockTime::ZERO)), None);
        assert_eq!(ClockTime::NONE.opt_gt(ClockTime::ZERO), None);
        assert_eq!(ClockTime::NONE.opt_ge(ClockTime::ZERO), None);

        assert!(!(Some(ClockTime::ZERO) < ClockTime::NONE));
        assert_eq!(Some(ClockTime::ZERO).opt_lt(ClockTime::NONE), None);
        assert_eq!(Some(ClockTime::ZERO).opt_le(ClockTime::NONE), None);

        assert_eq!(CT_3.opt_min(CT_2), Some(CT_2));
        assert_eq!(CT_3.opt_min(Some(CT_2)), Some(CT_2));
        assert_eq!(Some(CT_3).opt_min(Some(CT_2)), Some(CT_2));
        assert_eq!(ClockTime::NONE.opt_min(Some(CT_2)), None);
        assert_eq!(Some(CT_3).opt_min(ClockTime::NONE), None);

        assert_eq!(P_CT_3.opt_min(P_CT_2), Some(P_CT_2));
        assert_eq!(P_CT_2.opt_min(P_CT_3), Some(P_CT_2));
        assert_eq!(N_CT_3.opt_min(N_CT_2), Some(N_CT_3));
        assert_eq!(N_CT_2.opt_min(N_CT_3), Some(N_CT_3));
        assert_eq!(P_CT_2.opt_min(N_CT_3), Some(N_CT_3));

        assert_eq!(CT_3.opt_max(CT_2), Some(CT_3));
        assert_eq!(CT_3.opt_max(Some(CT_2)), Some(CT_3));
        assert_eq!(Some(CT_3).opt_max(Some(CT_2)), Some(CT_3));
        assert_eq!(ClockTime::NONE.opt_max(Some(CT_2)), None);
        assert_eq!(Some(CT_3).opt_max(ClockTime::NONE), None);

        assert_eq!(P_CT_3.opt_max(P_CT_2), Some(P_CT_3));
        assert_eq!(P_CT_2.opt_max(P_CT_3), Some(P_CT_3));
        assert_eq!(N_CT_3.opt_max(N_CT_2), Some(N_CT_2));
        assert_eq!(N_CT_2.opt_max(N_CT_3), Some(N_CT_2));
        assert_eq!(P_CT_2.opt_max(N_CT_3), Some(P_CT_2));
    }

    #[test]
    fn display() {
        let none = Option::<ClockTime>::None;
        let some = Some(45_834_908_569_837 * ClockTime::NSECOND);
        let lots = ClockTime::from_nseconds(std::u64::MAX - 1);

        // Simple

        assert_eq!(format!("{:.0}", DisplayableOptClockTime(none)), "--:--:--");
        assert_eq!(
            format!("{:.3}", DisplayableOptClockTime(none)),
            "--:--:--.---"
        );
        assert_eq!(
            format!("{}", DisplayableOptClockTime(none)),
            "--:--:--.---------"
        );

        assert_eq!(format!("{:.0}", DisplayableOptClockTime(some)), "12:43:54");
        assert_eq!(
            format!("{:.3}", DisplayableOptClockTime(some)),
            "12:43:54.908"
        );
        assert_eq!(
            format!("{}", DisplayableOptClockTime(some)),
            "12:43:54.908569837"
        );

        assert_eq!(format!("{lots:.0}"), "5124095:34:33");
        assert_eq!(format!("{lots:.3}"), "5124095:34:33.709");
        assert_eq!(format!("{lots}"), "5124095:34:33.709551614");

        // Precision caps at 9
        assert_eq!(
            format!("{:.10}", DisplayableOptClockTime(none)),
            "--:--:--.---------"
        );
        assert_eq!(
            format!("{:.10}", DisplayableOptClockTime(some)),
            "12:43:54.908569837"
        );
        assert_eq!(format!("{lots:.10}"), "5124095:34:33.709551614");

        // Short width

        assert_eq!(format!("{:4.0}", DisplayableOptClockTime(none)), "--:--:--");
        assert_eq!(
            format!("{:4.3}", DisplayableOptClockTime(none)),
            "--:--:--.---"
        );
        assert_eq!(
            format!("{:4}", DisplayableOptClockTime(none)),
            "--:--:--.---------"
        );

        assert_eq!(format!("{:4.0}", DisplayableOptClockTime(some)), "12:43:54");
        assert_eq!(
            format!("{:4.3}", DisplayableOptClockTime(some)),
            "12:43:54.908"
        );
        assert_eq!(
            format!("{:4}", DisplayableOptClockTime(some)),
            "12:43:54.908569837"
        );

        assert_eq!(format!("{lots:4.0}"), "5124095:34:33");
        assert_eq!(format!("{lots:4.3}"), "5124095:34:33.709");
        assert_eq!(format!("{lots:4}"), "5124095:34:33.709551614");

        // Simple padding

        assert_eq!(
            format!("{:>9.0}", DisplayableOptClockTime(none)),
            " --:--:--"
        );
        assert_eq!(
            format!("{:<9.0}", DisplayableOptClockTime(none)),
            "--:--:-- "
        );
        assert_eq!(
            format!("{:^10.0}", DisplayableOptClockTime(none)),
            " --:--:-- "
        );
        assert_eq!(
            format!("{:>13.3}", DisplayableOptClockTime(none)),
            " --:--:--.---"
        );
        assert_eq!(
            format!("{:<13.3}", DisplayableOptClockTime(none)),
            "--:--:--.--- "
        );
        assert_eq!(
            format!("{:^14.3}", DisplayableOptClockTime(none)),
            " --:--:--.--- "
        );
        assert_eq!(
            format!("{:>19}", DisplayableOptClockTime(none)),
            " --:--:--.---------"
        );
        assert_eq!(
            format!("{:<19}", DisplayableOptClockTime(none)),
            "--:--:--.--------- "
        );
        assert_eq!(
            format!("{:^20}", DisplayableOptClockTime(none)),
            " --:--:--.--------- "
        );

        assert_eq!(
            format!("{:>9.0}", DisplayableOptClockTime(some)),
            " 12:43:54"
        );
        assert_eq!(
            format!("{:<9.0}", DisplayableOptClockTime(some)),
            "12:43:54 "
        );
        assert_eq!(
            format!("{:^10.0}", DisplayableOptClockTime(some)),
            " 12:43:54 "
        );
        assert_eq!(
            format!("{:>13.3}", DisplayableOptClockTime(some)),
            " 12:43:54.908"
        );
        assert_eq!(
            format!("{:<13.3}", DisplayableOptClockTime(some)),
            "12:43:54.908 "
        );
        assert_eq!(
            format!("{:^14.3}", DisplayableOptClockTime(some)),
            " 12:43:54.908 "
        );
        assert_eq!(
            format!("{:>19}", DisplayableOptClockTime(some)),
            " 12:43:54.908569837"
        );
        assert_eq!(
            format!("{:<19}", DisplayableOptClockTime(some)),
            "12:43:54.908569837 "
        );
        assert_eq!(
            format!("{:^20}", DisplayableOptClockTime(some)),
            " 12:43:54.908569837 "
        );

        assert_eq!(format!("{lots:>14.0}"), " 5124095:34:33");
        assert_eq!(format!("{lots:<14.0}"), "5124095:34:33 ");
        assert_eq!(format!("{lots:^15.0}"), " 5124095:34:33 ");
        assert_eq!(format!("{lots:>18.3}"), " 5124095:34:33.709");
        assert_eq!(format!("{lots:<18.3}"), "5124095:34:33.709 ");
        assert_eq!(format!("{lots:^19.3}"), " 5124095:34:33.709 ");
        assert_eq!(format!("{lots:>24}"), " 5124095:34:33.709551614");
        assert_eq!(format!("{lots:<24}"), "5124095:34:33.709551614 ");
        assert_eq!(format!("{lots:^25}"), " 5124095:34:33.709551614 ");

        // Padding with sign or zero-extension

        assert_eq!(
            format!("{:+11.0}", DisplayableOptClockTime(none)),
            "   --:--:--"
        );
        assert_eq!(
            format!("{:011.0}", DisplayableOptClockTime(none)),
            "-----:--:--"
        );
        assert_eq!(
            format!("{:+011.0}", DisplayableOptClockTime(none)),
            "-----:--:--"
        );
        assert_eq!(
            format!("{:+15.3}", DisplayableOptClockTime(none)),
            "   --:--:--.---"
        );
        assert_eq!(
            format!("{:015.3}", DisplayableOptClockTime(none)),
            "-----:--:--.---"
        );
        assert_eq!(
            format!("{:+015.3}", DisplayableOptClockTime(none)),
            "-----:--:--.---"
        );
        assert_eq!(
            format!("{:+21}", DisplayableOptClockTime(none)),
            "   --:--:--.---------"
        );
        assert_eq!(
            format!("{:021}", DisplayableOptClockTime(none)),
            "-----:--:--.---------"
        );
        assert_eq!(
            format!("{:+021}", DisplayableOptClockTime(none)),
            "-----:--:--.---------"
        );

        assert_eq!(
            format!("{:+11.0}", DisplayableOptClockTime(some)),
            "  +12:43:54"
        );
        assert_eq!(
            format!("{:011.0}", DisplayableOptClockTime(some)),
            "00012:43:54"
        );
        assert_eq!(
            format!("{:+011.0}", DisplayableOptClockTime(some)),
            "+0012:43:54"
        );
        assert_eq!(
            format!("{:+15.3}", DisplayableOptClockTime(some)),
            "  +12:43:54.908"
        );
        assert_eq!(
            format!("{:015.3}", DisplayableOptClockTime(some)),
            "00012:43:54.908"
        );
        assert_eq!(
            format!("{:+015.3}", DisplayableOptClockTime(some)),
            "+0012:43:54.908"
        );
        assert_eq!(
            format!("{:+21}", DisplayableOptClockTime(some)),
            "  +12:43:54.908569837"
        );
        assert_eq!(
            format!("{:021}", DisplayableOptClockTime(some)),
            "00012:43:54.908569837"
        );
        assert_eq!(
            format!("{:+021}", DisplayableOptClockTime(some)),
            "+0012:43:54.908569837"
        );

        assert_eq!(format!("{lots:+16.0}"), "  +5124095:34:33");
        assert_eq!(format!("{lots:016.0}"), "0005124095:34:33");
        assert_eq!(format!("{lots:+016.0}"), "+005124095:34:33");
        assert_eq!(format!("{lots:+20.3}"), "  +5124095:34:33.709");
        assert_eq!(format!("{lots:020.3}"), "0005124095:34:33.709");
        assert_eq!(format!("{lots:+020.3}"), "+005124095:34:33.709");
        assert_eq!(format!("{lots:+26}"), "  +5124095:34:33.709551614");
        assert_eq!(format!("{lots:026}"), "0005124095:34:33.709551614");
        assert_eq!(format!("{lots:+026}"), "+005124095:34:33.709551614");
    }

    #[test]
    fn iter_sum() {
        let s: ClockTime = vec![ClockTime::from_seconds(1), ClockTime::from_seconds(2)]
            .into_iter()
            .sum();
        assert_eq!(s, ClockTime::from_seconds(3));
    }

    #[test]
    #[should_panic]
    fn attempt_to_build_from_clock_time_none() {
        let _ = ClockTime::from_nseconds(ffi::GST_CLOCK_TIME_NONE);
    }

    #[test]
    #[should_panic]
    fn attempt_to_build_from_u64max() {
        let _ = ClockTime::from_nseconds(u64::MAX);
    }

    #[test]
    fn try_into_signed() {
        let time = crate::Signed::Positive(ClockTime::from_nseconds(0));
        assert_eq!(i64::try_from(time), Ok(0));

        let time = crate::Signed::Positive(ClockTime::from_nseconds(123));
        assert_eq!(i64::try_from(time), Ok(123));

        let time = crate::Signed::Positive(ClockTime::from_nseconds(u64::MAX - 1));
        assert!(i64::try_from(time).is_err());

        let time = crate::Signed::Positive(ClockTime::from_nseconds(u64::MAX >> 1));
        assert_eq!(i64::MAX as i128, (u64::MAX >> 1) as i128);
        assert_eq!(i64::try_from(time), Ok(i64::MAX));

        let time = crate::Signed::Negative(ClockTime::from_nseconds(0));
        assert_eq!(i64::try_from(time), Ok(0));

        let time = crate::Signed::Negative(ClockTime::from_nseconds(123));
        assert_eq!(i64::try_from(time), Ok(-123));

        let time = crate::Signed::Negative(ClockTime::from_nseconds(u64::MAX - 1));
        assert!(i64::try_from(time).is_err());

        let time = crate::Signed::Negative(ClockTime::from_nseconds(u64::MAX >> 1));
        assert_eq!(i64::MIN as i128 + 1, -((u64::MAX >> 1) as i128));
        assert_eq!(i64::try_from(time), Ok(i64::MIN + 1));

        let time = crate::Signed::Negative(ClockTime::from_nseconds((u64::MAX >> 1) + 1));
        assert_eq!(i64::MIN as i128, -(((u64::MAX >> 1) + 1) as i128));
        assert_eq!(i64::try_from(time), Ok(i64::MIN));
    }

    #[test]
    fn properties_macro_usage() {
        use super::ClockTime;
        use glib::{prelude::*, subclass::prelude::*, ParamSpecBuilderExt};
        use std::cell::Cell;

        #[derive(Default, glib::Properties)]
        #[properties(wrapper_type = TestObject)]
        pub struct TestObjectImp {
            #[property(get, set)]
            clock_time: Cell<ClockTime>,
            #[property(get, set)]
            optional_clock_time: Cell<Option<ClockTime>>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for TestObjectImp {
            const NAME: &'static str = "GstTestObject";
            type Type = TestObject;
        }

        impl ObjectImpl for TestObjectImp {
            fn properties() -> &'static [glib::ParamSpec] {
                Self::derived_properties()
            }

            fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
                self.derived_set_property(id, value, pspec);
            }

            fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
                self.derived_property(id, pspec)
            }
        }

        glib::wrapper! {
            pub struct TestObject(ObjectSubclass<TestObjectImp>);
        }

        let obj: TestObject = glib::Object::new();

        assert_eq!(obj.clock_time(), ClockTime::default());
        obj.set_clock_time(ClockTime::MAX);
        assert_eq!(obj.clock_time(), ClockTime::MAX);

        assert_eq!(obj.optional_clock_time(), None);
        obj.set_optional_clock_time(ClockTime::MAX);
        assert_eq!(obj.optional_clock_time(), Some(ClockTime::MAX));
    }

    #[test]
    fn seconds_float() {
        let res = ClockTime::ZERO;
        assert_eq!(res.seconds_f32(), 0.0);
        assert_eq!(res.seconds_f64(), 0.0);

        let res = ClockTime::from_nseconds(2_700_000_000);
        assert_eq!(res.seconds_f32(), 2.7);
        assert_eq!(res.seconds_f64(), 2.7);

        let res = ClockTime::MAX;
        assert_eq!(res.seconds_f32(), 18_446_744_073.709_553);
        assert_eq!(res.seconds_f64(), 18_446_744_073.709_553);
    }

    #[test]
    fn seconds_float_signed() {
        let pos = Signed::Positive(ClockTime::ZERO);
        assert_eq!(pos.seconds_f32(), 0.0);
        assert_eq!(pos.seconds_f64(), 0.0);
        let neg = Signed::Negative(ClockTime::ZERO);
        assert_eq!(neg.seconds_f32(), 0.0);
        assert_eq!(neg.seconds_f64(), 0.0);

        let pos = Signed::Positive(ClockTime::from_nseconds(2_700_000_000));
        assert_eq!(pos.seconds_f32(), 2.7);
        assert_eq!(pos.seconds_f64(), 2.7);
        let neg = Signed::Negative(ClockTime::from_nseconds(2_700_000_000));
        assert_eq!(neg.seconds_f32(), -2.7);
        assert_eq!(neg.seconds_f64(), -2.7);

        let pos = Signed::Positive(ClockTime::MAX);
        assert_eq!(pos.seconds_f32(), 18_446_744_073.709_553);
        assert_eq!(pos.seconds_f64(), 18_446_744_073.709_553);
        let neg = Signed::Negative(ClockTime::MAX);
        assert_eq!(neg.seconds_f32(), -18_446_744_073.709_553);
        assert_eq!(neg.seconds_f64(), -18_446_744_073.709_553);
    }

    #[test]
    fn try_from_seconds_f32() {
        let res = ClockTime::try_from_seconds_f32(0.0);
        assert_eq!(res, Ok(ClockTime::ZERO));
        let res = ClockTime::try_from_seconds_f32(1e-20);
        assert_eq!(res, Ok(ClockTime::ZERO));
        let res = ClockTime::try_from_seconds_f32(4.2e-7);
        assert_eq!(res, Ok(ClockTime::from_nseconds(420)));
        let res = ClockTime::try_from_seconds_f32(2.7);
        assert_eq!(res, Ok(ClockTime::from_nseconds(2_700_000_048)));
        // subnormal float:
        let res = ClockTime::try_from_seconds_f32(f32::from_bits(1));
        assert_eq!(res, Ok(ClockTime::ZERO));

        // the conversion uses rounding with tie resolution to even
        let res = ClockTime::try_from_seconds_f32(0.999e-9);
        assert_eq!(res, Ok(ClockTime::from_nseconds(1)));

        let res = ClockTime::try_from_seconds_f32(-5.0);
        assert!(res.is_err());
        let res = ClockTime::try_from_seconds_f32(f32::NAN);
        assert!(res.is_err());
        let res = ClockTime::try_from_seconds_f32(2e19);
        assert!(res.is_err());

        // this float represents exactly 976562.5e-9
        let val = f32::from_bits(0x3A80_0000);
        let res = ClockTime::try_from_seconds_f32(val);
        assert_eq!(res, Ok(ClockTime::from_nseconds(976_562)));

        // this float represents exactly 2929687.5e-9
        let val = f32::from_bits(0x3B40_0000);
        let res = ClockTime::try_from_seconds_f32(val);
        assert_eq!(res, Ok(ClockTime::from_nseconds(2_929_688)));

        // this float represents exactly 1.000_976_562_5
        let val = f32::from_bits(0x3F802000);
        let res = ClockTime::try_from_seconds_f32(val);
        assert_eq!(res, Ok(ClockTime::from_nseconds(1_000_976_562)));

        // this float represents exactly 1.002_929_687_5
        let val = f32::from_bits(0x3F806000);
        let res = ClockTime::try_from_seconds_f32(val);
        assert_eq!(res, Ok(ClockTime::from_nseconds(1_002_929_688)));
    }

    #[test]
    fn try_from_seconds_f64() {
        let res = ClockTime::try_from_seconds_f64(0.0);
        assert_eq!(res, Ok(ClockTime::ZERO));
        let res = ClockTime::try_from_seconds_f64(1e-20);
        assert_eq!(res, Ok(ClockTime::ZERO));
        let res = ClockTime::try_from_seconds_f64(4.2e-7);
        assert_eq!(res, Ok(ClockTime::from_nseconds(420)));
        let res = ClockTime::try_from_seconds_f64(2.7);
        assert_eq!(res, Ok(ClockTime::from_nseconds(2_700_000_000)));
        // subnormal float:
        let res = ClockTime::try_from_seconds_f64(f64::from_bits(1));
        assert_eq!(res, Ok(ClockTime::ZERO));

        // the conversion uses rounding with tie resolution to even
        let res = ClockTime::try_from_seconds_f64(0.999e-9);
        assert_eq!(res, Ok(ClockTime::from_nseconds(1)));
        let res = ClockTime::try_from_seconds_f64(0.999_999_999_499);
        assert_eq!(res, Ok(ClockTime::from_nseconds(999_999_999)));
        let res = ClockTime::try_from_seconds_f64(0.999_999_999_501);
        assert_eq!(res, Ok(ClockTime::from_seconds(1)));
        let res = ClockTime::try_from_seconds_f64(42.999_999_999_499);
        assert_eq!(res, Ok(ClockTime::from_nseconds(42_999_999_999)));
        let res = ClockTime::try_from_seconds_f64(42.999_999_999_501);
        assert_eq!(res, Ok(ClockTime::from_seconds(43)));

        let res = ClockTime::try_from_seconds_f64(-5.0);
        assert!(res.is_err());
        let res = ClockTime::try_from_seconds_f64(f64::NAN);
        assert!(res.is_err());
        let res = ClockTime::try_from_seconds_f64(2e19);
        assert!(res.is_err());

        // this float represents exactly 976562.5e-9
        let val = f64::from_bits(0x3F50_0000_0000_0000);
        let res = ClockTime::try_from_seconds_f64(val);
        assert_eq!(res, Ok(ClockTime::from_nseconds(976_562)));

        // this float represents exactly 2929687.5e-9
        let val = f64::from_bits(0x3F68_0000_0000_0000);
        let res = ClockTime::try_from_seconds_f64(val);
        assert_eq!(res, Ok(ClockTime::from_nseconds(2_929_688)));

        // this float represents exactly 1.000_976_562_5
        let val = f64::from_bits(0x3FF0_0400_0000_0000);
        let res = ClockTime::try_from_seconds_f64(val);
        assert_eq!(res, Ok(ClockTime::from_nseconds(1_000_976_562)));

        // this float represents exactly 1.002_929_687_5
        let val = f64::from_bits(0x3FF0_0C00_0000_0000);
        let res = ClockTime::try_from_seconds_f64(val);
        assert_eq!(res, Ok(ClockTime::from_nseconds(1_002_929_688)));
    }

    #[test]
    fn try_from_seconds_f32_signed() {
        let pos = Signed::<ClockTime>::from_seconds_f32(5.0);
        assert!(pos.is_positive());

        let neg = Signed::<ClockTime>::from_seconds_f32(-5.0);
        assert!(neg.is_negative());
    }

    #[test]
    fn try_from_seconds_f64_signed() {
        let pos = Signed::<ClockTime>::from_seconds_f64(5.0);
        assert!(pos.is_positive());

        let neg = Signed::<ClockTime>::from_seconds_f64(-5.0);
        assert!(neg.is_negative());
    }
}
