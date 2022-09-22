// Take a look at the license at the top of the repository in the LICENSE file.

use thiserror::Error;

#[macro_use]
mod macros;

mod clock_time;
pub use clock_time::ClockTime;
#[cfg(feature = "serde")]
mod clock_time_serde;

mod compatible;
pub use compatible::*;

mod generic;
pub use generic::*;

mod signed;
pub use signed::*;

mod specific;
pub use specific::*;

mod undefined;
pub use undefined::*;

use crate::Format;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
#[error("invalid formatted value format {:?}", .0)]
pub struct FormattedValueError(Format);

pub trait FormattedValue: Copy + Clone + Sized + Into<GenericFormattedValue> + 'static {
    // rustdoc-stripper-ignore-next
    /// Type which allows building a `FormattedValue` of this format from any raw value.
    type FullRange: FormattedValueFullRange + From<Self>;

    #[doc(alias = "get_default_format")]
    fn default_format() -> Format;

    #[doc(alias = "get_format")]
    fn format(&self) -> Format;

    // rustdoc-stripper-ignore-next
    /// Returns `true` if this `FormattedValue` represents a defined value.
    fn is_some(&self) -> bool;

    // rustdoc-stripper-ignore-next
    /// Returns `true` if this `FormattedValue` represents an undefined value.
    fn is_none(&self) -> bool {
        !self.is_some()
    }

    unsafe fn into_raw_value(self) -> i64;
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

pub trait FormattedValueNoneBuilder: FormattedValueFullRange {
    // rustdoc-stripper-ignore-next
    /// Returns the `None` value for `Self` as a `FullRange` if such a value can be represented.
    ///
    /// - For `SpecificFormattedValue`s, this results in `Option::<FormattedValueIntrinsic>::None`.
    /// - For `GenericFormattedValue`, this can only be obtained using [`Self::none_for_format`]
    ///   because the `None` is an inner value of some of the variants.
    ///
    /// # Panics
    ///
    /// Panics if `Self` is `GenericFormattedValue` in which case, the `Format` must be known.
    fn none() -> Self;

    // rustdoc-stripper-ignore-next
    /// Returns the `None` value for `Self` if such a value can be represented.
    ///
    /// - For `SpecificFormattedValue`s, this is the same as `Self::none()`
    ///   if the `format` matches the `SpecificFormattedValue`'s format.
    /// - For `GenericFormattedValue` this is the variant for the specified `format`,
    ///   initialized with `None` as a value, if the `format` can represent that value.
    ///
    /// # Panics
    ///
    /// Panics if `None` can't be represented by `Self` for `format` or by the requested
    /// `GenericFormattedValue` variant.
    #[track_caller]
    fn none_for_format(format: Format) -> Self {
        skip_assert_initialized!();
        // This is the default impl. `GenericFormattedValue` must override.
        if Self::default_format() != format {
            panic!(
                "Expected: {:?}, requested {format:?}",
                Self::default_format()
            );
        }

        Self::none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::Displayable;

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
    fn none_builder() {
        let ct_none: Option<ClockTime> = Option::<ClockTime>::none();
        assert!(ct_none.is_none());

        let ct_none: Option<ClockTime> = Option::<ClockTime>::none_for_format(Format::Time);
        assert!(ct_none.is_none());

        let gen_ct_none: GenericFormattedValue =
            GenericFormattedValue::none_for_format(Format::Time);
        assert!(gen_ct_none.is_none());

        assert!(ClockTime::ZERO.is_some());
        assert!(!ClockTime::ZERO.is_none());
    }

    #[test]
    #[should_panic]
    fn none_for_inconsistent_format() {
        let _ = Option::<ClockTime>::none_for_format(Format::Percent);
    }

    #[test]
    #[should_panic]
    fn none_for_unsupported_format() {
        let _ = GenericFormattedValue::none_for_format(Format::Undefined);
    }

    #[test]
    fn none_signed_builder() {
        let ct_none: Option<Signed<ClockTime>> = Option::<ClockTime>::none_signed();
        assert!(ct_none.is_none());

        let ct_none: Option<Signed<ClockTime>> =
            Option::<ClockTime>::none_signed_for_format(Format::Time);
        assert!(ct_none.is_none());

        let gen_ct_none: Signed<GenericFormattedValue> =
            GenericFormattedValue::none_signed_for_format(Format::Time);
        assert!(gen_ct_none.abs().is_none());
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

        let def = Default(1);

        let signed = def.into_positive();
        assert_eq!(signed, Signed::Positive(def));
        assert!(signed.is_positive());
        assert_eq!(signed.positive(), Some(def));
        assert!(!signed.is_negative());
        assert!(signed.negative().is_none());

        let signed = def.into_negative();
        assert_eq!(signed, Signed::Negative(def));
        assert!(signed.is_negative());
        assert_eq!(signed.negative(), Some(def));
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

    #[test]
    fn display_new_types() {
        let bytes = Bytes(42);
        assert_eq!(&format!("{bytes}"), "42 bytes");
        assert_eq!(&format!("{}", bytes.display()), "42 bytes");

        assert_eq!(&format!("{}", Some(bytes).display()), "42 bytes");
        assert_eq!(&format!("{}", Bytes::NONE.display()), "undef. bytes");

        let gv_1 = GenericFormattedValue::Percent(Some(Percent(42)));
        assert_eq!(&format!("{gv_1}"), "42 %");
        assert_eq!(
            &format!("{}", GenericFormattedValue::Percent(None)),
            "undef. %"
        );
    }

    #[test]
    fn display_signed() {
        let p_bytes = Bytes(42).into_positive();
        assert_eq!(&format!("{p_bytes}"), "+42 bytes");
        assert_eq!(&format!("{}", p_bytes.display()), "+42 bytes");

        let some_p_bytes = Some(p_bytes);
        assert_eq!(&format!("{}", some_p_bytes.display()), "+42 bytes");

        let p_some_bytes = Signed::Positive(Some(Bytes(42)));
        assert_eq!(&format!("{}", p_some_bytes.display()), "+42 bytes");

        let n_bytes = Bytes(42).into_negative();
        assert_eq!(&format!("{n_bytes}"), "-42 bytes");
        assert_eq!(&format!("{}", n_bytes.display()), "-42 bytes");

        let some_n_bytes = Some(n_bytes);
        assert_eq!(&format!("{}", some_n_bytes.display()), "-42 bytes");

        let n_some_bytes = Signed::Negative(Some(Bytes(42)));
        assert_eq!(&format!("{}", n_some_bytes.display()), "-42 bytes");

        let p_none_bytes = Signed::Positive(Bytes::NONE);
        assert_eq!(&format!("{}", p_none_bytes.display()), "undef. bytes");
        let n_none_bytes = Signed::Negative(Bytes::NONE);
        assert_eq!(&format!("{}", n_none_bytes.display()), "undef. bytes");

        let none_s_bytes = Option::<Signed<Bytes>>::None;
        assert_eq!(&format!("{}", none_s_bytes.display()), "undef. bytes");
    }
}
