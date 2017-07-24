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

use glib;
use glib::value::{Value, FromValue, FromValueOptional, SetValue};
use glib::translate::{from_glib, ToGlibPtr, ToGlibPtrMut};

use ffi;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let f: Fraction = (1, 2).into();
        println!("{}", f * 2);
    }
}
