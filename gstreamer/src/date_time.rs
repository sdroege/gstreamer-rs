// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cmp;
use std::convert;
use std::fmt;

use DateTime;

impl DateTime {
    pub fn to_utc(&self) -> Result<DateTime, glib::BoolError> {
        if !self.has_time() {
            // No time => no TZ offset
            return Ok(self.clone());
        }

        assert!(self.has_year() && self.has_month() && self.has_day() && self.has_time());

        // Can instantiate `gst::DateTime` without seconds using `gst::DateTime::new`
        // with `-1f64` for the `second` argument
        // however, the resulting instance can't be translated to `glib::DateTime`
        if self.has_second() {
            self.to_g_date_time()
                .and_then(|d| {
                    d.to_utc()
                        .ok_or_else(|| glib_bool_error!("Can't convert datetime to UTC"))
                })
                .and_then(|d| DateTime::from_g_date_time(&d))
        } else {
            // It would be cheaper to build a `glib::DateTime` direcly, unfortunetaly
            // this would require using `glib::TimeZone::new_offset` which is feature-gated
            // to `glib/v2_58`. So we need to build a new `gst::DateTime` with `0f64`
            // and then discard seconds again
            DateTime::new(
                self.get_time_zone_offset(),
                self.get_year(),
                self.get_month(),
                self.get_day(),
                self.get_hour(),
                self.get_minute(),
                0f64,
            )
            .and_then(|d| d.to_g_date_time())
            .and_then(|d| {
                d.to_utc()
                    .ok_or_else(|| glib_bool_error!("Can't convert datetime to UTC"))
            })
            .and_then(|d| {
                DateTime::new(
                    0f32, // UTC TZ offset
                    d.get_year(),
                    d.get_month(),
                    d.get_day_of_month(),
                    d.get_hour(),
                    d.get_minute(),
                    -1f64, // No second
                )
            })
        }
    }
}

impl cmp::PartialOrd for DateTime {
    // *NOTE 1:* When comparing a partially defined [`DateTime`](struct.DateTime.html) `d1`
    // such as *"2019/8/20"* with a [`DateTime`](struct.DateTime.html) with a time part `d2`
    // such as *"2019/8/20 21:10"*:
    //
    // - `d1` includes `d2`,
    // - neither `d1` < `d2` nor `d1` > `d2`,
    // - and `d1` != `d2`,
    //
    // so we can only return `None`.
    //
    // This is the reason why [`DateTime`](struct.DateTime.html) neither implements
    // [`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html)
    // nor [`Eq`](https://doc.rust-lang.org/nightly/std/cmp/trait.Eq.html).
    //
    // *NOTE 2:* When comparing a [`DateTime`](struct.DateTime.html) `d1` without a TZ offset
    // such as *"2019/8/20"* with a [`DateTime`](struct.DateTime.html) `d2` with a TZ offset
    // such as *"2019/8/20 21:10 +02:00"*, we can't tell in which TZ `d1` is expressed and which
    // time should be considered for an offset, therefore the two [`DateTime`s](struct.DateTime.html)
    // are compared in the same TZ.
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        #[inline]
        fn get_cmp(delta: i32) -> Option<cmp::Ordering> {
            skip_assert_initialized!();
            Some(delta.cmp(&0))
        }

        if !(self.has_year() && other.has_year()) {
            // Can't compare anything
            return None;
        }

        // Normalize to UTC only if both members have time (see note 2).
        let (self_norm, other_norm) = if self.has_time() && other.has_time() {
            (self.to_utc().ok()?, other.to_utc().ok()?)
        } else {
            (self.clone(), other.clone())
        };

        let year_delta = self_norm.get_year() - other_norm.get_year();
        if year_delta != 0 {
            return get_cmp(year_delta);
        }

        // Same year

        if !self.has_month() && !other.has_month() {
            // Nothing left to compare
            return get_cmp(year_delta);
        }

        if !(self.has_month() && other.has_month()) {
            // One has month, the other doesn't => can't compare (note 1)
            return None;
        }

        let month_delta = self_norm.get_month() - other_norm.get_month();
        if month_delta != 0 {
            return get_cmp(month_delta);
        }

        // Same year, same month

        if !self.has_day() && !other.has_day() {
            // Nothing left to compare
            return Some(cmp::Ordering::Equal);
        }

        if !(self.has_day() && other.has_day()) {
            // One has day, the other doesn't => can't compare (note 1)
            return None;
        }

        let day_delta = self_norm.get_day() - other_norm.get_day();
        if day_delta != 0 {
            return get_cmp(day_delta);
        }

        // Same year, same month, same day

        if !self.has_time() && !other.has_time() {
            // Nothing left to compare
            return Some(cmp::Ordering::Equal);
        }

        if !(self.has_time() && other.has_time()) {
            // One has time, the other doesn't => can't compare (note 1)
            return None;
        }

        let hour_delta = self_norm.get_hour() - other_norm.get_hour();
        if hour_delta != 0 {
            return get_cmp(hour_delta);
        }

        let minute_delta = self_norm.get_minute() - other_norm.get_minute();
        if minute_delta != 0 {
            return get_cmp(minute_delta);
        }

        // Same year, same month, same day, same time

        if !self.has_second() && !other.has_second() {
            // Nothing left to compare
            return Some(cmp::Ordering::Equal);
        }

        if !(self.has_second() && other.has_second()) {
            // One has second, the other doesn't => can't compare (note 1)
            return None;
        }
        let second_delta = self_norm.get_second() - other_norm.get_second();
        if second_delta != 0 {
            return get_cmp(second_delta);
        }

        get_cmp(self_norm.get_microsecond() - other_norm.get_microsecond())
    }
}

impl cmp::PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .map_or_else(|| false, |cmp| cmp == cmp::Ordering::Equal)
    }
}

impl fmt::Debug for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_struct = f.debug_struct("DateTime");
        if self.has_year() {
            debug_struct.field("year", &self.get_year());
        }
        if self.has_month() {
            debug_struct.field("month", &self.get_month());
        }
        if self.has_day() {
            debug_struct.field("day", &self.get_day());
        }
        if self.has_time() {
            debug_struct.field("hour", &self.get_hour());
            debug_struct.field("minute", &self.get_minute());

            if self.has_second() {
                debug_struct.field("second", &self.get_second());
                debug_struct.field("microsecond", &self.get_microsecond());
            }

            debug_struct.field("tz_offset", &self.get_time_zone_offset());
        }

        debug_struct.finish()
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            self.to_iso8601_string()
                .unwrap_or_else(|_| "None".into())
                .as_str(),
        )
    }
}

impl<'a> convert::TryFrom<&'a glib::DateTime> for DateTime {
    type Error = glib::BoolError;

    fn try_from(v: &'a glib::DateTime) -> Result<DateTime, glib::BoolError> {
        skip_assert_initialized!();
        DateTime::from_g_date_time(v)
    }
}

impl convert::TryFrom<glib::DateTime> for DateTime {
    type Error = glib::BoolError;

    fn try_from(v: glib::DateTime) -> Result<DateTime, glib::BoolError> {
        skip_assert_initialized!();
        DateTime::from_g_date_time(&v)
    }
}

impl<'a> convert::TryFrom<&'a DateTime> for glib::DateTime {
    type Error = glib::BoolError;

    fn try_from(v: &'a DateTime) -> Result<glib::DateTime, glib::BoolError> {
        skip_assert_initialized!();
        v.to_g_date_time()
    }
}

impl convert::TryFrom<DateTime> for glib::DateTime {
    type Error = glib::BoolError;

    fn try_from(v: DateTime) -> Result<glib::DateTime, glib::BoolError> {
        skip_assert_initialized!();
        v.to_g_date_time()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn test_to_utc() {
        ::init().unwrap();

        // Hour offset
        let utc_date_time = DateTime::new(2f32, 2019, 8, 20, 20, 9, 42.123_456f64)
            .unwrap()
            .to_utc()
            .unwrap();
        assert_eq!(utc_date_time.get_year(), 2019);
        assert_eq!(utc_date_time.get_month(), 8);
        assert_eq!(utc_date_time.get_day(), 20);
        assert_eq!(utc_date_time.get_hour(), 18);
        assert_eq!(utc_date_time.get_minute(), 9);
        assert_eq!(utc_date_time.get_second(), 42);
        assert_eq!(utc_date_time.get_microsecond(), 123_456);

        // Year, month, day and hour offset
        let utc_date_time = DateTime::new(2f32, 2019, 1, 1, 0, 0, 42.123_456f64)
            .unwrap()
            .to_utc()
            .unwrap();
        assert_eq!(utc_date_time.get_year(), 2018);
        assert_eq!(utc_date_time.get_month(), 12);
        assert_eq!(utc_date_time.get_day(), 31);
        assert_eq!(utc_date_time.get_hour(), 22);
        assert_eq!(utc_date_time.get_minute(), 0);
        assert_eq!(utc_date_time.get_second(), 42);
        assert_eq!(utc_date_time.get_microsecond(), 123_456);

        // Date without an hour (which implies no TZ)
        let utc_date_time = DateTime::new_ymd(2019, 1, 1).unwrap().to_utc().unwrap();
        assert_eq!(utc_date_time.get_year(), 2019);
        assert_eq!(utc_date_time.get_month(), 1);
        assert_eq!(utc_date_time.get_day(), 1);
        assert!(!utc_date_time.has_time());
        assert!(!utc_date_time.has_second());

        // Date without seconds
        let utc_date_time = DateTime::new(2f32, 2018, 5, 28, 16, 6, -1f64)
            .unwrap()
            .to_utc()
            .unwrap();
        assert_eq!(utc_date_time.get_year(), 2018);
        assert_eq!(utc_date_time.get_month(), 5);
        assert_eq!(utc_date_time.get_day(), 28);
        assert_eq!(utc_date_time.get_hour(), 14);
        assert_eq!(utc_date_time.get_minute(), 6);
        assert!(!utc_date_time.has_second());
    }

    #[test]
    fn test_partial_ord() {
        ::init().unwrap();

        // Different years
        assert!(
            DateTime::new(2f32, 2020, 8, 20, 19, 43, 42.123_456f64).unwrap()
                > DateTime::new(2f32, 2019, 8, 20, 19, 43, 42.123_456f64).unwrap()
        );

        // Different months (order intentionally reversed)
        assert!(
            DateTime::new(2f32, 2019, 8, 20, 19, 43, 42.123_456f64).unwrap()
                < DateTime::new(2f32, 2019, 9, 19, 19, 43, 42.123_456f64).unwrap()
        );

        // Different days
        assert!(
            DateTime::new(2f32, 2019, 8, 21, 19, 43, 42.123_456f64).unwrap()
                > DateTime::new(2f32, 2019, 8, 20, 19, 43, 42.123_456f64).unwrap()
        );

        // Different hours
        assert!(
            DateTime::new(2f32, 2019, 8, 20, 19, 44, 42.123_456f64).unwrap()
                > DateTime::new(2f32, 2019, 8, 20, 19, 43, 42.123_456f64).unwrap()
        );

        // Different minutes
        assert!(
            DateTime::new(2f32, 2019, 8, 20, 19, 43, 44.123_456f64).unwrap()
                > DateTime::new(2f32, 2019, 8, 20, 19, 43, 42.123_456f64).unwrap()
        );

        // Different seconds
        assert!(
            DateTime::new(2f32, 2019, 8, 20, 19, 43, 43.123_456f64).unwrap()
                > DateTime::new(2f32, 2019, 8, 20, 19, 43, 42.123_456f64).unwrap()
        );

        // Different micro-seconds
        assert!(
            DateTime::new(2f32, 2019, 8, 20, 19, 43, 42.123_457f64).unwrap()
                > DateTime::new(2f32, 2019, 8, 20, 19, 43, 42.123_456f64).unwrap()
        );

        // Different TZ offsets
        assert!(
            DateTime::new(1f32, 2019, 8, 20, 19, 43, 42.123_456f64).unwrap()
                > DateTime::new(2f32, 2019, 8, 20, 19, 43, 42.123_456f64).unwrap()
        );

        // TZ offset leading to year, month, day, hour offset
        assert!(
            DateTime::new(2f32, 2019, 1, 1, 0, 0, 0f64).unwrap()
                < DateTime::new(1f32, 2018, 12, 31, 23, 59, 0f64).unwrap()
        );

        // Partially defined `DateTime`
        assert!(DateTime::new_ymd(2020, 8, 20).unwrap() > DateTime::new_ymd(2019, 8, 20).unwrap());
        assert!(DateTime::new_ymd(2019, 9, 20).unwrap() > DateTime::new_ymd(2019, 8, 20).unwrap());
        assert!(DateTime::new_ymd(2019, 8, 21).unwrap() > DateTime::new_ymd(2019, 8, 20).unwrap());

        assert!(DateTime::new_ym(2020, 8).unwrap() > DateTime::new_ym(2019, 8).unwrap());
        assert!(DateTime::new_ym(2019, 9).unwrap() > DateTime::new_ym(2019, 8).unwrap());
        assert!(DateTime::new_ym(2019, 9).unwrap() > DateTime::new_ymd(2019, 8, 20).unwrap());

        assert!(DateTime::new_y(2020).unwrap() > DateTime::new_y(2019).unwrap());
        assert!(DateTime::new_ym(2020, 1).unwrap() > DateTime::new_y(2019).unwrap());

        assert!(
            DateTime::new(2f32, 2019, 8, 20, 19, 43, 44.123_456f64).unwrap()
                < DateTime::new_ymd(2020, 8, 20).unwrap()
        );

        assert!(
            DateTime::new_ymd(2020, 8, 20).unwrap()
                > DateTime::new(2f32, 2019, 8, 20, 19, 43, 44.123_456f64).unwrap()
        );

        // Comparison occurs on the same TZ when the `DateTime` doesn't have time (note 2)
        assert!(
            DateTime::new_ymd(2020, 1, 1).unwrap()
                > DateTime::new(-2f32, 2019, 12, 31, 23, 59, 0f64).unwrap()
        );

        // In the following cases, the partially defined `DateTime` is a range WRT
        // the fully defined `DateTime` and this range includes the fully defined `DateTime`,
        // but we can't tell if it's before or after and they are not equal (note 1)
        assert!(DateTime::new(2f32, 2019, 8, 20, 19, 43, 44.123_456f64)
            .unwrap()
            .partial_cmp(&DateTime::new_ymd(2019, 8, 20).unwrap())
            .is_none());

        assert!(DateTime::new_ymd(2019, 8, 20)
            .unwrap()
            .partial_cmp(&DateTime::new(2f32, 2019, 8, 20, 19, 43, 44.123_456f64).unwrap())
            .is_none());

        assert!(DateTime::new_ym(2019, 1)
            .unwrap()
            .partial_cmp(&DateTime::new_y(2019).unwrap())
            .is_none());
    }

    #[test]
    fn test_eq() {
        ::init().unwrap();

        assert_eq!(
            DateTime::new(2f32, 2018, 5, 28, 16, 6, 42.123_456f64).unwrap(),
            DateTime::new(2f32, 2018, 5, 28, 16, 6, 42.123_456f64).unwrap()
        );

        assert_eq!(
            DateTime::new(2f32, 2018, 5, 28, 16, 6, 0f64).unwrap(),
            DateTime::new(2f32, 2018, 5, 28, 16, 6, 0f64).unwrap()
        );

        assert_eq!(
            DateTime::new(2f32, 2018, 5, 28, 16, 6, -1f64).unwrap(),
            DateTime::new(2f32, 2018, 5, 28, 16, 6, -1f64).unwrap()
        );

        assert_eq!(
            DateTime::new_ymd(2018, 5, 28).unwrap(),
            DateTime::new_ymd(2018, 5, 28).unwrap()
        );

        // In the following cases, the partially defined `DateTime` is a range WRT
        // the fully defined `DateTime` and this range includes the fully defined `DateTime`,
        // but they are not equal (note 1)
        assert_ne!(
            DateTime::new_ymd(2018, 5, 28).unwrap(),
            DateTime::new(2f32, 2018, 5, 28, 16, 6, -1f64).unwrap()
        );

        assert_ne!(
            DateTime::new(2f32, 2018, 5, 28, 16, 6, -1f64).unwrap(),
            DateTime::new_ym(2018, 5).unwrap()
        );

        assert_ne!(
            DateTime::new(2f32, 2018, 5, 28, 16, 6, -1f64).unwrap(),
            DateTime::new_y(2018).unwrap()
        );
    }
}
