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
pub struct Buffers(pub u64);
impl Buffers {
    #[doc(alias = "GST_BUFFER_OFFSET_NONE")]
    pub const OFFSET_NONE: u64 = ffi::GST_BUFFER_OFFSET_NONE;
    pub const MAX: Self = Self(Self::OFFSET_NONE - 1);
}

impl_common_ops_for_newtype_uint!(Buffers, u64);
impl_signed_div_mul!(Buffers, u64);
impl_format_value_traits!(Buffers, Buffers, Buffers, u64);
option_glib_newtype_from_to!(Buffers, Buffers::OFFSET_NONE);
glib_newtype_display!(Buffers, DisplayableOptionBuffers, Format::Buffers);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Bytes(pub u64);
impl Bytes {
    pub const MAX: Self = Self(u64::MAX - 1);
}

impl_common_ops_for_newtype_uint!(Bytes, u64);
impl_signed_div_mul!(Bytes, u64);
impl_format_value_traits!(Bytes, Bytes, Bytes, u64);
option_glib_newtype_from_to!(Bytes, u64::MAX);
glib_newtype_display!(Bytes, DisplayableOptionBytes, Format::Bytes);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Default(pub u64);
impl Default {
    pub const MAX: Self = Self(u64::MAX - 1);
}

impl_common_ops_for_newtype_uint!(Default, u64);
impl_signed_div_mul!(Default, u64);
impl_format_value_traits!(Default, Default, Default, u64);
option_glib_newtype_from_to!(Default, u64::MAX);
glib_newtype_display!(Default, DisplayableOptionDefault, Format::Default);

pub type Time = super::ClockTime;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Percent(pub u32);
impl Percent {
    #[doc(alias = "GST_FORMAT_PERCENT_MAX")]
    pub const MAX: Self = Self(ffi::GST_FORMAT_PERCENT_MAX as u32);
    #[doc(alias = "GST_FORMAT_PERCENT_SCALE")]
    pub const SCALE: u32 = ffi::GST_FORMAT_PERCENT_SCALE as u32;
}

impl_common_ops_for_newtype_uint!(Percent, u32);
impl_signed_div_mul!(Percent, u32);

impl FormattedValue for Option<Percent> {
    type FullRange = Option<Percent>;

    fn default_format() -> Format {
        Format::Percent
    }

    fn format(&self) -> Format {
        Format::Percent
    }

    fn is_some(&self) -> bool {
        Option::is_some(self)
    }

    unsafe fn into_raw_value(self) -> i64 {
        self.map_or(-1, |v| v.0 as i64)
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

    fn default_format() -> Format {
        Format::Percent
    }

    fn format(&self) -> Format {
        Format::Percent
    }

    fn is_some(&self) -> bool {
        true
    }

    unsafe fn into_raw_value(self) -> i64 {
        self.0 as i64
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

impl TryFrom<u32> for Percent {
    type Error = FormattedValueError;

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
    fn none() -> Option<Percent> {
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
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

impl std::fmt::Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const ONE_PERCENT: f32 = Percent::SCALE as f32 / 100.0;
        std::fmt::Display::fmt(&(self.0 as f32 / ONE_PERCENT), f)?;
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
