// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use num_integer::div_rem;
use std::io::{self, prelude::*};
use std::time::Duration;
use std::{cmp, convert, fmt, str};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Default)]
pub struct ClockTime(pub Option<u64>);

impl ClockTime {
    pub fn hours(&self) -> Option<u64> {
        (*self / crate::SECOND / 60 / 60).0
    }

    pub fn minutes(&self) -> Option<u64> {
        (*self / crate::SECOND / 60).0
    }

    pub fn seconds(&self) -> Option<u64> {
        (*self / crate::SECOND).0
    }

    pub fn mseconds(&self) -> Option<u64> {
        (*self / crate::MSECOND).0
    }

    pub fn useconds(&self) -> Option<u64> {
        (*self / crate::USECOND).0
    }

    pub fn nseconds(&self) -> Option<u64> {
        (*self / crate::NSECOND).0
    }

    pub fn nanoseconds(&self) -> Option<u64> {
        self.0
    }

    pub fn from_seconds(seconds: u64) -> ClockTime {
        skip_assert_initialized!();
        seconds * crate::SECOND
    }

    pub fn from_mseconds(mseconds: u64) -> ClockTime {
        skip_assert_initialized!();
        mseconds * crate::MSECOND
    }

    pub fn from_useconds(useconds: u64) -> ClockTime {
        skip_assert_initialized!();
        useconds * crate::USECOND
    }

    pub fn from_nseconds(nseconds: u64) -> ClockTime {
        skip_assert_initialized!();
        nseconds * crate::NSECOND
    }
}

// This macro is also used by formats with an inner Option.
// It is defined here because the format module depends on ClockTime.
macro_rules! impl_common_ops_for_opt_int(
    ($name:ident) => {
        impl $name {
            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub fn saturating_add(self, rhs: Self) -> Option<Self> {
                match (self.0, rhs.0) {
                    (Some(this), Some(rhs)) => Some(Self(Some(this.saturating_add(rhs)))),
                    _ => None,
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub fn saturating_sub(self, rhs: Self) -> Option<Self> {
                match (self.0, rhs.0) {
                    (Some(this), Some(rhs)) => Some(Self(Some(this.saturating_sub(rhs)))),
                    _ => None,
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub fn min(self, rhs: Self) -> Option<Self> {
                match (self.0, rhs.0) {
                    (Some(this), Some(rhs)) => Some(Self(Some(this.min(rhs)))),
                    _ => None,
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub fn max(self, rhs: Self) -> Option<Self> {
                match (self.0, rhs.0) {
                    (Some(this), Some(rhs)) => Some(Self(Some(this.max(rhs)))),
                    _ => None,
                }
            }

            pub const fn zero() -> Self {
                Self(Some(0))
            }

            pub fn is_zero(&self) -> bool {
                matches!(self.0, Some(0))
            }

            pub const fn none() -> Self {
                Self(None)
            }
        }

        impl cmp::PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
                match (self.0, other.0) {
                    (Some(this), Some(other)) => this.partial_cmp(&other),
                    (None, None) => Some(cmp::Ordering::Equal),
                    _ => None,
                }
            }
        }
    };
);

impl_common_ops_for_opt_int!(ClockTime);

/// Tell [`pad_clocktime`] what kind of time we're formatting
enum Sign {
    /// An invalid time (`None`)
    Invalid,

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
        Invalid if sign_aware_zero_pad => '-', // Zero-padding an invalid time
        _ if sign_aware_zero_pad => '0',       // Zero-padding a valid time
        _ => f.fill(),                         // Otherwise, pad with the user-chosen character
    };

    // Choose the sign character
    let sign_plus = f.sign_plus();
    let sign_char = match sign {
        Invalid if sign_plus => Some(fill_char), // User requested sign, time is invalid
        NonNegative if sign_plus => Some('+'),   // User requested sign, time is zero or above
        Negative => Some('-'),                   // Time is below zero
        _ => None,                               // Otherwise, add no sign
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
    clocktime: Option<u64>,
    precision: usize,
) -> io::Result<()> {
    skip_assert_initialized!();
    let precision = cmp::min(9, precision);

    if let Some(ns) = clocktime {
        // Valid time

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
        // Invalid time

        // Write HH:MM:SS, but invalid
        write!(writer, "--:--:--")?;

        if precision > 0 {
            // Write decimal point and dashes for more precision
            write!(writer, ".{:->p$}", "", p = precision)?;
        }
    }

    Ok(())
}

impl fmt::Display for ClockTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let precision = f.precision().unwrap_or(9);

        // What the maximum time (u64::MAX - 1) would format to
        const MAX_SIZE: usize = "5124095:34:33.709551614".len();

        // Write the unpadded clocktime value into a stack-allocated string
        let mut buf = [0u8; MAX_SIZE];
        let mut cursor = io::Cursor::new(&mut buf[..]);
        write_clocktime(&mut cursor, self.0, precision).unwrap();
        let pos = cursor.position() as usize;
        let buf_str = str::from_utf8(&buf[..pos]).unwrap();

        let sign = if self.0.is_some() {
            Sign::NonNegative
        } else {
            Sign::Invalid
        };

        pad_clocktime(f, sign, buf_str)
    }
}

#[doc(hidden)]
impl ToGlib for ClockTime {
    type GlibType = ffi::GstClockTime;

    fn to_glib(&self) -> ffi::GstClockTime {
        match self.0 {
            None => ffi::GST_CLOCK_TIME_NONE,
            Some(v) => v,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstClockTime> for ClockTime {
    unsafe fn from_glib(value: ffi::GstClockTime) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_CLOCK_TIME_NONE => ClockTime(None),
            value => ClockTime(Some(value)),
        }
    }
}

#[doc(hidden)]
impl<'a> glib::value::FromValueOptional<'a> for ClockTime {
    unsafe fn from_value_optional(value: &'a glib::Value) -> Option<Self> {
        <u64 as glib::value::FromValueOptional>::from_value_optional(value)
            .map(|x| ClockTime::from_glib(x))
    }
}

#[doc(hidden)]
impl<'a> glib::value::FromValue<'a> for ClockTime {
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        ClockTime::from_glib(<u64 as glib::value::FromValue>::from_value(value))
    }
}

#[doc(hidden)]
impl glib::value::SetValue for ClockTime {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        <u64 as glib::value::SetValue>::set_value(value, &this.to_glib());
    }
}

#[doc(hidden)]
impl glib::StaticType for ClockTime {
    fn static_type() -> glib::Type {
        <u64 as glib::StaticType>::static_type()
    }
}

impl From<Duration> for ClockTime {
    fn from(d: Duration) -> Self {
        skip_assert_initialized!();

        let nanos = d.as_nanos();

        if nanos > std::u64::MAX as u128 {
            crate::CLOCK_TIME_NONE
        } else {
            ClockTime::from_nseconds(nanos as u64)
        }
    }
}

impl convert::TryFrom<ClockTime> for Duration {
    type Error = glib::BoolError;

    fn try_from(t: ClockTime) -> Result<Self, Self::Error> {
        skip_assert_initialized!();

        t.nanoseconds()
            .map(Duration::from_nanos)
            .ok_or_else(|| glib::bool_error!("Can't convert ClockTime::NONE to Duration"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::eq_op)]
    fn ops() {
        let ct_10 = ClockTime::from_mseconds(10);
        let ct_20 = ClockTime::from_mseconds(20);
        let ct_30 = ClockTime::from_mseconds(30);

        let ct_none = ClockTime::none();

        assert_eq!(ct_10 + ct_20, ct_30);
        assert_eq!(ct_10 + ct_none, ct_none);
        assert_eq!(ct_none + ct_10, ct_none);
        assert_eq!(ct_none + ct_none, ct_none);

        assert_eq!(ct_30 - ct_20, ct_10);
        assert_eq!(ct_30 - ct_30, ClockTime::zero());
        assert_eq!(ct_30 - ct_none, ct_none);
        assert_eq!(ct_none - ct_30, ct_none);
        assert_eq!(ct_none - ct_none, ct_none);
    }

    #[test]
    fn saturating_ops() {
        let ct_1 = ClockTime::from_nseconds(1);
        let ct_2 = ClockTime::from_nseconds(2);

        let ct_max = ClockTime::from_nseconds(std::u64::MAX);
        let ct_none = ClockTime::none();

        assert_eq!(ct_max.saturating_add(ct_1), Some(ct_max));
        assert!(ct_max.saturating_add(ct_none).is_none());
        assert!(ct_none.saturating_add(ct_max).is_none());

        assert!(ct_1.saturating_sub(ct_2).unwrap().is_zero());
        assert!(ct_1.saturating_sub(ct_none).is_none());
        assert!(ct_none.saturating_sub(ct_1).is_none());
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn eq() {
        let ct_10 = ClockTime::from_mseconds(10);
        let ct_10_2 = ClockTime::from_mseconds(10);
        let ct_10_3 = ClockTime::from_mseconds(10);
        let ct_20 = ClockTime::from_mseconds(20);

        let ct_none = ClockTime::none();
        let ct_none_2 = ClockTime::none();
        let ct_none_3 = ClockTime::none();

        // ## Eq

        // ### (a == b) and (a != b) are strict inverses
        assert!(ct_10 == ct_10_2);
        assert_ne!(ct_10 == ct_10_2, ct_10 != ct_10_2);

        assert!(ct_10 != ct_20);
        assert_ne!(ct_10 == ct_20, ct_10 != ct_20);

        assert!(ct_none == ct_none_2);
        assert_ne!(ct_none == ct_none_2, ct_none != ct_none_2);

        assert!(ct_10 != ct_none);
        assert_ne!(ct_10 == ct_none, ct_10 != ct_none);

        assert!(ct_none != ct_10);
        assert_ne!(ct_none == ct_10, ct_none != ct_10);

        // ### Reflexivity (a == a)
        assert!(ct_10 == ct_10);
        assert!(ct_none == ct_none);

        // ## PartialEq

        // ### Symmetric (a == b) => (b == a)
        assert!((ct_10 == ct_10_2) && (ct_10_2 == ct_10));
        assert!((ct_none == ct_none_2) && (ct_none_2 == ct_none));

        // ### Transitive (a == b) and (b == c) => (a == c)
        assert!((ct_10 == ct_10_2) && (ct_10_2 == ct_10_3) && (ct_10 == ct_10_3));
        assert!((ct_none == ct_none_2) && (ct_none_2 == ct_none_3) && (ct_none == ct_none_3));
    }

    #[test]
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    fn partial_ord() {
        let ct_10 = ClockTime::from_mseconds(10);
        let ct_20 = ClockTime::from_mseconds(20);
        let ct_30 = ClockTime::from_mseconds(30);

        let ct_none = ClockTime::none();

        // Special cases
        assert_eq!(ct_10 < ct_none, false);
        assert_eq!(ct_10 > ct_none, false);
        assert_eq!(ct_none < ct_10, false);
        assert_eq!(ct_none > ct_10, false);

        // Asymmetric a < b => !(a > b)
        // a < b => !(a > b)
        assert!((ct_10 < ct_20) && !(ct_10 > ct_20));
        // a > b => !(a < b)
        assert!((ct_20 > ct_10) && !(ct_20 < ct_10));

        // Transitive
        // a < b and b < c => a < c
        assert!((ct_10 < ct_20) && (ct_20 < ct_30) && (ct_10 < ct_30));
        // a > b and b > c => a > c
        assert!((ct_30 > ct_20) && (ct_20 > ct_10) && (ct_30 > ct_10));
    }

    #[test]
    fn not_ord() {
        let ct_10 = ClockTime::from_mseconds(10);
        let ct_20 = ClockTime::from_mseconds(20);
        let ct_none = ClockTime::none();

        // Total & Antisymmetric exactly one of a < b, a == b or a > b is true

        assert!((ct_10 < ct_20) ^ (ct_10 == ct_20) ^ (ct_10 > ct_20));

        // Not Ord due to:
        assert_eq!(
            (ct_10 < ct_none) ^ (ct_10 == ct_none) ^ (ct_10 > ct_none),
            false
        );
        assert_eq!(
            (ct_none < ct_10) ^ (ct_none == ct_10) ^ (ct_none > ct_10),
            false
        );
    }

    #[test]
    fn min_max() {
        let ct_10 = ClockTime::from_nseconds(10);
        let ct_20 = ClockTime::from_nseconds(20);
        let ct_none = ClockTime::none();

        assert_eq!(ct_10.min(ct_20).unwrap(), ct_10);
        assert_eq!(ct_20.min(ct_10).unwrap(), ct_10);
        assert!(ct_none.min(ct_10).is_none());
        assert!(ct_20.min(ct_none).is_none());

        assert_eq!(ct_10.max(ct_20).unwrap(), ct_20);
        assert_eq!(ct_20.max(ct_10).unwrap(), ct_20);
        assert!(ct_none.max(ct_10).is_none());
        assert!(ct_20.max(ct_none).is_none());
    }

    #[test]
    fn display() {
        let none = ClockTime::none();
        let some = ClockTime::from_nseconds(45834908569837);
        let lots = ClockTime::from_nseconds(std::u64::MAX - 1);

        // Simple

        assert_eq!(format!("{:.0}", none), "--:--:--");
        assert_eq!(format!("{:.3}", none), "--:--:--.---");
        assert_eq!(format!("{}", none), "--:--:--.---------");

        assert_eq!(format!("{:.0}", some), "12:43:54");
        assert_eq!(format!("{:.3}", some), "12:43:54.908");
        assert_eq!(format!("{}", some), "12:43:54.908569837");

        assert_eq!(format!("{:.0}", lots), "5124095:34:33");
        assert_eq!(format!("{:.3}", lots), "5124095:34:33.709");
        assert_eq!(format!("{}", lots), "5124095:34:33.709551614");

        // Precision caps at 9
        assert_eq!(format!("{:.10}", none), "--:--:--.---------");
        assert_eq!(format!("{:.10}", some), "12:43:54.908569837");
        assert_eq!(format!("{:.10}", lots), "5124095:34:33.709551614");

        // Short width

        assert_eq!(format!("{:4.0}", none), "--:--:--");
        assert_eq!(format!("{:4.3}", none), "--:--:--.---");
        assert_eq!(format!("{:4}", none), "--:--:--.---------");

        assert_eq!(format!("{:4.0}", some), "12:43:54");
        assert_eq!(format!("{:4.3}", some), "12:43:54.908");
        assert_eq!(format!("{:4}", some), "12:43:54.908569837");

        assert_eq!(format!("{:4.0}", lots), "5124095:34:33");
        assert_eq!(format!("{:4.3}", lots), "5124095:34:33.709");
        assert_eq!(format!("{:4}", lots), "5124095:34:33.709551614");

        // Simple padding

        assert_eq!(format!("{:>9.0}", none), " --:--:--");
        assert_eq!(format!("{:<9.0}", none), "--:--:-- ");
        assert_eq!(format!("{:^10.0}", none), " --:--:-- ");
        assert_eq!(format!("{:>13.3}", none), " --:--:--.---");
        assert_eq!(format!("{:<13.3}", none), "--:--:--.--- ");
        assert_eq!(format!("{:^14.3}", none), " --:--:--.--- ");
        assert_eq!(format!("{:>19}", none), " --:--:--.---------");
        assert_eq!(format!("{:<19}", none), "--:--:--.--------- ");
        assert_eq!(format!("{:^20}", none), " --:--:--.--------- ");

        assert_eq!(format!("{:>9.0}", some), " 12:43:54");
        assert_eq!(format!("{:<9.0}", some), "12:43:54 ");
        assert_eq!(format!("{:^10.0}", some), " 12:43:54 ");
        assert_eq!(format!("{:>13.3}", some), " 12:43:54.908");
        assert_eq!(format!("{:<13.3}", some), "12:43:54.908 ");
        assert_eq!(format!("{:^14.3}", some), " 12:43:54.908 ");
        assert_eq!(format!("{:>19}", some), " 12:43:54.908569837");
        assert_eq!(format!("{:<19}", some), "12:43:54.908569837 ");
        assert_eq!(format!("{:^20}", some), " 12:43:54.908569837 ");

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

        assert_eq!(format!("{:+11.0}", none), "   --:--:--");
        assert_eq!(format!("{:011.0}", none), "-----:--:--");
        assert_eq!(format!("{:+011.0}", none), "-----:--:--");
        assert_eq!(format!("{:+15.3}", none), "   --:--:--.---");
        assert_eq!(format!("{:015.3}", none), "-----:--:--.---");
        assert_eq!(format!("{:+015.3}", none), "-----:--:--.---");
        assert_eq!(format!("{:+21}", none), "   --:--:--.---------");
        assert_eq!(format!("{:021}", none), "-----:--:--.---------");
        assert_eq!(format!("{:+021}", none), "-----:--:--.---------");

        assert_eq!(format!("{:+11.0}", some), "  +12:43:54");
        assert_eq!(format!("{:011.0}", some), "00012:43:54");
        assert_eq!(format!("{:+011.0}", some), "+0012:43:54");
        assert_eq!(format!("{:+15.3}", some), "  +12:43:54.908");
        assert_eq!(format!("{:015.3}", some), "00012:43:54.908");
        assert_eq!(format!("{:+015.3}", some), "+0012:43:54.908");
        assert_eq!(format!("{:+21}", some), "  +12:43:54.908569837");
        assert_eq!(format!("{:021}", some), "00012:43:54.908569837");
        assert_eq!(format!("{:+021}", some), "+0012:43:54.908569837");

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
