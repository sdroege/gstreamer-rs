// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use glib::StaticType;
use num_integer::div_rem;
use std::io::{self, prelude::*};
use std::time::Duration;
use std::{cmp, convert, fmt, str};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct ClockTime(pub(crate) u64);

impl ClockTime {
    pub const SECOND: ClockTime = ClockTime(1_000_000_000);
    pub const MSECOND: ClockTime = ClockTime(1_000_000);
    pub const USECOND: ClockTime = ClockTime(1_000);
    pub const NSECOND: ClockTime = ClockTime(1);

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

    pub const fn from_seconds(seconds: u64) -> Self {
        skip_assert_initialized!();
        ClockTime(seconds * Self::SECOND.0)
    }

    pub const fn from_mseconds(mseconds: u64) -> Self {
        skip_assert_initialized!();
        ClockTime(mseconds * Self::MSECOND.0)
    }

    pub const fn from_useconds(useconds: u64) -> Self {
        skip_assert_initialized!();
        ClockTime(useconds * Self::USECOND.0)
    }

    pub const fn from_nseconds(nseconds: u64) -> Self {
        skip_assert_initialized!();
        ClockTime(nseconds * Self::NSECOND.0)
    }
}

macro_rules! option_glib_newtype_from_to {
    ($type_:ident, $none_value:expr) => {
        #[doc(hidden)]
        impl IntoGlib for $type_ {
            type GlibType = u64;

            fn into_glib(self) -> u64 {
                self.0
            }
        }

        #[doc(hidden)]
        impl OptionIntoGlib for $type_ {
            const GLIB_NONE: u64 = $none_value;
        }

        #[doc(hidden)]
        impl TryFromGlib<u64> for $type_ {
            type Error = GlibNoneError;
            #[inline]
            unsafe fn try_from_glib(val: u64) -> Result<Self, GlibNoneError> {
                skip_assert_initialized!();
                if val == $none_value {
                    return Err(GlibNoneError);
                }

                Ok($type_(val))
            }
        }
    };
}

option_glib_newtype_from_to!(ClockTime, ffi::GST_CLOCK_TIME_NONE);

impl glib::value::ValueType for ClockTime {
    type Type = Self;
}

pub struct ClockTimeValueTypeOrNoneChecker<T>(std::marker::PhantomData<T>);

unsafe impl<T: StaticType> glib::value::ValueTypeChecker for ClockTimeValueTypeOrNoneChecker<T> {
    type Error = glib::value::ValueTypeMismatchOrNoneError;

    fn check(value: &glib::Value) -> Result<(), Self::Error> {
        skip_assert_initialized!();
        glib::value::GenericValueTypeChecker::<T>::check(value)?;

        let gct = unsafe { glib::gobject_ffi::g_value_get_uint64(value.to_glib_none().0) };
        if gct == ffi::GST_CLOCK_TIME_NONE {
            return Err(glib::value::ValueTypeMismatchOrNoneError::UnexpectedNone);
        }

        Ok(())
    }
}

unsafe impl<'a> glib::value::FromValue<'a> for ClockTime {
    type Checker = ClockTimeValueTypeOrNoneChecker<Self>;

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
            crate::gst_warning!(
                crate::CAT_RUST,
                "converting a defined `ClockTime` with value `ffi::GST_CLOCK_TIME_NONE` to `Value`, this is probably not what you wanted.",
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

impl convert::TryFrom<Duration> for ClockTime {
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

impl convert::From<ClockTime> for Duration {
    fn from(t: ClockTime) -> Self {
        skip_assert_initialized!();

        Duration::from_nanos(t.nseconds())
    }
}

macro_rules! impl_common_ops_for_newtype_u64(
    ($name:ident) => {
        impl $name {
            pub const ZERO: Self = Self(0);
            pub const NONE: Option<Self> = None;

            pub const fn is_zero(self) -> bool {
                self.0 == Self::ZERO.0
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            // FIXME Can't use `map` in a `const fn` as of rustc 1.53.0-beta.2
            #[allow(clippy::manual_map)]
            pub const fn checked_add(self, rhs: Self) -> Option<Self> {
                match self.0.checked_add(rhs.0) {
                    Some(res) => Some(Self(res)),
                    None => None,
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn saturating_add(self, rhs: Self) -> Self {
                Self(self.0.saturating_add(rhs.0))
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn wrapping_add(self, rhs: Self) -> Self {
                Self(self.0.wrapping_add(rhs.0))
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            // FIXME Can't use `map` in a `const fn` as of rustc 1.53.0-beta.2
            #[allow(clippy::manual_map)]
            pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
                match self.0.checked_sub(rhs.0) {
                    Some(res) => Some(Self(res)),
                    None => None,
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn saturating_sub(self, rhs: Self) -> Self {
                Self(self.0.saturating_sub(rhs.0))
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn wrapping_sub(self, rhs: Self) -> Self {
                Self(self.0.wrapping_sub(rhs.0))
            }
        }
    };
);

impl_common_ops_for_newtype_u64!(ClockTime);

/// Tell [`pad_clocktime`] what kind of time we're formatting
enum Sign {
    /// An undefined time (`None`)
    Undefined,

    /// A non-negative time (zero or greater)
    NonNegative,

    // For a future ClockTimeDiff formatting
    #[allow(dead_code)]
    /// A negative time (below zero)
    Negative,
}

// Derived from libcore `Formatter::pad_integral` (same APACHE v2 + MIT licenses)
//
// TODO: Would be useful for formatting ClockTimeDiff
// if it was a new type instead of an alias for i64
//
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

#[derive(Debug)]
pub struct DisplayableOptClockTime(Option<ClockTime>);

impl fmt::Display for DisplayableOptClockTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_opt_clock_time(self.0, f)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opt_time_clock() {
        let ct_1 = ClockTime(1);
        let opt_ct_none: Option<ClockTime> = None;

        assert_eq!(ct_1.into_glib(), 1);
        assert_eq!(Some(ct_1).into_glib(), 1);
        assert_eq!(opt_ct_none.into_glib(), ffi::GST_CLOCK_TIME_NONE);

        let ct_1_from: ClockTime = unsafe { try_from_glib(1u64) }.unwrap();
        assert_eq!(ct_1_from, ct_1);

        let opt_ct_some: Option<ClockTime> = unsafe { from_glib(1u64) };
        assert_eq!(opt_ct_some, Some(ct_1));

        let opt_ct_none: Option<ClockTime> = unsafe { from_glib(ffi::GST_CLOCK_TIME_NONE) };
        assert_eq!(opt_ct_none, None);
    }

    #[test]
    #[allow(clippy::eq_op, clippy::op_ref)]
    fn ops() {
        let ct_10 = 10 * ClockTime::MSECOND;
        let ct_20 = 20 * ClockTime::MSECOND;
        let ct_30 = 30 * ClockTime::MSECOND;

        assert_eq!(ct_10 + ct_20, ct_30);
        assert_eq!(ct_10 + &ct_20, ct_30);
        assert_eq!(&ct_10 + &ct_20, ct_30);
        assert_eq!(ct_30 - ct_20, ct_10);
        assert_eq!(ct_30 - ct_30, ClockTime::ZERO);
        assert_eq!(ct_10 * 3, ct_30);
        assert_eq!(3 * ct_10, ct_30);
        assert_eq!(3 * &ct_10, ct_30);
        assert_eq!(ct_30.nseconds(), 30_000_000);
    }

    #[test]
    fn checked_ops() {
        let ct_1 = ClockTime::from_nseconds(1);
        let ct_2 = ClockTime::from_nseconds(2);

        let ct_max = ClockTime::from_nseconds(std::u64::MAX);

        assert_eq!(ct_1.checked_add(ct_1), Some(ct_2));
        assert_eq!(ct_1.checked_add(ct_1), Some(ct_2));
        assert!(ct_max.checked_add(ct_1).is_none());

        assert_eq!(ct_2.checked_sub(ct_1), Some(ct_1));
        assert_eq!(ct_2.checked_sub(ct_1), Some(ct_1));
        assert!(ct_1.checked_sub(ct_2).is_none());
    }

    #[test]
    fn saturating_ops() {
        let ct_1 = ClockTime::from_nseconds(1);
        let ct_2 = ClockTime::from_nseconds(2);
        let ct_3 = ClockTime::from_nseconds(3);

        let ct_max = ClockTime::from_nseconds(std::u64::MAX);

        assert_eq!(ct_1.saturating_add(ct_2), ct_3);
        assert_eq!(ct_1.saturating_add(ct_2), ct_3);
        assert_eq!(ct_max.saturating_add(ct_1), ct_max);

        assert_eq!(ct_3.saturating_sub(ct_2), ct_1);
        assert_eq!(ct_3.saturating_sub(ct_2), ct_1);
        assert!(ct_1.saturating_sub(ct_2).is_zero());
    }

    #[test]
    fn wrapping_ops() {
        let ct_1 = ClockTime::NSECOND;
        let ct_2 = 2 * ClockTime::NSECOND;
        let ct_3 = 3 * ClockTime::NSECOND;

        let ct_max = ClockTime::from_nseconds(std::u64::MAX);

        assert_eq!(ct_1.wrapping_add(ct_2), ct_3);
        assert_eq!(ct_1.wrapping_add(ct_2), ct_3);
        assert_eq!(ct_max.wrapping_add(ct_1), ClockTime::ZERO);

        assert_eq!(ct_3.wrapping_sub(ct_2), ct_1);
        assert_eq!(ct_3.wrapping_sub(ct_2), ct_1);
        assert_eq!(ct_1.wrapping_sub(ct_2), ct_max);
    }

    #[test]
    fn comp() {
        let ct_0 = ClockTime::ZERO;
        let ct_2 = 2 * ClockTime::NSECOND;
        let ct_3 = 3 * ClockTime::NSECOND;
        let opt_ct_none: Option<ClockTime> = None;

        assert!(ct_0 < ct_2);
        assert!(Some(ct_0) < Some(ct_2));
        assert!(ct_2 < ct_3);
        assert!(Some(ct_2) < Some(ct_3));
        assert!(ct_0 < ct_3);
        assert!(Some(ct_0) < Some(ct_3));

        assert!(ct_3 > ct_2);
        assert!(Some(ct_3) > Some(ct_2));
        assert!(ct_2 > ct_0);
        assert!(Some(ct_2) > Some(ct_0));
        assert!(ct_3 > ct_0);
        assert!(Some(ct_3) > Some(ct_0));

        assert!(!(opt_ct_none < None));
        assert!(!(opt_ct_none > None));
        // This doesn't work due to the `PartialOrd` impl on `Option<T>`
        //assert_eq!(Some(ct_0) > opt_ct_none, false);
        assert!(!(Some(ct_0) < opt_ct_none));
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
}
