// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use muldiv::MulDiv;
use std::ops;
use ClockTime;
use Format;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
#[cfg_attr(feature = "ser_de", derive(Serialize, Deserialize))]
pub enum GenericFormattedValue {
    Undefined(i64),
    Default(Default),
    Bytes(Bytes),
    Time(ClockTime),
    Buffers(Buffers),
    Percent(Option<u32>),
    Other(Format, i64),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "ser_de", derive(Serialize, Deserialize))]
pub struct Default(pub Option<u64>);
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "ser_de", derive(Serialize, Deserialize))]
pub struct Bytes(pub Option<u64>);
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "ser_de", derive(Serialize, Deserialize))]
pub struct Buffers(pub Option<u64>);
pub type Time = ClockTime;

pub trait FormattedValue: Copy + Clone + Sized + 'static {
    fn get_default_format() -> Format;
    fn try_from(v: GenericFormattedValue) -> Option<Self>;

    fn get_format(&self) -> Format;

    unsafe fn from_raw(format: Format, value: i64) -> Self;
    unsafe fn to_raw_value(&self) -> i64;
}

pub trait SpecificFormattedValue: FormattedValue {}

impl FormattedValue for GenericFormattedValue {
    fn get_default_format() -> Format {
        Format::Undefined
    }

    fn try_from(v: GenericFormattedValue) -> Option<Self> {
        Some(v)
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
        match format {
            Format::Undefined => GenericFormattedValue::Undefined(value),
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
            Format::Percent => GenericFormattedValue::Percent(if value == -1 {
                None
            } else {
                Some(value as u32)
            }),
            Format::__Unknown(_) => GenericFormattedValue::Other(format, value),
        }
    }

    pub fn from_undefined(v: i64) -> Self {
        GenericFormattedValue::Undefined(v)
    }

    pub fn from_default<V: Into<Default>>(v: V) -> Self {
        GenericFormattedValue::Default(v.into())
    }

    pub fn from_bytes<V: Into<Bytes>>(v: V) -> Self {
        GenericFormattedValue::Bytes(v.into())
    }

    pub fn from_time<V: Into<ClockTime>>(v: V) -> Self {
        GenericFormattedValue::Time(v.into())
    }

    pub fn from_buffers<V: Into<Buffers>>(v: V) -> Self {
        GenericFormattedValue::Buffers(v.into())
    }

    pub fn from_percent<V: Into<Option<u32>>>(v: V) -> Self {
        GenericFormattedValue::Percent(v.into())
    }

    pub fn from_other(format: Format, v: i64) -> Self {
        GenericFormattedValue::Other(format, v)
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
            GenericFormattedValue::Undefined(v) => v,
            GenericFormattedValue::Default(v) => v.map(|v| v as i64).unwrap_or(-1),
            GenericFormattedValue::Bytes(v) => v.map(|v| v as i64).unwrap_or(-1),
            GenericFormattedValue::Time(v) => v.map(|v| v as i64).unwrap_or(-1),
            GenericFormattedValue::Buffers(v) => v.map(|v| v as i64).unwrap_or(-1),
            GenericFormattedValue::Percent(v) => v.map(i64::from).unwrap_or(-1),
            GenericFormattedValue::Other(_, v) => v,
        }
    }

    pub fn try_into<F: FormattedValue>(self) -> Result<F, Self> {
        if F::get_default_format() == self.get_format()
            || F::get_default_format() == Format::Undefined
        {
            Ok(unsafe { F::from_raw(self.get_format(), self.to_raw_value()) })
        } else {
            Err(self)
        }
    }

    pub fn try_into_undefined(self) -> Result<i64, Self> {
        if let GenericFormattedValue::Undefined(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_default(self) -> Result<Default, Self> {
        if let GenericFormattedValue::Default(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_bytes(self) -> Result<Bytes, Self> {
        if let GenericFormattedValue::Bytes(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_time(self) -> Result<ClockTime, Self> {
        if let GenericFormattedValue::Time(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_buffers(self) -> Result<Buffers, Self> {
        if let GenericFormattedValue::Buffers(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_percent(self) -> Result<Option<u32>, Self> {
        if let GenericFormattedValue::Percent(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_other(self) -> Result<(Format, i64), Self> {
        if let GenericFormattedValue::Other(f, v) = self {
            Ok((f, v))
        } else {
            Err(self)
        }
    }
}

macro_rules! impl_op_same(
    ($name:ident, $op:ident, $op_name:ident, $op_assign:ident, $op_assign_name:ident, $e:expr) => {
        impl ops::$op<$name> for $name {
            type Output = $name;

            fn $op_name(self, other: $name) -> $name {
                match (self.0, other.0) {
                    (Some(a), Some(b)) => $name(Some($e(a, b))),
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

        impl ops::$op_assign<$name> for $name {
            fn $op_assign_name(&mut self, other: $name) {
                match (self.0, other.0) {
                    (Some(a), Some(b)) => self.0 = Some($e(a, b)),
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
                    Some(a) => $name(Some($e(a, other))),
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

        impl ops::$op_assign<u64> for $name {
            fn $op_assign_name(&mut self, other: u64) {
                match self.0 {
                    Some(a) => self.0 = Some($e(a, other)),
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
        impl From<$name> for GenericFormattedValue {
            fn from(v: $name) -> GenericFormattedValue {
                GenericFormattedValue::$format_value(v)
            }
        }

        impl FormattedValue for $name {
            fn get_default_format() -> Format {
                Format::$format
            }

            fn try_from(v: GenericFormattedValue) -> Option<Self> {
                if let GenericFormattedValue::$format_value(v) = v {
                    Some(v)
                } else {
                    None
                }
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

        impl SpecificFormattedValue for $name { }

        impl From<u64> for $name {
            fn from(v: u64) -> $name {
                $name(Some(v))
            }
        }

        impl From<Option<u64>> for $name {
            fn from(v: Option<u64>) -> $name {
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

        impl_op_same!($name, Add, add, AddAssign, add_assign, |a, b| a + b);
        impl_op_same!($name, Sub, sub, SubAssign, sub_assign, |a, b| a - b);
        impl_op_same!($name, Mul, mul, MulAssign, mul_assign, |a, b| a * b);
        impl_op_same!($name, Div, div, DivAssign, div_assign, |a, b| a / b);
        impl_op_same!($name, Rem, rem, RemAssign, rem_assign, |a, b| a % b);

        impl_op_u64!($name, Mul, mul, MulAssign, mul_assign, |a, b| a * b);
        impl_op_u64!($name, Div, div, DivAssign, div_assign, |a, b| a / b);
        impl_op_u64!($name, Rem, rem, RemAssign, rem_assign, |a, b| a % b);

        impl ops::Mul<$name> for u64 {
            type Output = $name;

            fn mul(self, other: $name) -> $name {
                other.mul(self)
            }
        }

        impl<'a> ops::Mul<&'a $name> for u64 {
            type Output = $name;

            fn mul(self, other: &'a $name) -> $name {
                other.mul(self)
            }
        }

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
    };
);

impl_format_value_traits!(Default, Default, Default);
impl_format_value_traits!(Bytes, Bytes, Bytes);
impl_format_value_traits!(ClockTime, Time, Time);
impl_format_value_traits!(Buffers, Buffers, Buffers);

#[cfg(test)]
mod tests {
    #[cfg(feature = "ser_de")]
    #[test]
    fn test_serialize() {
        extern crate ron;
        extern crate serde_json;

        use super::Buffers;
        use super::Bytes;
        use ClockTime;
        use super::Default;
        use Format;
        use GenericFormattedValue;

        ::init().unwrap();

        // don't use newlines
        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        let value = GenericFormattedValue::Undefined(42);
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Undefined(42)".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Undefined\":42}".to_owned(), res);

        let value = GenericFormattedValue::Default(Default(Some(42)));
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Default((Some(42)))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Default\":42}".to_owned(), res);

        let value = GenericFormattedValue::Default(Default(None));
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Default((None))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Default\":null}".to_owned(), res);

        let value = GenericFormattedValue::Bytes(Bytes(Some(42)));
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Bytes((Some(42)))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Bytes\":42}".to_owned(), res);

        let value = GenericFormattedValue::Time(ClockTime::from_nseconds(42_123_456_789));
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Time(Some(42123456789))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Time\":42123456789}".to_owned(), res);

        let value = GenericFormattedValue::Buffers(Buffers(Some(42)));
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Buffers((Some(42)))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Buffers\":42}".to_owned(), res);

        let value = GenericFormattedValue::Percent(Some(42));
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Percent(Some(42))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Percent\":42}".to_owned(), res);

        let value = GenericFormattedValue::Other(Format::Percent, 42);
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Other(Percent, 42)".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Other\":[\"Percent\",42]}".to_owned(), res);

        let value = GenericFormattedValue::Other(Format::__Unknown(7), 42);
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Other(__Unknown(7), 42)".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Other\":[{\"__Unknown\":7},42]}".to_owned(), res);
    }

    #[cfg(feature = "ser_de")]
    #[test]
    fn test_deserialize() {
        extern crate ron;
        extern crate serde_json;

        use GenericFormattedValue;
        use Format;

        ::init().unwrap();

        let format_ron = "Other(Percent, 42)";
        let format: GenericFormattedValue = ron::de::from_str(format_ron).unwrap();
        assert_eq!(format, GenericFormattedValue::Other(Format::Percent, 42));

        let format_json = "{\"Other\":[\"Percent\",42]}";
        let format: GenericFormattedValue = serde_json::from_str(format_json).unwrap();
        assert_eq!(format, GenericFormattedValue::Other(Format::Percent, 42));
    }
}
