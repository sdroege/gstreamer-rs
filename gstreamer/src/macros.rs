// Take a look at the license at the top of the repository in the LICENSE file.

macro_rules! impl_trait_op_same(
    ($name:ident, $op:ident, $op_name:ident, $op_assign:ident, $op_assign_name:ident) => {
        impl<RHS: Borrow<$name>> ops::$op<RHS> for $name {
            type Output = Self;

            fn $op_name(self, rhs: RHS) -> Self::Output {
                Self(self.0.$op_name(rhs.borrow().0))
            }
        }

        impl<RHS: Borrow<$name>> ops::$op<RHS> for &$name {
            type Output = $name;

            fn $op_name(self, rhs: RHS) -> Self::Output {
                (*self).$op_name(rhs)
            }
        }

        impl<RHS: Borrow<$name>> ops::$op_assign<RHS> for $name {
            fn $op_assign_name(&mut self, rhs: RHS) {
                self.0.$op_assign_name(rhs.borrow().0)
            }
        }
    };
);

macro_rules! impl_non_trait_op_same(
    ($name:ident, $inner_type:ty) => {
        impl $name {
            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn checked_add(self, rhs: Self) -> Option<Self> {
                match self.0.checked_add(rhs.0) {
                    Some(res) if res <= Self::MAX.0 => Some(Self(res)),
                    _ => None,
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn saturating_add(self, rhs: Self) -> Self {
                let res = self.0.saturating_add(rhs.0);
                if res < Self::MAX.0 {
                    Self(res)
                } else {
                    Self::MAX
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                let self_u128 = self.0 as u128;
                let rhs_128 = rhs.0 as u128;
                let res_u128 = self_u128 + rhs_128;
                if res_u128 <= Self::MAX.0 as u128 {
                    (Self(<$inner_type>::try_from(res_u128).unwrap()), false)
                } else {
                    (Self(<$inner_type>::try_from((res_u128 - Self::MAX.0 as u128 - 1) as u64).unwrap()), true)
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub fn wrapping_add(self, rhs: Self) -> Self {
                self.overflowing_add(rhs).0
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            // FIXME Can't use `map` in a `const fn` as of rustc 1.53.0-beta.2
            #[allow(clippy::manual_map)]
            pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
                match self.0.checked_sub(rhs.0) {
                    Some(res) => Some(Self(res)),
                    None => None,
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn saturating_sub(self, rhs: Self) -> Self {
                Self(self.0.saturating_sub(rhs.0))
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                if self.0 >= rhs.0 {
                    (Self(self.0 - rhs.0), false)
                } else {
                    (Self(Self::MAX.0 - rhs.0 + self.0 + 1), true)
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn wrapping_sub(self, rhs: Self) -> Self {
                self.overflowing_sub(rhs).0
            }
        }
    };
);

macro_rules! impl_trait_op_inner_type(
    ($name:ident, $inner_type:ty, $op:ident, $op_name:ident, $op_assign:ident, $op_assign_name:ident) => {
        impl ops::$op<$inner_type> for $name {
            type Output = $name;

            fn $op_name(self, rhs: $inner_type) -> Self::Output {
                $name(self.0.$op_name(rhs))
            }
        }

        impl ops::$op<$inner_type> for &$name {
            type Output = $name;

            fn $op_name(self, rhs: $inner_type) -> Self::Output {
                (*self).$op_name(rhs)
            }
        }

        impl ops::$op<$name> for $inner_type {
            type Output = $name;

            fn $op_name(self, rhs: $name) -> $name {
                $name(self.$op_name(rhs.0))
            }
        }

        impl ops::$op<&$name> for $inner_type {
            type Output = $name;

            fn $op_name(self, rhs: &$name) -> $name {
                self.$op_name(*rhs)
            }
        }

        impl ops::$op_assign<$inner_type> for $name {
            fn $op_assign_name(&mut self, rhs: $inner_type) {
                self.0.$op_assign_name(rhs)
            }
        }
    };
);

macro_rules! impl_non_trait_op_inner_type(
    ($name:ident, $inner_type:ty) => {
        impl $name {
            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn checked_div(self, rhs: $inner_type) -> Option<Self> {
                match self.0.checked_div(rhs) {
                    Some(val) => Some(Self(val)),
                    None => None,
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn checked_mul(self, rhs: $inner_type) -> Option<Self> {
                match self.0.checked_mul(rhs) {
                    Some(res) if res <= Self::MAX.0 => Some(Self(res)),
                    _ => None,
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn saturating_mul(self, rhs: $inner_type) -> Self {
                let res = self.0.saturating_mul(rhs);
                if res < Self::MAX.0 {
                    Self(res)
                } else {
                    Self::MAX
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub fn overflowing_mul(self, rhs: $inner_type) -> (Self, bool) {
                let self_u128 = self.0 as u128;
                let rhs_128 = rhs as u128;
                let res_u128 = self_u128 * rhs_128;
                if res_u128 <= Self::MAX.0 as u128 {
                    (Self(<$inner_type>::try_from(res_u128).unwrap()), false)
                } else {
                    (Self(<$inner_type>::try_from((res_u128 - Self::MAX.0 as u128 - 1) as u64).unwrap()), true)
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub fn wrapping_mul(self, rhs: $inner_type) -> Self {
                self.overflowing_mul(rhs).0
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn checked_rem(self, rhs: $inner_type) -> Option<Self> {
                match self.0.checked_rem(rhs) {
                    Some(val) => Some(Self(val)),
                    None => None,
                }
            }
        }
    };
);

macro_rules! impl_common_ops_for_newtype_uint(
    ($name:ident, $inner_type:ty) => {
        impl $name {
            pub const ZERO: Self = Self(0);
            pub const NONE: Option<Self> = None;

            pub const fn is_zero(self) -> bool {
                self.0 == Self::ZERO.0
            }
        }

        impl_trait_op_same!($name, Add, add, AddAssign, add_assign);
        impl_trait_op_same!($name, Sub, sub, SubAssign, sub_assign);
        impl_trait_op_same!($name, Mul, mul, MulAssign, mul_assign);
        impl_trait_op_same!($name, Div, div, DivAssign, div_assign);
        impl_trait_op_same!($name, Rem, rem, RemAssign, rem_assign);

        impl_non_trait_op_same!($name, $inner_type);

        impl_trait_op_inner_type!($name, $inner_type, Mul, mul, MulAssign, mul_assign);
        impl_trait_op_inner_type!($name, $inner_type, Div, div, DivAssign, div_assign);
        impl_trait_op_inner_type!($name, $inner_type, Rem, rem, RemAssign, rem_assign);

        impl_non_trait_op_inner_type!($name, $inner_type);

        impl<ND: Borrow<$inner_type>> MulDiv<ND> for $name {
            type Output = $name;

            fn mul_div_floor(self, num: ND, denom: ND) -> Option<Self::Output> {
                self.0
                    .mul_div_floor(*num.borrow(), *denom.borrow())
                    .map($name)
            }

            fn mul_div_round(self, num: ND, denom: ND) -> Option<Self::Output> {
                self.0
                    .mul_div_round(*num.borrow(), *denom.borrow())
                    .map($name)
            }

            fn mul_div_ceil(self, num: ND, denom: ND) -> Option<Self::Output> {
                self.0
                    .mul_div_ceil(*num.borrow(), *denom.borrow())
                    .map($name)
            }
        }

        impl OptionOperations for $name {}

        impl OptionCheckedAdd for $name {
            type Output = Self;
            fn opt_checked_add(
                self,
                rhs: Self,
            ) -> Result<Option<Self::Output>, opt_ops::Error> {
                self.checked_add(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl OptionSaturatingAdd for $name {
            type Output = Self;
            fn opt_saturating_add(self, rhs: Self) -> Option<Self::Output> {
                Some(self.saturating_add(rhs))
            }
        }

        impl OptionOverflowingAdd for $name {
            type Output = Self;
            fn opt_overflowing_add(self, rhs: Self) -> Option<(Self::Output, bool)> {
                let res = self.overflowing_add(rhs);
                Some((res.0, res.1))
            }
        }

        impl OptionWrappingAdd for $name {
            type Output = Self;
            fn opt_wrapping_add(self, rhs: Self) -> Option<Self::Output> {
                Some(self.wrapping_add(rhs))
            }
        }

        impl OptionCheckedDiv<$inner_type> for $name {
            type Output = Self;
            fn opt_checked_div(self, rhs: $inner_type) -> Result<Option<Self::Output>, opt_ops::Error> {
                if rhs == 0 {
                    return Err(opt_ops::Error::DivisionByZero);
                }
                self.0
                    .checked_div(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(|val| Some(Self(val)))
            }
        }

        impl OptionCheckedMul<$inner_type> for $name {
            type Output = Self;
            fn opt_checked_mul(
                self,
                rhs: $inner_type,
            ) -> Result<Option<Self::Output>, opt_ops::Error> {
                self.checked_mul(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl OptionSaturatingMul<$inner_type> for $name {
            type Output = Self;
            fn opt_saturating_mul(self, rhs: $inner_type) -> Option<Self::Output> {
                Some(self.saturating_mul(rhs))
            }
        }

        impl OptionOverflowingMul<$inner_type> for $name {
            type Output = Self;
            fn opt_overflowing_mul(self, rhs: $inner_type) -> Option<(Self::Output, bool)> {
                let res = self.overflowing_mul(rhs);
                Some((res.0, res.1))
            }
        }

        impl OptionWrappingMul<$inner_type> for $name {
            type Output = Self;
            fn opt_wrapping_mul(self, rhs: $inner_type) -> Option<Self::Output> {
                Some(self.wrapping_mul(rhs))
            }
        }

        impl OptionCheckedRem<$inner_type> for $name {
            type Output = Self;
            fn opt_checked_rem(self, rhs: $inner_type) -> Result<Option<Self::Output>, opt_ops::Error> {
                if rhs == 0 {
                    return Err(opt_ops::Error::DivisionByZero);
                }
                self.0
                    .checked_rem(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(|val| Some(Self(val)))
            }
        }

        impl OptionCheckedSub for $name {
            type Output = Self;
            fn opt_checked_sub(
                self,
                rhs: Self,
            ) -> Result<Option<Self::Output>, opt_ops::Error> {
                self.checked_sub(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl OptionSaturatingSub for $name {
            type Output = Self;
            fn opt_saturating_sub(self, rhs: Self) -> Option<Self::Output> {
                Some(self.saturating_sub(rhs))
            }
        }

        impl OptionOverflowingSub for $name {
            type Output = Self;
            fn opt_overflowing_sub(self, rhs: Self) -> Option<(Self::Output, bool)> {
                let res = self.overflowing_sub(rhs);
                Some((res.0, res.1))
            }
        }

        impl OptionWrappingSub for $name {
            type Output = Self;
            fn opt_wrapping_sub(self, rhs: Self) -> Option<Self::Output> {
                Some(self.wrapping_sub(rhs))
            }
        }
    };
);

macro_rules! impl_format_value_traits(
    ($name:ident, $format:ident, $format_value:ident, $inner_type:ty) => {
        impl FormattedValue for Option<$name> {
            type FullRange = Option<$name>;
            type Signed = Option<Signed<$name>>;

            fn default_format() -> Format {
                Format::$format
            }

            fn format(&self) -> Format {
                Format::$format
            }

            unsafe fn into_raw_value(self) -> i64 {
                IntoGlib::into_glib(self) as i64
            }

            fn into_positive(self) -> Option<Signed<$name>> {
                Some(Signed::Positive(self?))
            }

            fn into_negative(self) -> Option<Signed<$name>> {
                Some(Signed::Negative(self?))
            }
        }

        impl FormattedValueFullRange for Option<$name> {
            unsafe fn from_raw(format: Format, value: i64) -> Option<$name> {
                debug_assert_eq!(format, Format::$format);
                FromGlib::from_glib(value as u64)
            }
        }

        impl From<Option<$name>> for GenericFormattedValue {
            fn from(v: Option<$name>) -> Self {
                skip_assert_initialized!();
                Self::$format_value(v)
            }
        }

        impl From<$name> for GenericFormattedValue {
            fn from(v: $name) -> Self {
                skip_assert_initialized!();
                Self::$format_value(Some(v))
            }
        }
        impl FormattedValue for $name {
            type FullRange = Option<$name>;
            type Signed = Signed<$name>;

            fn default_format() -> Format {
                Format::$format
            }

            fn format(&self) -> Format {
                Format::$format
            }

            unsafe fn into_raw_value(self) -> i64 {
                IntoGlib::into_glib(self) as i64
            }

            fn into_positive(self) -> Signed<$name> {
                Signed::Positive(self)
            }

            fn into_negative(self) -> Signed<$name> {
                Signed::Negative(self)
            }
        }

        impl SpecificFormattedValue for Option<$name> {}
        impl SpecificFormattedValueFullRange for Option<$name> {}
        impl SpecificFormattedValue for $name {}
        impl FormattedValueIntrinsic for $name {}
        impl SpecificFormattedValueIntrinsic for $name {}

        impl TryFrom<GenericFormattedValue> for Option<$name> {
            type Error = FormattedValueError;

            fn try_from(v: GenericFormattedValue) -> Result<Option<$name>, Self::Error> {
                skip_assert_initialized!();
                if let GenericFormattedValue::$format_value(v) = v {
                    Ok(v)
                } else {
                    Err(FormattedValueError(v.format()))
                }
            }
        }

        impl TryFrom<$inner_type> for $name {
            type Error = GlibNoneError;

            fn try_from(v: $inner_type) -> Result<$name, GlibNoneError> {
                skip_assert_initialized!();
                unsafe { Self::try_from_glib(v as i64) }
            }
        }

        impl TryFromGlib<i64> for $name {
            type Error = GlibNoneError;
            #[inline]
            unsafe fn try_from_glib(val: i64) -> Result<Self, GlibNoneError> {
                skip_assert_initialized!();
                <$name as TryFromGlib<u64>>::try_from_glib(val as u64)
            }
        }

        impl ops::Deref for $name {
            type Target = $inner_type;

            fn deref(&self) -> &$inner_type {
                &self.0
            }
        }

        impl ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut $inner_type {
                &mut self.0
            }
        }

        impl AsRef<$inner_type> for $name {
            fn as_ref(&self) -> &$inner_type {
                &self.0
            }
        }

        impl AsMut<$inner_type> for $name {
            fn as_mut(&mut self) -> &mut $inner_type {
                &mut self.0
            }
        }
    };
);

macro_rules! option_glib_newtype_from_to {
    ($type_:ident, $none_value:expr) => {
        #[doc(hidden)]
        impl IntoGlib for $type_ {
            type GlibType = u64;

            fn into_glib(self) -> u64 {
                self.0
            }
        }

        #[doc(hidden)]
        impl OptionIntoGlib for $type_ {
            const GLIB_NONE: u64 = $none_value;
        }

        #[doc(hidden)]
        impl TryFromGlib<u64> for $type_ {
            type Error = GlibNoneError;
            #[inline]
            unsafe fn try_from_glib(val: u64) -> Result<Self, GlibNoneError> {
                skip_assert_initialized!();
                if val == $none_value {
                    return Err(GlibNoneError);
                }

                Ok($type_(val))
            }
        }
    };
}

// FIXME we could automatically build `$displayable_name` and
// `$displayable_option_name` if `concat_idents!` was stable.
// See: https://doc.rust-lang.org/std/macro.concat_idents.html
macro_rules! glib_newtype_display {
    ($name:ident, $displayable_name:ident, $unit:expr) => {
        pub struct $displayable_name($name);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                use std::fmt::Write;

                std::fmt::Display::fmt(&self.0, f)?;
                f.write_char(' ')?;
                f.write_str($unit)
            }
        }

        impl crate::utils::Displayable for $name {
            type DisplayImpl = $name;

            fn display(self) -> $name {
                self
            }
        }
    };

    ($name:ident, $displayable_name:ident, $displayable_option_name:ident, $unit:expr) => {
        glib_newtype_display!($name, $displayable_name, $unit);

        pub struct $displayable_option_name(Option<$name>);

        impl std::fmt::Display for $displayable_option_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                if let Some(val) = self.0.as_ref() {
                    std::fmt::Display::fmt(val, f)
                } else {
                    f.write_str("undef. ")?;
                    f.write_str($unit)
                }
            }
        }

        impl crate::utils::Displayable for Option<$name> {
            type DisplayImpl = $displayable_option_name;

            fn display(self) -> Self::DisplayImpl {
                $displayable_option_name(self)
            }
        }
    };
}
