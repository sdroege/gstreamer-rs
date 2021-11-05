// Take a look at the license at the top of the repository in the LICENSE file.

use num_rational::Rational32;
use std::borrow::{Borrow, Cow};
use std::cmp;
use std::fmt;
use std::ops;
use std::slice;

use glib::translate::{from_glib, FromGlibPtrFull, ToGlibPtr, ToGlibPtrMut, Uninitialized};
use glib::value::ToSendValue;
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
            ffi::gst_value_set_fraction(value.to_glib_none_mut().0, *self.numer(), *self.denom());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
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
                *self.min().numer(),
                *self.min().denom(),
                *self.max().numer(),
                *self.max().denom(),
            );
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
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

#[derive(Clone, Debug)]
pub struct Array<'a>(Cow<'a, [glib::SendValue]>);

unsafe impl<'a> Send for Array<'a> {}
unsafe impl<'a> Sync for Array<'a> {}

impl<'a> Array<'a> {
    pub fn new(values: &[&(dyn ToSendValue + Sync)]) -> Self {
        assert_initialized_main_thread!();

        Array(values.iter().map(|v| v.to_send_value()).collect())
    }

    pub fn from_borrowed(values: &'a impl AsRef<[glib::SendValue]>) -> Self {
        assert_initialized_main_thread!();

        Array(Cow::Borrowed(values.as_ref()))
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

impl<'a> ops::Deref for Array<'a> {
    type Target = [glib::SendValue];

    fn deref(&self) -> &[glib::SendValue] {
        self.as_slice()
    }
}

impl<'a> From<&'a [&'a (dyn ToSendValue + Sync)]> for Array<'a> {
    fn from(values: &'a [&'a (dyn ToSendValue + Sync)]) -> Self {
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

impl<'a> glib::value::ValueType for Array<'static> {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for Array<'a> {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        let arr = (*value.to_glib_none().0).data[0].v_pointer as *const glib::ffi::GArray;
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

impl<'a> glib::value::ToValue for Array<'a> {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Array<'static>>();
        unsafe {
            for v in self.as_slice() {
                ffi::gst_value_array_append_value(value.to_glib_none_mut().0, v.to_glib_none().0);
            }
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
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
unsafe impl<'a> Sync for List<'a> {}

impl<'a> List<'a> {
    pub fn new(values: &[&(dyn ToSendValue + Sync)]) -> Self {
        assert_initialized_main_thread!();

        List(values.iter().map(|v| v.to_send_value()).collect())
    }

    pub fn from_borrowed(values: &'a impl AsRef<[glib::SendValue]>) -> Self {
        assert_initialized_main_thread!();

        List(Cow::Borrowed(values.as_ref()))
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

impl<'a> ops::Deref for List<'a> {
    type Target = [glib::SendValue];

    fn deref(&self) -> &[glib::SendValue] {
        self.as_slice()
    }
}

impl<'a> From<&'a [&'a (dyn ToSendValue + Sync)]> for List<'a> {
    fn from(values: &'a [&'a (dyn ToSendValue + Sync)]) -> Self {
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

impl glib::value::ValueType for List<'static> {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for List<'a> {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        let arr = (*value.to_glib_none().0).data[0].v_pointer as *const glib::ffi::GArray;
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

impl<'a> glib::value::ToValue for List<'a> {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<List<'static>>();
        unsafe {
            for v in self.as_slice() {
                ffi::gst_value_list_append_value(value.to_glib_none_mut().0, v.to_glib_none().0);
            }
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl<'a> glib::types::StaticType for List<'a> {
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
    fn test_deserialize() {
        crate::init().unwrap();

        let v = glib::Value::deserialize("123", i32::static_type()).unwrap();
        assert_eq!(v.get::<i32>(), Ok(123));
    }
}
