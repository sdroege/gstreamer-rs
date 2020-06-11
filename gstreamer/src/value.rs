// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use num_rational::Rational32;
use std::borrow::{Borrow, Cow};
use std::cmp;
use std::fmt;
use std::ops;
use std::slice;

use glib;
use glib::translate::{from_glib, FromGlibPtrFull, ToGlibPtr, ToGlibPtrMut, Uninitialized};
use glib::value::{FromValue, FromValueOptional, SetValue, ToSendValue, Value};

use glib_sys;
use gst_sys;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Fraction(pub Rational32);

impl Fraction {
    pub fn new(num: i32, den: i32) -> Fraction {
        assert_initialized_main_thread!();
        (num, den).into()
    }

    pub fn approximate_f32(x: f32) -> Option<Fraction> {
        assert_initialized_main_thread!();
        Rational32::approximate_float(x).map(|r| r.into())
    }

    pub fn approximate_f64(x: f64) -> Option<Fraction> {
        assert_initialized_main_thread!();
        Rational32::approximate_float(x).map(|r| r.into())
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl ops::Deref for Fraction {
    type Target = Rational32;

    fn deref(&self) -> &Rational32 {
        &self.0
    }
}

impl ops::DerefMut for Fraction {
    fn deref_mut(&mut self) -> &mut Rational32 {
        &mut self.0
    }
}

impl AsRef<Rational32> for Fraction {
    fn as_ref(&self) -> &Rational32 {
        &self.0
    }
}

macro_rules! impl_fraction_binop {
    ($name:ident, $f:ident, $name_assign:ident, $f_assign:ident) => {
        impl ops::$name<Fraction> for Fraction {
            type Output = Fraction;

            fn $f(self, other: Fraction) -> Fraction {
                Fraction((self.0).$f(other.0))
            }
        }

        impl ops::$name<Fraction> for &Fraction {
            type Output = Fraction;

            fn $f(self, other: Fraction) -> Fraction {
                Fraction((self.0).$f(other.0))
            }
        }

        impl ops::$name<&Fraction> for Fraction {
            type Output = Fraction;

            fn $f(self, other: &Fraction) -> Fraction {
                Fraction((self.0).$f(other.0))
            }
        }

        impl ops::$name<&Fraction> for &Fraction {
            type Output = Fraction;

            fn $f(self, other: &Fraction) -> Fraction {
                Fraction((self.0).$f(other.0))
            }
        }

        impl ops::$name<i32> for Fraction {
            type Output = Fraction;

            fn $f(self, other: i32) -> Fraction {
                self.$f(Fraction::from(other))
            }
        }

        impl ops::$name<i32> for &Fraction {
            type Output = Fraction;

            fn $f(self, other: i32) -> Fraction {
                self.$f(Fraction::from(other))
            }
        }

        impl ops::$name<&i32> for Fraction {
            type Output = Fraction;

            fn $f(self, other: &i32) -> Fraction {
                self.$f(Fraction::from(*other))
            }
        }

        impl ops::$name<&i32> for &Fraction {
            type Output = Fraction;

            fn $f(self, other: &i32) -> Fraction {
                self.$f(Fraction::from(*other))
            }
        }

        impl ops::$name<Fraction> for i32 {
            type Output = Fraction;

            fn $f(self, other: Fraction) -> Fraction {
                Fraction::from(self).$f(other)
            }
        }

        impl ops::$name<&Fraction> for i32 {
            type Output = Fraction;

            fn $f(self, other: &Fraction) -> Fraction {
                Fraction::from(self).$f(other)
            }
        }

        impl ops::$name<Fraction> for &i32 {
            type Output = Fraction;

            fn $f(self, other: Fraction) -> Fraction {
                Fraction::from(*self).$f(other)
            }
        }

        impl ops::$name<&Fraction> for &i32 {
            type Output = Fraction;

            fn $f(self, other: &Fraction) -> Fraction {
                Fraction::from(*self).$f(other)
            }
        }

        impl ops::$name_assign<Fraction> for Fraction {
            fn $f_assign(&mut self, other: Fraction) {
                (self.0).$f_assign(other.0)
            }
        }

        impl ops::$name_assign<&Fraction> for Fraction {
            fn $f_assign(&mut self, other: &Fraction) {
                (self.0).$f_assign(other.0)
            }
        }

        impl ops::$name_assign<i32> for Fraction {
            fn $f_assign(&mut self, other: i32) {
                (self.0).$f_assign(other)
            }
        }

        impl ops::$name_assign<&i32> for Fraction {
            fn $f_assign(&mut self, other: &i32) {
                (self.0).$f_assign(other)
            }
        }
    };
}

impl_fraction_binop!(Add, add, AddAssign, add_assign);
impl_fraction_binop!(Sub, sub, SubAssign, sub_assign);
impl_fraction_binop!(Div, div, DivAssign, div_assign);
impl_fraction_binop!(Mul, mul, MulAssign, mul_assign);
impl_fraction_binop!(Rem, rem, RemAssign, rem_assign);

impl ops::Neg for Fraction {
    type Output = Fraction;

    fn neg(self) -> Fraction {
        Fraction(self.0.neg())
    }
}

impl ops::Neg for &Fraction {
    type Output = Fraction;

    fn neg(self) -> Fraction {
        Fraction(self.0.neg())
    }
}

impl From<i32> for Fraction {
    fn from(x: i32) -> Fraction {
        assert_initialized_main_thread!();
        Fraction(x.into())
    }
}

impl From<(i32, i32)> for Fraction {
    fn from(x: (i32, i32)) -> Fraction {
        assert_initialized_main_thread!();
        Fraction(x.into())
    }
}

impl Into<(i32, i32)> for Fraction {
    fn into(self) -> (i32, i32) {
        self.0.into()
    }
}

impl From<Rational32> for Fraction {
    fn from(x: Rational32) -> Fraction {
        assert_initialized_main_thread!();
        Fraction(x)
    }
}

impl From<Fraction> for Rational32 {
    fn from(x: Fraction) -> Rational32 {
        skip_assert_initialized!();
        x.0
    }
}

impl glib::types::StaticType for Fraction {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(gst_sys::gst_fraction_get_type()) }
    }
}

impl<'a> FromValue<'a> for Fraction {
    unsafe fn from_value(v: &'a Value) -> Fraction {
        let n = gst_sys::gst_value_get_fraction_numerator(v.to_glib_none().0);
        let d = gst_sys::gst_value_get_fraction_denominator(v.to_glib_none().0);

        Fraction::new(n, d)
    }
}

impl<'a> FromValueOptional<'a> for Fraction {
    unsafe fn from_value_optional(v: &'a Value) -> Option<Fraction> {
        Some(Fraction::from_value(v))
    }
}

impl SetValue for Fraction {
    unsafe fn set_value(v: &mut Value, f: &Self) {
        gst_sys::gst_value_set_fraction(v.to_glib_none_mut().0, *f.numer(), *f.denom());
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "ser_de", derive(Serialize, Deserialize))]
pub struct IntRange<T> {
    min: T,
    max: T,
    step: T,
}

impl<T: Copy> IntRange<T> {
    pub fn min(&self) -> T {
        self.min
    }

    pub fn max(&self) -> T {
        self.max
    }

    pub fn step(&self) -> T {
        self.step
    }
}

impl IntRange<i32> {
    pub fn new(min: i32, max: i32) -> Self {
        skip_assert_initialized!();
        Self::with_step(min, max, 1)
    }

    pub fn with_step(min: i32, max: i32, step: i32) -> Self {
        assert_initialized_main_thread!();

        assert!(min <= max);
        assert!(step > 0);

        Self { min, max, step }
    }
}

impl IntRange<i64> {
    pub fn new(min: i64, max: i64) -> Self {
        skip_assert_initialized!();
        Self::with_step(min, max, 1)
    }

    pub fn with_step(min: i64, max: i64, step: i64) -> Self {
        assert_initialized_main_thread!();

        assert!(min <= max);
        assert!(step > 0);

        Self { min, max, step }
    }
}

impl From<(i32, i32)> for IntRange<i32> {
    fn from((min, max): (i32, i32)) -> Self {
        skip_assert_initialized!();
        Self::new(min, max)
    }
}

impl From<(i32, i32, i32)> for IntRange<i32> {
    fn from((min, max, step): (i32, i32, i32)) -> Self {
        skip_assert_initialized!();
        Self::with_step(min, max, step)
    }
}

impl From<(i64, i64)> for IntRange<i64> {
    fn from((min, max): (i64, i64)) -> Self {
        skip_assert_initialized!();
        Self::new(min, max)
    }
}

impl From<(i64, i64, i64)> for IntRange<i64> {
    fn from((min, max, step): (i64, i64, i64)) -> Self {
        skip_assert_initialized!();
        Self::with_step(min, max, step)
    }
}

impl glib::types::StaticType for IntRange<i32> {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(gst_sys::gst_int_range_get_type()) }
    }
}

impl<'a> FromValue<'a> for IntRange<i32> {
    unsafe fn from_value(v: &'a Value) -> Self {
        let min = gst_sys::gst_value_get_int_range_min(v.to_glib_none().0);
        let max = gst_sys::gst_value_get_int_range_max(v.to_glib_none().0);
        let step = gst_sys::gst_value_get_int_range_step(v.to_glib_none().0);

        Self::with_step(min, max, step)
    }
}

impl<'a> FromValueOptional<'a> for IntRange<i32> {
    unsafe fn from_value_optional(v: &'a Value) -> Option<Self> {
        Some(Self::from_value(v))
    }
}

impl SetValue for IntRange<i32> {
    unsafe fn set_value(v: &mut Value, r: &Self) {
        gst_sys::gst_value_set_int_range_step(v.to_glib_none_mut().0, r.min(), r.max(), r.step());
    }
}

impl glib::types::StaticType for IntRange<i64> {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(gst_sys::gst_int64_range_get_type()) }
    }
}

impl<'a> FromValue<'a> for IntRange<i64> {
    unsafe fn from_value(v: &'a Value) -> Self {
        let min = gst_sys::gst_value_get_int64_range_min(v.to_glib_none().0);
        let max = gst_sys::gst_value_get_int64_range_max(v.to_glib_none().0);
        let step = gst_sys::gst_value_get_int64_range_step(v.to_glib_none().0);

        Self::with_step(min, max, step)
    }
}

impl<'a> FromValueOptional<'a> for IntRange<i64> {
    unsafe fn from_value_optional(v: &'a Value) -> Option<Self> {
        Some(Self::from_value(v))
    }
}

impl SetValue for IntRange<i64> {
    unsafe fn set_value(v: &mut Value, r: &Self) {
        gst_sys::gst_value_set_int64_range_step(v.to_glib_none_mut().0, r.min(), r.max(), r.step());
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "ser_de", derive(Serialize, Deserialize))]
pub struct FractionRange {
    min: Fraction,
    max: Fraction,
}

impl FractionRange {
    pub fn new<T: Into<Fraction>, U: Into<Fraction>>(min: T, max: U) -> Self {
        assert_initialized_main_thread!();

        let min = min.into();
        let max = max.into();

        assert!(min <= max);

        FractionRange { min, max }
    }

    pub fn min(&self) -> Fraction {
        self.min
    }

    pub fn max(&self) -> Fraction {
        self.max
    }
}

impl From<(Fraction, Fraction)> for FractionRange {
    fn from((min, max): (Fraction, Fraction)) -> Self {
        skip_assert_initialized!();

        Self::new(min, max)
    }
}

impl glib::types::StaticType for FractionRange {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(gst_sys::gst_fraction_range_get_type()) }
    }
}

impl<'a> FromValue<'a> for FractionRange {
    unsafe fn from_value(v: &'a Value) -> Self {
        let min = gst_sys::gst_value_get_fraction_range_min(v.to_glib_none().0);
        let max = gst_sys::gst_value_get_fraction_range_max(v.to_glib_none().0);

        let min_n = gst_sys::gst_value_get_fraction_numerator(min);
        let min_d = gst_sys::gst_value_get_fraction_denominator(min);
        let max_n = gst_sys::gst_value_get_fraction_numerator(max);
        let max_d = gst_sys::gst_value_get_fraction_denominator(max);

        Self::new((min_n, min_d), (max_n, max_d))
    }
}

impl<'a> FromValueOptional<'a> for FractionRange {
    unsafe fn from_value_optional(v: &'a Value) -> Option<Self> {
        Some(Self::from_value(v))
    }
}

impl SetValue for FractionRange {
    unsafe fn set_value(v: &mut Value, r: &Self) {
        gst_sys::gst_value_set_fraction_range_full(
            v.to_glib_none_mut().0,
            *r.min().numer(),
            *r.min().denom(),
            *r.max().numer(),
            *r.max().denom(),
        );
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "ser_de", derive(Serialize, Deserialize))]
pub struct Bitmask(pub u64);

impl Bitmask {
    pub fn new(v: u64) -> Self {
        assert_initialized_main_thread!();
        Bitmask(v)
    }
}

impl ops::Deref for Bitmask {
    type Target = u64;

    fn deref(&self) -> &u64 {
        &self.0
    }
}

impl ops::DerefMut for Bitmask {
    fn deref_mut(&mut self) -> &mut u64 {
        &mut self.0
    }
}

impl ops::BitAnd for Bitmask {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Bitmask(self.0.bitand(rhs.0))
    }
}

impl ops::BitOr for Bitmask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Bitmask(self.0.bitor(rhs.0))
    }
}

impl ops::BitXor for Bitmask {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Bitmask(self.0.bitxor(rhs.0))
    }
}

impl ops::Not for Bitmask {
    type Output = Self;

    fn not(self) -> Self {
        Bitmask(self.0.not())
    }
}

impl From<u64> for Bitmask {
    fn from(v: u64) -> Self {
        skip_assert_initialized!();
        Self::new(v)
    }
}

impl glib::types::StaticType for Bitmask {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(gst_sys::gst_bitmask_get_type()) }
    }
}

impl<'a> FromValue<'a> for Bitmask {
    unsafe fn from_value(v: &'a Value) -> Self {
        let v = gst_sys::gst_value_get_bitmask(v.to_glib_none().0);
        Self::new(v)
    }
}

impl<'a> FromValueOptional<'a> for Bitmask {
    unsafe fn from_value_optional(v: &'a Value) -> Option<Self> {
        Some(Self::from_value(v))
    }
}

impl SetValue for Bitmask {
    unsafe fn set_value(v: &mut Value, r: &Self) {
        gst_sys::gst_value_set_bitmask(v.to_glib_none_mut().0, r.0);
    }
}

#[derive(Clone, Debug)]
pub struct Array<'a>(Cow<'a, [glib::SendValue]>);

unsafe impl<'a> Send for Array<'a> {}

impl<'a> Array<'a> {
    pub fn new(values: &[&dyn ToSendValue]) -> Self {
        assert_initialized_main_thread!();

        Array(values.iter().map(|v| v.to_send_value()).collect())
    }

    pub fn from_owned(values: Vec<glib::SendValue>) -> Self {
        assert_initialized_main_thread!();

        Array(Cow::Owned(values))
    }

    pub fn into_owned(self) -> Array<'static> {
        Array(self.0.into_owned().into())
    }

    pub fn as_slice(&self) -> &[glib::SendValue] {
        self.0.borrow()
    }
}

impl<'a> From<&'a [&'a dyn ToSendValue]> for Array<'a> {
    fn from(values: &'a [&'a dyn ToSendValue]) -> Self {
        skip_assert_initialized!();

        Self::new(values)
    }
}

impl<'a> From<&'a [glib::SendValue]> for Array<'a> {
    fn from(values: &'a [glib::SendValue]) -> Self {
        assert_initialized_main_thread!();

        Array(Cow::Borrowed(values))
    }
}

impl<'a> FromValue<'a> for Array<'a> {
    unsafe fn from_value(v: &'a Value) -> Self {
        let arr = (*v.to_glib_none().0).data[0].v_pointer as *const glib_sys::GArray;
        if arr.is_null() {
            Array(Cow::Borrowed(&[]))
        } else {
            #[allow(clippy::cast_ptr_alignment)]
            Array(Cow::Borrowed(slice::from_raw_parts(
                (*arr).data as *const glib::SendValue,
                (*arr).len as usize,
            )))
        }
    }
}

impl<'a> FromValueOptional<'a> for Array<'a> {
    unsafe fn from_value_optional(v: &'a Value) -> Option<Self> {
        Some(Array::from_value(v))
    }
}

impl<'a> SetValue for Array<'a> {
    unsafe fn set_value(v: &mut Value, a: &Self) {
        for value in a.as_slice() {
            gst_sys::gst_value_array_append_value(v.to_glib_none_mut().0, value.to_glib_none().0);
        }
    }
}

impl<'a> glib::types::StaticType for Array<'a> {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(gst_sys::gst_value_array_get_type()) }
    }
}

#[derive(Clone, Debug)]
pub struct List<'a>(Cow<'a, [glib::SendValue]>);

unsafe impl<'a> Send for List<'a> {}

impl<'a> List<'a> {
    pub fn new(values: &[&dyn ToSendValue]) -> Self {
        assert_initialized_main_thread!();

        List(values.iter().map(|v| v.to_send_value()).collect())
    }

    pub fn from_owned(values: Vec<glib::SendValue>) -> Self {
        assert_initialized_main_thread!();

        List(Cow::Owned(values))
    }

    pub fn into_owned(self) -> List<'static> {
        List(self.0.into_owned().into())
    }

    pub fn as_slice(&self) -> &[glib::SendValue] {
        self.0.borrow()
    }
}

impl<'a> From<&'a [&'a dyn ToSendValue]> for List<'a> {
    fn from(values: &'a [&'a dyn ToSendValue]) -> Self {
        skip_assert_initialized!();

        Self::new(values)
    }
}

impl<'a> From<&'a [glib::SendValue]> for List<'a> {
    fn from(values: &'a [glib::SendValue]) -> Self {
        assert_initialized_main_thread!();

        List(Cow::Borrowed(values))
    }
}

impl<'a> FromValue<'a> for List<'a> {
    unsafe fn from_value(v: &'a Value) -> Self {
        let arr = (*v.to_glib_none().0).data[0].v_pointer as *const glib_sys::GArray;
        if arr.is_null() {
            List(Cow::Borrowed(&[]))
        } else {
            #[allow(clippy::cast_ptr_alignment)]
            List(Cow::Borrowed(slice::from_raw_parts(
                (*arr).data as *const glib::SendValue,
                (*arr).len as usize,
            )))
        }
    }
}

impl<'a> FromValueOptional<'a> for List<'a> {
    unsafe fn from_value_optional(v: &'a Value) -> Option<Self> {
        Some(List::from_value(v))
    }
}

impl<'a> SetValue for List<'a> {
    unsafe fn set_value(v: &mut Value, a: &Self) {
        for value in a.as_slice() {
            gst_sys::gst_value_list_append_value(v.to_glib_none_mut().0, value.to_glib_none().0);
        }
    }
}

impl<'a> glib::types::StaticType for List<'a> {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(gst_sys::gst_value_list_get_type()) }
    }
}

pub trait GstValueExt: Sized {
    fn can_compare(&self, other: &Self) -> bool;
    fn compare(&self, other: &Self) -> Option<cmp::Ordering>;
    fn eq(&self, other: &Self) -> bool;
    fn can_intersect(&self, other: &Self) -> bool;
    fn intersect(&self, other: &Self) -> Option<Self>;
    fn can_subtract(&self, other: &Self) -> bool;
    fn subtract(&self, other: &Self) -> Option<Self>;
    fn can_union(&self, other: &Self) -> bool;
    fn union(&self, other: &Self) -> Option<Self>;
    fn fixate(&self) -> Option<Self>;
    fn is_fixed(&self) -> bool;
    fn is_subset(&self, superset: &Self) -> bool;
    fn serialize(&self) -> Result<glib::GString, glib::BoolError>;
    fn deserialize<'a, T: Into<&'a str>>(s: T) -> Result<glib::Value, glib::BoolError>;
}

impl GstValueExt for glib::Value {
    fn can_compare(&self, other: &Self) -> bool {
        unsafe {
            from_glib(gst_sys::gst_value_can_compare(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    fn compare(&self, other: &Self) -> Option<cmp::Ordering> {
        unsafe {
            let val = gst_sys::gst_value_compare(self.to_glib_none().0, other.to_glib_none().0);

            match val {
                gst_sys::GST_VALUE_LESS_THAN => Some(cmp::Ordering::Less),
                gst_sys::GST_VALUE_EQUAL => Some(cmp::Ordering::Equal),
                gst_sys::GST_VALUE_GREATER_THAN => Some(cmp::Ordering::Greater),
                _ => None,
            }
        }
    }

    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == Some(cmp::Ordering::Equal)
    }

    fn can_intersect(&self, other: &Self) -> bool {
        unsafe {
            from_glib(gst_sys::gst_value_can_intersect(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    fn intersect(&self, other: &Self) -> Option<Self> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let ret: bool = from_glib(gst_sys::gst_value_intersect(
                value.to_glib_none_mut().0,
                self.to_glib_none().0,
                other.to_glib_none().0,
            ));
            if ret {
                Some(value)
            } else {
                None
            }
        }
    }

    fn can_subtract(&self, other: &Self) -> bool {
        unsafe {
            from_glib(gst_sys::gst_value_can_subtract(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    fn subtract(&self, other: &Self) -> Option<Self> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let ret: bool = from_glib(gst_sys::gst_value_subtract(
                value.to_glib_none_mut().0,
                self.to_glib_none().0,
                other.to_glib_none().0,
            ));
            if ret {
                Some(value)
            } else {
                None
            }
        }
    }

    fn can_union(&self, other: &Self) -> bool {
        unsafe {
            from_glib(gst_sys::gst_value_can_union(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    fn union(&self, other: &Self) -> Option<Self> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let ret: bool = from_glib(gst_sys::gst_value_union(
                value.to_glib_none_mut().0,
                self.to_glib_none().0,
                other.to_glib_none().0,
            ));
            if ret {
                Some(value)
            } else {
                None
            }
        }
    }

    fn fixate(&self) -> Option<Self> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let ret: bool = from_glib(gst_sys::gst_value_fixate(
                value.to_glib_none_mut().0,
                self.to_glib_none().0,
            ));
            if ret {
                Some(value)
            } else {
                None
            }
        }
    }

    fn is_fixed(&self) -> bool {
        unsafe { from_glib(gst_sys::gst_value_is_fixed(self.to_glib_none().0)) }
    }

    fn is_subset(&self, superset: &Self) -> bool {
        unsafe {
            from_glib(gst_sys::gst_value_is_subset(
                self.to_glib_none().0,
                superset.to_glib_none().0,
            ))
        }
    }

    fn serialize(&self) -> Result<glib::GString, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(gst_sys::gst_value_serialize(self.to_glib_none().0))
                .ok_or_else(|| glib_bool_error!("Failed to serialize value"))
        }
    }

    fn deserialize<'a, T: Into<&'a str>>(s: T) -> Result<glib::Value, glib::BoolError> {
        assert_initialized_main_thread!();

        let s = s.into();

        unsafe {
            let mut value = glib::Value::uninitialized();
            let ret: bool = from_glib(gst_sys::gst_value_deserialize(
                value.to_glib_none_mut().0,
                s.to_glib_none().0,
            ));
            if ret {
                Ok(value)
            } else {
                Err(glib_bool_error!("Failed to deserialize value"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fraction() {
        ::init().unwrap();

        let f1 = ::Fraction::new(1, 2);
        let f2 = ::Fraction::new(2, 3);
        let mut f3 = f1 * f2;
        let f4 = f1 * f2;
        f3 *= f2;
        f3 *= f4;

        assert_eq!(f3, ::Fraction::new(2, 27));
    }
}
