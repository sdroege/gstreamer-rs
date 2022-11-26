// Take a look at the license at the top of the repository in the LICENSE file.

use num_rational::Rational32;
use std::cmp;
use std::fmt;
use std::ops;
use std::slice;

use glib::translate::*;
use glib::StaticType;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Fraction(pub Rational32);

impl Fraction {
    pub fn new(num: i32, den: i32) -> Self {
        assert_initialized_main_thread!();
        (num, den).into()
    }

    pub fn approximate_f32(x: f32) -> Option<Self> {
        assert_initialized_main_thread!();
        Rational32::approximate_float(x).map(|r| r.into())
    }

    pub fn approximate_f64(x: f64) -> Option<Self> {
        assert_initialized_main_thread!();
        Rational32::approximate_float(x).map(|r| r.into())
    }

    pub fn numer(&self) -> i32 {
        *self.0.numer()
    }

    pub fn denom(&self) -> i32 {
        *self.0.denom()
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl ops::Deref for Fraction {
    type Target = Rational32;

    fn deref(&self) -> &Self::Target {
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

            fn $f(self, other: Fraction) -> Self::Output {
                Fraction((self.0).$f(other.0))
            }
        }

        impl ops::$name<Fraction> for &Fraction {
            type Output = Fraction;

            fn $f(self, other: Fraction) -> Self::Output {
                Fraction((self.0).$f(other.0))
            }
        }

        impl ops::$name<&Fraction> for Fraction {
            type Output = Fraction;

            fn $f(self, other: &Fraction) -> Self::Output {
                Fraction((self.0).$f(other.0))
            }
        }

        impl ops::$name<&Fraction> for &Fraction {
            type Output = Fraction;

            fn $f(self, other: &Fraction) -> Self::Output {
                Fraction((self.0).$f(other.0))
            }
        }

        impl ops::$name<i32> for Fraction {
            type Output = Fraction;

            fn $f(self, other: i32) -> Self::Output {
                self.$f(Fraction::from(other))
            }
        }

        impl ops::$name<i32> for &Fraction {
            type Output = Fraction;

            fn $f(self, other: i32) -> Self::Output {
                self.$f(Fraction::from(other))
            }
        }

        impl ops::$name<&i32> for Fraction {
            type Output = Fraction;

            fn $f(self, other: &i32) -> Self::Output {
                self.$f(Fraction::from(*other))
            }
        }

        impl ops::$name<&i32> for &Fraction {
            type Output = Fraction;

            fn $f(self, other: &i32) -> Self::Output {
                self.$f(Fraction::from(*other))
            }
        }

        impl ops::$name<Fraction> for i32 {
            type Output = Fraction;

            fn $f(self, other: Fraction) -> Self::Output {
                Fraction::from(self).$f(other)
            }
        }

        impl ops::$name<&Fraction> for i32 {
            type Output = Fraction;

            fn $f(self, other: &Fraction) -> Self::Output {
                Fraction::from(self).$f(other)
            }
        }

        impl ops::$name<Fraction> for &i32 {
            type Output = Fraction;

            fn $f(self, other: Fraction) -> Self::Output {
                Fraction::from(*self).$f(other)
            }
        }

        impl ops::$name<&Fraction> for &i32 {
            type Output = Fraction;

            fn $f(self, other: &Fraction) -> Self::Output {
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

    fn neg(self) -> Self::Output {
        Fraction(self.0.neg())
    }
}

impl ops::Neg for &Fraction {
    type Output = Fraction;

    fn neg(self) -> Self::Output {
        Fraction(self.0.neg())
    }
}

impl From<i32> for Fraction {
    fn from(x: i32) -> Self {
        assert_initialized_main_thread!();
        Fraction(x.into())
    }
}

impl From<(i32, i32)> for Fraction {
    fn from(x: (i32, i32)) -> Self {
        assert_initialized_main_thread!();
        Fraction(x.into())
    }
}

impl From<Fraction> for (i32, i32) {
    fn from(f: Fraction) -> Self {
        skip_assert_initialized!();
        f.0.into()
    }
}

impl From<Rational32> for Fraction {
    fn from(x: Rational32) -> Self {
        assert_initialized_main_thread!();
        Fraction(x)
    }
}

impl From<Fraction> for Rational32 {
    fn from(x: Fraction) -> Self {
        skip_assert_initialized!();
        x.0
    }
}

impl glib::types::StaticType for Fraction {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_fraction_get_type()) }
    }
}

impl glib::value::ValueType for Fraction {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for Fraction {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        let n = ffi::gst_value_get_fraction_numerator(value.to_glib_none().0);
        let d = ffi::gst_value_get_fraction_denominator(value.to_glib_none().0);

        Fraction::new(n, d)
    }
}

impl glib::value::ToValue for Fraction {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            ffi::gst_value_set_fraction(value.to_glib_none_mut().0, self.numer(), self.denom());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<Fraction> for glib::Value {
    fn from(v: Fraction) -> glib::Value {
        skip_assert_initialized!();
        glib::value::ToValue::to_value(&v)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

#[doc(hidden)]
pub trait IntRangeType: Sized + Clone + Copy + 'static {
    fn with_min_max(min: Self, max: Self) -> IntRange<Self>;
    fn with_step(min: Self, max: Self, step: Self) -> IntRange<Self>;
}

impl IntRangeType for i32 {
    fn with_min_max(min: i32, max: i32) -> IntRange<Self> {
        skip_assert_initialized!();
        IntRange { min, max, step: 1 }
    }

    fn with_step(min: i32, max: i32, step: i32) -> IntRange<Self> {
        assert_initialized_main_thread!();

        assert!(min <= max);
        assert!(step > 0);

        IntRange { min, max, step }
    }
}

impl IntRangeType for i64 {
    fn with_min_max(min: i64, max: i64) -> IntRange<Self> {
        skip_assert_initialized!();
        IntRange { min, max, step: 1 }
    }

    fn with_step(min: i64, max: i64, step: i64) -> IntRange<Self> {
        assert_initialized_main_thread!();

        assert!(min <= max);
        assert!(step > 0);

        IntRange { min, max, step }
    }
}

impl<T: IntRangeType> IntRange<T> {
    pub fn new(min: T, max: T) -> IntRange<T> {
        assert_initialized_main_thread!();
        T::with_min_max(min, max)
    }

    pub fn with_step(min: T, max: T, step: T) -> IntRange<T> {
        assert_initialized_main_thread!();
        T::with_step(min, max, step)
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
        unsafe { from_glib(ffi::gst_int_range_get_type()) }
    }
}

impl glib::value::ValueType for IntRange<i32> {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for IntRange<i32> {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        let min = ffi::gst_value_get_int_range_min(value.to_glib_none().0);
        let max = ffi::gst_value_get_int_range_max(value.to_glib_none().0);
        let step = ffi::gst_value_get_int_range_step(value.to_glib_none().0);

        Self::with_step(min, max, step)
    }
}

impl glib::value::ToValue for IntRange<i32> {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            ffi::gst_value_set_int_range_step(
                value.to_glib_none_mut().0,
                self.min(),
                self.max(),
                self.step(),
            );
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<IntRange<i32>> for glib::Value {
    fn from(v: IntRange<i32>) -> glib::Value {
        skip_assert_initialized!();
        glib::value::ToValue::to_value(&v)
    }
}

impl glib::types::StaticType for IntRange<i64> {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_int64_range_get_type()) }
    }
}

impl glib::value::ValueType for IntRange<i64> {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for IntRange<i64> {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        let min = ffi::gst_value_get_int64_range_min(value.to_glib_none().0);
        let max = ffi::gst_value_get_int64_range_max(value.to_glib_none().0);
        let step = ffi::gst_value_get_int64_range_step(value.to_glib_none().0);

        Self::with_step(min, max, step)
    }
}

impl glib::value::ToValue for IntRange<i64> {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            ffi::gst_value_set_int64_range_step(
                value.to_glib_none_mut().0,
                self.min(),
                self.max(),
                self.step(),
            );
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<IntRange<i64>> for glib::Value {
    fn from(v: IntRange<i64>) -> glib::Value {
        skip_assert_initialized!();
        glib::value::ToValue::to_value(&v)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl glib::value::ValueType for FractionRange {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for FractionRange {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        let min = ffi::gst_value_get_fraction_range_min(value.to_glib_none().0);
        let max = ffi::gst_value_get_fraction_range_max(value.to_glib_none().0);

        let min_n = ffi::gst_value_get_fraction_numerator(min);
        let min_d = ffi::gst_value_get_fraction_denominator(min);
        let max_n = ffi::gst_value_get_fraction_numerator(max);
        let max_d = ffi::gst_value_get_fraction_denominator(max);

        Self::new((min_n, min_d), (max_n, max_d))
    }
}

impl glib::value::ToValue for FractionRange {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            ffi::gst_value_set_fraction_range_full(
                value.to_glib_none_mut().0,
                self.min().numer(),
                self.min().denom(),
                self.max().numer(),
                self.max().denom(),
            );
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<FractionRange> for glib::Value {
    fn from(v: FractionRange) -> glib::Value {
        skip_assert_initialized!();
        glib::value::ToValue::to_value(&v)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl glib::value::ValueType for Bitmask {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for Bitmask {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        let v = ffi::gst_value_get_bitmask(value.to_glib_none().0);
        Self::new(v)
    }
}

impl glib::value::ToValue for Bitmask {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            ffi::gst_value_set_bitmask(value.to_glib_none_mut().0, self.0);
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<Bitmask> for glib::Value {
    fn from(v: Bitmask) -> glib::Value {
        skip_assert_initialized!();
        glib::value::ToValue::to_value(&v)
    }
}

#[derive(Clone)]
pub struct Array(glib::SendValue);

unsafe impl Send for Array {}
unsafe impl Sync for Array {}

impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Array").field(&self.as_slice()).finish()
    }
}

impl Array {
    pub fn new(values: impl IntoIterator<Item = impl Into<glib::Value> + Send>) -> Self {
        assert_initialized_main_thread!();

        unsafe {
            let mut value = glib::Value::for_value_type::<Array>();
            for v in values.into_iter() {
                let mut v = v.into().into_raw();
                ffi::gst_value_array_append_and_take_value(value.to_glib_none_mut().0, &mut v);
            }

            Self(glib::SendValue::unsafe_from(value.into_raw()))
        }
    }

    pub fn from_values(values: impl IntoIterator<Item = glib::SendValue>) -> Self {
        assert_initialized_main_thread!();

        Self::new(values)
    }

    pub fn as_slice(&self) -> &[glib::SendValue] {
        unsafe {
            let arr = (*self.0.as_ptr()).data[0].v_pointer as *const glib::ffi::GArray;
            if arr.is_null() || (*arr).len == 0 {
                &[]
            } else {
                #[allow(clippy::cast_ptr_alignment)]
                slice::from_raw_parts((*arr).data as *const glib::SendValue, (*arr).len as usize)
            }
        }
    }

    pub fn append_value(&mut self, value: glib::SendValue) {
        unsafe {
            ffi::gst_value_array_append_and_take_value(
                self.0.to_glib_none_mut().0,
                &mut value.into_raw(),
            );
        }
    }

    pub fn append(&mut self, value: impl Into<glib::Value> + Send) {
        self.append_value(glib::SendValue::from_owned(value));
    }
}

impl Default for Array {
    fn default() -> Self {
        assert_initialized_main_thread!();

        unsafe {
            let value = glib::Value::for_value_type::<Array>();

            Self(glib::SendValue::unsafe_from(value.into_raw()))
        }
    }
}

impl ops::Deref for Array {
    type Target = [glib::SendValue];

    fn deref(&self) -> &[glib::SendValue] {
        self.as_slice()
    }
}

impl AsRef<[glib::SendValue]> for Array {
    fn as_ref(&self) -> &[glib::SendValue] {
        self.as_slice()
    }
}

impl std::iter::FromIterator<glib::SendValue> for Array {
    fn from_iter<T: IntoIterator<Item = glib::SendValue>>(iter: T) -> Self {
        assert_initialized_main_thread!();
        Self::from_values(iter)
    }
}

impl std::iter::Extend<glib::SendValue> for Array {
    fn extend<T: IntoIterator<Item = glib::SendValue>>(&mut self, iter: T) {
        for v in iter.into_iter() {
            self.append_value(v);
        }
    }
}

impl glib::value::ValueType for Array {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for Array {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        Self(glib::SendValue::unsafe_from(value.clone().into_raw()))
    }
}

impl glib::value::ToValue for Array {
    fn to_value(&self) -> glib::Value {
        self.0.clone().into()
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<Array> for glib::Value {
    fn from(v: Array) -> glib::Value {
        skip_assert_initialized!();
        v.0.into()
    }
}

impl glib::types::StaticType for Array {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_value_array_get_type()) }
    }
}

#[derive(Debug, Clone)]
pub struct ArrayRef<'a>(&'a [glib::SendValue]);

unsafe impl<'a> Send for ArrayRef<'a> {}
unsafe impl<'a> Sync for ArrayRef<'a> {}

impl<'a> ArrayRef<'a> {
    pub fn new(values: &'a [glib::SendValue]) -> Self {
        assert_initialized_main_thread!();

        Self(values)
    }

    pub fn as_slice(&self) -> &'a [glib::SendValue] {
        self.0
    }
}

impl<'a> ops::Deref for ArrayRef<'a> {
    type Target = [glib::SendValue];

    fn deref(&self) -> &[glib::SendValue] {
        self.as_slice()
    }
}

impl<'a> AsRef<[glib::SendValue]> for ArrayRef<'a> {
    fn as_ref(&self) -> &[glib::SendValue] {
        self.as_slice()
    }
}

unsafe impl<'a> glib::value::FromValue<'a> for ArrayRef<'a> {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        let arr = (*value.as_ptr()).data[0].v_pointer as *const glib::ffi::GArray;
        if arr.is_null() || (*arr).len == 0 {
            Self(&[])
        } else {
            #[allow(clippy::cast_ptr_alignment)]
            Self(slice::from_raw_parts(
                (*arr).data as *const glib::SendValue,
                (*arr).len as usize,
            ))
        }
    }
}

impl<'a> glib::value::ToValue for ArrayRef<'a> {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Array>();
        unsafe {
            for v in self.0 {
                ffi::gst_value_array_append_value(value.to_glib_none_mut().0, v.to_glib_none().0);
            }
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl<'a> From<ArrayRef<'a>> for glib::Value {
    fn from(v: ArrayRef<'a>) -> glib::Value {
        skip_assert_initialized!();
        glib::value::ToValue::to_value(&v)
    }
}

impl<'a> glib::types::StaticType for ArrayRef<'a> {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_value_array_get_type()) }
    }
}

#[derive(Clone)]
pub struct List(glib::SendValue);

unsafe impl Send for List {}
unsafe impl Sync for List {}

impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("List").field(&self.as_slice()).finish()
    }
}

impl List {
    pub fn new(values: impl IntoIterator<Item = impl Into<glib::Value> + Send>) -> Self {
        assert_initialized_main_thread!();

        unsafe {
            let mut value = glib::Value::for_value_type::<List>();
            for v in values.into_iter() {
                let mut v = v.into().into_raw();
                ffi::gst_value_list_append_and_take_value(value.to_glib_none_mut().0, &mut v);
            }

            Self(glib::SendValue::unsafe_from(value.into_raw()))
        }
    }

    pub fn from_values(values: impl IntoIterator<Item = glib::SendValue>) -> Self {
        assert_initialized_main_thread!();

        Self::new(values)
    }

    pub fn as_slice(&self) -> &[glib::SendValue] {
        unsafe {
            let arr = (*self.0.as_ptr()).data[0].v_pointer as *const glib::ffi::GArray;
            if arr.is_null() || (*arr).len == 0 {
                &[]
            } else {
                #[allow(clippy::cast_ptr_alignment)]
                slice::from_raw_parts((*arr).data as *const glib::SendValue, (*arr).len as usize)
            }
        }
    }

    pub fn append_value(&mut self, value: glib::SendValue) {
        unsafe {
            ffi::gst_value_list_append_and_take_value(
                self.0.to_glib_none_mut().0,
                &mut value.into_raw(),
            );
        }
    }

    pub fn append(&mut self, value: impl Into<glib::Value> + Send) {
        self.append_value(glib::SendValue::from_owned(value));
    }
}

impl Default for List {
    fn default() -> Self {
        assert_initialized_main_thread!();

        unsafe {
            let value = glib::Value::for_value_type::<List>();

            Self(glib::SendValue::unsafe_from(value.into_raw()))
        }
    }
}

impl ops::Deref for List {
    type Target = [glib::SendValue];

    fn deref(&self) -> &[glib::SendValue] {
        self.as_slice()
    }
}

impl AsRef<[glib::SendValue]> for List {
    fn as_ref(&self) -> &[glib::SendValue] {
        self.as_slice()
    }
}

impl std::iter::FromIterator<glib::SendValue> for List {
    fn from_iter<T: IntoIterator<Item = glib::SendValue>>(iter: T) -> Self {
        assert_initialized_main_thread!();
        Self::from_values(iter)
    }
}

impl std::iter::Extend<glib::SendValue> for List {
    fn extend<T: IntoIterator<Item = glib::SendValue>>(&mut self, iter: T) {
        for v in iter.into_iter() {
            self.append_value(v);
        }
    }
}

impl glib::value::ValueType for List {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for List {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        Self(glib::SendValue::unsafe_from(value.clone().into_raw()))
    }
}

impl glib::value::ToValue for List {
    fn to_value(&self) -> glib::Value {
        self.0.clone().into()
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<List> for glib::Value {
    fn from(v: List) -> glib::Value {
        skip_assert_initialized!();
        v.0.into()
    }
}

impl glib::types::StaticType for List {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_value_list_get_type()) }
    }
}

#[derive(Debug, Clone)]
pub struct ListRef<'a>(&'a [glib::SendValue]);

unsafe impl<'a> Send for ListRef<'a> {}
unsafe impl<'a> Sync for ListRef<'a> {}

impl<'a> ListRef<'a> {
    pub fn new(values: &'a [glib::SendValue]) -> Self {
        assert_initialized_main_thread!();

        Self(values)
    }

    pub fn as_slice(&self) -> &'a [glib::SendValue] {
        self.0
    }
}

impl<'a> ops::Deref for ListRef<'a> {
    type Target = [glib::SendValue];

    fn deref(&self) -> &[glib::SendValue] {
        self.as_slice()
    }
}

impl<'a> AsRef<[glib::SendValue]> for ListRef<'a> {
    fn as_ref(&self) -> &[glib::SendValue] {
        self.as_slice()
    }
}

unsafe impl<'a> glib::value::FromValue<'a> for ListRef<'a> {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        let arr = (*value.as_ptr()).data[0].v_pointer as *const glib::ffi::GArray;
        if arr.is_null() || (*arr).len == 0 {
            Self(&[])
        } else {
            #[allow(clippy::cast_ptr_alignment)]
            Self(slice::from_raw_parts(
                (*arr).data as *const glib::SendValue,
                (*arr).len as usize,
            ))
        }
    }
}

impl<'a> glib::value::ToValue for ListRef<'a> {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<List>();
        unsafe {
            for v in self.0 {
                ffi::gst_value_list_append_value(value.to_glib_none_mut().0, v.to_glib_none().0);
            }
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl<'a> From<ListRef<'a>> for glib::Value {
    fn from(v: ListRef<'a>) -> glib::Value {
        skip_assert_initialized!();
        glib::value::ToValue::to_value(&v)
    }
}

impl<'a> glib::types::StaticType for ListRef<'a> {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_value_list_get_type()) }
    }
}

pub trait GstValueExt: Sized {
    #[doc(alias = "gst_value_can_compare")]
    fn can_compare(&self, other: &Self) -> bool;
    #[doc(alias = "gst_value_compare")]
    fn compare(&self, other: &Self) -> Option<cmp::Ordering>;
    fn eq(&self, other: &Self) -> bool;
    #[doc(alias = "gst_value_can_intersect")]
    fn can_intersect(&self, other: &Self) -> bool;
    #[doc(alias = "gst_value_intersect")]
    fn intersect(&self, other: &Self) -> Option<Self>;
    #[doc(alias = "gst_value_can_subtract")]
    fn can_subtract(&self, other: &Self) -> bool;
    #[doc(alias = "gst_value_subtract")]
    fn subtract(&self, other: &Self) -> Option<Self>;
    #[doc(alias = "gst_value_can_union")]
    fn can_union(&self, other: &Self) -> bool;
    #[doc(alias = "gst_value_union")]
    fn union(&self, other: &Self) -> Option<Self>;
    #[doc(alias = "gst_value_fixate")]
    fn fixate(&self) -> Option<Self>;
    #[doc(alias = "gst_value_is_fixed")]
    fn is_fixed(&self) -> bool;
    #[doc(alias = "gst_value_is_subset")]
    fn is_subset(&self, superset: &Self) -> bool;
    #[doc(alias = "gst_value_serialize")]
    fn serialize(&self) -> Result<glib::GString, glib::BoolError>;
    #[doc(alias = "gst_value_deserialize")]
    fn deserialize(s: &str, type_: glib::Type) -> Result<glib::Value, glib::BoolError>;
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_value_deserialize_with_pspec")]
    fn deserialize_with_pspec(
        s: &str,
        pspec: &glib::ParamSpec,
    ) -> Result<glib::Value, glib::BoolError>;
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

    fn compare(&self, other: &Self) -> Option<cmp::Ordering> {
        unsafe {
            let val = ffi::gst_value_compare(self.to_glib_none().0, other.to_glib_none().0);

            match val {
                ffi::GST_VALUE_LESS_THAN => Some(cmp::Ordering::Less),
                ffi::GST_VALUE_EQUAL => Some(cmp::Ordering::Equal),
                ffi::GST_VALUE_GREATER_THAN => Some(cmp::Ordering::Greater),
                _ => None,
            }
        }
    }

    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == Some(cmp::Ordering::Equal)
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

    fn serialize(&self) -> Result<glib::GString, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_value_serialize(self.to_glib_none().0))
                .ok_or_else(|| glib::bool_error!("Failed to serialize value"))
        }
    }

    fn deserialize(s: &str, type_: glib::Type) -> Result<glib::Value, glib::BoolError> {
        assert_initialized_main_thread!();

        unsafe {
            let mut value = glib::Value::from_type(type_);
            let ret: bool = from_glib(ffi::gst_value_deserialize(
                value.to_glib_none_mut().0,
                s.to_glib_none().0,
            ));
            if ret {
                Ok(value)
            } else {
                Err(glib::bool_error!("Failed to deserialize value"))
            }
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn deserialize_with_pspec(
        s: &str,
        pspec: &glib::ParamSpec,
    ) -> Result<glib::Value, glib::BoolError> {
        assert_initialized_main_thread!();

        unsafe {
            let mut value = glib::Value::from_type(pspec.value_type());
            let ret: bool = from_glib(ffi::gst_value_deserialize_with_pspec(
                value.to_glib_none_mut().0,
                s.to_glib_none().0,
                pspec.to_glib_none().0,
            ));
            if ret {
                Ok(value)
            } else {
                Err(glib::bool_error!("Failed to deserialize value"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraction() {
        crate::init().unwrap();

        let f1 = crate::Fraction::new(1, 2);
        let f2 = crate::Fraction::new(2, 3);
        let mut f3 = f1 * f2;
        let f4 = f1 * f2;
        f3 *= f2;
        f3 *= f4;

        assert_eq!(f3, crate::Fraction::new(2, 27));
    }

    #[test]
    fn test_int_range_constructor() {
        crate::init().unwrap();

        // Type inference should figure out the type
        let _r1 = crate::IntRange::new(1i32, 2i32);
        let _r2 = crate::IntRange::with_step(2i64, 3i64, 4i64);
    }

    #[test]
    fn test_deserialize() {
        crate::init().unwrap();

        let v = glib::Value::deserialize("123", i32::static_type()).unwrap();
        assert_eq!(v.get::<i32>(), Ok(123));
    }
}
