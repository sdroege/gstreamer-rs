// Take a look at the license at the top of the repository in the LICENSE file.

use super::{
    Format, FormattedValue, FormattedValueError, GenericFormattedValue, SpecificFormattedValue,
};

// rustdoc-stripper-ignore-next
/// A trait implemented on types which can hold [`FormattedValue`]s compatible with parameter `F`.
///
/// This trait is auto-implemented based on [`FormattedValue`]s additional traits
/// such as [`SpecificFormattedValue`].
///
/// # Example
///
/// Consider the following function:
///
/// ```rust
/// # use gstreamer::format::{ClockTime, CompatibleFormattedValue, FormattedValue, GenericFormattedValue};
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
    #[inline]
    fn try_into_checked(self, _other: V) -> Result<Self, FormattedValueError> {
        skip_assert_initialized!();
        Ok(self)
    }

    #[inline]
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
    #[inline]
    fn try_into_checked(self, other: GenericFormattedValue) -> Result<Self, FormattedValueError> {
        skip_assert_initialized!();
        if self.format() == other.format() {
            Ok(self)
        } else {
            Err(FormattedValueError(self.format()))
        }
    }

    #[inline]
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
    #[inline]
    fn try_into_checked(self, _other: V) -> Result<Self, FormattedValueError> {
        skip_assert_initialized!();
        if self.format() == V::default_format() {
            Ok(self)
        } else {
            Err(FormattedValueError(self.format()))
        }
    }

    #[inline]
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
