// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::TryFromGlib;
use std::ops::{Deref, DerefMut};

use super::{FormattedValue, FormattedValueFullRange, FormattedValueIntrinsic};
use super::{FormattedValueError, GenericFormattedValue, Signed};
use crate::Format;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Undefined(pub i64);

impl Undefined {
    pub const ONE: Undefined = Undefined(1);
}

impl FormattedValue for Undefined {
    type FullRange = Undefined;

    fn default_format() -> Format {
        Format::Undefined
    }

    fn format(&self) -> Format {
        Format::Undefined
    }

    fn is_some(&self) -> bool {
        true
    }

    unsafe fn into_raw_value(self) -> i64 {
        self.0
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

impl Deref for Undefined {
    type Target = i64;

    fn deref(&self) -> &i64 {
        &self.0
    }
}

impl DerefMut for Undefined {
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

impl From<Undefined> for Signed<u64> {
    fn from(val: Undefined) -> Signed<u64> {
        skip_assert_initialized!();
        val.0.into()
    }
}

glib_newtype_display!(Undefined, Format::Undefined);
