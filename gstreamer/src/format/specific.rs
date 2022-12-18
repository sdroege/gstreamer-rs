// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::{FromGlib, GlibNoneError, IntoGlib, OptionIntoGlib, TryFromGlib};

use super::{
    Format, FormattedValue, FormattedValueError, FormattedValueFullRange, FormattedValueIntrinsic,
    FormattedValueNoneBuilder, GenericFormattedValue,
};

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

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Buffers(u64);
impl Buffers {
    #[doc(alias = "GST_BUFFER_OFFSET_NONE")]
    pub const OFFSET_NONE: u64 = ffi::GST_BUFFER_OFFSET_NONE;
    pub const MAX: Self = Self(Self::OFFSET_NONE - 1);
}

impl Buffers {
    // rustdoc-stripper-ignore-next
    /// Builds a new `Buffers` formatted value with the provided buffers count.
    ///
    /// # Panics
    ///
    /// Panics if the provided count equals `u64::MAX`,
    /// which is reserved for `None` in C.
    #[track_caller]
    #[inline]
    pub const fn from_u64(buffers: u64) -> Self {
        if buffers == ffi::GST_BUFFER_OFFSET_NONE {
            panic!("`Buffers` value out of range");
        }

        Buffers(buffers)
    }

    // rustdoc-stripper-ignore-next
    /// Builds a new `Buffers` formatted value with the provided buffers count.
    ///
    /// # Panics
    ///
    /// Panics if the provided count equals `u64::MAX`,
    /// which is reserved for `None` in C.
    #[track_caller]
    #[inline]
    pub fn from_usize(buffers: usize) -> Self {
        Buffers::from_u64(buffers.try_into().unwrap())
    }
}

impl_common_ops_for_newtype_uint!(Buffers, u64);
impl_signed_div_mul!(Buffers, u64);
impl_signed_int_into_signed!(Buffers, u64);
impl_format_value_traits!(Buffers, Buffers, Buffers, u64);
option_glib_newtype_from_to!(Buffers, Buffers::OFFSET_NONE);
glib_newtype_display!(Buffers, DisplayableOptionBuffers, Format::Buffers);

// FIXME `functions in traits cannot be const` (rustc 1.64.0)
// rustdoc-stripper-ignore-next
/// `Buffers` formatted value constructor trait.
pub trait BuffersFormatConstructor {
    // rustdoc-stripper-ignore-next
    /// Builds a `Buffers` formatted value from `self`.
    fn buffers(self) -> Buffers;
}

impl BuffersFormatConstructor for u64 {
    #[track_caller]
    #[inline]
    fn buffers(self) -> Buffers {
        Buffers::from_u64(self)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Bytes(u64);
impl Bytes {
    #[allow(non_upper_case_globals)]
    // rustdoc-stripper-ignore-next
    /// 1 kibibyte (1024).
    #[allow(non_upper_case_globals)]
    pub const KiB: Self = Self(1024);
    // rustdoc-stripper-ignore-next
    /// 1 mebibyte (1024 * 1024).
    #[allow(non_upper_case_globals)]
    pub const MiB: Self = Self(1024 * 1024);
    // rustdoc-stripper-ignore-next
    /// 1 gibibyte (1024 * 1024 * 1024).
    #[allow(non_upper_case_globals)]
    pub const GiB: Self = Self(1024 * 1024 * 1024);
    pub const MAX: Self = Self(u64::MAX - 1);
}

impl Bytes {
    // rustdoc-stripper-ignore-next
    /// Builds a new `Bytes` formatted value with the provided bytes count.
    ///
    /// # Panics
    ///
    /// Panics if the provided count equals `u64::MAX`,
    /// which is reserved for `None` in C.
    #[track_caller]
    #[inline]
    pub const fn from_u64(bytes: u64) -> Self {
        if bytes == u64::MAX {
            panic!("`Bytes` value out of range");
        }

        Bytes(bytes)
    }

    // rustdoc-stripper-ignore-next
    /// Builds a new `Bytes` formatted value with the provided bytes count.
    ///
    /// # Panics
    ///
    /// Panics if the provided count equals `u64::MAX`,
    /// which is reserved for `None` in C.
    #[track_caller]
    #[inline]
    pub fn from_usize(bytes: usize) -> Self {
        // FIXME can't use `try_into` in `const` (rustc 1.64.0)
        Bytes::from_u64(bytes.try_into().unwrap())
    }
}

impl_common_ops_for_newtype_uint!(Bytes, u64);
impl_signed_div_mul!(Bytes, u64);
impl_signed_int_into_signed!(Bytes, u64);
impl_format_value_traits!(Bytes, Bytes, Bytes, u64);
option_glib_newtype_from_to!(Bytes, u64::MAX);
glib_newtype_display!(Bytes, DisplayableOptionBytes, Format::Bytes);

// FIXME `functions in traits cannot be const` (rustc 1.64.0)
// rustdoc-stripper-ignore-next
/// `Bytes` formatted value constructor trait.
///
/// These constructors use the [unambiguous conventions] for byte units.
///
/// [unambiguous conventions]: https://en.wikipedia.org/wiki/Byte#Multiple-byte_units
pub trait BytesFormatConstructor {
    // rustdoc-stripper-ignore-next
    /// Builds a `Bytes` formatted value from `self`.
    fn bytes(self) -> Bytes;

    // rustdoc-stripper-ignore-next
    /// Builds a `Bytes` formatted value from `self` interpreted as kibibytes (1024).
    fn kibibytes(self) -> Bytes;

    // rustdoc-stripper-ignore-next
    /// Builds a `Bytes` formatted value from `self` interpreted as mebibytes (1024²).
    fn mebibytes(self) -> Bytes;

    // rustdoc-stripper-ignore-next
    /// Builds a `Bytes` formatted value from `self` interpreted as gibibytes (1024³).
    fn gibibytes(self) -> Bytes;
}

impl BytesFormatConstructor for u64 {
    #[track_caller]
    #[inline]
    fn bytes(self) -> Bytes {
        Bytes::from_u64(self)
    }

    #[track_caller]
    #[inline]
    fn kibibytes(self) -> Bytes {
        Bytes::from_u64(self * 1024)
    }

    #[track_caller]
    #[inline]
    fn mebibytes(self) -> Bytes {
        Bytes::from_u64(self * 1024 * 1024)
    }

    #[track_caller]
    #[inline]
    fn gibibytes(self) -> Bytes {
        Bytes::from_u64(self * 1024 * 1024 * 1024)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Default(u64);
impl Default {
    pub const MAX: Self = Self(u64::MAX - 1);
}

impl Default {
    // rustdoc-stripper-ignore-next
    /// Builds a new `Default` formatted value with the provided quantity.
    ///
    /// # Panics
    ///
    /// Panics if the provided quantity equals `u64::MAX`,
    /// which is reserved for `None` in C.
    #[track_caller]
    #[inline]
    pub const fn from_u64(quantity: u64) -> Self {
        if quantity == u64::MAX {
            panic!("`Default` value out of range");
        }

        Default(quantity)
    }

    // rustdoc-stripper-ignore-next
    /// Builds a new `Default` formatted value with the provided quantity.
    ///
    /// # Panics
    ///
    /// Panics if the provided quantity equals `u64::MAX`,
    /// which is reserved for `None` in C.
    #[track_caller]
    #[inline]
    pub fn from_usize(quantity: usize) -> Self {
        // FIXME can't use `try_into` in `const` (rustc 1.64.0)
        Default::from_u64(quantity.try_into().unwrap())
    }
}

impl_common_ops_for_newtype_uint!(Default, u64);
impl_signed_div_mul!(Default, u64);
impl_signed_int_into_signed!(Default, u64);
impl_format_value_traits!(Default, Default, Default, u64);
option_glib_newtype_from_to!(Default, u64::MAX);
glib_newtype_display!(Default, DisplayableOptionDefault, Format::Default);

// FIXME `functions in traits cannot be const` (rustc 1.64.0)
// rustdoc-stripper-ignore-next
/// `Default` formatted value constructor trait.
pub trait DefaultFormatConstructor {
    // rustdoc-stripper-ignore-next
    /// Builds a `Default` formatted value from `self`.
    fn default_format(self) -> Default;
}

impl DefaultFormatConstructor for u64 {
    #[track_caller]
    #[inline]
    fn default_format(self) -> Default {
        Default::from_u64(self)
    }
}

pub type Time = super::ClockTime;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Percent(u32);
impl Percent {
    #[doc(alias = "GST_FORMAT_PERCENT_MAX")]
    pub const MAX: Self = Self(ffi::GST_FORMAT_PERCENT_MAX as u32);
    #[doc(alias = "GST_FORMAT_PERCENT_SCALE")]
    pub const SCALE: Self = Self(ffi::GST_FORMAT_PERCENT_SCALE as u32);

    // rustdoc-stripper-ignore-next
    /// Builds a new `Percent` with the provided percent value.
    ///
    /// # Panics
    ///
    /// Panics if the provided value is larger than 100.
    #[track_caller]
    #[inline]
    pub const fn from_percent(percent: u32) -> Self {
        if percent > 100 {
            panic!("`Percent` value out of range");
        }

        Percent(ffi::GST_FORMAT_PERCENT_SCALE as u32 * percent)
    }

    // rustdoc-stripper-ignore-next
    /// Builds a new `Percent` with the provided parts per million value.
    ///
    /// # Panics
    ///
    /// Panics if the provided value is larger than [`Self::MAX`].
    #[track_caller]
    #[inline]
    pub const fn from_ppm(ppm: u32) -> Self {
        if ppm > ffi::GST_FORMAT_PERCENT_MAX as u32 {
            panic!("`Percent` ppm value out of range");
        }

        Percent(ppm)
    }

    // rustdoc-stripper-ignore-next
    /// Builds a new `Percent` with the provided ratio.
    ///
    /// # Panics
    ///
    /// Panics if the provided radio is out of the range [0.0, 1.0].
    #[track_caller]
    #[inline]
    pub fn from_ratio(ratio: f32) -> Self {
        // FIXME floating point arithmetic is not allowed in constant functions (rustc 1.64.0)
        Percent::try_from(ratio).expect("`Percent` ratio out of range")
    }
}

impl_common_ops_for_newtype_uint!(Percent, u32, one: ffi::GST_FORMAT_PERCENT_SCALE as u32);
impl_signed_div_mul!(Percent, u32);
impl_signed_int_into_signed!(Percent, u32);

impl FormattedValue for Option<Percent> {
    type FullRange = Option<Percent>;

    #[inline]
    fn default_format() -> Format {
        Format::Percent
    }

    #[inline]
    fn format(&self) -> Format {
        Format::Percent
    }

    #[inline]
    fn is_some(&self) -> bool {
        Option::is_some(self)
    }

    #[inline]
    unsafe fn into_raw_value(self) -> i64 {
        self.map_or(-1, |v| v.0 as i64)
    }
}

impl FormattedValueFullRange for Option<Percent> {
    #[inline]
    unsafe fn from_raw(format: Format, value: i64) -> Self {
        debug_assert_eq!(format, Format::Percent);
        Percent::try_from_glib(value).ok()
    }
}

impl From<Option<Percent>> for GenericFormattedValue {
    #[inline]
    fn from(v: Option<Percent>) -> Self {
        skip_assert_initialized!();
        GenericFormattedValue::Percent(v)
    }
}

impl From<Percent> for GenericFormattedValue {
    #[inline]
    fn from(v: Percent) -> Self {
        skip_assert_initialized!();
        GenericFormattedValue::Percent(Some(v))
    }
}

impl FormattedValue for Percent {
    type FullRange = Option<Percent>;

    #[inline]
    fn default_format() -> Format {
        Format::Percent
    }

    #[inline]
    fn format(&self) -> Format {
        Format::Percent
    }

    #[inline]
    fn is_some(&self) -> bool {
        true
    }

    #[inline]
    unsafe fn into_raw_value(self) -> i64 {
        self.0 as i64
    }
}

impl TryFrom<u64> for Percent {
    type Error = GlibNoneError;

    #[inline]
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

impl TryFrom<u32> for Percent {
    type Error = FormattedValueError;

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        skip_assert_initialized!();
        if value > ffi::GST_FORMAT_PERCENT_MAX as u32 {
            Err(FormattedValueError(Format::Percent))
        } else {
            Ok(Percent(value))
        }
    }
}

impl TryFrom<GenericFormattedValue> for Option<Percent> {
    type Error = FormattedValueError;

    #[inline]
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
impl FormattedValueNoneBuilder for Option<Percent> {
    #[inline]
    fn none() -> Option<Percent> {
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("value out of range")]
pub struct TryPercentFromFloatError(());

impl TryFrom<f64> for Percent {
    type Error = TryPercentFromFloatError;

    #[inline]
    fn try_from(v: f64) -> Result<Self, Self::Error> {
        skip_assert_initialized!();
        if v < 0.0 || v > 1.0 {
            Err(TryPercentFromFloatError(()))
        } else {
            Ok(Percent(
                (v * ffi::GST_FORMAT_PERCENT_MAX as f64).round() as u32
            ))
        }
    }
}

impl TryFrom<f32> for Percent {
    type Error = TryPercentFromFloatError;

    #[inline]
    fn try_from(v: f32) -> Result<Self, Self::Error> {
        skip_assert_initialized!();
        if v < 0.0 || v > 1.0 {
            Err(TryPercentFromFloatError(()))
        } else {
            Ok(Percent(
                (v * ffi::GST_FORMAT_PERCENT_MAX as f32).round() as u32
            ))
        }
    }
}

// FIXME `functions in traits cannot be const` (rustc 1.64.0)
// rustdoc-stripper-ignore-next
/// `Percent` formatted value from integer constructor trait.
pub trait PercentFormatIntegerConstructor {
    // rustdoc-stripper-ignore-next
    /// Builds a `Percent` formatted value from `self` interpreted as a percent.
    fn percent(self) -> Percent;

    // rustdoc-stripper-ignore-next
    /// Builds a `Percent` formatted value from `self` interpreted as parts per million.
    fn ppm(self) -> Percent;
}

impl PercentFormatIntegerConstructor for u32 {
    #[track_caller]
    #[inline]
    fn percent(self) -> Percent {
        Percent::from_percent(self)
    }

    #[track_caller]
    #[inline]
    fn ppm(self) -> Percent {
        Percent::from_ppm(self)
    }
}

// FIXME `functions in traits cannot be const` (rustc 1.64.0)
// rustdoc-stripper-ignore-next
/// `Percent` formatted value from float constructor trait.
pub trait PercentFormatFloatConstructor {
    // rustdoc-stripper-ignore-next
    /// Builds a `Percent` formatted value from `self` interpreted as a ratio.
    fn percent_ratio(self) -> Percent;
}

impl PercentFormatFloatConstructor for f32 {
    #[track_caller]
    #[inline]
    fn percent_ratio(self) -> Percent {
        Percent::try_from(self).unwrap()
    }
}

impl std::fmt::Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&(self.0 as f32 / (*Percent::SCALE) as f32), f)?;
        f.write_str(" %")
    }
}

impl crate::utils::Displayable for Percent {
    type DisplayImpl = Self;
    fn display(self) -> Self {
        self
    }
}
pub struct DisplayableOptionPercent(Option<Percent>);

impl std::fmt::Display for DisplayableOptionPercent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(val) = self.0.as_ref() {
            std::fmt::Display::fmt(val, f)
        } else {
            f.write_str("undef. %")
        }
    }
}

impl crate::utils::Displayable for Option<Percent> {
    type DisplayImpl = DisplayableOptionPercent;
    fn display(self) -> Self::DisplayImpl {
        DisplayableOptionPercent(self)
    }
}
