// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use num_rational::Rational32;
use std::fmt;
use std::ops;
use std::borrow::{Cow, Borrow};
use std::slice;

use glib;
use glib::value::{Value, FromValue, FromValueOptional, SetValue, ToValue};
use glib::translate::{from_glib, ToGlibPtr, ToGlibPtrMut};

use ffi;
use glib_ffi;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Fraction(pub Rational32);

impl Fraction {
    pub fn new(num: i32, den: i32) -> Fraction {
        (num, den).into()
    }

    pub fn approximate_f32(x: f32) -> Option<Fraction> {
        Rational32::approximate_float(x).map(|r| r.into())
    }

    pub fn approximate_f64(x: f64) -> Option<Fraction> {
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
        Fraction(x.into())
    }
}

impl From<(i32, i32)> for Fraction {
    fn from(x: (i32, i32)) -> Fraction {
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
        Fraction(x)
    }
}

impl From<Fraction> for Rational32 {
    fn from(x: Fraction) -> Rational32 {
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
        Self::new_with_step(min, max, 1)
    }

    pub fn new_with_step(min: i32, max: i32, step: i32) -> Self {
        assert!(min <= max);
        assert!(step > 0);

        Self {
            min: min,
            max: max,
            step: step,
        }
    }
}

impl IntRange<i64> {
    pub fn new(min: i64, max: i64) -> Self {
        Self::new_with_step(min, max, 1)
    }

    pub fn new_with_step(min: i64, max: i64, step: i64) -> Self {
        assert!(min <= max);
        assert!(step > 0);

        Self {
            min: min,
            max: max,
            step: step,
        }
    }
}

impl From<(i32, i32)> for IntRange<i32> {
    fn from((min, max): (i32, i32)) -> Self {
        Self::new(min, max)
    }
}

impl From<(i32, i32, i32)> for IntRange<i32> {
    fn from((min, max, step): (i32, i32, i32)) -> Self {
        Self::new_with_step(min, max, step)
    }
}

impl From<(i64, i64)> for IntRange<i64> {
    fn from((min, max): (i64, i64)) -> Self {
        Self::new(min, max)
    }
}

impl From<(i64, i64, i64)> for IntRange<i64> {
    fn from((min, max, step): (i64, i64, i64)) -> Self {
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
        let min = min.into();
        let max = max.into();

        assert!(min <= max);

        FractionRange {
            min: min,
            max: max,
        }
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
        ffi::gst_value_set_fraction_range_full(v.to_glib_none_mut().0, *r.min().numer(), *r.min().denom(), *r.max().numer(), *r.max().denom());
    }
}

#[derive(Clone, Debug)]
pub struct Array<'a>(Cow<'a, [glib::Value]>);

impl<'a> Array<'a> {
    pub fn new(values: &[&ToValue]) -> Self {
        Array(values.iter().map(|v| v.to_value()).collect())
    }

    pub fn into_owned(self) -> Array<'static> {
        Array(self.0.into_owned().into())
    }

    pub fn as_slice(&self) -> &[glib::Value] {
        self.0.borrow()
    }
}

impl<'a> From<&'a [&'a ToValue]> for Array<'a> {
    fn from(values: &'a [&'a ToValue]) -> Self {
        Self::new(values)
    }
}

impl<'a> From<&'a [glib::Value]> for Array<'a> {
    fn from(values: &'a [glib::Value]) -> Self {
        Array(Cow::Borrowed(values))
    }
}

impl<'a> FromValue<'a> for Array<'a> {
    unsafe fn from_value(v: &'a Value) -> Self {
        let arr = (*v.to_glib_none().0).data[0] as *const glib_ffi::GArray;
        if arr.is_null() {
            Array(Cow::Borrowed(&[]))
        } else {
            Array(Cow::Borrowed(slice::from_raw_parts((*arr).data as *const glib::Value, (*arr).len as usize)))
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
            ffi::gst_value_array_append_and_take_value(v.to_glib_none_mut().0, value.to_glib_full() as *mut _);
        }
    }
}

impl<'a> glib::types::StaticType for Array<'a> {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_value_array_get_type()) }
    }
}

#[derive(Clone, Debug)]
pub struct List<'a>(Cow<'a, [glib::Value]>);

impl<'a> List<'a> {
    pub fn new(values: &[&ToValue]) -> Self {
        List(values.iter().map(|v| v.to_value()).collect())
    }

    pub fn into_owned(self) -> List<'static> {
        List(self.0.into_owned().into())
    }

    pub fn as_slice(&self) -> &[glib::Value] {
        self.0.borrow()
    }
}

impl<'a> From<&'a [&'a ToValue]> for List<'a> {
    fn from(values: &'a [&'a ToValue]) -> Self {
        Self::new(values)
    }
}

impl<'a> From<&'a [glib::Value]> for List<'a> {
    fn from(values: &'a [glib::Value]) -> Self {
        List(Cow::Borrowed(values))
    }
}

impl<'a> FromValue<'a> for List<'a> {
    unsafe fn from_value(v: &'a Value) -> Self {
        let arr = (*v.to_glib_none().0).data[0] as *const glib_ffi::GArray;
        if arr.is_null() {
            List(Cow::Borrowed(&[]))
        } else {
            List(Cow::Borrowed(slice::from_raw_parts((*arr).data as *const glib::Value, (*arr).len as usize)))
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
            ffi::gst_value_list_append_and_take_value(v.to_glib_none_mut().0, value.to_glib_full() as *mut _);
        }
    }
}

impl<'a> glib::types::StaticType for List<'a> {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_value_list_get_type()) }
    }
}

// TODO: GStreamer value operations
