// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::{FromGlib, GlibNoneError, IntoGlib, OptionIntoGlib, TryFromGlib};
use std::fmt;

use crate::utils::Displayable;

use super::{
    Buffers, Bytes, ClockTime, Default, Format, FormattedValueError, Percent, Signed, Undefined,
};
use super::{
    CompatibleFormattedValue, FormattedValue, FormattedValueFullRange, FormattedValueIntrinsic,
    FormattedValueNoneBuilder, SignedIntrinsic, UnsignedIntoSigned,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct Other(u64);
impl Other {
    pub const MAX: Self = Self(u64::MAX - 1);
}

impl_common_ops_for_newtype_uint!(Other, u64);
impl_signed_div_mul!(Other, u64);
option_glib_newtype_from_to!(Other, u64::MAX);
glib_newtype_display!(Other, DisplayableOther, DisplayableOptionOther);

impl TryFrom<u64> for Other {
    type Error = GlibNoneError;
    fn try_from(val: u64) -> Result<Self, GlibNoneError> {
        skip_assert_initialized!();
        unsafe { Self::try_from_glib(val) }
    }
}

impl TryFromGlib<i64> for Other {
    type Error = GlibNoneError;
    #[inline]
    unsafe fn try_from_glib(val: i64) -> Result<Self, GlibNoneError> {
        skip_assert_initialized!();
        Self::try_from_glib(val as u64)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GenericFormattedValue {
    Undefined(Undefined),
    Default(Option<Default>),
    Bytes(Option<Bytes>),
    Time(Option<ClockTime>),
    Buffers(Option<Buffers>),
    Percent(Option<Percent>),
    Other(Format, Option<Other>),
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
            Self::Other(format, val) => {
                val.display().fmt(f)?;
                fmt::Write::write_char(f, ' ')?;
                fmt::Display::fmt(&format, f)
            }
        }
    }
}

impl Displayable for GenericFormattedValue {
    type DisplayImpl = Self;
    fn display(self) -> Self {
        self
    }
}

impl GenericFormattedValue {
    pub fn new(format: Format, value: i64) -> Self {
        skip_assert_initialized!();
        match format {
            Format::Undefined => Self::Undefined(Undefined(value)),
            Format::Default => Self::Default(unsafe { FromGlib::from_glib(value) }),
            Format::Bytes => Self::Bytes(unsafe { FromGlib::from_glib(value) }),
            Format::Time => Self::Time(unsafe { FromGlib::from_glib(value) }),
            Format::Buffers => Self::Buffers(unsafe { FromGlib::from_glib(value) }),
            Format::Percent => Self::Percent(unsafe { FromGlib::from_glib(value) }),
            Format::__Unknown(_) => Self::Other(format, unsafe { FromGlib::from_glib(value) }),
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
                Self::Other(_, v) => v.into_glib() as i64,
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
            Self::Other(_, v) => v.is_some(),
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
impl SignedIntrinsic for GenericFormattedValue {}

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
            unknown => Self::Other(unknown, Other::NONE),
        }
    }
}

impl UnsignedIntoSigned for GenericFormattedValue {
    type Signed = GenericSignedFormattedValue;

    #[track_caller]
    fn into_positive(self) -> Self::Signed {
        use Signed::Positive;
        match self {
            Self::Undefined(_) => {
                unimplemented!("`GenericFormattedValue::Undefined` is already signed")
            }
            Self::Default(val) => Self::Signed::Default(val.map(Positive)),
            Self::Bytes(val) => Self::Signed::Bytes(val.map(Positive)),
            Self::Time(val) => Self::Signed::Time(val.map(Positive)),
            Self::Buffers(val) => Self::Signed::Buffers(val.map(Positive)),
            Self::Percent(val) => Self::Signed::Percent(val.map(Positive)),
            Self::Other(format, val) => Self::Signed::Other(format, val.map(Positive)),
        }
    }

    #[track_caller]
    fn into_negative(self) -> Self::Signed {
        use Signed::Negative;
        match self {
            Self::Undefined(_) => {
                unimplemented!("`GenericFormattedValue::Undefined` is already signed")
            }
            Self::Default(val) => Self::Signed::Default(val.map(Negative)),
            Self::Bytes(val) => Self::Signed::Bytes(val.map(Negative)),
            Self::Time(val) => Self::Signed::Time(val.map(Negative)),
            Self::Buffers(val) => Self::Signed::Buffers(val.map(Negative)),
            Self::Percent(val) => Self::Signed::Percent(val.map(Negative)),
            Self::Other(format, val) => Self::Signed::Other(format, val.map(Negative)),
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GenericSignedFormattedValue {
    Default(Option<Signed<Default>>),
    Bytes(Option<Signed<Bytes>>),
    Time(Option<Signed<ClockTime>>),
    Buffers(Option<Signed<Buffers>>),
    Percent(Option<Signed<Percent>>),
    Other(Format, Option<Signed<Other>>),
}

impl GenericSignedFormattedValue {
    #[doc(alias = "get_format")]
    pub fn format(&self) -> Format {
        match *self {
            Self::Default(_) => Format::Default,
            Self::Bytes(_) => Format::Bytes,
            Self::Time(_) => Format::Time,
            Self::Buffers(_) => Format::Buffers,
            Self::Percent(_) => Format::Percent,
            Self::Other(format, _) => format,
        }
    }

    pub fn abs(self) -> GenericFormattedValue {
        use GenericFormattedValue as Unsigned;
        match self {
            Self::Default(opt_signed) => Unsigned::Default(opt_signed.map(Signed::abs)),
            Self::Bytes(opt_signed) => Unsigned::Bytes(opt_signed.map(Signed::abs)),
            Self::Time(opt_signed) => Unsigned::Time(opt_signed.map(Signed::abs)),
            Self::Buffers(opt_signed) => Unsigned::Buffers(opt_signed.map(Signed::abs)),
            Self::Percent(opt_signed) => Unsigned::Percent(opt_signed.map(Signed::abs)),
            Self::Other(format, opt_signed) => Unsigned::Other(format, opt_signed.map(Signed::abs)),
        }
    }

    pub fn is_some(&self) -> bool {
        match self {
            Self::Default(v) => v.is_some(),
            Self::Bytes(v) => v.is_some(),
            Self::Time(v) => v.is_some(),
            Self::Buffers(v) => v.is_some(),
            Self::Percent(v) => v.is_some(),
            Self::Other(_, v) => v.is_some(),
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    #[track_caller]
    pub fn none_for_format(format: Format) -> Self {
        skip_assert_initialized!();
        match format {
            Format::Default => Self::Default(None),
            Format::Bytes => Self::Bytes(None),
            Format::Time => Self::Time(None),
            Format::Buffers => Self::Buffers(None),
            Format::Percent => Self::Percent(None),
            Format::Undefined => {
                panic!("`Undefined` is already signed, use `GenericFormattedValue`")
            }
            other => Self::Other(other, None),
        }
    }
}

macro_rules! impl_gsfv_fn_opt_ret(
    ($fn:ident(self) -> Option<$ret_ty:ty>) => {
        pub fn $fn(self) -> Option<$ret_ty> {
            match self {
                Self::Default(opt_signed) => opt_signed.map(|signed| signed.$fn()),
                Self::Bytes(opt_signed) => opt_signed.map(|signed| signed.$fn()),
                Self::Time(opt_signed) => opt_signed.map(|signed| signed.$fn()),
                Self::Buffers(opt_signed) => opt_signed.map(|signed| signed.$fn()),
                Self::Percent(opt_signed) => opt_signed.map(|signed| signed.$fn()),
                Self::Other(_, opt_signed) => opt_signed.map(|signed| signed.$fn()),
            }
        }
    };
);

impl GenericSignedFormattedValue {
    impl_gsfv_fn_opt_ret!(is_positive(self) -> Option<bool>);
    impl_gsfv_fn_opt_ret!(is_negative(self) -> Option<bool>);
    impl_gsfv_fn_opt_ret!(signum(self) -> Option<i32>);
}

impl std::ops::Neg for GenericSignedFormattedValue {
    type Output = Self;

    fn neg(self) -> Self {
        use std::ops::Neg;
        match self {
            Self::Default(opt_signed) => Self::Default(opt_signed.map(Neg::neg)),
            Self::Bytes(opt_signed) => Self::Bytes(opt_signed.map(Neg::neg)),
            Self::Time(opt_signed) => Self::Time(opt_signed.map(Neg::neg)),
            Self::Buffers(opt_signed) => Self::Buffers(opt_signed.map(Neg::neg)),
            Self::Percent(opt_signed) => Self::Percent(opt_signed.map(Neg::neg)),
            Self::Other(format, opt_signed) => Self::Other(format, opt_signed.map(Neg::neg)),
        }
    }
}

impl fmt::Display for GenericSignedFormattedValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Default(opt_signed) => opt_signed.display().fmt(f),
            Self::Bytes(opt_signed) => opt_signed.display().fmt(f),
            Self::Time(opt_signed) => opt_signed.display().fmt(f),
            Self::Buffers(opt_signed) => opt_signed.display().fmt(f),
            Self::Percent(opt_signed) => opt_signed.display().fmt(f),
            Self::Other(format, opt_signed) => {
                opt_signed.display().fmt(f)?;
                fmt::Write::write_char(f, ' ')?;
                fmt::Display::fmt(&format, f)
            }
        }
    }
}

impl Displayable for GenericSignedFormattedValue {
    type DisplayImpl = Self;

    fn display(self) -> Self::DisplayImpl {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::eq_op, clippy::op_ref)]
    fn other() {
        // Check a few ops on `Other`, better converage for
        // the macro ops impl ensured as part of the `clock_time` module.

        use opt_ops::prelude::*;

        let other_none: Option<Other> = Other::try_from(u64::MAX).ok();
        assert!(other_none.is_none());

        let other_10 = Other::try_from(10).unwrap();
        let other_20 = Other::try_from(20).unwrap();
        let other_30 = Other::try_from(30).unwrap();

        assert_eq!(other_10 + other_20, other_30);
        assert_eq!(other_30 - other_20, other_10);

        assert!(other_10 < Other::MAX);

        assert_eq!(Some(other_10).opt_add(other_20), Some(other_30));
    }

    #[test]
    #[allow(clippy::eq_op, clippy::op_ref)]
    fn generic_other() {
        let gen_other_42: GenericFormattedValue =
            GenericFormattedValue::new(Format::__Unknown(128), 42);
        assert_eq!(
            gen_other_42,
            GenericFormattedValue::Other(Format::__Unknown(128), Some(Other(42)))
        );
        assert_eq!(gen_other_42.format(), Format::__Unknown(128));
        assert_eq!(gen_other_42.value(), 42);
        assert!(gen_other_42.is_some());

        let other_none: Option<Other> = Other::NONE;
        assert!(other_none.is_none());

        let gen_other_none: GenericFormattedValue =
            GenericFormattedValue::none_for_format(Format::__Unknown(128));
        assert!(gen_other_none.is_none());
        assert_eq!(
            gen_other_none,
            GenericFormattedValue::Other(Format::__Unknown(128), None)
        );
    }

    #[test]
    #[allow(clippy::eq_op, clippy::op_ref)]
    fn generic_signed_other() {
        let gen_other_42: GenericFormattedValue =
            GenericFormattedValue::new(Format::__Unknown(128), 42);

        let p_gen_other_42 = gen_other_42.into_positive();
        assert_eq!(
            p_gen_other_42,
            GenericSignedFormattedValue::Other(
                Format::__Unknown(128),
                Some(Signed::Positive(Other(42))),
            ),
        );

        let n_gen_other_42 = gen_other_42.into_negative();
        assert_eq!(
            n_gen_other_42,
            GenericSignedFormattedValue::Other(
                Format::__Unknown(128),
                Some(Signed::Negative(Other(42))),
            ),
        );
    }
}
