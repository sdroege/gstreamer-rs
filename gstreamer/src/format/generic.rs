// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::FromGlib;
use std::fmt;

use crate::utils::Displayable;

use super::{
    Buffers, Bytes, ClockTime, Default, Format, FormattedValueError, Percent, Signed, Undefined,
};
use super::{
    CompatibleFormattedValue, FormattedValue, FormattedValueFullRange, FormattedValueIntrinsic,
    FormattedValueNoneBuilder, UnsignedIntoSigned,
};

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

impl fmt::Display for GenericFormattedValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
}

impl FormattedValue for GenericFormattedValue {
    // The intrinsic value for `GenericFormattedValue` is also
    // `GenericFormattedValue`. We can't dissociate the `Option`
    // from the variants' inner type since they are not all `Option`s.
    type FullRange = GenericFormattedValue;

    fn default_format() -> Format {
        Format::Undefined
    }

    fn format(&self) -> Format {
        self.format()
    }

    fn is_some(&self) -> bool {
        match self {
            Self::Undefined(_) => true,
            Self::Default(v) => v.is_some(),
            Self::Bytes(v) => v.is_some(),
            Self::Time(v) => v.is_some(),
            Self::Buffers(v) => v.is_some(),
            Self::Percent(v) => v.is_some(),
            Self::Other(..) => true,
        }
    }

    unsafe fn into_raw_value(self) -> i64 {
        self.value()
    }
}

impl FormattedValueFullRange for GenericFormattedValue {
    unsafe fn from_raw(format: Format, value: i64) -> Self {
        GenericFormattedValue::new(format, value)
    }
}

impl FormattedValueIntrinsic for GenericFormattedValue {}

impl FormattedValueNoneBuilder for GenericFormattedValue {
    #[track_caller]
    fn none() -> Self {
        panic!(concat!(
            "`GenericFormattedValue` can't build `None` without knowing",
            "the target format. Use `GenericFormattedValue::none_for_format`",
        ));
    }

    #[track_caller]
    fn none_for_format(format: Format) -> Self {
        skip_assert_initialized!();
        match format {
            Format::Undefined => panic!("`None` can't be represented by `Undefined`"),
            Format::Default => Self::Default(None),
            Format::Bytes => Self::Bytes(None),
            Format::Time => Self::Time(None),
            Format::Buffers => Self::Buffers(None),
            Format::Percent => Self::Percent(None),
            Format::__Unknown(_) => panic!("`None` can't be represented by `__Unknown`"),
        }
    }
}

impl UnsignedIntoSigned for GenericFormattedValue {
    type Signed = Signed<GenericFormattedValue>;

    #[track_caller]
    fn into_positive(self) -> Signed<GenericFormattedValue> {
        match self {
            GenericFormattedValue::Undefined(_) => {
                unimplemented!("`GenericFormattedValue::Undefined` is already signed")
            }
            GenericFormattedValue::Other(..) => {
                unimplemented!("`GenericFormattedValue::Other` is already signed")
            }
            unsigned_inner => Signed::Positive(unsigned_inner),
        }
    }

    #[track_caller]
    fn into_negative(self) -> Signed<GenericFormattedValue> {
        match self {
            GenericFormattedValue::Undefined(_) => {
                unimplemented!("`GenericFormattedValue::Undefined` is already signed")
            }
            GenericFormattedValue::Other(..) => {
                unimplemented!("`GenericFormattedValue::Other` is already signed")
            }
            unsigned_inner => Signed::Negative(unsigned_inner),
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
