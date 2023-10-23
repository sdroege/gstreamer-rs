// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};
use std::fmt;
use std::ops;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[doc(alias = "GstRank")]
pub struct Rank(i32);

impl Rank {
    #[doc(alias = "GST_RANK_NONE")]
    pub const NONE: Rank = Self(ffi::GST_RANK_NONE);
    #[doc(alias = "GST_RANK_MARGINAL")]
    pub const MARGINAL: Rank = Self(ffi::GST_RANK_MARGINAL);
    #[doc(alias = "GST_RANK_SECONDARY")]
    pub const SECONDARY: Rank = Self(ffi::GST_RANK_SECONDARY);
    #[doc(alias = "GST_RANK_PRIMARY")]
    pub const PRIMARY: Rank = Self(ffi::GST_RANK_PRIMARY);
}

impl IntoGlib for Rank {
    type GlibType = i32;

    #[inline]
    fn into_glib(self) -> i32 {
        self.0
    }
}

#[doc(hidden)]
impl FromGlib<i32> for Rank {
    #[inline]
    unsafe fn from_glib(value: i32) -> Self {
        Rank(value)
    }
}

impl StaticType for Rank {
    #[inline]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_rank_get_type()) }
    }
}

impl glib::HasParamSpec for Rank {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder_with_default
    }
}

impl glib::value::ValueType for Rank {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for Rank {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for Rank {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<Rank> for glib::Value {
    #[inline]
    fn from(v: Rank) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

impl From<i32> for Rank {
    #[inline]
    fn from(v: i32) -> Self {
        skip_assert_initialized!();
        Rank(v)
    }
}

impl From<Rank> for i32 {
    #[inline]
    fn from(v: Rank) -> Self {
        skip_assert_initialized!();
        v.0
    }
}

impl ops::Add<i32> for Rank {
    type Output = Rank;

    #[inline]
    fn add(self, rhs: i32) -> Rank {
        Rank(self.0 + rhs)
    }
}

impl ops::Add<Rank> for i32 {
    type Output = Rank;

    #[inline]
    fn add(self, rhs: Rank) -> Rank {
        Rank(self + rhs.0)
    }
}

impl ops::AddAssign<i32> for Rank {
    #[inline]
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs;
    }
}

impl ops::Sub<i32> for Rank {
    type Output = Rank;

    #[inline]
    fn sub(self, rhs: i32) -> Rank {
        Rank(self.0 - rhs)
    }
}

impl ops::Sub<Rank> for i32 {
    type Output = Rank;

    #[inline]
    fn sub(self, rhs: Rank) -> Rank {
        Rank(self - rhs.0)
    }
}

impl ops::SubAssign<i32> for Rank {
    #[inline]
    fn sub_assign(&mut self, rhs: i32) {
        self.0 -= rhs
    }
}

impl std::cmp::PartialEq<i32> for Rank {
    #[inline]
    fn eq(&self, rhs: &i32) -> bool {
        self.0 == *rhs
    }
}

impl std::cmp::PartialEq<Rank> for i32 {
    #[inline]
    fn eq(&self, rhs: &Rank) -> bool {
        *self == rhs.0
    }
}

impl fmt::Display for Rank {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank = self.into_glib();
        let names: [&str; 4] = ["none", "marginal", "secondary", "primary"];
        let ranks: [Rank; 4] = [Rank::NONE, Rank::MARGINAL, Rank::SECONDARY, Rank::PRIMARY];

        let mut best_i = 0;
        for i in 0..4 {
            if rank == ranks[i].into_glib() {
                return f.write_str(names[i]);
            }
            if (rank - ranks[i]).into_glib().abs() < (rank - ranks[best_i]).into_glib().abs() {
                best_i = i;
            }
        }

        let diff = (rank - ranks[best_i]).into_glib();
        let op_str = if diff > 0 { '+' } else { '-' };

        write!(f, "{} {} {}", names[best_i], op_str, diff.abs())
    }
}
