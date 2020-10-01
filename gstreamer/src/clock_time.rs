// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib;
use glib::translate::*;
use gst_sys;
use std::time::Duration;
use std::{cmp, convert, fmt};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct ClockTime(pub Option<u64>);

impl ClockTime {
    pub fn hours(&self) -> Option<u64> {
        (*self / ::SECOND / 60 / 60).0
    }

    pub fn minutes(&self) -> Option<u64> {
        (*self / ::SECOND / 60).0
    }

    pub fn seconds(&self) -> Option<u64> {
        (*self / ::SECOND).0
    }

    pub fn mseconds(&self) -> Option<u64> {
        (*self / ::MSECOND).0
    }

    pub fn useconds(&self) -> Option<u64> {
        (*self / ::USECOND).0
    }

    pub fn nseconds(&self) -> Option<u64> {
        (*self / ::NSECOND).0
    }

    pub fn nanoseconds(&self) -> Option<u64> {
        self.0
    }

    pub fn from_seconds(seconds: u64) -> ClockTime {
        skip_assert_initialized!();
        seconds * ::SECOND
    }

    pub fn from_mseconds(mseconds: u64) -> ClockTime {
        skip_assert_initialized!();
        mseconds * ::MSECOND
    }

    pub fn from_useconds(useconds: u64) -> ClockTime {
        skip_assert_initialized!();
        useconds * ::USECOND
    }

    pub fn from_nseconds(nseconds: u64) -> ClockTime {
        skip_assert_initialized!();
        nseconds * ::NSECOND
    }

    pub fn none() -> ClockTime {
        ClockTime(None)
    }
}

impl fmt::Display for ClockTime {
    #[allow(clippy::many_single_char_names)]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let precision = f.precision().unwrap_or(9);
        // TODO: Could also check width and pad the hours as needed

        let (h, m, s, ns) = match self.0 {
            Some(v) => {
                let mut s = v / 1_000_000_000;
                let mut m = s / 60;
                let h = m / 60;
                s %= 60;
                m %= 60;
                let ns = v % 1_000_000_000;

                (h, m, s, ns)
            }
            None => (99, 99, 99, 999_999_999),
        };

        if precision == 0 {
            f.write_fmt(format_args!("{:02}:{:02}:{:02}", h, m, s))
        } else {
            let mut divisor = 1;
            let precision = cmp::min(precision, 9);
            for _ in 0..(9 - precision) {
                divisor *= 10;
            }

            f.write_fmt(format_args!(
                "{:02}:{:02}:{:02}.{:0width$}",
                h,
                m,
                s,
                ns / divisor,
                width = precision
            ))
        }
    }
}

#[doc(hidden)]
impl ToGlib for ClockTime {
    type GlibType = gst_sys::GstClockTime;

    fn to_glib(&self) -> gst_sys::GstClockTime {
        match self.0 {
            None => gst_sys::GST_CLOCK_TIME_NONE,
            Some(v) => v,
        }
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstClockTime> for ClockTime {
    fn from_glib(value: gst_sys::GstClockTime) -> Self {
        skip_assert_initialized!();
        match value {
            gst_sys::GST_CLOCK_TIME_NONE => ClockTime(None),
            value => ClockTime(Some(value)),
        }
    }
}

#[doc(hidden)]
impl<'a> glib::value::FromValueOptional<'a> for ClockTime {
    unsafe fn from_value_optional(value: &'a glib::Value) -> Option<Self> {
        <u64 as glib::value::FromValueOptional>::from_value_optional(value)
            .map(ClockTime::from_glib)
    }
}

#[doc(hidden)]
impl<'a> glib::value::FromValue<'a> for ClockTime {
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        ClockTime::from_glib(<u64 as glib::value::FromValue>::from_value(value))
    }
}

#[doc(hidden)]
impl glib::value::SetValue for ClockTime {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        <u64 as glib::value::SetValue>::set_value(value, &this.to_glib());
    }
}

#[doc(hidden)]
impl glib::StaticType for ClockTime {
    fn static_type() -> glib::Type {
        <u64 as glib::StaticType>::static_type()
    }
}

impl From<Duration> for ClockTime {
    fn from(d: Duration) -> Self {
        skip_assert_initialized!();

        let nanos = d.as_nanos();

        if nanos > std::u64::MAX as u128 {
            ::CLOCK_TIME_NONE
        } else {
            ClockTime::from_nseconds(nanos as u64)
        }
    }
}

impl convert::TryFrom<ClockTime> for Duration {
    type Error = glib::BoolError;

    fn try_from(t: ClockTime) -> Result<Self, Self::Error> {
        skip_assert_initialized!();

        t.nanoseconds()
            .map(Duration::from_nanos)
            .ok_or_else(|| glib_bool_error!("Can't convert ClockTime::NONE to Duration"))
    }
}
