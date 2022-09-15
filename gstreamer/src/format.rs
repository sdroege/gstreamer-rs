// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ClockTime;
use crate::Format;
use glib::translate::{FromGlib, GlibNoneError, IntoGlib, OptionIntoGlib, TryFromGlib};
use muldiv::MulDiv;
use opt_ops::prelude::*;
use std::borrow::Borrow;
use std::fmt;
use std::ops;
use thiserror::Error;

// rustdoc-stripper-ignore-next
/// A signed wrapper.
///
/// This wrapper allows representing a signed value from a type
/// which is originaly unsigned. In C APIs, this is represented
/// by a tuple with a signed integer positive or negative and
/// the absolute value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Signed<T> {
    Negative(T),
    Positive(T),
}

impl<T> Signed<T> {
    pub fn is_positive(self) -> bool {
        matches!(self, Signed::Positive(_))
    }

    // rustdoc-stripper-ignore-next
    /// Returns `Some(value)`, where `value` is the inner value,
    /// if `self` is positive.
    pub fn positive(self) -> Option<T> {
        match self {
            Signed::Positive(val) => Some(val),
            Signed::Negative(_) => None,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transforms the `Signed<T>` into a `Result<T, E>`,
    /// mapping `Positive(v)` to `Ok(v)` and `Negative(_)` to `Err(err)`.
    pub fn positive_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Signed::Positive(val) => Ok(val),
            Signed::Negative(_) => Err(err),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transforms the `Signed<T>` into a `Result<T, E>`,
    /// mapping `Positive(v)` to `Ok(v)` and `Negative(v)` to `Err(err(v))`.
    pub fn positive_or_else<E, F: FnOnce(T) -> E>(self, err: F) -> Result<T, E> {
        match self {
            Signed::Positive(val) => Ok(val),
            Signed::Negative(val) => Err(err(val)),
        }
    }

    pub fn is_negative(self) -> bool {
        matches!(self, Signed::Negative(_))
    }

    // rustdoc-stripper-ignore-next
    /// Returns `Some(value)`, where `value` is the inner value,
    /// if `self` is negative.
    pub fn negative(self) -> Option<T> {
        match self {
            Signed::Negative(val) => Some(val),
            Signed::Positive(_) => None,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transforms the `Signed<T>` into a `Result<T, E>`,
    /// mapping `Negative(v)` to `Ok(v)` and `Positive(_)` to `Err(err)`.
    pub fn negative_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Signed::Negative(val) => Ok(val),
            Signed::Positive(_) => Err(err),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transforms the `Signed<T>` into a `Result<T, E>`,
    /// mapping `Negative(v)` to `Ok(v)` and `Positive(_)` to `Err(err(v))`.
    pub fn negative_or_else<E, F: FnOnce(T) -> E>(self, err: F) -> Result<T, E> {
        match self {
            Signed::Negative(val) => Ok(val),
            Signed::Positive(val) => Err(err(val)),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the multiplication factor for this `Signed`.
    ///
    /// Returns:
    ///
    /// - `1` if the value must be considered as positive.
    /// - `-1` if the value must be considered as negative.
    pub fn factor(self) -> i32 {
        match self {
            Signed::Positive(_) => 1i32,
            Signed::Negative(_) => -1i32,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the absolute value of `self`.
    pub fn abs(self) -> T {
        match self {
            Signed::Positive(val) | Signed::Negative(val) => val,
        }
    }
}

impl<T> std::ops::Neg for Signed<T> {
    type Output = Signed<T>;

    fn neg(self) -> Self {
        match self {
            Signed::Positive(val) => Signed::Negative(val),
            Signed::Negative(val) => Signed::Positive(val),
        }
    }
}

impl Signed<ClockTime> {
    // rustdoc-stripper-ignore-next
    /// Returns the `self` in nanoseconds.
    pub fn nseconds(self) -> Signed<u64> {
        match self {
            Signed::Positive(val) => Signed::Positive(val.nseconds()),
            Signed::Negative(val) => Signed::Negative(val.nseconds()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates new value from nanoseconds.
    pub fn from_nseconds(val: Signed<u64>) -> Self {
        skip_assert_initialized!();
        match val {
            Signed::Positive(val) => Signed::Positive(ClockTime::from_nseconds(val)),
            Signed::Negative(val) => Signed::Negative(ClockTime::from_nseconds(val)),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the `self` in microseconds.
    pub fn useconds(self) -> Signed<u64> {
        match self {
            Signed::Positive(val) => Signed::Positive(val.useconds()),
            Signed::Negative(val) => Signed::Negative(val.useconds()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates new value from microseconds.
    pub fn from_useconds(val: Signed<u64>) -> Self {
        skip_assert_initialized!();
        match val {
            Signed::Positive(val) => Signed::Positive(ClockTime::from_useconds(val)),
            Signed::Negative(val) => Signed::Negative(ClockTime::from_useconds(val)),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the `self` in milliseconds.
    pub fn mseconds(self) -> Signed<u64> {
        match self {
            Signed::Positive(val) => Signed::Positive(val.mseconds()),
            Signed::Negative(val) => Signed::Negative(val.mseconds()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates new value from milliseconds.
    pub fn from_mseconds(val: Signed<u64>) -> Self {
        skip_assert_initialized!();
        match val {
            Signed::Positive(val) => Signed::Positive(ClockTime::from_mseconds(val)),
            Signed::Negative(val) => Signed::Negative(ClockTime::from_mseconds(val)),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the `self` in seconds.
    pub fn seconds(self) -> Signed<u64> {
        match self {
            Signed::Positive(val) => Signed::Positive(val.seconds()),
            Signed::Negative(val) => Signed::Negative(val.seconds()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates new value from seconds.
    pub fn from_seconds(val: Signed<u64>) -> Self {
        skip_assert_initialized!();
        match val {
            Signed::Positive(val) => Signed::Positive(ClockTime::from_seconds(val)),
            Signed::Negative(val) => Signed::Negative(ClockTime::from_seconds(val)),
        }
    }
}

macro_rules! impl_signed_ops(
    ($type:ty, $zero:expr) => {
        impl Signed<$type> {
            // rustdoc-stripper-ignore-next
            /// Returns the signum for this `Signed`.
            ///
            /// Returns:
            ///
            /// - `0` if the number is zero.
            /// - `1` if the value must be considered as positive.
            /// - `-1` if the value must be considered as negative.
            pub fn signum(self) -> i32 {
                match self {
                    Signed::Positive(val) | Signed::Negative(val) if val == $zero => 0i32,
                    Signed::Positive(_) => 1i32,
                    Signed::Negative(_) => -1i32,
                }
            }

            // rustdoc-stripper-ignore-next
            /// Returns the checked subtraction `self - other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_sub(self, other: Self) -> Option<Self> {
                match (self, other) {
                    (Signed::Positive(a), Signed::Positive(b)) if a >= b => Some(Signed::Positive(a - b)),
                    (Signed::Positive(a), Signed::Positive(b)) => Some(Signed::Negative(b - a)),
                    (Signed::Negative(a), Signed::Negative(b)) if a >= b => Some(Signed::Negative(a - b)),
                    (Signed::Negative(a), Signed::Negative(b)) => Some(Signed::Positive(b - a)),
                    (Signed::Positive(a), Signed::Negative(b)) => a.checked_add(b).map(Signed::Positive),
                    (Signed::Negative(a), Signed::Positive(b)) => a.checked_sub(b).map(Signed::Negative),
                }
            }

            // rustdoc-stripper-ignore-next
            /// Returns the checked subtraction `self - other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_sub_unsigned(self, other: $type) -> Option<Self> {
                self.checked_sub(Signed::Positive(other))
            }

            // rustdoc-stripper-ignore-next
            /// Returns the checked addition `self + other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_add(self, other: Self) -> Option<Self> {
                match (self, other) {
                    (Signed::Positive(a), Signed::Positive(b)) => a.checked_add(b).map(Signed::Positive),
                    (Signed::Negative(a), Signed::Negative(b)) => a.checked_add(b).map(Signed::Negative),
                    (Signed::Positive(_), Signed::Negative(_)) => self.checked_sub(-other),
                    (Signed::Negative(_), Signed::Positive(_)) => Some(-((-self).checked_sub(other)?))
                }
            }

            // rustdoc-stripper-ignore-next
            /// Returns the checked addition `self + other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_add_unsigned(self, other: $type) -> Option<Self> {
                self.checked_add(Signed::Positive(other))
            }

            // rustdoc-stripper-ignore-next
            /// Returns the saturating subtraction `self - other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn saturating_sub(self, other: Self) -> Self {
                match (self, other) {
                    (Signed::Positive(a), Signed::Positive(b)) if a >= b => Signed::Positive(a - b),
                    (Signed::Positive(a), Signed::Positive(b)) => Signed::Negative(b - a),
                    (Signed::Negative(a), Signed::Negative(b)) if a >= b => Signed::Negative(a - b),
                    (Signed::Negative(a), Signed::Negative(b)) => Signed::Positive(b - a),
                    (Signed::Positive(a), Signed::Negative(b)) => Signed::Positive(a.saturating_add(b)),
                    (Signed::Negative(a), Signed::Positive(b)) => Signed::Negative(a.saturating_sub(b)),
                }
            }

            // rustdoc-stripper-ignore-next
            /// Returns the saturating subtraction `self - other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn saturating_sub_unsigned(self, other: $type) -> Self {
                self.saturating_sub(Signed::Positive(other))
            }

            // rustdoc-stripper-ignore-next
            /// Returns the saturating addition `self + other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn saturating_add(self, other: Self) -> Self {
                match (self, other) {
                    (Signed::Positive(a), Signed::Positive(b)) => Signed::Positive(a.saturating_add(b)),
                    (Signed::Negative(a), Signed::Negative(b)) => Signed::Negative(a.saturating_add(b)),
                    (Signed::Positive(_), Signed::Negative(_)) => self.saturating_sub(-other),
                    (Signed::Negative(_), Signed::Positive(_)) => -((-other).saturating_sub(other)),
                }
            }

            // rustdoc-stripper-ignore-next
            /// Returns the saturating addition `self + other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn saturating_add_unsigned(self, other: $type) -> Self {
                self.saturating_add(Signed::Positive(other))
            }
        }

        impl std::ops::Add<Signed<$type>> for Signed<$type> {
            type Output = Signed<$type>;

            fn add(self, other: Signed<$type>) -> Signed<$type> {
                self.checked_add(other).expect("Overflowing addition")
            }
        }

        impl std::ops::AddAssign<Signed<$type>> for Signed<$type> {
            fn add_assign(&mut self, other: Signed<$type>) {
                *self = self.checked_add(other).expect("Overflowing addition")
            }
        }

        impl std::ops::Sub<Signed<$type>> for Signed<$type> {
            type Output = Signed<$type>;

            fn sub(self, other: Signed<$type>) -> Signed<$type> {
                self.checked_sub(other).expect("Overflowing subtraction")
            }
        }

        impl std::ops::SubAssign<Signed<$type>> for Signed<$type> {
            fn sub_assign(&mut self, other: Signed<$type>) {
                *self = self.checked_sub(other).expect("Overflowing subtraction")
            }
        }

        impl std::ops::Add<$type> for Signed<$type> {
            type Output = Signed<$type>;

            fn add(self, other: $type) -> Signed<$type> {
                self.checked_add(Signed::Positive(other)).expect("Overflowing addition")
            }
        }

        impl std::ops::AddAssign<$type> for Signed<$type> {
            fn add_assign(&mut self, other: $type) {
                *self = self.checked_add(Signed::Positive(other)).expect("Overflowing addition")
            }
        }

        impl std::ops::Sub<$type> for Signed<$type> {
            type Output = Signed<$type>;

            fn sub(self, other: $type) -> Signed<$type> {
                self.checked_sub(Signed::Positive(other)).expect("Overflowing subtraction")
            }
        }

        impl std::ops::SubAssign<$type> for Signed<$type> {
            fn sub_assign(&mut self, other: $type) {
                *self = self.checked_sub(Signed::Positive(other)).expect("Overflowing subtraction")
            }
        }

        impl From<$type> for Signed<$type> {
            fn from(val: $type) -> Self {
                skip_assert_initialized!();
                Signed::Positive(val)
            }
        }

        impl PartialOrd<Signed<$type>> for Signed<$type> {
            fn partial_cmp(&self, other: &Signed<$type>) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl PartialEq<$type> for Signed<$type> {
            fn eq(&self, other: &$type) -> bool {
                self.eq(&Signed::Positive(*other))
            }
        }

        impl PartialOrd<$type> for Signed<$type> {
            fn partial_cmp(&self, other: &$type) -> Option<std::cmp::Ordering> {
                Some(self.cmp(&Signed::Positive(*other)))
            }
        }

        impl Ord for Signed<$type> {
            fn cmp(&self, other: &Signed<$type>) -> std::cmp::Ordering {
                match (self, other) {
                    (Signed::Positive(a), Signed::Positive(b)) => a.cmp(b),
                    (Signed::Negative(a), Signed::Negative(b)) => a.cmp(b),
                    (Signed::Positive(_), Signed::Negative(_)) => std::cmp::Ordering::Greater,
                    (Signed::Negative(_), Signed::Positive(_)) => std::cmp::Ordering::Less,
                }
            }
        }
    };
);

impl_signed_ops!(u64, 0);
impl_signed_ops!(ClockTime, ClockTime::ZERO);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    #[doc(alias = "GST_BUFFER_OFFSET_NONE")]
    pub const OFFSET_NONE: u64 = ffi::GST_BUFFER_OFFSET_NONE;
    pub const MAX: Self = Self(Self::OFFSET_NONE - 1);
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Percent(pub u32);
impl Percent {
    #[doc(alias = "GST_FORMAT_PERCENT_MAX")]
    pub const MAX: Self = Self(ffi::GST_FORMAT_PERCENT_MAX as u32);
    #[doc(alias = "GST_FORMAT_PERCENT_SCALE")]
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
#[error("invalid formatted value format {:?}", .0)]
pub struct FormattedValueError(Format);

pub trait FormattedValue: Copy + Clone + Sized + Into<GenericFormattedValue> + 'static {
    // rustdoc-stripper-ignore-next
    /// Type which allows building a `FormattedValue` of this format from any raw value.
    type FullRange: FormattedValueFullRange + From<Self>;

    // rustdoc-stripper-ignore-next
    /// The `Signed` type for this `FormattedValue`.
    type Signed;

    #[doc(alias = "get_default_format")]
    fn default_format() -> Format;

    #[doc(alias = "get_format")]
    fn format(&self) -> Format;

    unsafe fn into_raw_value(self) -> i64;

    fn into_positive(self) -> Self::Signed;
    fn into_negative(self) -> Self::Signed;

    fn into_signed(self, sign: i32) -> Self::Signed {
        if sign < 0 {
            self.into_negative()
        } else {
            self.into_positive()
        }
    }
}

// rustdoc-stripper-ignore-next
/// A [`FormattedValue`] which can be built from any raw value.
///
/// # Examples:
///
/// - `GenericFormattedValue` is the `FormattedValueFullRange` type for `GenericFormattedValue`.
/// - `Undefined` is the `FormattedValueFullRange` type for `Undefined`.
/// - `Option<Percent>` is the `FormattedValueFullRange` type for `Percent`.
pub trait FormattedValueFullRange: FormattedValue + TryFrom<GenericFormattedValue> {
    unsafe fn from_raw(format: Format, value: i64) -> Self;
}

// rustdoc-stripper-ignore-next
/// A trait implemented on the intrinsic type of a `FormattedValue`.
///
/// # Examples
///
/// - `GenericFormattedValue` is the intrinsic type for `GenericFormattedValue`.
/// - `Undefined` is the intrinsic type for `Undefined`.
/// - `Bytes` is the intrinsic type for `Option<Bytes>`.
pub trait FormattedValueIntrinsic: FormattedValue {}

pub trait SpecificFormattedValue: FormattedValue {}

pub trait SpecificFormattedValueFullRange: FormattedValueFullRange {}

// rustdoc-stripper-ignore-next
/// A trait implemented on the intrinsic type of a `SpecificFormattedValue`.
///
/// # Examples
///
/// - `Undefined` is the intrinsic type for `Undefined`.
/// - `Bytes` is the intrinsic type for `Option<Bytes>`.
pub trait SpecificFormattedValueIntrinsic: TryFromGlib<i64> + FormattedValueIntrinsic {}

// rustdoc-stripper-ignore-next
/// A trait implemented on types which can hold [`FormattedValue`]s compatible with parameter `F`.
///
/// This trait is auto-implemented based on [`FormattedValue`]s additional traits
/// such as [`SpecificFormattedValue`].
///
/// # Example:
///
/// Consider the following function:
///
/// ```rust
/// # use gstreamer::{ClockTime, CompatibleFormattedValue, FormattedValue, GenericFormattedValue};
/// fn with_compatible_formats<V: FormattedValue>(
///     arg1: V,
///     arg2: impl CompatibleFormattedValue<V>,
/// ) {
///     // This is required to access arg2 as a FormattedValue:
///     let _arg2 = arg2.try_into_checked(arg1).unwrap();
/// }
///
/// // This is Ok because arg1 is a ClockTime and arg2 is
/// // an Option<ClockTime> which are compatible format-wise.
/// with_compatible_formats(ClockTime::ZERO, ClockTime::NONE);
///
/// // This is Ok because arg1 is a ClockTime and arg2 is
/// // a GenericFormattedValue which are compatible format-wise.
/// with_compatible_formats(
///     ClockTime::ZERO,
///     GenericFormattedValue::Time(None),
/// );
/// ```
///
/// Users are able to call the function with arguments:
///
/// 1. of the same type (e.g. `ClockTime`),
/// 2. of different types, but able to hold a value of the same [`Format`]
///    (e.g. `ClockTime` and `Option<ClockTime>`).
/// 3. One of a Formatted Value (specific or generic), the other being
///    a `GenericFormattedValue`.
///
/// Format compatibility for cases 1 and 2 is enforced by
/// the type system, while case 3 will be checked at runtime time.
///
/// ```compile_fail
/// # use gstreamer::{ClockTime, CompatibleFormattedValue, FormattedValue, format::Bytes};
/// # fn with_compatible_formats<V: FormattedValue>(
/// #     arg1: V,
/// #     arg2: impl CompatibleFormattedValue<V>,
/// # ) {}
/// // This doesn't compile because the arguments are not compatible:
/// let _ = with_compatible_formats(ClockTime::ZERO, Bytes(Some(42)));
/// ```
///
/// Note: users will not be able use `arg2` directly unless format
/// check succeeds:
///
/// ```compile_fail
/// # use gstreamer::{CompatibleFormattedValue, FormattedValue};
/// fn with_compatible_formats<V: FormattedValue>(
///     arg1: V,
///     arg2: impl CompatibleFormattedValue<V>,
/// ) {
///     // This doesn't compile because arg2 hasn't been checked:
///     let _format = arg2.format();
/// }
/// ```
pub trait CompatibleFormattedValue<V: FormattedValue> {
    type Original: FormattedValue;

    // rustdoc-stripper-ignore-next
    /// Returns `Ok(self)` with its type restored if it is compatible with the format of `other`.
    ///
    /// When used with compatible [`SpecificFormattedValue`]s, checks
    /// are enforced by the type system, no runtime checks are performed.
    ///
    /// When used with [`FormattedValue`] / [`GenericFormattedValue`] and
    /// vice versa, a runtime format check is performed. If the check fails,
    /// `Err(FormattedValueError)` is returned.
    fn try_into_checked(self, other: V) -> Result<Self::Original, FormattedValueError>;

    // rustdoc-stripper-ignore-next
    /// Returns `Ok(self)` with its type restored if it is compatible with the format of `V`.
    ///
    /// When possible, prefer using [`Self::try_into_checked`] which
    /// reduces the risk of missuse.
    ///
    /// When used with compatible [`SpecificFormattedValue`]s, checks
    /// are enforced by the type system, no runtime checks are performed.
    ///
    /// When used with [`SpecificFormattedValue`] as a parameter and
    /// a [`GenericFormattedValue`] as `Self`, a runtime check is perfomed
    /// against the default format of the parameter. If the check fails,
    /// `Err(FormattedValueError)` is returned.
    ///
    /// When used with [`GenericFormattedValue`] as a parameter and
    /// a [`SpecificFormattedValue`] as `Self`, the `format` argument
    /// used. If the check fails, `Err(FormattedValueError)` is returned.
    fn try_into_checked_explicit(
        self,
        format: Format,
    ) -> Result<Self::Original, FormattedValueError>;
}

impl<T, V> CompatibleFormattedValue<V> for T
where
    V: SpecificFormattedValue,
    T: SpecificFormattedValue<FullRange = V::FullRange>,
{
    type Original = Self;
    fn try_into_checked(self, _other: V) -> Result<Self, FormattedValueError> {
        skip_assert_initialized!();
        Ok(self)
    }

    fn try_into_checked_explicit(
        self,
        _format: Format,
    ) -> Result<Self::Original, FormattedValueError> {
        skip_assert_initialized!();
        Ok(self)
    }
}

impl<T: SpecificFormattedValue> CompatibleFormattedValue<GenericFormattedValue> for T {
    type Original = Self;
    fn try_into_checked(self, other: GenericFormattedValue) -> Result<Self, FormattedValueError> {
        skip_assert_initialized!();
        if self.format() == other.format() {
            Ok(self)
        } else {
            Err(FormattedValueError(self.format()))
        }
    }

    fn try_into_checked_explicit(
        self,
        format: Format,
    ) -> Result<Self::Original, FormattedValueError> {
        skip_assert_initialized!();
        if self.format() == format {
            Ok(self)
        } else {
            Err(FormattedValueError(self.format()))
        }
    }
}

impl<V: SpecificFormattedValue> CompatibleFormattedValue<V> for GenericFormattedValue {
    type Original = Self;
    fn try_into_checked(self, _other: V) -> Result<Self, FormattedValueError> {
        skip_assert_initialized!();
        if self.format() == V::default_format() {
            Ok(self)
        } else {
            Err(FormattedValueError(self.format()))
        }
    }

    fn try_into_checked_explicit(
        self,
        _format: Format,
    ) -> Result<Self::Original, FormattedValueError> {
        skip_assert_initialized!();
        if self.format() == V::default_format() {
            Ok(self)
        } else {
            Err(FormattedValueError(self.format()))
        }
    }
}

impl CompatibleFormattedValue<GenericFormattedValue> for GenericFormattedValue {
    type Original = Self;
    fn try_into_checked(self, other: GenericFormattedValue) -> Result<Self, FormattedValueError> {
        skip_assert_initialized!();
        if self.format() == other.format() {
            Ok(self)
        } else {
            Err(FormattedValueError(self.format()))
        }
    }

    fn try_into_checked_explicit(
        self,
        format: Format,
    ) -> Result<Self::Original, FormattedValueError> {
        skip_assert_initialized!();
        if self.format() == format {
            Ok(self)
        } else {
            Err(FormattedValueError(self.format()))
        }
    }
}

impl FormattedValue for GenericFormattedValue {
    type FullRange = GenericFormattedValue;

    // The intrinsic value for `GenericFormattedValue` is also
    // `GenericFormattedValue`. We can't dissociate the `Option`
    // from the variants' inner type since they are not all `Option`s.
    type Signed = Signed<GenericFormattedValue>;

    fn default_format() -> Format {
        Format::Undefined
    }

    fn format(&self) -> Format {
        self.format()
    }

    unsafe fn into_raw_value(self) -> i64 {
        self.value()
    }

    fn into_positive(self) -> Signed<GenericFormattedValue> {
        Signed::Positive(self)
    }

    fn into_negative(self) -> Signed<GenericFormattedValue> {
        Signed::Negative(self)
    }
}

impl FormattedValueFullRange for GenericFormattedValue {
    unsafe fn from_raw(format: Format, value: i64) -> Self {
        GenericFormattedValue::new(format, value)
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
            Format::Percent => {
                Self::Percent(unsafe { FormattedValueFullRange::from_raw(format, value) })
            }
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

    pub fn is_some(&self) -> bool {
        match *self {
            Self::Undefined(_) => true,
            Self::Default(v) => v.is_some(),
            Self::Bytes(v) => v.is_some(),
            Self::Time(v) => v.is_some(),
            Self::Buffers(v) => v.is_some(),
            Self::Percent(v) => v.is_some(),
            Self::Other(..) => true,
        }
    }

    pub fn is_none(&self) -> bool {
        match *self {
            Self::Undefined(_) => false,
            Self::Default(v) => v.is_none(),
            Self::Bytes(v) => v.is_none(),
            Self::Time(v) => v.is_none(),
            Self::Buffers(v) => v.is_none(),
            Self::Percent(v) => v.is_none(),
            Self::Other(..) => false,
        }
    }
}

impl FormattedValueIntrinsic for GenericFormattedValue {}

impl_common_ops_for_newtype_uint!(Default, u64);
impl_format_value_traits!(Default, Default, Default, u64);
option_glib_newtype_from_to!(Default, u64::MAX);
option_glib_newtype_display!(Default, "(Default)");

impl_common_ops_for_newtype_uint!(Bytes, u64);
impl_format_value_traits!(Bytes, Bytes, Bytes, u64);
option_glib_newtype_from_to!(Bytes, u64::MAX);
option_glib_newtype_display!(Bytes, "bytes");

impl_format_value_traits!(ClockTime, Time, Time, u64);

impl_common_ops_for_newtype_uint!(Buffers, u64);
impl_format_value_traits!(Buffers, Buffers, Buffers, u64);
option_glib_newtype_from_to!(Buffers, Buffers::OFFSET_NONE);
option_glib_newtype_display!(Buffers, "buffers");

impl FormattedValue for Undefined {
    type FullRange = Undefined;
    type Signed = Signed<Undefined>;

    fn default_format() -> Format {
        Format::Undefined
    }

    fn format(&self) -> Format {
        Format::Undefined
    }

    unsafe fn into_raw_value(self) -> i64 {
        self.0
    }

    fn into_positive(self) -> Signed<Undefined> {
        Signed::Positive(self)
    }

    fn into_negative(self) -> Signed<Undefined> {
        Signed::Negative(self)
    }
}

impl FormattedValueFullRange for Undefined {
    unsafe fn from_raw(format: Format, value: i64) -> Self {
        debug_assert_eq!(format, Format::Undefined);
        Undefined(value)
    }
}

impl From<Undefined> for GenericFormattedValue {
    fn from(v: Undefined) -> Self {
        skip_assert_initialized!();
        GenericFormattedValue::Undefined(v)
    }
}

impl TryFrom<GenericFormattedValue> for Undefined {
    type Error = FormattedValueError;

    fn try_from(v: GenericFormattedValue) -> Result<Undefined, Self::Error> {
        skip_assert_initialized!();
        if let GenericFormattedValue::Undefined(v) = v {
            Ok(v)
        } else {
            Err(FormattedValueError(v.format()))
        }
    }
}

impl FormattedValueIntrinsic for Undefined {}

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

impl_common_ops_for_newtype_uint!(Percent, u32);
option_glib_newtype_display!(Percent, "%");

impl FormattedValue for Option<Percent> {
    type FullRange = Option<Percent>;
    type Signed = Option<Signed<Percent>>;

    fn default_format() -> Format {
        Format::Percent
    }

    fn format(&self) -> Format {
        Format::Percent
    }

    unsafe fn into_raw_value(self) -> i64 {
        self.map_or(-1, |v| v.0 as i64)
    }

    fn into_positive(self) -> Option<Signed<Percent>> {
        Some(Signed::Positive(self?))
    }

    fn into_negative(self) -> Option<Signed<Percent>> {
        Some(Signed::Negative(self?))
    }
}

impl FormattedValueFullRange for Option<Percent> {
    unsafe fn from_raw(format: Format, value: i64) -> Self {
        debug_assert_eq!(format, Format::Percent);
        Percent::try_from_glib(value as i64).ok()
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

impl FormattedValue for Percent {
    type FullRange = Option<Percent>;
    type Signed = Signed<Percent>;

    fn default_format() -> Format {
        Format::Percent
    }

    fn format(&self) -> Format {
        Format::Percent
    }

    unsafe fn into_raw_value(self) -> i64 {
        self.0 as i64
    }

    fn into_positive(self) -> Signed<Percent> {
        Signed::Positive(self)
    }

    fn into_negative(self) -> Signed<Percent> {
        Signed::Negative(self)
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
    type Error = FormattedValueError;

    fn try_from(v: GenericFormattedValue) -> Result<Option<Percent>, Self::Error> {
        skip_assert_initialized!();
        if let GenericFormattedValue::Percent(v) = v {
            Ok(v)
        } else {
            Err(FormattedValueError(v.format()))
        }
    }
}

impl FormattedValueIntrinsic for Percent {}
impl SpecificFormattedValue for Option<Percent> {}
impl SpecificFormattedValueFullRange for Option<Percent> {}
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClockTime;

    fn with_compatible_formats<V1, V2>(
        arg1: V1,
        arg2: V2,
    ) -> Result<V2::Original, FormattedValueError>
    where
        V1: FormattedValue,
        V2: CompatibleFormattedValue<V1>,
    {
        skip_assert_initialized!();
        arg2.try_into_checked(arg1)
    }

    #[test]
    fn compatible() {
        assert_eq!(
            with_compatible_formats(ClockTime::ZERO, ClockTime::ZERO),
            Ok(ClockTime::ZERO),
        );
        assert_eq!(
            with_compatible_formats(ClockTime::ZERO, ClockTime::NONE),
            Ok(ClockTime::NONE),
        );
        assert_eq!(
            with_compatible_formats(ClockTime::NONE, ClockTime::ZERO),
            Ok(ClockTime::ZERO),
        );
        assert_eq!(
            with_compatible_formats(
                ClockTime::ZERO,
                GenericFormattedValue::Time(Some(ClockTime::ZERO)),
            ),
            Ok(GenericFormattedValue::Time(Some(ClockTime::ZERO))),
        );
        assert_eq!(
            with_compatible_formats(
                GenericFormattedValue::Time(Some(ClockTime::ZERO)),
                ClockTime::NONE,
            ),
            Ok(ClockTime::NONE),
        );
    }

    #[test]
    fn incompatible() {
        with_compatible_formats(
            ClockTime::ZERO,
            GenericFormattedValue::Buffers(Some(Buffers(42))),
        )
        .unwrap_err();
        with_compatible_formats(
            GenericFormattedValue::Buffers(Some(Buffers(42))),
            ClockTime::NONE,
        )
        .unwrap_err();
    }

    fn with_compatible_explicit<T, V>(arg: V, f: Format) -> Result<V::Original, FormattedValueError>
    where
        T: FormattedValue,
        V: CompatibleFormattedValue<T>,
    {
        skip_assert_initialized!();
        arg.try_into_checked_explicit(f)
    }

    #[test]
    fn compatible_explicit() {
        assert_eq!(
            with_compatible_explicit::<ClockTime, _>(ClockTime::ZERO, Format::Time),
            Ok(ClockTime::ZERO),
        );
        assert_eq!(
            with_compatible_explicit::<ClockTime, _>(ClockTime::NONE, Format::Time),
            Ok(ClockTime::NONE),
        );
        assert_eq!(
            with_compatible_explicit::<ClockTime, _>(ClockTime::ZERO, Format::Time),
            Ok(ClockTime::ZERO),
        );
        assert_eq!(
            with_compatible_explicit::<ClockTime, _>(
                GenericFormattedValue::Time(None),
                Format::Time
            ),
            Ok(GenericFormattedValue::Time(None)),
        );
        assert_eq!(
            with_compatible_explicit::<GenericFormattedValue, _>(ClockTime::NONE, Format::Time),
            Ok(ClockTime::NONE),
        );
    }

    #[test]
    fn incompatible_explicit() {
        with_compatible_explicit::<Buffers, _>(GenericFormattedValue::Time(None), Format::Buffers)
            .unwrap_err();
        with_compatible_explicit::<GenericFormattedValue, _>(Buffers::ZERO, Format::Time)
            .unwrap_err();
        with_compatible_explicit::<GenericFormattedValue, _>(
            GenericFormattedValue::Time(None),
            Format::Buffers,
        )
        .unwrap_err();
    }

    #[test]
    fn signed_optional() {
        let ct_1 = Some(ClockTime::SECOND);

        let signed = ct_1.into_positive().unwrap();
        assert_eq!(signed, Signed::Positive(ClockTime::SECOND));
        assert!(signed.is_positive());
        assert_eq!(signed.positive_or(()).unwrap(), ClockTime::SECOND);
        assert_eq!(signed.positive_or_else(|_| ()).unwrap(), ClockTime::SECOND);
        signed.negative_or(()).unwrap_err();
        assert_eq!(
            signed.negative_or_else(|val| val).unwrap_err(),
            ClockTime::SECOND
        );

        let signed = ct_1.into_negative().unwrap();
        assert_eq!(signed, Signed::Negative(ClockTime::SECOND));
        assert!(signed.is_negative());
        assert_eq!(signed.negative_or(()).unwrap(), ClockTime::SECOND);
        assert_eq!(signed.negative_or_else(|_| ()).unwrap(), ClockTime::SECOND);
        signed.positive_or(()).unwrap_err();
        assert_eq!(
            signed.positive_or_else(|val| val).unwrap_err(),
            ClockTime::SECOND
        );

        let ct_none = ClockTime::NONE;
        assert!(ct_none.into_positive().is_none());
        assert!(ct_none.into_negative().is_none());
    }

    #[test]
    fn signed_mandatory() {
        let ct_1 = ClockTime::SECOND;

        let signed = ct_1.into_positive();
        assert_eq!(signed, Signed::Positive(ct_1));
        assert!(signed.is_positive());
        assert_eq!(signed.positive(), Some(ct_1));
        assert!(!signed.is_negative());
        assert!(signed.negative().is_none());

        let signed = ct_1.into_negative();
        assert_eq!(signed, Signed::Negative(ct_1));
        assert!(signed.is_negative());
        assert_eq!(signed.negative(), Some(ct_1));
        assert!(!signed.is_positive());
        assert!(signed.positive().is_none());

        let und = Undefined(1);

        let signed = und.into_positive();
        assert_eq!(signed, Signed::Positive(und));
        assert!(signed.is_positive());
        assert_eq!(signed.positive(), Some(und));
        assert!(!signed.is_negative());
        assert!(signed.negative().is_none());

        let signed = und.into_negative();
        assert_eq!(signed, Signed::Negative(und));
        assert!(signed.is_negative());
        assert_eq!(signed.negative(), Some(und));
        assert!(!signed.is_positive());
        assert!(signed.positive().is_none());
    }

    #[test]
    fn signed_generic() {
        let ct_1 = GenericFormattedValue::Time(Some(ClockTime::SECOND));
        assert!(ct_1.is_some());

        let signed = ct_1.into_positive();
        assert_eq!(signed, Signed::Positive(ct_1));
        assert!(signed.is_positive());
        assert_eq!(signed.positive(), Some(ct_1));

        let signed = ct_1.into_negative();
        assert_eq!(signed, Signed::Negative(ct_1));
        assert!(signed.is_negative());
        assert_eq!(signed.negative(), Some(ct_1));

        let ct_none = GenericFormattedValue::Time(ClockTime::NONE);
        assert!(ct_none.is_none());

        let signed = ct_none.into_positive();
        assert_eq!(signed, Signed::Positive(ct_none));
        assert!(signed.is_positive());

        let signed = ct_none.into_negative();
        assert_eq!(signed, Signed::Negative(ct_none));
        assert!(signed.is_negative());
    }

    #[test]
    fn signed_roundtrip() {
        let ct_1 = Some(ClockTime::SECOND);
        let raw_ct_1 = unsafe { ct_1.into_raw_value() };

        let signed = unsafe { Option::<ClockTime>::from_raw(Format::Time, raw_ct_1) }
            .into_signed(1)
            .unwrap();
        assert_eq!(signed, Signed::Positive(ClockTime::SECOND));
        assert!(signed.is_positive());

        let signed = unsafe { Option::<ClockTime>::from_raw(Format::Time, raw_ct_1) }
            .into_signed(-1)
            .unwrap();
        assert_eq!(signed, Signed::Negative(ClockTime::SECOND));
        assert!(signed.is_negative());

        let ct_none = ClockTime::NONE;
        let raw_ct_none = unsafe { ct_none.into_raw_value() };

        let signed =
            unsafe { Option::<ClockTime>::from_raw(Format::Time, raw_ct_none) }.into_signed(1);
        assert!(signed.is_none());

        let signed =
            unsafe { Option::<ClockTime>::from_raw(Format::Time, raw_ct_none) }.into_signed(-1);
        assert!(signed.is_none());
    }
}
