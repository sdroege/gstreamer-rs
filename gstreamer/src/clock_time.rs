// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use glib::StaticType;
use muldiv::MulDiv;
use num_integer::div_rem;
use opt_ops::prelude::*;
use std::borrow::Borrow;
use std::io::{self, prelude::*};
use std::ops;
use std::time::Duration;
use std::{cmp, fmt, str};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub struct ClockTime(pub(crate) u64);

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

    pub const fn hours(self) -> u64 {
        self.0 / Self::SECOND.0 / 60 / 60
    }

    pub const fn minutes(self) -> u64 {
        self.0 / Self::SECOND.0 / 60
    }

    pub const fn seconds(self) -> u64 {
        self.0 / Self::SECOND.0
    }

    pub const fn mseconds(self) -> u64 {
        self.0 / Self::MSECOND.0
    }

    pub const fn useconds(self) -> u64 {
        self.0 / Self::USECOND.0
    }

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
    pub const fn from_seconds(seconds: u64) -> Self {
        skip_assert_initialized!();
        // `Option::expect` is not `const` as of rustc 1.63.0.
        ClockTime(match seconds.checked_mul(Self::SECOND.0) {
            Some(res) => res,
            None => panic!("Out of `ClockTime` range"),
        })
    }

    // rustdoc-stripper-ignore-next
    /// Builds a new `ClockTime` which value is the given number of milliseconds.
    ///
    /// # Panics
    ///
    /// Panics if the resulting duration in nanoseconds exceeds the `u64` range.
    #[track_caller]
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
    pub const fn from_nseconds(nseconds: u64) -> Self {
        skip_assert_initialized!();
        assert!(
            nseconds != ffi::GST_CLOCK_TIME_NONE,
            "Attempt to build a `ClockTime` with value `GST_CLOCK_TIME_NONE`",
        );
        ClockTime(nseconds * Self::NSECOND.0)
    }
}

option_glib_newtype_from_to!(ClockTime, ffi::GST_CLOCK_TIME_NONE);

impl glib::value::ValueType for ClockTime {
    type Type = Self;
}

pub enum ClockTimeValueTypeOrNoneChecker {}

unsafe impl glib::value::ValueTypeChecker for ClockTimeValueTypeOrNoneChecker {
    type Error = glib::value::ValueTypeMismatchOrNoneError<glib::value::ValueTypeMismatchError>;

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

    unsafe fn from_value(value: &glib::Value) -> ClockTime {
        skip_assert_initialized!();
        ClockTime(glib::gobject_ffi::g_value_get_uint64(
            value.to_glib_none().0,
        ))
    }
}

impl glib::value::ToValue for ClockTime {
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

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl glib::value::ToValueOptional for ClockTime {
    fn to_value_optional(opt: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<ClockTime>();
        let inner = opt.map(|inner| inner.0).unwrap_or(ffi::GST_CLOCK_TIME_NONE);
        unsafe { glib::gobject_ffi::g_value_set_uint64(value.to_glib_none_mut().0, inner) };

        value
    }
}

#[doc(hidden)]
impl glib::StaticType for ClockTime {
    fn static_type() -> glib::Type {
        <u64 as glib::StaticType>::static_type()
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

    fn try_from(d: Duration) -> Result<Self, Self::Error> {
        skip_assert_initialized!();

        let nanos = d.as_nanos();

        // Note: `std::u64::MAX` is `ClockTime::None`.
        if nanos >= std::u64::MAX as u128 {
            return Err(DurationError);
        }

        Ok(ClockTime::from_nseconds(nanos as u64))
    }
}

impl From<ClockTime> for Duration {
    fn from(t: ClockTime) -> Self {
        skip_assert_initialized!();

        Duration::from_nanos(t.nseconds())
    }
}

impl_common_ops_for_newtype_uint!(ClockTime, u64);
impl_signed_div_mul!(ClockTime, u64);

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
    use self::Sign::*;
    use std::fmt::{Alignment, Write};

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
    let precision = cmp::min(9, precision);

    if let Some(ns) = clocktime.map(ClockTime::nseconds) {
        // Split the time into parts
        let (s, ns) = div_rem(ns, 1_000_000_000);
        let (m, s) = div_rem(s, 60);
        let (h, m) = div_rem(m, 60);

        // Write HH:MM:SS
        write!(writer, "{}:{:02}:{:02}", h, m, s)?;

        if precision > 0 {
            // Format the nanoseconds into a stack-allocated string
            // The value is zero-padded so always 9 digits long
            let mut buf = [0u8; 9];
            write!(&mut buf[..], "{:09}", ns).unwrap();
            let buf_str = str::from_utf8(&buf[..]).unwrap();

            // Write decimal point and a prefix of the nanoseconds for more precision
            write!(writer, ".{:.p$}", buf_str, p = precision)?;
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
    let buf_str = str::from_utf8(&buf[..pos]).unwrap();

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
    use super::*;
    use crate::{Signed, UnsignedIntoSigned};

    const CT_1: ClockTime = ClockTime::from_nseconds(1);
    const CT_2: ClockTime = ClockTime::from_nseconds(2);
    const CT_3: ClockTime = ClockTime::from_nseconds(3);
    const CT_10: ClockTime = ClockTime::from_nseconds(10);
    const CT_20: ClockTime = ClockTime::from_nseconds(20);
    const CT_30: ClockTime = ClockTime::from_nseconds(30);

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
        assert_eq!(CT_10 + &CT_20, CT_30);
        assert_eq!(&CT_10 + &CT_20, CT_30);
        assert_eq!(CT_30 - CT_20, CT_10);
        assert_eq!(CT_30 - CT_30, ClockTime::ZERO);
        assert_eq!(CT_10 * 3, CT_30);
        assert_eq!(3 * CT_10, CT_30);
        assert_eq!(3 * &CT_10, CT_30);
        assert_eq!(CT_30.nseconds(), 30);

        assert_eq!(P_CT_1 + P_CT_2, P_CT_3);
        assert_eq!(P_CT_3 + N_CT_2, P_CT_1);
        assert_eq!(P_CT_2 + N_CT_3, N_CT_1);
        assert_eq!(N_CT_3 + P_CT_1, N_CT_2);
        assert_eq!(N_CT_2 + P_CT_3, P_CT_1);
        assert_eq!(N_CT_2 + N_CT_1, N_CT_3);

        assert_eq!(P_CT_3 - P_CT_2, P_CT_1);
        assert_eq!(P_CT_2 - P_CT_3, N_CT_1);
        assert_eq!(P_CT_2 - N_CT_1, P_CT_3);
        assert_eq!(N_CT_2 - P_CT_1, N_CT_3);
        assert_eq!(N_CT_3 - N_CT_1, N_CT_2);

        assert_eq!(P_CT_1 * 2i64, P_CT_2);
        assert_eq!(P_CT_1 * -2i64, N_CT_2);
        assert_eq!(N_CT_1 * 2i64, N_CT_2);
        assert_eq!(N_CT_1 * -2i64, P_CT_2);

        assert_eq!(P_CT_1 * 2u64, P_CT_2);
        assert_eq!(N_CT_1 * 2u64, N_CT_2);

        assert_eq!(P_CT_2 / 2i64, P_CT_1);
        assert_eq!(P_CT_2 / -2i64, N_CT_1);
        assert_eq!(N_CT_2 / 2i64, N_CT_1);
        assert_eq!(N_CT_2 / -2i64, P_CT_1);

        assert_eq!(P_CT_2 / 2u64, P_CT_1);
        assert_eq!(N_CT_2 / 2u64, N_CT_1);

        assert_eq!(P_CT_3 % 2i64, P_CT_1);
        assert_eq!(P_CT_3 % -2i64, P_CT_1);
        assert_eq!(N_CT_3 % 2i64, N_CT_1);
        assert_eq!(N_CT_3 % -2i64, N_CT_1);

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
        assert_eq!(CT_1.opt_checked_add(None), Ok(None));
        assert_eq!(Some(CT_1).opt_checked_add(None), Ok(None));

        assert!(ClockTime::MAX.checked_add(CT_1).is_none());
        assert_eq!(
            ClockTime::MAX.opt_checked_add(Some(CT_1)),
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
        assert_eq!(CT_2.opt_checked_sub(None), Ok(None));
        assert_eq!(Some(CT_2).opt_checked_sub(None), Ok(None));

        assert!(CT_1.checked_sub(CT_2).is_none());
        assert_eq!(
            Some(CT_1).opt_checked_sub(CT_2),
            Err(opt_ops::Error::Overflow)
        );

        assert_eq!(CT_1.checked_mul(2), Some(CT_2));
        assert_eq!(P_CT_1.checked_mul(2), Some(P_CT_2));
        assert_eq!(P_CT_1.checked_mul(-2), Some(N_CT_2));
        assert_eq!(N_CT_1.checked_mul(2), Some(N_CT_2));
        assert_eq!(N_CT_1.checked_mul(-2), Some(P_CT_2));

        assert_eq!(P_CT_1.checked_mul_unsigned(2u64), Some(P_CT_2));
        assert_eq!(N_CT_1.checked_mul_unsigned(2u64), Some(N_CT_2));

        assert_eq!(CT_3.checked_div(3), Some(CT_1));
        assert_eq!(P_CT_3.checked_div(3), Some(P_CT_1));
        assert_eq!(P_CT_3.checked_div(-3), Some(N_CT_1));
        assert_eq!(N_CT_3.checked_div(3), Some(N_CT_1));
        assert_eq!(N_CT_3.checked_div(-3), Some(P_CT_1));

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
        assert_eq!(Some(CT_1).opt_saturating_add(None), None);

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
        assert_eq!(Some(CT_3).opt_saturating_sub(None), None);

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

        assert_eq!(P_CT_1.saturating_mul_unsigned(2u64), P_CT_2);
        assert_eq!(N_CT_1.saturating_mul_unsigned(2u64), N_CT_2);

        assert_eq!(p_ct_max.saturating_mul(2), p_ct_max);
        assert_eq!(n_ct_max.saturating_mul(2), n_ct_max);

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
        assert_eq!(Some(CT_3).opt_wrapping_sub(None), None);

        assert_eq!(CT_1.wrapping_sub(CT_2), ClockTime::MAX);
        assert_eq!(
            Some(CT_1).opt_wrapping_sub(Some(CT_2)),
            Some(ClockTime::MAX)
        );
    }

    #[test]
    fn mul_div_ops() {
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

        assert!(ClockTime::ZERO.into_positive() < P_CT_1);
        assert!(ClockTime::ZERO.into_positive() > N_CT_1);
        assert!(P_CT_1 < P_CT_2);
        assert!(P_CT_1 > N_CT_2);
        assert!(N_CT_1 < P_CT_2);
        assert!(N_CT_3 < N_CT_2);

        assert_eq!(Some(CT_2).opt_lt(Some(CT_3)), Some(true));
        assert_eq!(Some(CT_3).opt_lt(CT_2), Some(false));
        assert_eq!(Some(CT_2).opt_le(Some(CT_3)), Some(true));
        assert_eq!(Some(CT_3).opt_le(CT_3), Some(true));

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

        assert_eq!(CT_3.opt_max(CT_2), Some(CT_3));
        assert_eq!(CT_3.opt_max(Some(CT_2)), Some(CT_3));
        assert_eq!(Some(CT_3).opt_max(Some(CT_2)), Some(CT_3));
        assert_eq!(ClockTime::NONE.opt_max(Some(CT_2)), None);
        assert_eq!(Some(CT_3).opt_max(ClockTime::NONE), None);
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

        assert_eq!(format!("{:.0}", lots), "5124095:34:33");
        assert_eq!(format!("{:.3}", lots), "5124095:34:33.709");
        assert_eq!(format!("{}", lots), "5124095:34:33.709551614");

        // Precision caps at 9
        assert_eq!(
            format!("{:.10}", DisplayableOptClockTime(none)),
            "--:--:--.---------"
        );
        assert_eq!(
            format!("{:.10}", DisplayableOptClockTime(some)),
            "12:43:54.908569837"
        );
        assert_eq!(format!("{:.10}", lots), "5124095:34:33.709551614");

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

        assert_eq!(format!("{:4.0}", lots), "5124095:34:33");
        assert_eq!(format!("{:4.3}", lots), "5124095:34:33.709");
        assert_eq!(format!("{:4}", lots), "5124095:34:33.709551614");

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

        assert_eq!(format!("{:>14.0}", lots), " 5124095:34:33");
        assert_eq!(format!("{:<14.0}", lots), "5124095:34:33 ");
        assert_eq!(format!("{:^15.0}", lots), " 5124095:34:33 ");
        assert_eq!(format!("{:>18.3}", lots), " 5124095:34:33.709");
        assert_eq!(format!("{:<18.3}", lots), "5124095:34:33.709 ");
        assert_eq!(format!("{:^19.3}", lots), " 5124095:34:33.709 ");
        assert_eq!(format!("{:>24}", lots), " 5124095:34:33.709551614");
        assert_eq!(format!("{:<24}", lots), "5124095:34:33.709551614 ");
        assert_eq!(format!("{:^25}", lots), " 5124095:34:33.709551614 ");

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

        assert_eq!(format!("{:+16.0}", lots), "  +5124095:34:33");
        assert_eq!(format!("{:016.0}", lots), "0005124095:34:33");
        assert_eq!(format!("{:+016.0}", lots), "+005124095:34:33");
        assert_eq!(format!("{:+20.3}", lots), "  +5124095:34:33.709");
        assert_eq!(format!("{:020.3}", lots), "0005124095:34:33.709");
        assert_eq!(format!("{:+020.3}", lots), "+005124095:34:33.709");
        assert_eq!(format!("{:+26}", lots), "  +5124095:34:33.709551614");
        assert_eq!(format!("{:026}", lots), "0005124095:34:33.709551614");
        assert_eq!(format!("{:+026}", lots), "+005124095:34:33.709551614");
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
}
