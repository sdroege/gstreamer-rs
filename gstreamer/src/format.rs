// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ClockTime;
use crate::Format;
use glib::translate::{FromGlib, GlibNoneError, IntoGlib, OptionIntoGlib, TryFromGlib};
use muldiv::MulDiv;
use opt_ops::prelude::*;
use std::borrow::Borrow;
use std::convert::TryFrom;
use std::fmt;
use std::ops;
use thiserror::Error;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
pub enum GenericFormattedValue {
    Undefined(Undefined),
    Default(Option<Default>),
    Bytes(Option<Bytes>),
    Time(Option<ClockTime>),
    Buffers(Option<Buffers>),
    Percent(Option<Percent>),
    Other(Format, i64),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Undefined(pub i64);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Default(pub u64);
impl Default {
    pub const MAX: Self = Self(u64::MAX - 1);
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Bytes(pub u64);
impl Bytes {
    pub const MAX: Self = Self(u64::MAX - 1);
}

pub type Time = ClockTime;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Buffers(pub u64);
impl Buffers {
    pub const OFFSET_NONE: u64 = ffi::GST_BUFFER_OFFSET_NONE;
    pub const MAX: Self = Self(Self::OFFSET_NONE - 1);
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Percent(pub u32);
impl Percent {
    pub const MAX: Self = Self(ffi::GST_FORMAT_PERCENT_MAX as u32);
    pub const SCALE: u32 = ffi::GST_FORMAT_PERCENT_SCALE as u32;
}

impl fmt::Display for GenericFormattedValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::utils::Displayable;

        match self {
            Self::Undefined(val) => val.fmt(f),
            Self::Default(val) => val.display().fmt(f),
            Self::Bytes(val) => val.display().fmt(f),
            Self::Time(val) => val.display().fmt(f),
            Self::Buffers(val) => val.display().fmt(f),
            Self::Percent(val) => val.display().fmt(f),
            Self::Other(format, val) => write!(f, "{} ({:?})", val, format),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
#[error("invalid generic value format")]
pub struct TryFromGenericFormattedValueError(());

pub trait FormattedValue: Copy + Clone + Sized + Into<GenericFormattedValue> + 'static {
    #[doc(alias = "get_default_format")]
    fn default_format() -> Format;
    #[doc(alias = "get_format")]
    fn format(&self) -> Format;

    unsafe fn from_raw(format: Format, value: i64) -> Self;
    unsafe fn into_raw_value(self) -> i64;
}

// rustdoc-stripper-ignore-next
/// A trait implemented on the intrinsic type of a `FormattedValue`.
///
/// # Examples
///
/// - `GenericFormattedValue` is the intrinsic type for `GenericFormattedValue`.
/// - `Undefined` is the intrinsic type for `Undefined`.
/// - `Bytes` is the intrinsic type for `Option<Bytes>`.
pub trait FormattedValueIntrinsic: Copy + Clone + Sized + 'static {
    type FormattedValueType: FormattedValue;
}

pub trait SpecificFormattedValue: FormattedValue + TryFrom<GenericFormattedValue> {}

// rustdoc-stripper-ignore-next
/// A trait implemented on the intrinsic type of a `SpecificFormattedValue`.
///
/// # Examples
///
/// - `Undefined` is the intrinsic type for `Undefined`.
/// - `Bytes` is the intrinsic type for `Option<Bytes>`.
pub trait SpecificFormattedValueIntrinsic: TryFromGlib<i64> + FormattedValueIntrinsic {}

impl FormattedValue for GenericFormattedValue {
    fn default_format() -> Format {
        Format::Undefined
    }

    fn format(&self) -> Format {
        self.format()
    }

    unsafe fn from_raw(format: Format, value: i64) -> Self {
        GenericFormattedValue::new(format, value)
    }

    unsafe fn into_raw_value(self) -> i64 {
        self.value()
    }
}

impl GenericFormattedValue {
    pub fn new(format: Format, value: i64) -> Self {
        skip_assert_initialized!();
        match format {
            Format::Undefined => Self::Undefined(Undefined(value)),
            Format::Default => Self::Default(unsafe { FromGlib::from_glib(value as u64) }),
            Format::Bytes => Self::Bytes(unsafe { FromGlib::from_glib(value as u64) }),
            Format::Time => Self::Time(unsafe { FromGlib::from_glib(value as u64) }),
            Format::Buffers => Self::Buffers(unsafe { FromGlib::from_glib(value as u64) }),
            Format::Percent => Self::Percent(unsafe { FormattedValue::from_raw(format, value) }),
            Format::__Unknown(_) => Self::Other(format, value),
        }
    }

    #[doc(alias = "get_format")]
    pub fn format(&self) -> Format {
        match *self {
            Self::Undefined(_) => Format::Undefined,
            Self::Default(_) => Format::Default,
            Self::Bytes(_) => Format::Bytes,
            Self::Time(_) => Format::Time,
            Self::Buffers(_) => Format::Buffers,
            Self::Percent(_) => Format::Percent,
            Self::Other(f, _) => f,
        }
    }

    #[doc(alias = "get_value")]
    pub fn value(&self) -> i64 {
        unsafe {
            match *self {
                Self::Undefined(v) => v.0,
                Self::Default(v) => v.into_raw_value(),
                Self::Bytes(v) => v.into_raw_value(),
                Self::Time(v) => v.into_raw_value(),
                Self::Buffers(v) => v.into_raw_value(),
                Self::Percent(v) => v.into_raw_value(),
                Self::Other(_, v) => v,
            }
        }
    }
}

impl FormattedValueIntrinsic for GenericFormattedValue {
    type FormattedValueType = GenericFormattedValue;
}

macro_rules! impl_op_same(
    ($name:ident, $op:ident, $op_name:ident, $op_assign:ident, $op_assign_name:ident) => {
        impl<RHS: Borrow<$name>> ops::$op<RHS> for $name {
            type Output = Self;

            fn $op_name(self, rhs: RHS) -> Self::Output {
                Self(self.0.$op_name(rhs.borrow().0))
            }
        }

        impl<RHS: Borrow<$name>> ops::$op<RHS> for &$name {
            type Output = $name;

            fn $op_name(self, rhs: RHS) -> Self::Output {
                (*self).$op_name(rhs)
            }
        }

        impl<RHS: Borrow<$name>> ops::$op_assign<RHS> for $name {
            fn $op_assign_name(&mut self, rhs: RHS) {
                self.0.$op_assign_name(rhs.borrow().0)
            }
        }
    };
);

macro_rules! impl_op_u64(
    ($name:ident, $op:ident, $op_name:ident, $op_assign:ident, $op_assign_name:ident) => {
        impl ops::$op<u64> for $name {
            type Output = $name;

            fn $op_name(self, rhs: u64) -> Self::Output {
                $name(self.0.$op_name(rhs))
            }
        }

        impl ops::$op<u64> for &$name {
            type Output = $name;

            fn $op_name(self, rhs: u64) -> Self::Output {
                (*self).$op_name(rhs)
            }
        }

        impl ops::$op<$name> for u64 {
            type Output = $name;

            fn $op_name(self, rhs: $name) -> $name {
                $name(self.$op_name(rhs.0))
            }
        }

        impl ops::$op<&$name> for u64 {
            type Output = $name;

            fn $op_name(self, rhs: &$name) -> $name {
                self.$op_name(*rhs)
            }
        }

        impl ops::$op_assign<u64> for $name {
            fn $op_assign_name(&mut self, rhs: u64) {
                self.0.$op_assign_name(rhs)
            }
        }
    };
);

macro_rules! impl_format_value_traits(
    ($name:ident, $format:ident, $format_value:ident) => {
        impl FormattedValue for Option<$name> {
            fn default_format() -> Format {
                Format::$format
            }

            fn format(&self) -> Format {
                Format::$format
            }

            unsafe fn from_raw(format: Format, value: i64) -> Option<$name> {
                debug_assert_eq!(format, Format::$format);
                FromGlib::from_glib(value as u64)
            }

            unsafe fn into_raw_value(self) -> i64 {
                IntoGlib::into_glib(self) as i64
            }
        }

        impl From<Option<$name>> for GenericFormattedValue {
            fn from(v: Option<$name>) -> Self {
                skip_assert_initialized!();
                Self::$format_value(v)
            }
        }

        impl From<$name> for GenericFormattedValue {
            fn from(v: $name) -> Self {
                skip_assert_initialized!();
                Self::$format_value(Some(v))
            }
        }

        impl FormattedValueIntrinsic for $name {
            type FormattedValueType = Option<$name>;
        }

        impl TryFrom<GenericFormattedValue> for Option<$name> {
            type Error = TryFromGenericFormattedValueError;

            fn try_from(v: GenericFormattedValue) -> Result<Option<$name>, Self::Error> {
                skip_assert_initialized!();
                if let GenericFormattedValue::$format_value(v) = v {
                    Ok(v)
                } else {
                    Err(TryFromGenericFormattedValueError(()))
                }
            }
        }

        impl TryFrom<u64> for $name {
            type Error = GlibNoneError;

            fn try_from(v: u64) -> Result<$name, GlibNoneError> {
                skip_assert_initialized!();
                unsafe { Self::try_from_glib(v) }
            }
        }

        impl TryFromGlib<i64> for $name {
            type Error = GlibNoneError;
            #[inline]
            unsafe fn try_from_glib(val: i64) -> Result<Self, GlibNoneError> {
                skip_assert_initialized!();
                <$name as TryFromGlib<u64>>::try_from_glib(val as u64)
            }
        }

        impl SpecificFormattedValue for Option<$name> {}
        impl SpecificFormattedValueIntrinsic for $name {}

        impl ops::Deref for $name {
            type Target = u64;

            fn deref(&self) -> &u64 {
                &self.0
            }
        }

        impl ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut u64 {
                &mut self.0
            }
        }

        impl AsRef<u64> for $name {
            fn as_ref(&self) -> &u64 {
                &self.0
            }
        }

        impl AsMut<u64> for $name {
            fn as_mut(&mut self) -> &mut u64 {
                &mut self.0
            }
        }

        impl_op_same!($name, Add, add, AddAssign, add_assign);
        impl_op_same!($name, Sub, sub, SubAssign, sub_assign);
        impl_op_same!($name, Mul, mul, MulAssign, mul_assign);
        impl_op_same!($name, Div, div, DivAssign, div_assign);
        impl_op_same!($name, Rem, rem, RemAssign, rem_assign);

        impl_op_u64!($name, Mul, mul, MulAssign, mul_assign);
        impl_op_u64!($name, Div, div, DivAssign, div_assign);
        impl_op_u64!($name, Rem, rem, RemAssign, rem_assign);

        impl<ND: Borrow<u64>> MulDiv<ND> for $name {
            type Output = $name;

            fn mul_div_floor(self, num: ND, denom: ND) -> Option<Self::Output> {
                self.0
                    .mul_div_floor(*num.borrow(), *denom.borrow())
                    .map($name)
            }

            fn mul_div_round(self, num: ND, denom: ND) -> Option<Self::Output> {
                self.0
                    .mul_div_round(*num.borrow(), *denom.borrow())
                    .map($name)
            }

            fn mul_div_ceil(self, num: ND, denom: ND) -> Option<Self::Output> {
                self.0
                    .mul_div_ceil(*num.borrow(), *denom.borrow())
                    .map($name)
            }
        }
    };
);

macro_rules! option_glib_newtype_display {
    ($name:ident, $unit:expr) => {
        impl crate::utils::Displayable for Option<$name> {
            type DisplayImpl = String;

            fn display(self) -> String {
                if let Some(val) = self {
                    val.display()
                } else {
                    format!("undef. {}", $unit)
                }
            }
        }

        impl crate::utils::Displayable for $name {
            type DisplayImpl = String;

            fn display(self) -> String {
                format!("{} {}", self.0, $unit)
            }
        }
    };
}

impl_common_ops_for_newtype_u64!(Default);
impl_format_value_traits!(Default, Default, Default);
option_glib_newtype_from_to!(Default, u64::MAX);
option_glib_newtype_display!(Default, "(Default)");

impl_common_ops_for_newtype_u64!(Bytes);
impl_format_value_traits!(Bytes, Bytes, Bytes);
option_glib_newtype_from_to!(Bytes, u64::MAX);
option_glib_newtype_display!(Bytes, "bytes");

impl_format_value_traits!(ClockTime, Time, Time);

impl_common_ops_for_newtype_u64!(Buffers);
impl_format_value_traits!(Buffers, Buffers, Buffers);
option_glib_newtype_from_to!(Buffers, Buffers::OFFSET_NONE);
option_glib_newtype_display!(Buffers, "buffers");

impl FormattedValue for Undefined {
    fn default_format() -> Format {
        Format::Undefined
    }

    fn format(&self) -> Format {
        Format::Undefined
    }

    unsafe fn from_raw(format: Format, value: i64) -> Self {
        debug_assert_eq!(format, Format::Undefined);
        Undefined(value)
    }

    unsafe fn into_raw_value(self) -> i64 {
        self.0
    }
}

impl From<Undefined> for GenericFormattedValue {
    fn from(v: Undefined) -> Self {
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

impl FormattedValueIntrinsic for Undefined {
    type FormattedValueType = Undefined;
}

impl SpecificFormattedValue for Undefined {}

impl SpecificFormattedValueIntrinsic for Undefined {}

impl TryFromGlib<i64> for Undefined {
    type Error = std::convert::Infallible;
    #[inline]
    unsafe fn try_from_glib(v: i64) -> Result<Self, Self::Error> {
        skip_assert_initialized!();
        Ok(Undefined(v))
    }
}

impl From<i64> for Undefined {
    fn from(v: i64) -> Self {
        skip_assert_initialized!();
        Undefined(v)
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

impl fmt::Display for Undefined {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (Undefined)", self.0)
    }
}

impl crate::utils::Displayable for Undefined {
    type DisplayImpl = Undefined;

    fn display(self) -> Undefined {
        self
    }
}

impl_common_ops_for_newtype_u64!(Percent);
option_glib_newtype_display!(Percent, "%");

impl FormattedValue for Option<Percent> {
    fn default_format() -> Format {
        Format::Percent
    }

    fn format(&self) -> Format {
        Format::Percent
    }

    unsafe fn from_raw(format: Format, value: i64) -> Self {
        debug_assert_eq!(format, Format::Percent);
        Percent::try_from_glib(value as i64).ok()
    }

    unsafe fn into_raw_value(self) -> i64 {
        self.map_or(-1, |v| v.0 as i64)
    }
}

impl From<Option<Percent>> for GenericFormattedValue {
    fn from(v: Option<Percent>) -> Self {
        skip_assert_initialized!();
        GenericFormattedValue::Percent(v)
    }
}

impl From<Percent> for GenericFormattedValue {
    fn from(v: Percent) -> Self {
        skip_assert_initialized!();
        GenericFormattedValue::Percent(Some(v))
    }
}
impl TryFrom<u64> for Percent {
    type Error = GlibNoneError;

    fn try_from(v: u64) -> Result<Percent, GlibNoneError> {
        skip_assert_initialized!();
        unsafe { Self::try_from_glib(v as i64) }
    }
}

impl TryFromGlib<i64> for Percent {
    type Error = GlibNoneError;
    #[inline]
    unsafe fn try_from_glib(value: i64) -> Result<Self, Self::Error> {
        skip_assert_initialized!();
        if value < 0 || value > ffi::GST_FORMAT_PERCENT_MAX {
            Err(GlibNoneError)
        } else {
            Ok(Percent(value as u32))
        }
    }
}

impl TryFrom<GenericFormattedValue> for Option<Percent> {
    type Error = TryFromGenericFormattedValueError;

    fn try_from(
        v: GenericFormattedValue,
    ) -> Result<Option<Percent>, TryFromGenericFormattedValueError> {
        skip_assert_initialized!();
        if let GenericFormattedValue::Percent(v) = v {
            Ok(v)
        } else {
            Err(TryFromGenericFormattedValueError(()))
        }
    }
}

impl FormattedValueIntrinsic for Percent {
    type FormattedValueType = Option<Percent>;
}

impl SpecificFormattedValue for Option<Percent> {}

impl SpecificFormattedValueIntrinsic for Percent {}

impl ops::Deref for Percent {
    type Target = u32;

    fn deref(&self) -> &u32 {
        &self.0
    }
}

impl ops::DerefMut for Percent {
    fn deref_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}

impl AsRef<u32> for Percent {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

impl AsMut<u32> for Percent {
    fn as_mut(&mut self) -> &mut u32 {
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
            Ok(Percent(
                (v * ffi::GST_FORMAT_PERCENT_SCALE as f64).round() as u32
            ))
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
            Ok(Percent(
                (v * ffi::GST_FORMAT_PERCENT_SCALE as f32).round() as u32
            ))
        }
    }
}
