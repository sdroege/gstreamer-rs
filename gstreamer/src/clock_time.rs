// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops;
use ffi;
use glib::translate::*;
use std::{cmp, fmt};
use muldiv::MulDiv;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct ClockTime(pub Option<u64>);

impl ClockTime {
    pub fn hours(&self) -> Option<u64> {
        (*self / ::SECOND / 60 / 60).0
    }

    pub fn minutes(&self) -> Option<u64> {
        (*self / ::SECOND / 60).0
    }

    pub fn seconds(&self) -> Option<u64> {
        (*self / ::SECOND).0
    }

    pub fn nanoseconds(&self) -> Option<u64> {
        self.0
    }

    pub fn from_seconds(seconds: u64) -> ClockTime {
        seconds * ::SECOND
    }

    pub fn new(nanoseconds: u64) -> ClockTime {
        ClockTime(Some(nanoseconds))
    }

    pub fn none() -> ClockTime {
        ClockTime(None)
    }
}

impl From<u64> for ClockTime {
    fn from(v: u64) -> ClockTime {
        from_glib(v)
    }
}

impl From<Option<u64>> for ClockTime {
    fn from(v: Option<u64>) -> ClockTime {
        ClockTime(v)
    }
}

impl Into<u64> for ClockTime {
    fn into(self) -> u64 {
        self.to_glib()
    }
}

impl Into<Option<u64>> for ClockTime {
    fn into(self) -> Option<u64> {
        self.0
    }
}

impl ops::Deref for ClockTime {
    type Target = Option<u64>;

    fn deref(&self) -> &Option<u64> {
        &self.0
    }
}

impl ops::DerefMut for ClockTime {
    fn deref_mut(&mut self) -> &mut Option<u64> {
        &mut self.0
    }
}

impl AsRef<Option<u64>> for ClockTime {
    fn as_ref(&self) -> &Option<u64> {
        &self.0
    }
}

impl AsMut<Option<u64>> for ClockTime {
    fn as_mut(&mut self) -> &mut Option<u64> {
        &mut self.0
    }
}

impl fmt::Display for ClockTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
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
            let precision = cmp::max(precision, 9);
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

macro_rules! impl_op_same(
    ($op:ident, $op_name:ident, $op_assign:ident, $op_assign_name:ident, $e:expr) => {
        impl ops::$op<ClockTime> for ClockTime {
            type Output = ClockTime;

            fn $op_name(self, other: ClockTime) -> ClockTime {
                match (self.0, other.0) {
                    (Some(a), Some(b)) => ClockTime(Some($e(a, b))),
                    _ => ClockTime(None),
                }
            }
        }

        impl<'a> ops::$op<&'a ClockTime> for ClockTime {
            type Output = ClockTime;

            fn $op_name(self, other: &'a ClockTime) -> ClockTime {
                self.$op_name(*other)
            }
        }

        impl ops::$op_assign<ClockTime> for ClockTime {
            fn $op_assign_name(&mut self, other: ClockTime) {
                match (self.0, other.0) {
                    (Some(a), Some(b)) => self.0 = Some($e(a, b)),
                    _ => self.0 = None,
                }
            }
        }

        impl<'a> ops::$op_assign<&'a ClockTime> for ClockTime {
            fn $op_assign_name(&mut self, other: &'a ClockTime) {
                self.$op_assign_name(*other)
            }
        }
    };
);

impl_op_same!(Add, add, AddAssign, add_assign, |a, b| a + b);
impl_op_same!(Sub, sub, SubAssign, sub_assign, |a, b| a - b);
impl_op_same!(Mul, mul, MulAssign, mul_assign, |a, b| a * b);
impl_op_same!(Div, div, DivAssign, div_assign, |a, b| a / b);
impl_op_same!(Rem, rem, RemAssign, rem_assign, |a, b| a % b);

macro_rules! impl_op_u64(
    ($op:ident, $op_name:ident, $op_assign:ident, $op_assign_name:ident, $e:expr) => {
        impl ops::$op<u64> for ClockTime {
            type Output = ClockTime;

            fn $op_name(self, other: u64) -> ClockTime {
                match self.0 {
                    Some(a) => ClockTime(Some($e(a, other))),
                    _ => ClockTime(None),
                }
            }
        }

        impl<'a> ops::$op<&'a u64> for ClockTime {
            type Output = ClockTime;

            fn $op_name(self, other: &'a u64) -> ClockTime {
                self.$op_name(*other)
            }
        }

        impl ops::$op_assign<u64> for ClockTime {
            fn $op_assign_name(&mut self, other: u64) {
                match self.0 {
                    Some(a) => self.0 = Some($e(a, other)),
                    _ => self.0 = None,
                }
            }
        }

        impl<'a> ops::$op_assign<&'a u64> for ClockTime {
            fn $op_assign_name(&mut self, other: &'a u64) {
                self.$op_assign_name(*other)
            }
        }
    };
);

impl_op_u64!(Mul, mul, MulAssign, mul_assign, |a, b| a * b);
impl_op_u64!(Div, div, DivAssign, div_assign, |a, b| a / b);
impl_op_u64!(Rem, rem, RemAssign, rem_assign, |a, b| a % b);

impl ops::Mul<ClockTime> for u64 {
    type Output = ClockTime;

    fn mul(self, other: ClockTime) -> ClockTime {
        other.mul(self)
    }
}

impl<'a> ops::Mul<&'a ClockTime> for u64 {
    type Output = ClockTime;

    fn mul(self, other: &'a ClockTime) -> ClockTime {
        other.mul(self)
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
    fn from_glib(value: ffi::GstClockTime) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_CLOCK_TIME_NONE => ClockTime(None),
            value => ClockTime(Some(value)),
        }
    }
}

impl MulDiv<ClockTime> for ClockTime {
    type Output = ClockTime;

    fn mul_div_floor(self, num: ClockTime, denom: ClockTime) -> Option<Self::Output> {
        match (self.0, num.0, denom.0) {
            (Some(s), Some(n), Some(d)) => s.mul_div_floor(n, d).map(ClockTime::new),
            _ => Some(ClockTime(None)),
        }
    }

    fn mul_div_round(self, num: ClockTime, denom: ClockTime) -> Option<Self::Output> {
        match (self.0, num.0, denom.0) {
            (Some(s), Some(n), Some(d)) => s.mul_div_round(n, d).map(ClockTime::new),
            _ => Some(ClockTime(None)),
        }
    }

    fn mul_div_ceil(self, num: ClockTime, denom: ClockTime) -> Option<Self::Output> {
        match (self.0, num.0, denom.0) {
            (Some(s), Some(n), Some(d)) => s.mul_div_ceil(n, d).map(ClockTime::new),
            _ => Some(ClockTime(None)),
        }
    }
}

impl<'a> MulDiv<&'a ClockTime> for ClockTime {
    type Output = ClockTime;

    fn mul_div_floor(self, num: &ClockTime, denom: &ClockTime) -> Option<Self::Output> {
        self.mul_div_floor(*num, *denom)
    }

    fn mul_div_round(self, num: &ClockTime, denom: &ClockTime) -> Option<Self::Output> {
        self.mul_div_round(*num, *denom)
    }

    fn mul_div_ceil(self, num: &ClockTime, denom: &ClockTime) -> Option<Self::Output> {
        self.mul_div_ceil(*num, *denom)
    }
}

impl<'a> MulDiv<u64> for ClockTime {
    type Output = ClockTime;

    fn mul_div_floor(self, num: u64, denom: u64) -> Option<Self::Output> {
        self.mul_div_floor(ClockTime::from(num), ClockTime::from(denom))
    }

    fn mul_div_round(self, num: u64, denom: u64) -> Option<Self::Output> {
        self.mul_div_round(ClockTime::from(num), ClockTime::from(denom))
    }

    fn mul_div_ceil(self, num: u64, denom: u64) -> Option<Self::Output> {
        self.mul_div_ceil(ClockTime::from(num), ClockTime::from(denom))
    }
}

impl<'a> MulDiv<&'a u64> for ClockTime {
    type Output = ClockTime;

    fn mul_div_floor(self, num: &u64, denom: &u64) -> Option<Self::Output> {
        self.mul_div_floor(*num, *denom)
    }

    fn mul_div_round(self, num: &u64, denom: &u64) -> Option<Self::Output> {
        self.mul_div_round(*num, *denom)
    }

    fn mul_div_ceil(self, num: &u64, denom: &u64) -> Option<Self::Output> {
        self.mul_div_ceil(*num, *denom)
    }
}
