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

#[cfg(feature = "ser_de")]
#[macro_use]
pub(crate) mod serde {
    use glib;
    use glib::{StaticType, ToValue};

    use num_rational::Rational32;

    use serde::de;
    use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
    use serde::ser;
    use serde::ser::{Serialize, Serializer, SerializeTuple};

    use std::mem;

    use DateTime;
    //use Sample;

    use super::*;

    pub const ARRAY_TYPE_NAME: &'static str = "Array";
    pub const LIST_TYPE_NAME: &'static str = "List";

    fn get_other_type_id<T: StaticType>() -> usize {
        match T::static_type() {
            glib::Type::Other(type_id) => type_id,
            type_ => panic!("Expecting `Other` variant, found `{}`", type_),
        }
    }

    lazy_static! {
        pub(crate) static ref ARRAY_OTHER_TYPE_ID: usize = get_other_type_id::<Array>();
        pub(crate) static ref BITMASK_OTHER_TYPE_ID: usize = get_other_type_id::<Bitmask>();
        pub(crate) static ref DATE_TIME_OTHER_TYPE_ID: usize = get_other_type_id::<DateTime>();
        pub(crate) static ref FRACTION_OTHER_TYPE_ID: usize = get_other_type_id::<Fraction>();
        pub(crate) static ref FRACTION_RANGE_OTHER_TYPE_ID: usize =
            get_other_type_id::<FractionRange>();
        pub(crate) static ref INT_RANGE_I32_OTHER_TYPE_ID: usize =
            get_other_type_id::<IntRange<i32>>();
        pub(crate) static ref INT_RANGE_I64_OTHER_TYPE_ID: usize =
            get_other_type_id::<IntRange<i64>>();
        pub(crate) static ref LIST_OTHER_TYPE_ID: usize = get_other_type_id::<List>();
        //pub(crate) static ref SAMPLE_OTHER_TYPE_ID: usize = get_other_type_id::<Sample>();
    }

    impl<'a> Serialize for Fraction {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.0.serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for Fraction {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            Rational32::deserialize(deserializer)
                .and_then(|rational| Ok(Fraction::new(*rational.numer(), *rational.denom())))
        }
    }

    macro_rules! ser_value (
        ($value:expr, $t_str:expr, $t:ty, $ser_closure:expr) => (
            {
                let value = $value.get::<$t>().unwrap();
                $ser_closure($t_str, value)
            }
        );
        ($value:expr, $ser_closure:expr) => (
            match $value.type_() {
                glib::Type::I8 => ser_value!($value, "i8", i8, $ser_closure),
                glib::Type::U8 => ser_value!($value, "ui8", u8, $ser_closure),
                glib::Type::Bool => ser_value!($value, "bool", bool, $ser_closure),
                glib::Type::I32 => ser_value!($value, "i32", i32, $ser_closure),
                glib::Type::U32 => ser_value!($value, "u32", u32, $ser_closure),
                glib::Type::ILong => ser_value!($value, "ilong", i32, $ser_closure),
                glib::Type::ULong => ser_value!($value, "ulong", u32, $ser_closure),
                glib::Type::I64 => ser_value!($value, "i64", i64, $ser_closure),
                glib::Type::U64 => ser_value!($value, "u64", u64, $ser_closure),
                glib::Type::F32 => ser_value!($value, "f32", f32, $ser_closure),
                glib::Type::F64 => ser_value!($value, "f64", f64, $ser_closure),
                glib::Type::String => ser_value!($value, "String", String, $ser_closure),
                glib::Type::Other(type_id) => {
                    if *ARRAY_OTHER_TYPE_ID == type_id {
                        ser_value!($value, ARRAY_TYPE_NAME, Array, $ser_closure)
                    } else if *BITMASK_OTHER_TYPE_ID == type_id {
                        ser_value!($value, "Bitmask", Bitmask, $ser_closure)
                    } else if *DATE_TIME_OTHER_TYPE_ID == type_id {
                        ser_value!($value, "DateTime", DateTime, $ser_closure)
                    } else if *FRACTION_OTHER_TYPE_ID == type_id {
                        ser_value!($value, "Fraction", Fraction, $ser_closure)
                    } else if *FRACTION_RANGE_OTHER_TYPE_ID == type_id {
                        ser_value!($value, "FractionRange", FractionRange, $ser_closure)
                    } else if *INT_RANGE_I32_OTHER_TYPE_ID == type_id {
                        ser_value!($value, "IntRange<i32>", IntRange<i32>, $ser_closure)
                    } else if *INT_RANGE_I64_OTHER_TYPE_ID == type_id {
                        ser_value!($value, "IntRange<i64>", IntRange<i64>, $ser_closure)
                    } else if *LIST_OTHER_TYPE_ID == type_id {
                        ser_value!($value, LIST_TYPE_NAME, List, $ser_closure)
                    /*} else if *SAMPLE_OTHER_TYPE_ID == type_id {
                        ser_value!($value, "Sample", Sample, $ser_closure)*/
                    } else {
                        Err(
                            ser::Error::custom(
                                format!("unimplemented `Value` serialization for type {}",
                                    glib::Type::Other(type_id),
                                )
                            )
                        )
                    }
                }
                type_ => {
                    Err(
                        ser::Error::custom(
                            format!("unimplemented `Value` serialization for type {}", type_)
                        )
                    )
                }
            }
        )
    );

    pub(crate) struct SendValue(glib::SendValue);
    impl SendValue {
        pub(crate) fn from(send_value: glib::SendValue) -> Self {
            SendValue(send_value)
        }
    }

    impl From<SendValue> for glib::SendValue {
        fn from(send_value: SendValue) -> Self {
            send_value.0
        }
    }

    impl Serialize for SendValue {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            ser_value!(self.0, |type_, value| {
                let mut tup = serializer.serialize_tuple(2)?;
                tup.serialize_element(type_)?;
                tup.serialize_element(&value)?;
                tup.end()
            })
        }
    }

    macro_rules! impl_ser_send_value_collection (
        ($t:ident) => (
            impl<'a> Serialize for $t<'a> {
                fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                    let send_value_vec = unsafe {
                        mem::transmute::<&[glib::SendValue], &[SendValue]>(
                            self.as_slice()
                        )
                    };
                    send_value_vec.serialize(serializer)
                }
            }
        );
    );

    impl_ser_send_value_collection!(Array);
    impl_ser_send_value_collection!(List);

    macro_rules! de_value(
        ($outer_type:expr, $type_name:expr, $seq:expr, $t:ty) => (
            $seq
                .next_element::<$t>()?
                .ok_or_else(||
                    de::Error::custom(format!(
                        "Expected a value for `{}` with type {:?}, found `None`",
                        $outer_type,
                        $type_name,
                    ))
                )?
                .to_value()
        );
    );

    macro_rules! de_send_value(
        ($type_name:expr, $seq:expr, $t:ty) => (
            SendValue::from(
                de_value!("Value", $type_name, $seq, $t)
                    .try_into_send_value::<$t>()
                    .map_err(|_|
                        de::Error::custom(format!(
                            "Failed to convert `Value` with type {:?} to `SendValue`",
                            $type_name,
                        ))
                    )?
            )
        );
        ($type_name:expr, $seq:expr) => (
            match $type_name.as_str() {
                "i8" => de_send_value!($type_name, $seq, i8),
                "u8" => de_send_value!($type_name, $seq, u8),
                "bool" => de_send_value!($type_name, $seq, bool),
                "i32" => de_send_value!($type_name, $seq, i32),
                "u32" => de_send_value!($type_name, $seq, u32),
                "ilong" => de_send_value!($type_name, $seq, i32),
                "ulong" => de_send_value!($type_name, $seq, u32),
                "i64" => de_send_value!($type_name, $seq, i64),
                "u64" => de_send_value!($type_name, $seq, u64),
                "f32" => de_send_value!($type_name, $seq, f32),
                "f64" => de_send_value!($type_name, $seq, f64),
                "String" => de_send_value!($type_name, $seq, String),
                "Array" => de_send_value!($type_name, $seq, Array),
                "Bitmask" => de_send_value!($type_name, $seq, Bitmask),
                "DateTime" => de_send_value!($type_name, $seq, DateTime),
                "Fraction" => de_send_value!($type_name, $seq, Fraction),
                "FractionRange" => de_send_value!($type_name, $seq, FractionRange),
                "IntRange<i32>" => de_send_value!($type_name, $seq, IntRange<i32>),
                "IntRange<i64>" => de_send_value!($type_name, $seq, IntRange<i64>),
                //"Sample" => de_send_value!($type_name, $seq, Sample),
                _ => return Err(
                    de::Error::custom(
                        format!(
                            "unimplemented deserialization for `Value` with type `{}`",
                            $type_name,
                        ),
                    )
                ),
            }
        );
    );

    struct SendValueVisitor;
    impl<'de> Visitor<'de> for SendValueVisitor {
        type Value = SendValue;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a tuple of 2 elements (type name: String, value: Value type)")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let type_name = seq.next_element::<String>()?
                .ok_or(de::Error::custom("Expected a value for `Value` type, found `None`"))?;
            Ok(de_send_value!(type_name, seq))
        }
    }

    impl<'de> Deserialize<'de> for SendValue {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_tuple(2, SendValueVisitor{})
        }
    }

    macro_rules! impl_de_send_value_collection (
        ($t:ident) => {
            impl<'a, 'de> Deserialize<'de> for $t<'a> {
                fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    let send_value_vec = Vec::<SendValue>::deserialize(deserializer)?;
                    Ok($t(Cow::Owned(unsafe{
                        mem::transmute::<Vec<SendValue>, Vec<glib::SendValue>>(send_value_vec)
                    })))
                }
            }
        }
    );

    impl_de_send_value_collection!(Array);
    impl_de_send_value_collection!(List);
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "ser_de")]
    #[test]
    fn test_serialize_simple() {
        extern crate ron;
        extern crate serde_json;

        use Fraction;
        use FractionRange;
        use IntRange;
        use Bitmask;

        ::init().unwrap();

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        // Fraction
        let fraction = Fraction::new(1, 3);

        let res = ron::ser::to_string_pretty(&fraction, pretty_config.clone());
        assert_eq!(Ok("(1, 3)".to_owned()), res);

        let res = serde_json::to_string(&fraction).unwrap();
        assert_eq!("[1,3]".to_owned(), res);

        // FractionRange
        let fraction_range = FractionRange::new(Fraction::new(1, 3), Fraction::new(1, 2));

        let res = ron::ser::to_string_pretty(&fraction_range, pretty_config.clone());
        assert_eq!(
            Ok(
                concat!(
                    "(",
                    "    min: (1, 3),",
                    "    max: (1, 2),",
                    ")"
                )
                    .to_owned()
            ),
            res,
        );

        let res = serde_json::to_string(&fraction_range).unwrap();
        assert_eq!("{\"min\":[1,3],\"max\":[1,2]}".to_owned(), res);

        // IntRange
        let int_range = IntRange::<i32>::new_with_step(0, 42, 21);
        let res = ron::ser::to_string_pretty(&int_range, pretty_config.clone());
        assert_eq!(
            Ok(
                concat!(
                    "(",
                    "    min: 0,",
                    "    max: 42,",
                    "    step: 21,",
                    ")"
                )
                    .to_owned()
            ),
            res,
        );

        let res = serde_json::to_string(&int_range).unwrap();
        assert_eq!("{\"min\":0,\"max\":42,\"step\":21}".to_owned(), res);

        // Bitmask
        let bitmask = Bitmask::new(1024 + 128 + 32);

        let res = ron::ser::to_string_pretty(&bitmask, pretty_config.clone());
        assert_eq!(Ok("(1184)".to_owned()), res);

        let res = serde_json::to_string(&bitmask).unwrap();
        assert_eq!("1184".to_owned(), res);
    }

    #[cfg(feature = "ser_de")]
    #[test]
    fn test_serialize_collections() {
        extern crate ron;
        extern crate serde_json;

        use glib::value::ToValue;

        use Array;
        use Fraction;
        use List;

        ::init().unwrap();

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        // Array
        let value_13 = Fraction::new(1, 3).to_value();
        let send_value_13 = value_13.try_into_send_value::<Fraction>().unwrap();

        let value_12 = Fraction::new(1, 2).to_value();
        let send_value_12 = value_12.try_into_send_value::<Fraction>().unwrap();

        let value_str = "test str".to_value();
        let send_value_str = value_str.try_into_send_value::<String>().unwrap();

        let array = Array::new(&[&send_value_13, &send_value_12, &send_value_str]);

        let res = ron::ser::to_string_pretty(&array, pretty_config.clone());
        assert_eq!(
            Ok(
                concat!(
                    "[",
                    "    (\"Fraction\", (1, 3)),",
                    "    (\"Fraction\", (1, 2)),",
                    "    (\"String\", \"test str\"),",
                    "]"
                )
                    .to_owned()
            ),
            res,
        );

        let res = serde_json::to_string(&array).unwrap();
        assert_eq!(
            "[[\"Fraction\",[1,3]],[\"Fraction\",[1,2]],[\"String\",\"test str\"]]"
                .to_owned(),
            res
        );

        // List
        let value_12 = Fraction::new(1, 2).to_value();
        let send_value_12 = value_12.try_into_send_value::<Fraction>().unwrap();

        let value_str = "test str".to_value();
        let send_value_str = value_str.try_into_send_value::<String>().unwrap();

        let list = List::new(&[&send_value_12, &send_value_str]);

        let res = ron::ser::to_string_pretty(&list, pretty_config.clone());
        assert_eq!(
            Ok(
                concat!(
                    "[",
                    "    (\"Fraction\", (1, 2)),",
                    "    (\"String\", \"test str\"),",
                    "]"
                )
                    .to_owned()
            ),
            res,
        );
    }

    #[cfg(feature = "ser_de")]
    #[test]
    fn test_deserialize_simple() {
        extern crate ron;
        extern crate serde_json;

        use Fraction;
        use FractionRange;
        use IntRange;
        use Bitmask;

        ::init().unwrap();

        // Fraction
        let fraction_ron = "(1, 3)";
        let fraction: Fraction = ron::de::from_str(fraction_ron).unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &3);

        let fraction_json = "[1,3]";
        let fraction: Fraction = serde_json::from_str(fraction_json).unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &3);

        // FractionRange
        let fraction_range_ron = "(min: (1, 3), max: (1, 2))";
        let fraction_range: FractionRange = ron::de::from_str(fraction_range_ron).unwrap();
        assert_eq!(fraction_range.min().0.denom(), &3);
        assert_eq!(fraction_range.max().0.denom(), &2);

        let fraction_range_json = "{\"min\":[1,3],\"max\":[1,2]}";
        let fraction_range: FractionRange = serde_json::from_str(fraction_range_json).unwrap();
        assert_eq!(fraction_range.min().0.denom(), &3);
        assert_eq!(fraction_range.max().0.denom(), &2);

        // IntRange
        let int_range_ron = "(min: 0, max: 42, step: 21)";
        let int_range: IntRange<i32> = ron::de::from_str(int_range_ron).unwrap();
        assert_eq!(int_range.min(), 0);
        assert_eq!(int_range.max(), 42);
        assert_eq!(int_range.step(), 21);

        let int_range_json = "{\"min\":0,\"max\":42,\"step\":21}";
        let int_range: IntRange<i32> = serde_json::from_str(int_range_json).unwrap();
        assert_eq!(int_range.min(), 0);
        assert_eq!(int_range.max(), 42);
        assert_eq!(int_range.step(), 21);

        // Bitmask
        let bitmask_ref = Bitmask::new(1024 + 128 + 32);

        let bitmask_ron = "(1184)";
        let bitmask: Bitmask = ron::de::from_str(bitmask_ron).unwrap();
        assert_eq!(bitmask_ref, bitmask);

        let bitmask_json = "1184";
        let bitmask: Bitmask = serde_json::from_str(bitmask_json).unwrap();
        assert_eq!(bitmask_ref, bitmask);
    }

    #[cfg(feature = "ser_de")]
    #[test]
    fn test_deserialize_collections() {
        extern crate ron;
        extern crate serde_json;

        use Array;
        use Fraction;
        use List;

        ::init().unwrap();

        // Array
        let array_ron =
            r#"[
                ("Fraction", (1, 3)),
                ("Fraction", (1, 2)),
                ("String", "test str"),
            ]"#;
        let array: Array = ron::de::from_str(array_ron).unwrap();
        assert_eq!(3, array.0.len());

        let fraction = array.0[0].get::<Fraction>().unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &3);

        let fraction = array.0[1].get::<Fraction>().unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &2);

        assert_eq!("test str".to_owned(), array.0[2].get::<String>().unwrap());

        let array_json =
            r#"[["Fraction",[1,3]],["Fraction",[1,2]],["String","test str"]]"#;
        let array: Array = serde_json::from_str(array_json).unwrap();
        assert_eq!(3, array.0.len());

        let fraction = array.0[0].get::<Fraction>().unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &3);

        let fraction = array.0[1].get::<Fraction>().unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &2);

        assert_eq!("test str".to_owned(), array.0[2].get::<String>().unwrap());

        // List
        let list_ron =
            r#"[
                ("Fraction", (1, 2)),
                ("String", "test str"),
            ]"#;
        let list: List = ron::de::from_str(list_ron).unwrap();
        assert_eq!(2, list.0.len());

        let fraction = list.0[0].get::<Fraction>().unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &2);

        assert_eq!("test str".to_owned(), list.0[1].get::<String>().unwrap());
    }
}
