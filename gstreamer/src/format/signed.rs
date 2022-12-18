// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use super::{Format, FormattedValueNoneBuilder};
use crate::utils::Displayable;

// rustdoc-stripper-ignore-next
/// A signed wrapper.
///
/// This wrapper allows representing a signed value from a type
/// which is originaly unsigned. In C APIs, this is represented
/// by a tuple with a signed integer positive or negative and
/// the absolute value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Signed<T> {
    Negative(T),
    Positive(T),
}

impl<T> Signed<T> {
    #[inline]
    pub fn is_positive(self) -> bool {
        matches!(self, Signed::Positive(_))
    }

    // rustdoc-stripper-ignore-next
    /// Returns `Some(value)`, where `value` is the inner value,
    /// if `self` is positive.
    #[inline]
    pub fn positive(self) -> Option<T> {
        match self {
            Signed::Positive(val) => Some(val),
            Signed::Negative(_) => None,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transforms the `Signed<T>` into a `Result<T, E>`,
    /// mapping `Positive(v)` to `Ok(v)` and `Negative(_)` to `Err(err)`.
    #[inline]
    pub fn positive_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Signed::Positive(val) => Ok(val),
            Signed::Negative(_) => Err(err),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transforms the `Signed<T>` into a `Result<T, E>`,
    /// mapping `Positive(v)` to `Ok(v)` and `Negative(v)` to `Err(err(v))`.
    #[inline]
    pub fn positive_or_else<E, F: FnOnce(T) -> E>(self, err: F) -> Result<T, E> {
        match self {
            Signed::Positive(val) => Ok(val),
            Signed::Negative(val) => Err(err(val)),
        }
    }

    #[inline]
    pub fn is_negative(self) -> bool {
        matches!(self, Signed::Negative(_))
    }

    // rustdoc-stripper-ignore-next
    /// Returns `Some(value)`, where `value` is the inner value,
    /// if `self` is negative.
    #[inline]
    pub fn negative(self) -> Option<T> {
        match self {
            Signed::Negative(val) => Some(val),
            Signed::Positive(_) => None,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transforms the `Signed<T>` into a `Result<T, E>`,
    /// mapping `Negative(v)` to `Ok(v)` and `Positive(_)` to `Err(err)`.
    #[inline]
    pub fn negative_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Signed::Negative(val) => Ok(val),
            Signed::Positive(_) => Err(err),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transforms the `Signed<T>` into a `Result<T, E>`,
    /// mapping `Negative(v)` to `Ok(v)` and `Positive(_)` to `Err(err(v))`.
    #[inline]
    pub fn negative_or_else<E, F: FnOnce(T) -> E>(self, err: F) -> Result<T, E> {
        match self {
            Signed::Negative(val) => Ok(val),
            Signed::Positive(val) => Err(err(val)),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the absolute value of `self`.
    #[inline]
    pub fn abs(self) -> T {
        match self {
            Signed::Positive(val) | Signed::Negative(val) => val,
        }
    }
}

impl<T> std::ops::Neg for Signed<T> {
    type Output = Signed<T>;

    #[inline]
    fn neg(self) -> Self {
        match self {
            Signed::Positive(val) => Signed::Negative(val),
            Signed::Negative(val) => Signed::Positive(val),
        }
    }
}

pub trait SignedIntrinsic {}

impl<T> fmt::Display for Signed<T>
where
    T: fmt::Display + SignedIntrinsic,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Write;

        let (sign, val) = match self {
            Signed::Positive(val) => ('+', val),
            Signed::Negative(val) => ('-', val),
        };

        f.write_char(sign)?;
        fmt::Display::fmt(&val, f)
    }
}

impl<T> Displayable for Signed<T>
where
    T: fmt::Display + SignedIntrinsic,
{
    type DisplayImpl = Signed<T>;

    fn display(self) -> Self::DisplayImpl {
        self
    }
}

impl<T> Signed<Option<T>> {
    // rustdoc-stripper-ignore-next
    /// Transposes a `Signed` `Option` into an `Option` of a `Signed`.
    ///
    /// Note that if the inner value was `None`, the sign is lost.
    #[inline]
    pub fn transpose(self) -> Option<Signed<T>> {
        use Signed::*;

        match self {
            Positive(Some(val)) => Some(Positive(val)),
            Negative(Some(val)) => Some(Negative(val)),
            _ => None,
        }
    }
}

pub struct DisplayableOptionSigned<T>(Option<Signed<T>>);

impl<T> fmt::Display for DisplayableOptionSigned<T>
where
    T: fmt::Display + SignedIntrinsic,
    Option<T>: Displayable,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(ref signed) => fmt::Display::fmt(signed, f),
            None => fmt::Display::fmt(&Option::<T>::None.display(), f),
        }
    }
}

impl<T> Displayable for Option<Signed<T>>
where
    T: fmt::Display + SignedIntrinsic,
    Option<T>: Displayable,
{
    type DisplayImpl = DisplayableOptionSigned<T>;

    fn display(self) -> Self::DisplayImpl {
        DisplayableOptionSigned(self)
    }
}

impl<T> Displayable for Signed<Option<T>>
where
    T: fmt::Display + SignedIntrinsic,
    Option<T>: Displayable,
{
    type DisplayImpl = DisplayableOptionSigned<T>;

    fn display(self) -> Self::DisplayImpl {
        DisplayableOptionSigned(self.transpose())
    }
}

// rustdoc-stripper-ignore-next
/// A trait implemented on unsigned types which can be converted into [`crate::Signed`]s.
pub trait UnsignedIntoSigned: Copy + Sized {
    type Signed;

    // rustdoc-stripper-ignore-next
    /// Converts `self` into a `Signed` matching the given `sign`.
    fn into_signed(self, sign: i32) -> Self::Signed {
        if sign.is_positive() {
            self.into_positive()
        } else {
            self.into_negative()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Converts `self` into a `Signed::Positive`.
    fn into_positive(self) -> Self::Signed;

    // rustdoc-stripper-ignore-next
    /// Converts `self` into a `Signed::Negative`.
    fn into_negative(self) -> Self::Signed;
}

impl_unsigned_int_into_signed!(u64);
impl_signed_ops!(u64);
impl_signed_div_mul!(u64);
impl_signed_int_into_signed!(u64);

impl_unsigned_int_into_signed!(u32);
impl_signed_ops!(u32);
impl_signed_div_mul!(u32);
impl_signed_int_into_signed!(u32);

impl_unsigned_int_into_signed!(usize);
impl_signed_ops!(usize);
impl_signed_div_mul!(usize);
impl_signed_int_into_signed!(usize);

pub trait NoneSignedBuilder: FormattedValueNoneBuilder {
    type Signed;

    // rustdoc-stripper-ignore-next
    /// Returns the `None` value for `Self` as a `Signed<FullRange>` if such a value can be represented.
    ///
    /// See details in [`FormattedValueNoneBuilder::none`].
    ///
    /// # Panics
    ///
    /// Panics if `Self` is `GenericFormattedValue` in which case, the `Format` must be known.
    fn none_signed() -> Self::Signed;

    // rustdoc-stripper-ignore-next
    /// Returns the `None` value for `Self` as a `Signed<FullRange>`, if such a value can be represented.
    ///
    /// See details in [`FormattedValueNoneBuilder::none_for_format`].
    ///
    /// # Panics
    ///
    /// Panics if `None` can't be represented by `Self` for `format` or by the requested
    /// `GenericFormattedValue` variant.
    fn none_signed_for_format(format: Format) -> Self::Signed;
}

impl<T> NoneSignedBuilder for T
where
    T: UnsignedIntoSigned + FormattedValueNoneBuilder,
{
    type Signed = <T as UnsignedIntoSigned>::Signed;

    #[inline]
    fn none_signed() -> Self::Signed {
        Self::none().into_positive()
    }

    #[inline]
    fn none_signed_for_format(format: Format) -> Self::Signed {
        skip_assert_initialized!();
        Self::none_for_format(format).into_positive()
    }
}
