// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use muldiv::MulDiv;
use std::convert::TryFrom;
use std::ops;
use thiserror::Error;
use ClockTime;
use Format;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
#[cfg_attr(feature = "ser_de", derive(Serialize, Deserialize))]
pub enum GenericFormattedValue {
    Undefined(Undefined),
    Default(Default),
    Bytes(Bytes),
    Time(ClockTime),
    Buffers(Buffers),
    Percent(Percent),
    Other(Format, i64),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Undefined(pub i64);
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Default(pub Option<u64>);
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Bytes(pub Option<u64>);
pub type Time = ClockTime;
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Buffers(pub Option<u64>);
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Percent(pub Option<u32>);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
#[error("invalid generic value format")]
pub struct TryFromGenericFormattedValueError(());

pub trait FormattedValue: Copy + Clone + Sized + Into<GenericFormattedValue> + 'static {
    fn get_default_format() -> Format;
    fn get_format(&self) -> Format;

    unsafe fn from_raw(format: Format, value: i64) -> Self;
    unsafe fn to_raw_value(&self) -> i64;
}

pub trait SpecificFormattedValue: FormattedValue + TryFrom<GenericFormattedValue> {}

impl FormattedValue for GenericFormattedValue {
    fn get_default_format() -> Format {
        Format::Undefined
    }

    fn get_format(&self) -> Format {
        self.get_format()
    }

    unsafe fn from_raw(format: Format, value: i64) -> Self {
        GenericFormattedValue::new(format, value)
    }

    unsafe fn to_raw_value(&self) -> i64 {
        self.get_value()
    }
}

impl GenericFormattedValue {
    pub fn new(format: Format, value: i64) -> Self {
        skip_assert_initialized!();
        match format {
            Format::Undefined => GenericFormattedValue::Undefined(Undefined(value)),
            Format::Default => GenericFormattedValue::Default(if value == -1 {
                Default(None)
            } else {
                Default(Some(value as u64))
            }),
            Format::Bytes => GenericFormattedValue::Bytes(if value == -1 {
                Bytes(None)
            } else {
                Bytes(Some(value as u64))
            }),
            Format::Time => GenericFormattedValue::Time(if value == -1 {
                ClockTime::none()
            } else {
                ClockTime::from_nseconds(value as u64)
            }),
            Format::Buffers => GenericFormattedValue::Buffers(if value == -1 {
                Buffers(None)
            } else {
                Buffers(Some(value as u64))
            }),
            Format::Percent => {
                GenericFormattedValue::Percent(unsafe { Percent::from_raw(format, value) })
            }
            Format::__Unknown(_) => GenericFormattedValue::Other(format, value),
        }
    }

    pub fn get_format(&self) -> Format {
        match *self {
            GenericFormattedValue::Undefined(_) => Format::Undefined,
            GenericFormattedValue::Default(_) => Format::Default,
            GenericFormattedValue::Bytes(_) => Format::Bytes,
            GenericFormattedValue::Time(_) => Format::Time,
            GenericFormattedValue::Buffers(_) => Format::Buffers,
            GenericFormattedValue::Percent(_) => Format::Percent,
            GenericFormattedValue::Other(f, _) => f,
        }
    }

    pub fn get_value(&self) -> i64 {
        match *self {
            GenericFormattedValue::Undefined(v) => v.0,
            GenericFormattedValue::Default(v) => v.map(|v| v as i64).unwrap_or(-1),
            GenericFormattedValue::Bytes(v) => v.map(|v| v as i64).unwrap_or(-1),
            GenericFormattedValue::Time(v) => v.map(|v| v as i64).unwrap_or(-1),
            GenericFormattedValue::Buffers(v) => v.map(|v| v as i64).unwrap_or(-1),
            GenericFormattedValue::Percent(v) => v.map(i64::from).unwrap_or(-1),
            GenericFormattedValue::Other(_, v) => v,
        }
    }
}

macro_rules! impl_op_same(
    ($name:ident, $op:ident, $op_name:ident, $op_assign:ident, $op_assign_name:ident, $e:expr) => {
        impl ops::$op<$name> for $name {
            type Output = $name;

            fn $op_name(self, other: $name) -> $name {
                match (self.0, other.0) {
                    (Some(a), Some(b)) => $name($e(a, b)),
                    _ => $name(None),
                }
            }
        }

        impl<'a> ops::$op<&'a $name> for $name {
            type Output = $name;

            fn $op_name(self, other: &'a $name) -> $name {
                self.$op_name(*other)
            }
        }

        impl<'a> ops::$op<$name> for &'a $name {
            type Output = $name;

            fn $op_name(self, other: $name) -> $name {
                (*self).$op_name(other)
            }
        }

        impl<'a, 'b> ops::$op<&'a $name> for &'b $name {
            type Output = $name;

            fn $op_name(self, other: &'a $name) -> $name {
                (*self).$op_name(*other)
            }
        }

        impl ops::$op_assign<$name> for $name {
            fn $op_assign_name(&mut self, other: $name) {
                match (self.0, other.0) {
                    (Some(a), Some(b)) => self.0 = $e(a, b),
                    _ => self.0 = None,
                }
            }
        }

        impl<'a> ops::$op_assign<&'a $name> for $name {
            fn $op_assign_name(&mut self, other: &'a $name) {
                self.$op_assign_name(*other)
            }
        }
    };
);

macro_rules! impl_op_u64(
    ($name:ident, $op:ident, $op_name:ident, $op_assign:ident, $op_assign_name:ident, $e:expr) => {
        impl ops::$op<u64> for $name {
            type Output = $name;

            fn $op_name(self, other: u64) -> $name {
                match self.0 {
                    Some(a) => $name($e(a, other)),
                    _ => $name(None),
                }
            }
        }

        impl<'a> ops::$op<&'a u64> for $name {
            type Output = $name;

            fn $op_name(self, other: &'a u64) -> $name {
                self.$op_name(*other)
            }
        }

        impl<'a> ops::$op<u64> for &'a $name {
            type Output = $name;

            fn $op_name(self, other: u64) -> $name {
                (*self).$op_name(other)
            }
        }

        impl<'a, 'b> ops::$op<&'a u64> for &'b $name {
            type Output = $name;

            fn $op_name(self, other: &'a u64) -> $name {
                self.$op_name(*other)
            }
        }

        impl ops::$op<$name> for u64 {
            type Output = $name;

            fn $op_name(self, other: $name) -> $name {
                other.$op_name(self)
            }
        }

        impl<'a> ops::$op<&'a $name> for u64 {
            type Output = $name;

            fn $op_name(self, other: &'a $name) -> $name {
                (*other).$op_name(self)
            }
        }

        impl<'a> ops::$op<$name> for &'a u64 {
            type Output = $name;

            fn $op_name(self, other: $name) -> $name {
                other.$op_name(*self)
            }
        }

        impl<'a, 'b> ops::$op<&'a $name> for &'b u64 {
            type Output = $name;

            fn $op_name(self, other: &'a $name) -> $name {
                (*other).$op_name(*self)
            }
        }

        impl ops::$op_assign<u64> for $name {
            fn $op_assign_name(&mut self, other: u64) {
                match self.0 {
                    Some(a) => self.0 = $e(a, other),
                    _ => self.0 = None,
                }
            }
        }

        impl<'a> ops::$op_assign<&'a u64> for $name {
            fn $op_assign_name(&mut self, other: &'a u64) {
                self.$op_assign_name(*other)
            }
        }
    };
);

macro_rules! impl_format_value_traits(
    ($name:ident, $format:ident, $format_value:ident) => {
        impl FormattedValue for $name {
            fn get_default_format() -> Format {
                Format::$format
            }

            fn get_format(&self) -> Format {
                Format::$format
            }

            unsafe fn from_raw(format: Format, value: i64) -> Self {
                debug_assert_eq!(format, Format::$format);
                if value == -1 {
                    $name(None)
                } else {
                    $name(Some(value as u64))
                }
            }

            unsafe fn to_raw_value(&self) -> i64 {
                self.0.map(|v| v as i64).unwrap_or(-1)
            }
        }

        impl From<$name> for GenericFormattedValue {
            fn from(v: $name) -> GenericFormattedValue {
	        skip_assert_initialized!();
                GenericFormattedValue::$format_value(v)
            }
        }

        impl TryFrom<GenericFormattedValue> for $name {
            type Error = TryFromGenericFormattedValueError;

            fn try_from(v: GenericFormattedValue) -> Result<$name, TryFromGenericFormattedValueError> {
	        skip_assert_initialized!();
                if let GenericFormattedValue::$format_value(v) = v {
                    Ok(v)
                } else {
                    Err(TryFromGenericFormattedValueError(()))
                }
            }
        }

        impl SpecificFormattedValue for $name { }

        impl From<u64> for $name {
            fn from(v: u64) -> $name {
	        skip_assert_initialized!();
                $name(Some(v))
            }
        }

        impl From<Option<u64>> for $name {
            fn from(v: Option<u64>) -> $name {
	        skip_assert_initialized!();
                $name(v)
            }
        }

        impl Into<Option<u64>> for $name {
            fn into(self) -> Option<u64> {
                self.0
            }
        }

        impl ops::Deref for $name {
            type Target = Option<u64>;

            fn deref(&self) -> &Option<u64> {
                &self.0
            }
        }

        impl ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Option<u64> {
                &mut self.0
            }
        }

        impl AsRef<Option<u64>> for $name {
            fn as_ref(&self) -> &Option<u64> {
                &self.0
            }
        }

        impl AsMut<Option<u64>> for $name {
            fn as_mut(&mut self) -> &mut Option<u64> {
                &mut self.0
            }
        }

        impl_op_same!($name, Add, add, AddAssign, add_assign, |a: u64, b: u64| a.checked_add(b));
        impl_op_same!($name, Sub, sub, SubAssign, sub_assign, |a: u64, b: u64| a.checked_sub(b));
        impl_op_same!($name, Mul, mul, MulAssign, mul_assign, |a: u64, b: u64| a.checked_mul(b));
        impl_op_same!($name, Div, div, DivAssign, div_assign, |a: u64, b: u64| a.checked_div(b));
        impl_op_same!($name, Rem, rem, RemAssign, rem_assign, |a: u64, b: u64| a.checked_rem(b));

        impl_op_u64!($name, Mul, mul, MulAssign, mul_assign, |a: u64, b: u64| a.checked_mul(b));
        impl_op_u64!($name, Div, div, DivAssign, div_assign, |a: u64, b: u64| a.checked_div(b));
        impl_op_u64!($name, Rem, rem, RemAssign, rem_assign, |a: u64, b: u64| a.checked_rem(b));

        impl MulDiv<$name> for $name {
            type Output = $name;

            fn mul_div_floor(self, num: $name, denom: $name) -> Option<Self::Output> {
                match (self.0, num.0, denom.0) {
                    (Some(s), Some(n), Some(d)) => s.mul_div_floor(n, d).map(|v| $name(Some(v))),
                    _ => Some($name(None)),
                }
            }

            fn mul_div_round(self, num: $name, denom: $name) -> Option<Self::Output> {
                match (self.0, num.0, denom.0) {
                    (Some(s), Some(n), Some(d)) => s.mul_div_round(n, d).map(|v| $name(Some(v))),
                    _ => Some($name(None)),
                }
            }

            fn mul_div_ceil(self, num: $name, denom: $name) -> Option<Self::Output> {
                match (self.0, num.0, denom.0) {
                    (Some(s), Some(n), Some(d)) => s.mul_div_ceil(n, d).map(|v| $name(Some(v))),
                    _ => Some($name(None)),
                }
            }
        }

        impl<'a> MulDiv<&'a $name> for $name {
            type Output = $name;

            fn mul_div_floor(self, num: &$name, denom: &$name) -> Option<Self::Output> {
                self.mul_div_floor(*num, *denom)
            }

            fn mul_div_round(self, num: &$name, denom: &$name) -> Option<Self::Output> {
                self.mul_div_round(*num, *denom)
            }

            fn mul_div_ceil(self, num: &$name, denom: &$name) -> Option<Self::Output> {
                self.mul_div_ceil(*num, *denom)
            }
        }

        impl<'a> MulDiv<$name> for &'a $name {
            type Output = $name;

            fn mul_div_floor(self, num: $name, denom: $name) -> Option<Self::Output> {
                (*self).mul_div_floor(num, denom)
            }

            fn mul_div_round(self, num: $name, denom: $name) -> Option<Self::Output> {
                (*self).mul_div_round(num, denom)
            }

            fn mul_div_ceil(self, num: $name, denom: $name) -> Option<Self::Output> {
                (*self).mul_div_ceil(num, denom)
            }
        }

        impl<'a, 'b> MulDiv<&'b $name> for &'a $name {
            type Output = $name;

            fn mul_div_floor(self, num: &$name, denom: &$name) -> Option<Self::Output> {
                (*self).mul_div_floor(*num, *denom)
            }

            fn mul_div_round(self, num: &$name, denom: &$name) -> Option<Self::Output> {
                (*self).mul_div_round(*num, *denom)
            }

            fn mul_div_ceil(self, num: &$name, denom: &$name) -> Option<Self::Output> {
                (*self).mul_div_ceil(*num, *denom)
            }
        }

        impl<'a> MulDiv<u64> for $name {
            type Output = $name;

            fn mul_div_floor(self, num: u64, denom: u64) -> Option<Self::Output> {
                self.mul_div_floor($name(Some(num)), $name(Some(denom)))
            }

            fn mul_div_round(self, num: u64, denom: u64) -> Option<Self::Output> {
                self.mul_div_round($name(Some(num)), $name(Some(denom)))
            }

            fn mul_div_ceil(self, num: u64, denom: u64) -> Option<Self::Output> {
                self.mul_div_ceil($name(Some(num)), $name(Some(denom)))
            }
        }

        impl<'a> MulDiv<&'a u64> for $name {
            type Output = $name;

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

        impl<'a> MulDiv<u64> for &'a $name {
            type Output = $name;

            fn mul_div_floor(self, num: u64, denom: u64) -> Option<Self::Output> {
                (*self).mul_div_floor(num, denom)
            }

            fn mul_div_round(self, num: u64, denom: u64) -> Option<Self::Output> {
                (*self).mul_div_round(num, denom)
            }

            fn mul_div_ceil(self, num: u64, denom: u64) -> Option<Self::Output> {
                (*self).mul_div_ceil(num, denom)
            }
        }

        impl<'a, 'b> MulDiv<&'a u64> for &'b $name {
            type Output = $name;

            fn mul_div_floor(self, num: &u64, denom: &u64) -> Option<Self::Output> {
                (*self).mul_div_floor(*num, *denom)
            }

            fn mul_div_round(self, num: &u64, denom: &u64) -> Option<Self::Output> {
                (*self).mul_div_round(*num, *denom)
            }

            fn mul_div_ceil(self, num: &u64, denom: &u64) -> Option<Self::Output> {
                (*self).mul_div_ceil(*num, *denom)
            }
        }
    };
);

impl_format_value_traits!(Default, Default, Default);
impl_format_value_traits!(Bytes, Bytes, Bytes);
impl_format_value_traits!(ClockTime, Time, Time);
impl_format_value_traits!(Buffers, Buffers, Buffers);

impl FormattedValue for Undefined {
    fn get_default_format() -> Format {
        Format::Undefined
    }

    fn get_format(&self) -> Format {
        Format::Undefined
    }

    unsafe fn from_raw(format: Format, value: i64) -> Self {
        debug_assert_eq!(format, Format::Undefined);
        Undefined(value)
    }

    unsafe fn to_raw_value(&self) -> i64 {
        self.0
    }
}

impl From<Undefined> for GenericFormattedValue {
    fn from(v: Undefined) -> GenericFormattedValue {
        skip_assert_initialized!();
        GenericFormattedValue::Undefined(v)
    }
}

impl TryFrom<GenericFormattedValue> for Undefined {
    type Error = TryFromGenericFormattedValueError;

    fn try_from(v: GenericFormattedValue) -> Result<Undefined, TryFromGenericFormattedValueError> {
        skip_assert_initialized!();
        if let GenericFormattedValue::Undefined(v) = v {
            Ok(v)
        } else {
            Err(TryFromGenericFormattedValueError(()))
        }
    }
}

impl SpecificFormattedValue for Undefined {}

impl From<i64> for Undefined {
    fn from(v: i64) -> Undefined {
        skip_assert_initialized!();
        Undefined(v)
    }
}

impl Into<i64> for Undefined {
    fn into(self) -> i64 {
        self.0
    }
}

impl ops::Deref for Undefined {
    type Target = i64;

    fn deref(&self) -> &i64 {
        &self.0
    }
}

impl ops::DerefMut for Undefined {
    fn deref_mut(&mut self) -> &mut i64 {
        &mut self.0
    }
}

impl AsRef<i64> for Undefined {
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

impl AsMut<i64> for Undefined {
    fn as_mut(&mut self) -> &mut i64 {
        &mut self.0
    }
}

impl FormattedValue for Percent {
    fn get_default_format() -> Format {
        Format::Percent
    }

    fn get_format(&self) -> Format {
        Format::Percent
    }

    unsafe fn from_raw(format: Format, value: i64) -> Self {
        debug_assert_eq!(format, Format::Percent);
        if value < 0 || value > gst_sys::GST_FORMAT_PERCENT_MAX {
            Percent(None)
        } else {
            Percent(Some(value as u32))
        }
    }

    unsafe fn to_raw_value(&self) -> i64 {
        self.0.map(|v| v as i64).unwrap_or(-1)
    }
}

impl From<Percent> for GenericFormattedValue {
    fn from(v: Percent) -> GenericFormattedValue {
        skip_assert_initialized!();
        GenericFormattedValue::Percent(v)
    }
}

impl TryFrom<GenericFormattedValue> for Percent {
    type Error = TryFromGenericFormattedValueError;

    fn try_from(v: GenericFormattedValue) -> Result<Percent, TryFromGenericFormattedValueError> {
        skip_assert_initialized!();
        if let GenericFormattedValue::Percent(v) = v {
            Ok(v)
        } else {
            Err(TryFromGenericFormattedValueError(()))
        }
    }
}

impl SpecificFormattedValue for Percent {}

impl ops::Deref for Percent {
    type Target = Option<u32>;

    fn deref(&self) -> &Option<u32> {
        &self.0
    }
}

impl ops::DerefMut for Percent {
    fn deref_mut(&mut self) -> &mut Option<u32> {
        &mut self.0
    }
}

impl AsRef<Option<u32>> for Percent {
    fn as_ref(&self) -> &Option<u32> {
        &self.0
    }
}

impl AsMut<Option<u32>> for Percent {
    fn as_mut(&mut self) -> &mut Option<u32> {
        &mut self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
#[error("value out of range")]
pub struct TryPercentFromFloatError(());

impl TryFrom<f64> for Percent {
    type Error = TryPercentFromFloatError;

    fn try_from(v: f64) -> Result<Self, Self::Error> {
        skip_assert_initialized!();
        if v < 0.0 || v > 1.0 {
            Err(TryPercentFromFloatError(()))
        } else {
            Ok(Percent(Some(
                (v * gst_sys::GST_FORMAT_PERCENT_SCALE as f64).round() as u32,
            )))
        }
    }
}

impl TryFrom<f32> for Percent {
    type Error = TryPercentFromFloatError;

    fn try_from(v: f32) -> Result<Self, Self::Error> {
        skip_assert_initialized!();
        if v < 0.0 || v > 1.0 {
            Err(TryPercentFromFloatError(()))
        } else {
            Ok(Percent(Some(
                (v * gst_sys::GST_FORMAT_PERCENT_SCALE as f32).round() as u32,
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_clock_time() {
        ::init().unwrap();

        let t1 = ::SECOND;
        let t2 = 2 * t1;
        let t3 = &t1 * 2;
        let mut t4 = t2 + t3;
        t4 += &t1;

        assert_eq!(t4.nanoseconds(), Some(5_000_000_000));

        let t5 = t4 - 6 * ::SECOND;
        assert!(t5.is_none());
    }
}
