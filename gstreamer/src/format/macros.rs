// Take a look at the license at the top of the repository in the LICENSE file.

macro_rules! impl_trait_op_same(
    ($typ:ty, $op:ident, $op_name:ident, $op_assign:ident, $op_assign_name:ident) => {
        impl std::ops::$op for $typ {
            type Output = Self;
            fn $op_name(self, rhs: $typ) -> Self {
                Self(self.0.$op_name(rhs.0))
            }
        }

        impl std::ops::$op_assign for $typ {
            fn $op_assign_name(&mut self, rhs: $typ) {
                self.0.$op_assign_name(rhs.0)
            }
        }
    };
);

macro_rules! impl_non_trait_op_same(
    ($typ:ty, $inner:ty) => {
        impl $typ {
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
                    (Self(<$inner>::try_from(res_u128).unwrap()), false)
                } else {
                    (Self(<$inner>::try_from((res_u128 - Self::MAX.0 as u128 - 1) as u64).unwrap()), true)
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
    ($typ:ty, $inner:ty, $op:ident, $op_name:ident, $op_assign:ident, $op_assign_name:ident) => {
        impl std::ops::$op<$inner> for $typ {
            type Output = Self;
            fn $op_name(self, rhs: $inner) -> Self {
                Self(self.0.$op_name(rhs))
            }
        }

        impl std::ops::$op_assign<$inner> for $typ {
            fn $op_assign_name(&mut self, rhs: $inner) {
                self.0.$op_assign_name(rhs)
            }
        }
    };
);

macro_rules! impl_non_trait_op_inner_type(
    ($typ:ty, $inner:ty) => {
        impl $typ {
            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn checked_div(self, rhs: $inner) -> Option<Self> {
                match self.0.checked_div(rhs) {
                    Some(val) => Some(Self(val)),
                    None => None,
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn saturating_div(self, rhs: $inner) -> Self {
                Self(self.0.saturating_div(rhs))
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn checked_mul(self, rhs: $inner) -> Option<Self> {
                match self.0.checked_mul(rhs) {
                    Some(res) if res <= Self::MAX.0 => Some(Self(res)),
                    _ => None,
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn saturating_mul(self, rhs: $inner) -> Self {
                let res = self.0.saturating_mul(rhs);
                if res < Self::MAX.0 {
                    Self(res)
                } else {
                    Self::MAX
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub fn overflowing_mul(self, rhs: $inner) -> (Self, bool) {
                let self_u128 = self.0 as u128;
                let rhs_128 = rhs as u128;
                let res_u128 = self_u128 * rhs_128;
                if res_u128 <= Self::MAX.0 as u128 {
                    (Self(<$inner>::try_from(res_u128).unwrap()), false)
                } else {
                    (Self(<$inner>::try_from((res_u128 - Self::MAX.0 as u128 - 1) as u64).unwrap()), true)
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub fn wrapping_mul(self, rhs: $inner) -> Self {
                self.overflowing_mul(rhs).0
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn checked_rem(self, rhs: $inner) -> Option<Self> {
                match self.0.checked_rem(rhs) {
                    Some(val) => Some(Self(val)),
                    None => None,
                }
            }
        }
    };
);

macro_rules! impl_unsigned_int_into_signed(
    ($typ:ty) => {
        impl crate::UnsignedIntoSigned for $typ {
            type Signed = crate::Signed<$typ>;

            fn into_positive(self) -> Self::Signed {
                crate::Signed::Positive(self)
            }

            fn into_negative(self) -> Self::Signed {
                crate::Signed::Negative(self)
            }
        }

        impl crate::UnsignedIntoSigned for Option<$typ> {
            type Signed = Option<crate::Signed<$typ>>;

            fn into_positive(self) -> Self::Signed {
                Some(self?.into_positive())
            }

            fn into_negative(self) -> Self::Signed {
                Some(self?.into_negative())
            }
        }
    };

    ($typ:ty, $inner:ty) => {
        impl_unsigned_int_into_signed!($typ);

        impl crate::Signed<$typ> {
            // rustdoc-stripper-ignore-next
            /// Returns a `Signed` containing the inner type of `self`.
            pub fn into_inner_signed(self) -> crate::Signed<$inner> {
                use crate::Signed::*;
                match self {
                    Positive(new_type) => Positive(*new_type),
                    Negative(new_type) => Negative(*new_type),
                }
            }
        }
    };
);

macro_rules! impl_common_ops_for_newtype_uint(
    ($typ:ty, $inner:ty) => {
        impl $typ {
            pub const ZERO: Self = Self(0);
            pub const NONE: Option<Self> = None;

            pub const MAX_SIGNED: crate::Signed::<$typ> = crate::Signed::Positive(Self::MAX);
            pub const MIN_SIGNED: crate::Signed::<$typ> = crate::Signed::Negative(Self::MAX);

            pub const fn is_zero(self) -> bool {
                self.0 == Self::ZERO.0
            }
        }

        impl_trait_op_same!($typ, Add, add, AddAssign, add_assign);
        impl_trait_op_same!($typ, Sub, sub, SubAssign, sub_assign);
        impl std::ops::Div for $typ {
            type Output = $inner;
            fn div(self, rhs: $typ) -> $inner {
                self.0.div(rhs.0)
            }
        }
        impl std::ops::Rem for $typ {
            type Output = Self;
            fn rem(self, rhs: Self) -> Self {
                Self(self.0.rem(rhs.0))
            }
        }

        impl_non_trait_op_same!($typ, $inner);

        impl_trait_op_inner_type!($typ, $inner, Mul, mul, MulAssign, mul_assign);
        impl std::ops::Mul<$typ> for $inner {
            type Output = $typ;
            fn mul(self, rhs: $typ) -> $typ {
                rhs.mul(self)
            }
        }

        impl_trait_op_inner_type!($typ, $inner, Div, div, DivAssign, div_assign);
        impl_trait_op_inner_type!($typ, $inner, Rem, rem, RemAssign, rem_assign);

        impl_non_trait_op_inner_type!($typ, $inner);

        impl_unsigned_int_into_signed!($typ, $inner);

        impl_signed_ops!($typ, $inner, <$typ>::ZERO);

        impl muldiv::MulDiv<$inner> for $typ {
            type Output = Self;

            fn mul_div_floor(self, num: $inner, denom: $inner) -> Option<Self> {
                self.0
                    .mul_div_floor(num, denom)
                    .map(Self)
            }

            fn mul_div_round(self, num: $inner, denom: $inner) -> Option<Self> {
                self.0
                    .mul_div_round(num, denom)
                    .map(Self)
            }

            fn mul_div_ceil(self, num: $inner, denom: $inner) -> Option<Self> {
                self.0
                    .mul_div_ceil(num, denom)
                    .map(Self)
            }
        }

        impl opt_ops::OptionOperations for $typ {}

        impl opt_ops::OptionCheckedAdd for $typ {
            type Output = Self;
            fn opt_checked_add(
                self,
                rhs: Self,
            ) -> Result<Option<Self>, opt_ops::Error> {
                self.checked_add(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionSaturatingAdd for $typ {
            type Output = Self;
            fn opt_saturating_add(self, rhs: Self) -> Option<Self> {
                Some(self.saturating_add(rhs))
            }
        }

        impl opt_ops::OptionOverflowingAdd for $typ {
            type Output = Self;
            fn opt_overflowing_add(self, rhs: Self) -> Option<(Self, bool)> {
                let res = self.overflowing_add(rhs);
                Some((res.0, res.1))
            }
        }

        impl opt_ops::OptionWrappingAdd for $typ {
            type Output = Self;
            fn opt_wrapping_add(self, rhs: Self) -> Option<Self> {
                Some(self.wrapping_add(rhs))
            }
        }

        impl opt_ops::OptionCheckedDiv<$inner> for $typ {
            type Output = Self;
            fn opt_checked_div(self, rhs: $inner) -> Result<Option<Self>, opt_ops::Error> {
                if rhs == 0 {
                    return Err(opt_ops::Error::DivisionByZero);
                }
                self
                    .checked_div(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionCheckedDiv for $typ {
            type Output = $inner;
            fn opt_checked_div(self, rhs: Self) -> Result<Option<$inner>, opt_ops::Error> {
                if rhs.0 == 0 {
                    return Err(opt_ops::Error::DivisionByZero);
                }
                self.0
                    .checked_div(rhs.0)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionCheckedMul<$inner> for $typ {
            type Output = Self;
            fn opt_checked_mul(
                self,
                rhs: $inner,
            ) -> Result<Option<Self>, opt_ops::Error> {
                self.checked_mul(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionCheckedMul<$typ> for $inner {
            type Output = $typ;
            fn opt_checked_mul(
                self,
                rhs: $typ,
            ) -> Result<Option<$typ>, opt_ops::Error> {
                rhs.checked_mul(self)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionSaturatingMul<$inner> for $typ {
            type Output = Self;
            fn opt_saturating_mul(self, rhs: $inner) -> Option<Self> {
                Some(self.saturating_mul(rhs))
            }
        }

        impl opt_ops::OptionSaturatingMul<$typ> for $inner {
            type Output = $typ;
            fn opt_saturating_mul(self, rhs: $typ) -> Option<$typ> {
                Some(rhs.saturating_mul(self))
            }
        }

        impl opt_ops::OptionOverflowingMul<$inner> for $typ {
            type Output = Self;
            fn opt_overflowing_mul(self, rhs: $inner) -> Option<(Self, bool)> {
                let res = self.overflowing_mul(rhs);
                Some((res.0, res.1))
            }
        }

        impl opt_ops::OptionOverflowingMul<$typ> for $inner {
            type Output = $typ;
            fn opt_overflowing_mul(self, rhs: $typ) -> Option<($typ, bool)> {
                let res = rhs.overflowing_mul(self);
                Some((res.0, res.1))
            }
        }

        impl opt_ops::OptionWrappingMul<$inner> for $typ {
            type Output = Self;
            fn opt_wrapping_mul(self, rhs: $inner) -> Option<Self> {
                Some(self.wrapping_mul(rhs))
            }
        }

        impl opt_ops::OptionWrappingMul<$typ> for $inner {
            type Output = $typ;
            fn opt_wrapping_mul(self, rhs: $typ) -> Option<$typ> {
                Some(rhs.wrapping_mul(self))
            }
        }

        impl opt_ops::OptionCheckedRem<$inner> for $typ {
            type Output = Self;
            fn opt_checked_rem(self, rhs: $inner) -> Result<Option<Self>, opt_ops::Error> {
                if rhs == 0 {
                    return Err(opt_ops::Error::DivisionByZero);
                }
                self.checked_rem(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionCheckedRem for $typ {
            type Output = Self;
            fn opt_checked_rem(self, rhs: Self) -> Result<Option<Self>, opt_ops::Error> {
                if rhs.0 == 0 {
                    return Err(opt_ops::Error::DivisionByZero);
                }
                self.checked_rem(rhs.0)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionCheckedSub for $typ {
            type Output = Self;
            fn opt_checked_sub(
                self,
                rhs: Self,
            ) -> Result<Option<Self>, opt_ops::Error> {
                self.checked_sub(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionSaturatingSub for $typ {
            type Output = Self;
            fn opt_saturating_sub(self, rhs: Self) -> Option<Self> {
                Some(self.saturating_sub(rhs))
            }
        }

        impl opt_ops::OptionOverflowingSub for $typ {
            type Output = Self;
            fn opt_overflowing_sub(self, rhs: Self) -> Option<(Self, bool)> {
                let res = self.overflowing_sub(rhs);
                Some((res.0, res.1))
            }
        }

        impl opt_ops::OptionWrappingSub for $typ {
            type Output = Self;
            fn opt_wrapping_sub(self, rhs: Self) -> Option<Self> {
                Some(self.wrapping_sub(rhs))
            }
        }
    };
);

macro_rules! impl_signed_ops(
    (u64) => {
        impl_signed_ops!(u64, u64, 0);
    };

    (u32) => {
        impl_signed_ops!(u32, u32, 0);
    };

    ($typ:ty, $inner:ty, $zero:expr) => {
        impl crate::Signed<$typ> {
            // rustdoc-stripper-ignore-next
            /// Returns the signum for this `Signed`.
            ///
            /// Returns:
            ///
            /// - `0` if the number is zero.
            /// - `1` if the value must be considered as positive.
            /// - `-1` if the value must be considered as negative.
            pub fn signum(self) -> i32 {
                use crate::Signed::*;
                match self {
                    Positive(val) | Negative(val) if val == $zero => 0i32,
                    Positive(_) => 1i32,
                    Negative(_) => -1i32,
                }
            }

            // rustdoc-stripper-ignore-next
            /// Returns the checked subtraction `self - other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_sub(self, other: Self) -> Option<Self> {
                use crate::Signed::*;
                match (self, other) {
                    (Positive(a), Positive(b)) if a >= b => Some(Positive(a - b)),
                    (Positive(a), Positive(b)) => Some(Negative(b - a)),
                    (Negative(a), Negative(b)) if a >= b => Some(Negative(a - b)),
                    (Negative(a), Negative(b)) => Some(Positive(b - a)),
                    (Positive(a), Negative(b)) => a.checked_add(b).map(Positive),
                    (Negative(a), Positive(b)) => a.checked_add(b).map(Negative),
                }
            }

            // rustdoc-stripper-ignore-next
            /// Returns the checked subtraction `self - other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_sub_unsigned(self, other: $typ) -> Option<Self> {
                self.checked_sub(crate::Signed::Positive(other))
            }

            // rustdoc-stripper-ignore-next
            /// Returns the checked addition `self + other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_add(self, other: Self) -> Option<Self> {
                use crate::Signed::*;
                match (self, other) {
                    (Positive(a), Positive(b)) => a.checked_add(b).map(Positive),
                    (Negative(a), Negative(b)) => a.checked_add(b).map(Negative),
                    (Positive(_), Negative(_)) => self.checked_sub(-other),
                    (Negative(_), Positive(_)) => Some(-((-self).checked_sub(other)?))
                }
            }

            // rustdoc-stripper-ignore-next
            /// Returns the checked addition `self + other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_add_unsigned(self, other: $typ) -> Option<Self> {
                self.checked_add(crate::Signed::Positive(other))
            }

            // rustdoc-stripper-ignore-next
            /// Returns the saturating subtraction `self - other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn saturating_sub(self, other: Self) -> Self {
                use crate::Signed::*;
                match (self, other) {
                    (Positive(a), Positive(b)) if a >= b => Positive(a - b),
                    (Positive(a), Positive(b)) => Negative(b - a),
                    (Negative(a), Negative(b)) if a >= b => Negative(a - b),
                    (Negative(a), Negative(b)) => Positive(b - a),
                    (Positive(a), Negative(b)) => Positive(a.saturating_add(b)),
                    (Negative(a), Positive(b)) => Negative(a.saturating_add(b)),
                }
            }

            // rustdoc-stripper-ignore-next
            /// Returns the saturating subtraction `self - other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn saturating_sub_unsigned(self, other: $typ) -> Self {
                self.saturating_sub(crate::Signed::Positive(other))
            }

            // rustdoc-stripper-ignore-next
            /// Returns the saturating addition `self + other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn saturating_add(self, other: Self) -> Self {
                use crate::Signed::*;
                match (self, other) {
                    (Positive(a), Positive(b)) => Positive(a.saturating_add(b)),
                    (Negative(a), Negative(b)) => Negative(a.saturating_add(b)),
                    (Positive(_), Negative(_)) => self.saturating_sub(-other),
                    (Negative(_), Positive(_)) => -((-self).saturating_sub(other)),
                }
            }

            // rustdoc-stripper-ignore-next
            /// Returns the saturating addition `self + other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn saturating_add_unsigned(self, other: $typ) -> Self {
                self.saturating_add(crate::Signed::Positive(other))
            }
        }

        impl std::ops::Add for crate::Signed<$typ> {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                self.checked_add(other).expect("Overflowing addition")
            }
        }

        impl std::ops::AddAssign for crate::Signed<$typ> {
            fn add_assign(&mut self, other: Self) {
                *self = self.checked_add(other).expect("Overflowing addition")
            }
        }

        impl std::ops::Sub for crate::Signed<$typ> {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                self.checked_sub(other).expect("Overflowing subtraction")
            }
        }

        impl std::ops::SubAssign for crate::Signed<$typ> {
            fn sub_assign(&mut self, other: Self) {
                *self = self.checked_sub(other).expect("Overflowing subtraction")
            }
        }

        impl std::ops::Add<$typ> for crate::Signed<$typ> {
            type Output = Self;
            fn add(self, other: $typ) -> Self {
                self.checked_add(crate::Signed::Positive(other)).expect("Overflowing addition")
            }
        }

        impl std::ops::AddAssign<$typ> for crate::Signed<$typ> {
            fn add_assign(&mut self, other: $typ) {
                *self = self.checked_add(crate::Signed::Positive(other)).expect("Overflowing addition")
            }
        }

        impl std::ops::Sub<$typ> for crate::Signed<$typ> {
            type Output = Self;

            fn sub(self, other: $typ) -> Self {
                self.checked_sub(crate::Signed::Positive(other)).expect("Overflowing subtraction")
            }
        }

        impl std::ops::SubAssign<$typ> for crate::Signed<$typ> {
            fn sub_assign(&mut self, other: $typ) {
                *self = self.checked_sub(crate::Signed::Positive(other)).expect("Overflowing subtraction")
            }
        }

        impl std::ops::Add<crate::Signed<$typ>> for $typ {
            type Output = crate::Signed<$typ>;
            fn add(self, other: crate::Signed<$typ>) -> crate::Signed<$typ> {
                crate::Signed::Positive(self).checked_add(other).expect("Overflowing addition")
            }
        }

        impl std::ops::Sub<crate::Signed<$typ>> for $typ {
            type Output = crate::Signed<$typ>;
            fn sub(self, other: crate::Signed<$typ>) -> crate::Signed<$typ> {
                crate::Signed::Positive(self).checked_sub(other).expect("Overflowing subtraction")
            }
        }

        impl std::cmp::PartialOrd for crate::Signed<$typ> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl std::cmp::PartialEq<$typ> for crate::Signed<$typ> {
            fn eq(&self, other: &$typ) -> bool {
                self.eq(&crate::Signed::Positive(*other))
            }
        }

        impl std::cmp::PartialEq<crate::Signed<$typ>> for $typ {
            fn eq(&self, other: &crate::Signed<$typ>) -> bool {
                crate::Signed::Positive(*self).eq(other)
            }
        }

        impl std::cmp::PartialOrd<$typ> for crate::Signed<$typ> {
            fn partial_cmp(&self, other: &$typ) -> Option<std::cmp::Ordering> {
                Some(self.cmp(&crate::Signed::Positive(*other)))
            }
        }

        impl std::cmp::PartialOrd<crate::Signed<$typ>> for $typ {
            fn partial_cmp(&self, other: &crate::Signed<$typ>) -> Option<std::cmp::Ordering> {
                Some(crate::Signed::Positive(*self).cmp(other))
            }
        }

        impl std::cmp::Ord for crate::Signed<$typ> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                use crate::Signed::*;
                match (self, other) {
                    (Positive(a), Positive(b)) => a.cmp(b),
                    (Negative(a), Negative(b)) => b.cmp(a),
                    (Positive(_), Negative(_)) => std::cmp::Ordering::Greater,
                    (Negative(_), Positive(_)) => std::cmp::Ordering::Less,
                }
            }
        }

        impl opt_ops::OptionOperations for crate::Signed<$typ> {}

        impl opt_ops::OptionCheckedAdd for crate::Signed<$typ> {
            type Output = Self;
            fn opt_checked_add(
                self,
                rhs: Self,
            ) -> Result<Option<Self>, opt_ops::Error> {
                self.checked_add(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionSaturatingAdd for crate::Signed<$typ> {
            type Output = Self;
            fn opt_saturating_add(self, rhs: Self) -> Option<Self> {
                Some(self.saturating_add(rhs))
            }
        }

        impl opt_ops::OptionCheckedSub for crate::Signed<$typ> {
            type Output = Self;
            fn opt_checked_sub(
                self,
                rhs: Self,
            ) -> Result<Option<Self>, opt_ops::Error> {
                self.checked_sub(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionSaturatingSub for crate::Signed<$typ> {
            type Output = Self;
            fn opt_saturating_sub(self, rhs: Self) -> Option<Self> {
                Some(self.saturating_sub(rhs))
            }
        }

        impl opt_ops::OptionCheckedAdd<$typ> for crate::Signed<$typ> {
            type Output = Self;
            fn opt_checked_add(
                self,
                rhs: $typ,
            ) -> Result<Option<Self>, opt_ops::Error> {
                self.opt_checked_add(crate::Signed::Positive(rhs))
            }
        }

        impl opt_ops::OptionSaturatingAdd<$typ> for crate::Signed<$typ> {
            type Output = Self;
            fn opt_saturating_add(self, rhs: $typ) -> Option<Self> {
                self.opt_saturating_add(crate::Signed::Positive(rhs))
            }
        }

        impl opt_ops::OptionCheckedSub<$typ> for crate::Signed<$typ> {
            type Output = Self;
            fn opt_checked_sub(
                self,
                rhs: $typ,
            ) -> Result<Option<Self>, opt_ops::Error> {
                self.opt_checked_sub(crate::Signed::Positive(rhs))
            }
        }

        impl opt_ops::OptionSaturatingSub<$typ> for crate::Signed<$typ> {
            type Output = Self;
            fn opt_saturating_sub(self, rhs: $typ) -> Option<Self> {
                self.opt_saturating_sub(crate::Signed::Positive(rhs))
            }
        }

        impl opt_ops::OptionCheckedAdd<crate::Signed<$typ>> for $typ {
            type Output = crate::Signed<$typ>;
            fn opt_checked_add(
                self,
                rhs: crate::Signed<$typ>,
            ) -> Result<Option<Self::Output>, opt_ops::Error> {
                crate::Signed::Positive(self).opt_checked_add(rhs)
            }
        }

        impl opt_ops::OptionSaturatingAdd<crate::Signed<$typ>> for $typ {
            type Output = crate::Signed<$typ>;
            fn opt_saturating_add(
                self,
                rhs: crate::Signed<$typ>
            ) -> Option<Self::Output> {
                crate::Signed::Positive(self).opt_saturating_add(rhs)
            }
        }

        impl opt_ops::OptionCheckedSub<crate::Signed<$typ>> for $typ {
            type Output = crate::Signed<$typ>;
            fn opt_checked_sub(
                self,
                rhs: crate::Signed<$typ>,
            ) -> Result<Option<Self::Output>, opt_ops::Error> {
                crate::Signed::Positive(self).opt_checked_sub(rhs)
            }
        }

        impl opt_ops::OptionSaturatingSub<crate::Signed<$typ>> for $typ {
            type Output = crate::Signed<$typ>;
            fn opt_saturating_sub(
                self,
                rhs: crate::Signed<$typ>
            ) -> Option<Self::Output> {
                crate::Signed::Positive(self).opt_saturating_sub(rhs)
            }
        }
    };
);

macro_rules! impl_signed_div_mul(
    (u64) => {
        impl_signed_div_mul!(u64, u64, i64, |val: u64| val);
        impl_signed_extra_div_mul!(u64, i64);
        impl_signed_div_mul_trait!(u64, u64, i64, |val: u64| val);
    };

    (usize) => {
        impl_signed_div_mul!(usize, usize, isize, |val: usize| val);
        impl_signed_extra_div_mul!(usize, isize);
        // `MulDiv` not available for usize
    };

    (u32) => {
        impl_signed_div_mul!(u32, u32, i32, |val: u32| val);
        impl_signed_extra_div_mul!(u32, i32);
        impl_signed_div_mul_trait!(u32, u32, i32, |val: u32| val);
    };

    ($newtyp:ty, u64) => {
        impl_signed_div_mul!($newtyp, u64, i64, |val: $newtyp| *val);
        impl_signed_extra_div_mul!($newtyp, u64, i64);
        impl_signed_div_mul_trait!($newtyp, u64, i64, |val: $newtyp| *val);
    };

    ($newtyp:ty, u32) => {
        impl_signed_div_mul!($newtyp, u32, i32, |val: $newtyp| *val);
        impl_signed_extra_div_mul!($newtyp, u32, i32);
        impl_signed_div_mul_trait!($newtyp, u32, i32, |val: $newtyp| *val);
    };

    ($typ:ty, $inner:ty, $signed_rhs:ty, $into_inner:expr) => {
        impl crate::Signed<$typ> {
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_div(self, rhs:$signed_rhs) -> Option<Self> {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        if rhs.is_positive() {
                            lhs.checked_div(rhs as $inner).map(Positive)
                        } else {
                            lhs.checked_div(-rhs as $inner).map(Negative)
                        }
                    }
                    Negative(lhs) => {
                        if rhs.is_positive() {
                            lhs.checked_div(rhs as $inner).map(Negative)
                        } else {
                            lhs.checked_div(-rhs as $inner).map(Positive)
                        }
                    }
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_div_unsigned(self, rhs:$inner) -> Option<Self> {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => lhs.checked_div(rhs).map(Positive),
                    Negative(lhs) => lhs.checked_div(rhs).map(Negative),
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_rem(self, rhs:$signed_rhs) -> Option<Self> {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        if rhs.is_positive() {
                            lhs.checked_rem(rhs as $inner).map(Positive)
                        } else {
                            lhs.checked_rem(-rhs as $inner).map(Positive)
                        }
                    }
                    Negative(lhs) => {
                        if rhs.is_positive() {
                            lhs.checked_rem(rhs as $inner).map(Negative)
                        } else {
                            lhs.checked_rem(-rhs as $inner).map(Negative)
                        }
                    }
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_rem_unsigned(self, rhs:$inner) -> Option<Self> {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => lhs.checked_rem(rhs).map(Positive),
                    Negative(lhs) => lhs.checked_rem(rhs).map(Negative),
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_mul(self, rhs:$signed_rhs) -> Option<Self> {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        if rhs.is_positive() {
                            lhs.checked_mul(rhs as $inner).map(Positive)
                        } else {
                            lhs.checked_mul(-rhs as $inner).map(Negative)
                        }
                    }
                    Negative(lhs) => {
                        if rhs.is_positive() {
                            lhs.checked_mul(rhs as $inner).map(Negative)
                        } else {
                            lhs.checked_mul(-rhs as $inner).map(Positive)
                        }
                    }
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_mul_unsigned(self, rhs:$inner) -> Option<Self> {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => lhs.checked_mul(rhs).map(Positive),
                    Negative(lhs) => lhs.checked_mul(rhs).map(Negative),
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn saturating_mul(self, rhs:$signed_rhs) -> Self {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        if rhs.is_positive() {
                            Positive(lhs.saturating_mul(rhs as $inner))
                        } else {
                            Negative(lhs.saturating_mul(-rhs as $inner))
                        }
                    }
                    Negative(lhs) => {
                        if rhs.is_positive() {
                            Negative(lhs.saturating_mul(rhs as $inner))
                        } else {
                            Positive(lhs.saturating_mul(-rhs as $inner))
                        }
                    }
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn saturating_mul_unsigned(self, rhs:$inner) -> Self {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => Positive(lhs.saturating_mul(rhs)),
                    Negative(lhs) => Negative(lhs.saturating_mul(rhs)),
                }
            }
        }

        impl std::ops::Div<$signed_rhs> for crate::Signed<$typ> {
            type Output = Self;
            fn div(self, rhs: $signed_rhs) -> Self {
                self.checked_div(rhs).expect("division overflowed")
            }
        }

        impl std::ops::DivAssign<$signed_rhs> for crate::Signed<$typ> {
            fn div_assign(&mut self, rhs: $signed_rhs) {
                *self = std::ops::Div::div(*self, rhs);
            }
        }

        impl std::ops::Div<$inner> for crate::Signed<$typ> {
            type Output = Self;
            fn div(self, rhs: $inner) -> Self {
                self.checked_div_unsigned(rhs).expect("division overflowed")
            }
        }

        impl std::ops::DivAssign<$inner> for crate::Signed<$typ> {
            fn div_assign(&mut self, rhs: $inner) {
                *self = std::ops::Div::div(*self, rhs);
            }
        }

        impl std::ops::Rem<$signed_rhs> for crate::Signed<$typ> {
            type Output = Self;
            fn rem(self, rhs: $signed_rhs) -> Self {
                self.checked_rem(rhs).expect("division overflowed")
            }
        }

        impl std::ops::RemAssign<$signed_rhs> for crate::Signed<$typ> {
            fn rem_assign(&mut self, rhs: $signed_rhs) {
                *self = std::ops::Rem::rem(*self, rhs);
            }
        }

        impl std::ops::Rem<$inner> for crate::Signed<$typ> {
            type Output = Self;
            fn rem(self, rhs: $inner) -> Self {
                self.checked_rem_unsigned(rhs).expect("division overflowed")
            }
        }

        impl std::ops::RemAssign<$inner> for crate::Signed<$typ> {
            fn rem_assign(&mut self, rhs: $inner) {
                *self = std::ops::Rem::rem(*self, rhs);
            }
        }

        impl std::ops::Mul<$signed_rhs> for crate::Signed<$typ> {
            type Output = Self;
            fn mul(self, rhs: $signed_rhs) -> Self {
                self.checked_mul(rhs).expect("multiplication overflowed")
            }
        }

        impl std::ops::MulAssign<$signed_rhs> for crate::Signed<$typ> {
            fn mul_assign(&mut self, rhs: $signed_rhs) {
                *self = std::ops::Mul::mul(*self, rhs);
            }
        }

        impl std::ops::Mul<$inner> for crate::Signed<$typ> {
            type Output = Self;
            fn mul(self, rhs: $inner) -> Self {
                self.checked_mul_unsigned(rhs).expect("multiplication overflowed")
            }
        }

        impl std::ops::MulAssign<$inner> for crate::Signed<$typ> {
            fn mul_assign(&mut self, rhs: $inner) {
                *self = std::ops::Mul::mul(*self, rhs);
            }
        }

        impl opt_ops::OptionCheckedDiv<$signed_rhs> for crate::Signed<$typ> {
            type Output = Self;
            fn opt_checked_div(self, rhs: $signed_rhs) -> Result<Option<Self>, opt_ops::Error> {
                if rhs == 0 {
                    return Err(opt_ops::Error::DivisionByZero);
                }
                self.checked_div(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionCheckedMul<$signed_rhs> for crate::Signed<$typ> {
            type Output = Self;
            fn opt_checked_mul(self, rhs: $signed_rhs) -> Result<Option<Self>, opt_ops::Error> {
                self.checked_mul(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionSaturatingMul<$signed_rhs> for crate::Signed<$typ> {
            type Output = Self;
            fn opt_saturating_mul(self, rhs: $signed_rhs) -> Option<Self> {
                Some(self.saturating_mul(rhs))
            }
        }

        impl opt_ops::OptionCheckedRem<$signed_rhs> for crate::Signed<$typ> {
            type Output = Self;
            fn opt_checked_rem(self, rhs: $signed_rhs) -> Result<Option<Self>, opt_ops::Error> {
                if rhs == 0 {
                    return Err(opt_ops::Error::DivisionByZero);
                }
                self.checked_rem(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionCheckedDiv<$inner> for crate::Signed<$typ> {
            type Output = Self;
            fn opt_checked_div(self, rhs: $inner) -> Result<Option<Self>, opt_ops::Error> {
                if rhs == 0 {
                    return Err(opt_ops::Error::DivisionByZero);
                }
                self.checked_div_unsigned(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionCheckedMul<$inner> for crate::Signed<$typ> {
            type Output = Self;
            fn opt_checked_mul(self, rhs: $inner) -> Result<Option<Self>, opt_ops::Error> {
                self.checked_mul_unsigned(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionSaturatingMul<$inner> for crate::Signed<$typ> {
            type Output = Self;
            fn opt_saturating_mul(self, rhs: $inner) -> Option<Self> {
                Some(self.saturating_mul_unsigned(rhs))
            }
        }

        impl opt_ops::OptionCheckedRem<$inner> for crate::Signed<$typ> {
            type Output = Self;
            fn opt_checked_rem(self, rhs: $inner) -> Result<Option<Self>, opt_ops::Error> {
                if rhs == 0 {
                    return Err(opt_ops::Error::DivisionByZero);
                }
                self.checked_rem_unsigned(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }
    };
);

macro_rules! impl_signed_extra_div_mul(
    ($typ:ty, $signed:ty) => {
        impl std::ops::Div for crate::Signed<$typ> {
            type Output = Self;
            fn div(self, rhs: Self) -> Self {
                match rhs {
                    crate::Signed::Positive(rhs) => self.div(rhs),
                    crate::Signed::Negative(rhs) => std::ops::Neg::neg(self.div(rhs)),
                }
            }
        }

        impl std::ops::Rem for crate::Signed<$typ> {
            type Output = Self;
            fn rem(self, rhs: Self) -> Self {
                self.rem(rhs.abs())
            }
        }

        impl opt_ops::OptionCheckedDiv for crate::Signed<$typ> {
            type Output = Self;
            fn opt_checked_div(self, rhs: Self) -> Result<Option<Self>, opt_ops::Error> {
                match rhs {
                    crate::Signed::Positive(rhs) => self.opt_checked_div(rhs),
                    crate::Signed::Negative(rhs) => {
                        self.opt_checked_div(rhs)
                            .map(|res| res.map(std::ops::Neg::neg))
                    }
                }
            }
        }

        impl opt_ops::OptionCheckedRem for crate::Signed<$typ> {
            type Output = Self;
            fn opt_checked_rem(self, rhs: Self) -> Result<Option<Self>, opt_ops::Error> {
                self.opt_checked_rem(rhs.abs())
            }
        }
    };
    ($newtyp:ty, $inner:ty, $signed_inner:ty) => {
        impl std::ops::Div for crate::Signed<$newtyp> {
            type Output = crate::Signed<$inner>;
            fn div(self, rhs: Self) -> Self::Output {
                self.into_inner_signed().div(rhs.into_inner_signed())
            }
        }

        impl std::ops::Rem for crate::Signed<$newtyp> {
            type Output = Self;
            fn rem(self, rhs: Self) -> Self {
                self.rem(rhs.abs().0)
            }
        }

        impl std::ops::Mul<crate::Signed<$newtyp>> for $inner {
            type Output = crate::Signed<$newtyp>;
            fn mul(self, rhs: crate::Signed<$newtyp>) -> Self::Output {
                rhs.mul(self)
            }
        }

        impl std::ops::Mul<crate::Signed<$newtyp>> for $signed_inner {
            type Output = crate::Signed<$newtyp>;
            fn mul(self, rhs: crate::Signed<$newtyp>) -> Self::Output {
                rhs.mul(self)
            }
        }

        impl opt_ops::OptionCheckedDiv for crate::Signed<$newtyp> {
            type Output = crate::Signed<$inner>;
            fn opt_checked_div(self, rhs: Self) -> Result<Option<Self::Output>, opt_ops::Error> {
                self.into_inner_signed().opt_checked_div(rhs.into_inner_signed())
            }
        }

        impl opt_ops::OptionCheckedRem for crate::Signed<$newtyp> {
            type Output = crate::Signed<$inner>;
            fn opt_checked_rem(self, rhs: Self) -> Result<Option<Self::Output>, opt_ops::Error> {
                self.into_inner_signed().opt_checked_rem(rhs.abs().0)
            }
        }

        impl opt_ops::OptionCheckedMul<crate::Signed<$newtyp>> for $signed_inner {
            type Output = crate::Signed<$newtyp>;
            fn opt_checked_mul(self, rhs: crate::Signed<$newtyp>) -> Result<Option<Self::Output>, opt_ops::Error> {
                rhs.opt_checked_mul(self)
            }
        }

        impl opt_ops::OptionSaturatingMul<crate::Signed<$newtyp>> for $signed_inner {
            type Output = crate::Signed<$newtyp>;
            fn opt_saturating_mul(self, rhs: crate::Signed<$newtyp>) -> Option<Self::Output> {
                rhs.opt_saturating_mul(self)
            }
        }

        impl opt_ops::OptionCheckedMul<crate::Signed<$newtyp>> for $inner {
            type Output = crate::Signed<$newtyp>;
            fn opt_checked_mul(self, rhs: crate::Signed<$newtyp>) -> Result<Option<Self::Output>, opt_ops::Error> {
                rhs.opt_checked_mul(self)
            }
        }

        impl opt_ops::OptionSaturatingMul<crate::Signed<$newtyp>> for $inner {
            type Output = crate::Signed<$newtyp>;
            fn opt_saturating_mul(self, rhs: crate::Signed<$newtyp>) -> Option<Self::Output> {
                rhs.opt_saturating_mul(self)
            }
        }
    };
);

macro_rules! impl_signed_div_mul_trait(
    ($typ:ty, $inner:ty, $signed_rhs:ty, $into_inner:expr) => {
        impl crate::Signed<$typ> {
            fn signed_from_inner(val: $inner, sign: $signed_rhs) -> Option<crate::Signed<$typ>> {
                skip_assert_initialized!();
                if sign.is_positive() {
                    Self::positive_from_inner(val)
                } else {
                    Self::negative_from_inner(val)
                }
            }

            fn positive_from_inner(val: $inner) -> Option<Self> {
                skip_assert_initialized!();
                <$typ>::try_from(val).ok().map(crate::Signed::Positive)
            }

            fn negative_from_inner(val: $inner) -> Option<Self> {
                skip_assert_initialized!();
                <$typ>::try_from(val).ok().map(crate::Signed::Negative)
            }
        }

        impl muldiv::MulDiv<$signed_rhs> for crate::Signed<$typ> {
            type Output = Self;

            fn mul_div_floor(self, num: $signed_rhs, denom: $signed_rhs) -> Option<Self> {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        $into_inner(lhs)
                            .mul_div_floor(num.abs() as $inner, denom.abs() as $inner)
                            .and_then(|val| Self::signed_from_inner(val, num.signum() * denom.signum()))
                    }
                    Negative(lhs) => {
                        $into_inner(lhs)
                            .mul_div_floor(num.abs() as $inner, denom.abs() as $inner)
                            .and_then(|val| Self::signed_from_inner(val, -num.signum() * denom.signum()))
                    }
                }
            }

            fn mul_div_round(self, num: $signed_rhs, denom: $signed_rhs) -> Option<Self> {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        $into_inner(lhs)
                            .mul_div_round(num.abs() as $inner, denom.abs() as $inner)
                            .and_then(|val| Self::signed_from_inner(val, num.signum() * denom.signum()))
                    }
                    Negative(lhs) => {
                        $into_inner(lhs)
                            .mul_div_round(num.abs() as $inner, denom.abs() as $inner)
                            .and_then(|val| Self::signed_from_inner(val, -num.signum() * denom.signum()))
                    }
                }
            }

            fn mul_div_ceil(self, num: $signed_rhs, denom: $signed_rhs) -> Option<Self> {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        $into_inner(lhs)
                            .mul_div_ceil(num.abs() as $inner, denom.abs() as $inner)
                            .and_then(|val| Self::signed_from_inner(val, num.signum() * denom.signum()))
                    }
                    Negative(lhs) => {
                        $into_inner(lhs)
                            .mul_div_ceil(num.abs() as $inner, denom.abs() as $inner)
                            .and_then(|val| Self::signed_from_inner(val, -num.signum() * denom.signum()))
                    }
                }
            }
        }

        impl muldiv::MulDiv<$inner> for crate::Signed<$typ> {
            type Output = Self;

            fn mul_div_floor(self, num: $inner, denom: $inner) -> Option<Self> {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        $into_inner(lhs)
                            .mul_div_floor(num, denom)
                            .and_then(Self::positive_from_inner)
                    }
                    Negative(lhs) => {
                        $into_inner(lhs)
                            .mul_div_floor(num, denom)
                            .and_then(Self::negative_from_inner)
                    }
                }
            }

            fn mul_div_round(self, num: $inner, denom: $inner) -> Option<Self> {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        $into_inner(lhs)
                            .mul_div_round(num, denom)
                            .and_then(Self::positive_from_inner)
                    }
                    Negative(lhs) => {
                        $into_inner(lhs)
                            .mul_div_round(num, denom)
                            .and_then(Self::negative_from_inner)
                    }
                }
            }

            fn mul_div_ceil(self, num: $inner, denom: $inner) -> Option<Self> {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        $into_inner(lhs)
                            .mul_div_ceil(num, denom)
                            .and_then(Self::positive_from_inner)
                    }
                    Negative(lhs) => {
                        $into_inner(lhs)
                            .mul_div_ceil(num, denom)
                            .and_then(Self::negative_from_inner)
                    }
                }
            }
        }
    };
);

macro_rules! impl_format_value_traits(
    ($typ:ty, $format:ident, $format_value:ident, $inner:ty) => {
        impl FormattedValue for Option<$typ> {
            type FullRange = Self;

            fn default_format() -> Format {
                Format::$format
            }

            fn format(&self) -> Format {
                Format::$format
            }

            fn is_some(&self) -> bool {
                Option::is_some(self)
            }

            unsafe fn into_raw_value(self) -> i64 {
                IntoGlib::into_glib(self) as i64
            }
        }

        impl FormattedValueFullRange for Option<$typ> {
            unsafe fn from_raw(format: Format, value: i64) -> Self {
                debug_assert_eq!(format, Format::$format);
                FromGlib::from_glib(value as u64)
            }
        }

        impl FormattedValueNoneBuilder for Option<$typ> {
            fn none() -> Option<$typ> {
                None
            }
        }

        impl From<Option<$typ>> for GenericFormattedValue {
            fn from(v: Option<$typ>) -> Self {
                skip_assert_initialized!();
                Self::$format_value(v)
            }
        }

        impl From<$typ> for GenericFormattedValue {
            fn from(v: $typ) -> Self {
                skip_assert_initialized!();
                Self::$format_value(Some(v))
            }
        }

        impl FormattedValue for $typ {
            type FullRange = Option<$typ>;

            fn default_format() -> Format {
                Format::$format
            }

            fn format(&self) -> Format {
                Format::$format
            }

            fn is_some(&self) -> bool {
                true
            }

            unsafe fn into_raw_value(self) -> i64 {
                IntoGlib::into_glib(self) as i64
            }
        }

        impl SpecificFormattedValue for Option<$typ> {}
        impl SpecificFormattedValueFullRange for Option<$typ> {}
        impl SpecificFormattedValue for $typ {}
        impl FormattedValueIntrinsic for $typ {}
        impl SpecificFormattedValueIntrinsic for $typ {}

        impl TryFrom<GenericFormattedValue> for Option<$typ> {
            type Error = FormattedValueError;
            fn try_from(v: GenericFormattedValue) -> Result<Self, Self::Error> {
                skip_assert_initialized!();
                if let GenericFormattedValue::$format_value(v) = v {
                    Ok(v)
                } else {
                    Err(FormattedValueError(v.format()))
                }
            }
        }

        impl TryFrom<$inner> for $typ {
            type Error = GlibNoneError;
            fn try_from(v: $inner) -> Result<$typ, GlibNoneError> {
                skip_assert_initialized!();
                unsafe { Self::try_from_glib(v as i64) }
            }
        }

        impl TryFromGlib<i64> for $typ {
            type Error = GlibNoneError;
            #[inline]
            unsafe fn try_from_glib(val: i64) -> Result<Self, GlibNoneError> {
                skip_assert_initialized!();
                <$typ as TryFromGlib<u64>>::try_from_glib(val as u64)
            }
        }

        impl std::ops::Deref for $typ {
            type Target = $inner;

            fn deref(&self) -> &$inner {
                &self.0
            }
        }

        impl std::ops::DerefMut for $typ {
            fn deref_mut(&mut self) -> &mut $inner {
                &mut self.0
            }
        }

        impl AsRef<$inner> for $typ {
            fn as_ref(&self) -> &$inner {
                &self.0
            }
        }

        impl AsMut<$inner> for $typ {
            fn as_mut(&mut self) -> &mut $inner {
                &mut self.0
            }
        }
    };
);

macro_rules! option_glib_newtype_from_to {
    ($typ_:ident, $none_value:expr) => {
        #[doc(hidden)]
        impl IntoGlib for $typ_ {
            type GlibType = u64;
            fn into_glib(self) -> u64 {
                self.0
            }
        }

        #[doc(hidden)]
        impl OptionIntoGlib for $typ_ {
            const GLIB_NONE: u64 = $none_value;
        }

        #[doc(hidden)]
        impl TryFromGlib<u64> for $typ_ {
            type Error = GlibNoneError;
            #[inline]
            unsafe fn try_from_glib(val: u64) -> Result<Self, GlibNoneError> {
                skip_assert_initialized!();
                if val == $none_value {
                    return Err(GlibNoneError);
                }

                Ok($typ_(val))
            }
        }
    };
}

// FIXME we could automatically build `$displayable_name` and
// `$displayable_option_name` if `concat_idents!` was stable.
// See: https://doc.rust-lang.org/std/macro.concat_idents.html
macro_rules! glib_newtype_display {
    ($typ:ty, $displayable_name:ident, $unit:expr) => {
        pub struct $displayable_name($typ);

        impl std::fmt::Display for $typ {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                use std::fmt::Write;

                std::fmt::Display::fmt(&self.0, f)?;
                f.write_char(' ')?;
                f.write_str($unit)
            }
        }

        impl crate::utils::Displayable for $typ {
            type DisplayImpl = $typ;
            fn display(self) -> $typ {
                self
            }
        }
    };

    ($typ:ty, $displayable_name:ident, $displayable_option_name:ident, $unit:expr) => {
        glib_newtype_display!($typ, $displayable_name, $unit);

        pub struct $displayable_option_name(Option<$typ>);

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

        impl crate::utils::Displayable for Option<$typ> {
            type DisplayImpl = $displayable_option_name;
            fn display(self) -> Self::DisplayImpl {
                $displayable_option_name(self)
            }
        }
    };
}
