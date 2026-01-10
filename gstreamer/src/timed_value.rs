// Take a look at the license at the top of the repository in the LICENSE file.

use std::marker::PhantomData;

use glib::translate::*;

use crate::{ClockTime, ffi};

#[derive(Debug, Clone, Copy)]
#[doc(alias = "GstTimedValue")]
#[repr(transparent)]
pub struct TimedValue(ffi::GstTimedValue);

unsafe impl Send for TimedValue {}
unsafe impl Sync for TimedValue {}

impl TimedValue {
    #[doc(alias = "get_timestamp")]
    pub fn timestamp(&self) -> ClockTime {
        unsafe { try_from_glib(self.0.timestamp).expect("undefined timestamp") }
    }

    #[doc(alias = "get_value")]
    #[inline]
    pub fn value(&self) -> f64 {
        self.0.value
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ffi::GstTimedValue {
        &self.0
    }
}

impl From<ffi::GstTimedValue> for TimedValue {
    #[inline]
    fn from(value: ffi::GstTimedValue) -> Self {
        skip_assert_initialized!();
        TimedValue(value)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GstTimedValue> for TimedValue {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GstTimedValue, Self> {
        Stash(&self.0, PhantomData)
    }
}

impl FromGlib<ffi::GstTimedValue> for TimedValue {
    #[inline]
    unsafe fn from_glib(value: ffi::GstTimedValue) -> Self {
        skip_assert_initialized!();
        Self::from(value)
    }
}

impl PartialEq for TimedValue {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp() == other.timestamp() && self.value() == other.value()
    }
}

impl Eq for TimedValue {}

unsafe impl glib::translate::TransparentType for TimedValue {
    type GlibType = ffi::GstTimedValue;
}
