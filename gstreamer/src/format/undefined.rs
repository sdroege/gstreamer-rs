// Take a look at the license at the top of the repository in the LICENSE file.

use std::ops::{Deref, DerefMut};

use glib::translate::TryFromGlib;

use super::{
    FormattedValue, FormattedValueError, FormattedValueFullRange, FormattedValueIntrinsic,
    GenericFormattedValue, Signed,
};
use crate::Format;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Undefined(i64);

impl Undefined {
    pub const ONE: Undefined = Undefined(1);
}

// FIXME `functions in traits cannot be const` (rustc 1.64.0)
// rustdoc-stripper-ignore-next
/// `Undefined` formatted value constructor trait.
pub trait UndefinedFormatConstructor {
    // rustdoc-stripper-ignore-next
    /// Builds an `Undefined` formatted value from `self`.
    fn undefined_format(self) -> Undefined;
}

impl UndefinedFormatConstructor for i64 {
    #[track_caller]
    #[inline]
    fn undefined_format(self) -> Undefined {
        Undefined(self)
    }
}

impl FormattedValue for Undefined {
    type FullRange = Undefined;

    #[inline]
    fn default_format() -> Format {
        Format::Undefined
    }

    #[inline]
    fn format(&self) -> Format {
        Format::Undefined
    }

    #[inline]
    fn is_some(&self) -> bool {
        true
    }

    #[inline]
    unsafe fn into_raw_value(self) -> i64 {
        self.0
    }
}

impl FormattedValueFullRange for Undefined {
    #[inline]
    unsafe fn from_raw(format: Format, value: i64) -> Self {
        debug_assert_eq!(format, Format::Undefined);
        Undefined(value)
    }
}

impl From<Undefined> for GenericFormattedValue {
    #[inline]
    fn from(v: Undefined) -> Self {
        skip_assert_initialized!();
        GenericFormattedValue::Undefined(v)
    }
}

impl TryFrom<GenericFormattedValue> for Undefined {
    type Error = FormattedValueError;

    #[inline]
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
    #[inline]
    fn from(v: i64) -> Self {
        skip_assert_initialized!();
        Undefined(v)
    }
}

impl Deref for Undefined {
    type Target = i64;

    #[inline]
    fn deref(&self) -> &i64 {
        &self.0
    }
}

impl DerefMut for Undefined {
    #[inline]
    fn deref_mut(&mut self) -> &mut i64 {
        &mut self.0
    }
}

impl AsRef<i64> for Undefined {
    #[inline]
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

impl AsMut<i64> for Undefined {
    #[inline]
    fn as_mut(&mut self) -> &mut i64 {
        &mut self.0
    }
}

impl From<Undefined> for Signed<u64> {
    #[inline]
    fn from(val: Undefined) -> Signed<u64> {
        skip_assert_initialized!();
        val.0.into()
    }
}

glib_newtype_display!(Undefined, Format::Undefined);
