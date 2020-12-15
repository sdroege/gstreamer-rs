// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::time::Duration;
use std::{cmp, convert, fmt};

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

            pub fn zero() -> Self {
                Self(Some(0))
            }

            // FIXME `matches!` was introduced in rustc 1.42.0, current MSRV is 1.41.0
            // FIXME uncomment when CI can upgrade to 1.47.1
            //#[allow(clippy::match_like_matches_macro)]
            pub fn is_zero(&self) -> bool {
                match self.0 {
                    Some(0) => true,
                    _ => false,
                }
            }

            pub fn none() -> Self {
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

impl fmt::Display for ClockTime {
    #[allow(clippy::many_single_char_names)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let precision = f.precision().unwrap_or(9);
        // TODO: Could also check width and pad the hours as needed

        let (h, m, s, ns) = match self.0 {
            Some(v) => {
                let mut s = v / 1_000_000_000;
                let mut m = s / 60;
                let h = m / 60;
                s %= 60;
                m %= 60;
                let ns = v % 1_000_000_000;

                (h, m, s, ns)
            }
            None => (99, 99, 99, 999_999_999),
        };

        if precision == 0 {
            f.write_fmt(format_args!("{:02}:{:02}:{:02}", h, m, s))
        } else {
            let mut divisor = 1;
            let precision = cmp::min(precision, 9);
            for _ in 0..(9 - precision) {
                divisor *= 10;
            }

            f.write_fmt(format_args!(
                "{:02}:{:02}:{:02}.{:0width$}",
                h,
                m,
                s,
                ns / divisor,
                width = precision
            ))
        }
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
            .ok_or_else(|| glib::glib_bool_error!("Can't convert ClockTime::NONE to Duration"))
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
}
