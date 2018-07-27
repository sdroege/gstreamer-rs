// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use num_rational::Rational32;
use std::borrow::{Borrow, Cow};
use std::fmt;
use std::ops;
use std::slice;

use glib;
use glib::translate::{
    from_glib, from_glib_full, FromGlib, ToGlib, ToGlibPtr, ToGlibPtrMut, Uninitialized,
};
use glib::value::{FromValue, FromValueOptional, SetValue, ToSendValue, Value};

use ffi;
use glib_ffi;

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

impl ops::Mul<Fraction> for Fraction {
    type Output = Fraction;

    fn mul(self, other: Fraction) -> Fraction {
        Fraction(self.0.mul(other.0))
    }
}

impl ops::Mul<i32> for Fraction {
    type Output = Fraction;

    fn mul(self, other: i32) -> Fraction {
        self.mul(Fraction::from(other))
    }
}

impl ops::Div<Fraction> for Fraction {
    type Output = Fraction;

    fn div(self, other: Fraction) -> Fraction {
        Fraction(self.0.div(other.0))
    }
}

impl ops::Div<i32> for Fraction {
    type Output = Fraction;

    fn div(self, other: i32) -> Fraction {
        self.div(Fraction::from(other))
    }
}

impl ops::Add<Fraction> for Fraction {
    type Output = Fraction;

    fn add(self, other: Fraction) -> Fraction {
        Fraction(self.0.add(other.0))
    }
}

impl ops::Add<i32> for Fraction {
    type Output = Fraction;

    fn add(self, other: i32) -> Fraction {
        self.add(Fraction::from(other))
    }
}

impl ops::Sub<Fraction> for Fraction {
    type Output = Fraction;

    fn sub(self, other: Fraction) -> Fraction {
        Fraction(self.0.sub(other.0))
    }
}

impl ops::Sub<i32> for Fraction {
    type Output = Fraction;

    fn sub(self, other: i32) -> Fraction {
        self.sub(Fraction::from(other))
    }
}

impl ops::Rem<Fraction> for Fraction {
    type Output = Fraction;

    fn rem(self, other: Fraction) -> Fraction {
        Fraction(self.0.rem(other.0))
    }
}

impl ops::Rem<i32> for Fraction {
    type Output = Fraction;

    fn rem(self, other: i32) -> Fraction {
        self.rem(Fraction::from(other))
    }
}

impl ops::Neg for Fraction {
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
        unsafe { from_glib(ffi::gst_fraction_get_type()) }
    }
}

impl<'a> FromValue<'a> for Fraction {
    unsafe fn from_value(v: &'a Value) -> Fraction {
        let n = ffi::gst_value_get_fraction_numerator(v.to_glib_none().0);
        let d = ffi::gst_value_get_fraction_denominator(v.to_glib_none().0);

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
        ffi::gst_value_set_fraction(v.to_glib_none_mut().0, *f.numer(), *f.denom());
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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
        Self::new_with_step(min, max, 1)
    }

    pub fn new_with_step(min: i32, max: i32, step: i32) -> Self {
        assert_initialized_main_thread!();

        assert!(min <= max);
        assert!(step > 0);

        Self { min, max, step }
    }
}

impl IntRange<i64> {
    pub fn new(min: i64, max: i64) -> Self {
        skip_assert_initialized!();
        Self::new_with_step(min, max, 1)
    }

    pub fn new_with_step(min: i64, max: i64, step: i64) -> Self {
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
        Self::new_with_step(min, max, step)
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
        Self::new_with_step(min, max, step)
    }
}

impl glib::types::StaticType for IntRange<i32> {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_int_range_get_type()) }
    }
}

impl<'a> FromValue<'a> for IntRange<i32> {
    unsafe fn from_value(v: &'a Value) -> Self {
        let min = ffi::gst_value_get_int_range_min(v.to_glib_none().0);
        let max = ffi::gst_value_get_int_range_max(v.to_glib_none().0);
        let step = ffi::gst_value_get_int_range_step(v.to_glib_none().0);

        Self::new_with_step(min, max, step)
    }
}

impl<'a> FromValueOptional<'a> for IntRange<i32> {
    unsafe fn from_value_optional(v: &'a Value) -> Option<Self> {
        Some(Self::from_value(v))
    }
}

impl SetValue for IntRange<i32> {
    unsafe fn set_value(v: &mut Value, r: &Self) {
        ffi::gst_value_set_int_range_step(v.to_glib_none_mut().0, r.min(), r.max(), r.step());
    }
}

impl glib::types::StaticType for IntRange<i64> {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_int64_range_get_type()) }
    }
}

impl<'a> FromValue<'a> for IntRange<i64> {
    unsafe fn from_value(v: &'a Value) -> Self {
        let min = ffi::gst_value_get_int64_range_min(v.to_glib_none().0);
        let max = ffi::gst_value_get_int64_range_max(v.to_glib_none().0);
        let step = ffi::gst_value_get_int64_range_step(v.to_glib_none().0);

        Self::new_with_step(min, max, step)
    }
}

impl<'a> FromValueOptional<'a> for IntRange<i64> {
    unsafe fn from_value_optional(v: &'a Value) -> Option<Self> {
        Some(Self::from_value(v))
    }
}

impl SetValue for IntRange<i64> {
    unsafe fn set_value(v: &mut Value, r: &Self) {
        ffi::gst_value_set_int64_range_step(v.to_glib_none_mut().0, r.min(), r.max(), r.step());
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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
        unsafe { from_glib(ffi::gst_fraction_range_get_type()) }
    }
}

impl<'a> FromValue<'a> for FractionRange {
    unsafe fn from_value(v: &'a Value) -> Self {
        let min = ffi::gst_value_get_fraction_range_min(v.to_glib_none().0);
        let max = ffi::gst_value_get_fraction_range_max(v.to_glib_none().0);

        let min_n = ffi::gst_value_get_fraction_numerator(min);
        let min_d = ffi::gst_value_get_fraction_denominator(min);
        let max_n = ffi::gst_value_get_fraction_numerator(max);
        let max_d = ffi::gst_value_get_fraction_denominator(max);

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
        ffi::gst_value_set_fraction_range_full(
            v.to_glib_none_mut().0,
            *r.min().numer(),
            *r.min().denom(),
            *r.max().numer(),
            *r.max().denom(),
        );
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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
        unsafe { from_glib(ffi::gst_bitmask_get_type()) }
    }
}

impl<'a> FromValue<'a> for Bitmask {
    unsafe fn from_value(v: &'a Value) -> Self {
        let v = ffi::gst_value_get_bitmask(v.to_glib_none().0);
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
        ffi::gst_value_set_bitmask(v.to_glib_none_mut().0, r.0);
    }
}

#[derive(Clone, Debug)]
pub struct Array<'a>(Cow<'a, [glib::SendValue]>);

unsafe impl<'a> Send for Array<'a> {}

impl<'a> Array<'a> {
    pub fn new(values: &[&ToSendValue]) -> Self {
        assert_initialized_main_thread!();

        Array(values.iter().map(|v| v.to_send_value()).collect())
    }

    pub fn into_owned(self) -> Array<'static> {
        Array(self.0.into_owned().into())
    }

    pub fn as_slice(&self) -> &[glib::SendValue] {
        self.0.borrow()
    }
}

impl<'a> From<&'a [&'a ToSendValue]> for Array<'a> {
    fn from(values: &'a [&'a ToSendValue]) -> Self {
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
        let arr = (*v.to_glib_none().0).data[0].v_pointer as *const glib_ffi::GArray;
        if arr.is_null() {
            Array(Cow::Borrowed(&[]))
        } else {
            #[cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
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
            ffi::gst_value_array_append_value(v.to_glib_none_mut().0, value.to_glib_none().0);
        }
    }
}

impl<'a> glib::types::StaticType for Array<'a> {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_value_array_get_type()) }
    }
}

#[derive(Clone, Debug)]
pub struct List<'a>(Cow<'a, [glib::SendValue]>);

unsafe impl<'a> Send for List<'a> {}

impl<'a> List<'a> {
    pub fn new(values: &[&ToSendValue]) -> Self {
        assert_initialized_main_thread!();

        List(values.iter().map(|v| v.to_send_value()).collect())
    }

    pub fn into_owned(self) -> List<'static> {
        List(self.0.into_owned().into())
    }

    pub fn as_slice(&self) -> &[glib::SendValue] {
        self.0.borrow()
    }
}

impl<'a> From<&'a [&'a ToSendValue]> for List<'a> {
    fn from(values: &'a [&'a ToSendValue]) -> Self {
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
        let arr = (*v.to_glib_none().0).data[0].v_pointer as *const glib_ffi::GArray;
        if arr.is_null() {
            List(Cow::Borrowed(&[]))
        } else {
            #[cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
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
            ffi::gst_value_list_append_value(v.to_glib_none_mut().0, value.to_glib_none().0);
        }
    }
}

impl<'a> glib::types::StaticType for List<'a> {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_value_list_get_type()) }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum ValueOrder {
    LessThan,
    Equal,
    GreaterThan,
    Unordered,
}

impl ToGlib for ValueOrder {
    type GlibType = i32;

    fn to_glib(&self) -> Self::GlibType {
        match *self {
            ValueOrder::LessThan => ffi::GST_VALUE_LESS_THAN,
            ValueOrder::Equal => ffi::GST_VALUE_EQUAL,
            ValueOrder::GreaterThan => ffi::GST_VALUE_GREATER_THAN,
            ValueOrder::Unordered => ffi::GST_VALUE_UNORDERED,
        }
    }
}

impl FromGlib<i32> for ValueOrder {
    fn from_glib(v: i32) -> Self {
        skip_assert_initialized!();

        match v {
            ffi::GST_VALUE_LESS_THAN => ValueOrder::LessThan,
            ffi::GST_VALUE_EQUAL => ValueOrder::Equal,
            ffi::GST_VALUE_GREATER_THAN => ValueOrder::GreaterThan,
            ffi::GST_VALUE_UNORDERED => ValueOrder::Unordered,
            _ => unreachable!(),
        }
    }
}

pub trait GstValueExt: Sized {
    fn can_compare(&self, other: &Self) -> bool;
    fn compare(&self, other: &Self) -> ValueOrder;
    fn can_intersect(&self, other: &Self) -> bool;
    fn intersect(&self, other: &Self) -> Option<Self>;
    fn can_subtract(&self, other: &Self) -> bool;
    fn subtract(&self, other: &Self) -> Option<Self>;
    fn can_union(&self, other: &Self) -> bool;
    fn union(&self, other: &Self) -> Option<Self>;
    fn fixate(&self) -> Option<Self>;
    fn is_fixed(&self) -> bool;
    fn is_subset(&self, superset: &Self) -> bool;
    fn serialize(&self) -> Option<String>;
    fn deserialize<'a, T: Into<&'a str>>(s: T) -> Option<glib::Value>;
}

impl GstValueExt for glib::Value {
    fn can_compare(&self, other: &Self) -> bool {
        unsafe {
            from_glib(ffi::gst_value_can_compare(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    fn compare(&self, other: &Self) -> ValueOrder {
        unsafe {
            from_glib(ffi::gst_value_compare(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    fn can_intersect(&self, other: &Self) -> bool {
        unsafe {
            from_glib(ffi::gst_value_can_intersect(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    fn intersect(&self, other: &Self) -> Option<Self> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let ret: bool = from_glib(ffi::gst_value_intersect(
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
            from_glib(ffi::gst_value_can_subtract(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    fn subtract(&self, other: &Self) -> Option<Self> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let ret: bool = from_glib(ffi::gst_value_subtract(
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
            from_glib(ffi::gst_value_can_union(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    fn union(&self, other: &Self) -> Option<Self> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let ret: bool = from_glib(ffi::gst_value_union(
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
            let ret: bool = from_glib(ffi::gst_value_fixate(
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
        unsafe { from_glib(ffi::gst_value_is_fixed(self.to_glib_none().0)) }
    }

    fn is_subset(&self, superset: &Self) -> bool {
        unsafe {
            from_glib(ffi::gst_value_is_subset(
                self.to_glib_none().0,
                superset.to_glib_none().0,
            ))
        }
    }

    fn serialize(&self) -> Option<String> {
        unsafe { from_glib_full(ffi::gst_value_serialize(self.to_glib_none().0)) }
    }

    fn deserialize<'a, T: Into<&'a str>>(s: T) -> Option<glib::Value> {
        assert_initialized_main_thread!();

        let s = s.into();

        unsafe {
            let mut value = glib::Value::uninitialized();
            let ret: bool = from_glib(ffi::gst_value_deserialize(
                value.to_glib_none_mut().0,
                s.to_glib_none().0,
            ));
            if ret {
                Some(value)
            } else {
                None
            }
        }
    }
}
