// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::cmp;
use std::convert;
use std::fmt;

use crate::DateTime;

// Validate that the given values result in a valid DateTime
fn validate(
    tzoffset: Option<f32>,
    year: i32,
    month: Option<i32>,
    day: Option<i32>,
    hour: Option<i32>,
    minute: Option<i32>,
    seconds: Option<f64>,
) -> Result<(), glib::BoolError> {
    skip_assert_initialized!();

    // Check for valid ranges
    if year <= 0 || year > 9999 {
        return Err(glib::bool_error!(
            "Can't create DateTime: Year out of range"
        ));
    }

    if let Some(month) = month {
        if month <= 0 || month > 12 {
            return Err(glib::bool_error!(
                "Can't create DateTime: Month out of range"
            ));
        }
    }

    if let Some(day) = day {
        if day <= 0 || day > 31 {
            return Err(glib::bool_error!("Can't create DateTime: Day out of range"));
        }
    }

    if let Some(hour) = hour {
        if hour < 0 || hour >= 24 {
            return Err(glib::bool_error!(
                "Can't create DateTime: Hour out of range"
            ));
        }
    }

    if let Some(minute) = minute {
        if minute < 0 || minute >= 60 {
            return Err(glib::bool_error!(
                "Can't create DateTime: Minute out of range"
            ));
        }
    }

    if let Some(seconds) = seconds {
        if seconds < 0.0 || seconds >= 60.0 {
            return Err(glib::bool_error!(
                "Can't create DateTime: Seconds out of range"
            ));
        }
    }

    if let Some(tzoffset) = tzoffset {
        if tzoffset < -12.0 || tzoffset > 12.0 {
            return Err(glib::bool_error!(
                "Can't create DateTime: Timezone offset out of range"
            ));
        }
    }

    // If day is provided, month also has to be provided
    if day.is_some() && month.is_none() {
        return Err(glib::bool_error!(
            "Can't create DateTime: Need to provide month if providing day"
        ));
    }

    // If hour is provided, day also has to be provided
    if hour.is_some() && day.is_none() {
        return Err(glib::bool_error!(
            "Can't create DateTime: Need to provide day if providing hour"
        ));
    }

    // If minutes are provided, hours also need to be provided and the other way around
    if hour.is_none() && minute.is_some() {
        return Err(glib::bool_error!(
            "Can't create DateTime: Need to provide both hour and minute or neither"
        ));
    }

    if minute.is_some() && hour.is_none() {
        return Err(glib::bool_error!(
            "Can't create DateTime: Need to provide both hour and minute or neither"
        ));
    }

    // If seconds or tzoffset are provided then also hours and minutes must be provided
    if (seconds.is_some() || tzoffset.is_some()) && (hour.is_none() || minute.is_none()) {
        return Err(glib::bool_error!("Can't create DateTime: Need to provide hour and minute if providing seconds or timezone offset"));
    }

    Ok(())
}

impl DateTime {
    #[doc(alias = "gst_date_time_new")]
    pub fn new(
        tzoffset: impl Into<Option<f32>>,
        year: impl Into<i32>,
        month: impl Into<Option<i32>>,
        day: impl Into<Option<i32>>,
        hour: impl Into<Option<i32>>,
        minute: impl Into<Option<i32>>,
        seconds: impl Into<Option<f64>>,
    ) -> Result<DateTime, glib::BoolError> {
        assert_initialized_main_thread!();

        let tzoffset = tzoffset.into();
        let year = year.into();
        let month = month.into();
        let day = day.into();
        let hour = hour.into();
        let minute = minute.into();
        let seconds = seconds.into();

        validate(tzoffset, year, month, day, hour, minute, seconds)?;

        unsafe {
            Option::<_>::from_glib_full(ffi::gst_date_time_new(
                tzoffset.unwrap_or(0.0),
                year,
                month.unwrap_or(-1),
                day.unwrap_or(-1),
                hour.unwrap_or(-1),
                minute.unwrap_or(-1),
                seconds.unwrap_or(-1.0),
            ))
            .ok_or_else(|| glib::bool_error!("Can't create DateTime"))
        }
    }

    #[doc(alias = "gst_date_time_new_local_time")]
    pub fn new_local_time(
        year: impl Into<i32>,
        month: impl Into<Option<i32>>,
        day: impl Into<Option<i32>>,
        hour: impl Into<Option<i32>>,
        minute: impl Into<Option<i32>>,
        seconds: impl Into<Option<f64>>,
    ) -> Result<DateTime, glib::BoolError> {
        assert_initialized_main_thread!();

        let year = year.into();
        let month = month.into();
        let day = day.into();
        let hour = hour.into();
        let minute = minute.into();
        let seconds = seconds.into();

        validate(None, year, month, day, hour, minute, seconds)?;

        unsafe {
            Option::<_>::from_glib_full(ffi::gst_date_time_new_local_time(
                year,
                month.unwrap_or(-1),
                day.unwrap_or(-1),
                hour.unwrap_or(-1),
                minute.unwrap_or(-1),
                seconds.unwrap_or(-1.0),
            ))
            .ok_or_else(|| glib::bool_error!("Can't create DateTime"))
        }
    }

    #[doc(alias = "gst_date_time_new_y")]
    pub fn new_y(year: i32) -> Result<DateTime, glib::BoolError> {
        assert_initialized_main_thread!();

        validate(None, year, None, None, None, None, None)?;

        unsafe {
            Option::<_>::from_glib_full(ffi::gst_date_time_new_y(year))
                .ok_or_else(|| glib::bool_error!("Can't create DateTime"))
        }
    }

    #[doc(alias = "gst_date_time_new_ym")]
    pub fn new_ym(year: i32, month: i32) -> Result<DateTime, glib::BoolError> {
        assert_initialized_main_thread!();

        validate(None, year, Some(month), None, None, None, None)?;

        unsafe {
            Option::<_>::from_glib_full(ffi::gst_date_time_new_ym(year, month))
                .ok_or_else(|| glib::bool_error!("Can't create DateTime"))
        }
    }

    #[doc(alias = "gst_date_time_new_ymd")]
    pub fn new_ymd(year: i32, month: i32, day: i32) -> Result<DateTime, glib::BoolError> {
        assert_initialized_main_thread!();

        validate(None, year, Some(month), Some(day), None, None, None)?;

        unsafe {
            Option::<_>::from_glib_full(ffi::gst_date_time_new_ymd(year, month, day))
                .ok_or_else(|| glib::bool_error!("Can't create DateTime"))
        }
    }

    #[doc(alias = "get_day")]
    #[doc(alias = "gst_date_time_get_day")]
    pub fn day(&self) -> Option<i32> {
        if !self.has_day() {
            return None;
        }

        unsafe { Some(ffi::gst_date_time_get_day(self.to_glib_none().0)) }
    }

    #[doc(alias = "get_hour")]
    #[doc(alias = "gst_date_time_get_hour")]
    pub fn hour(&self) -> Option<i32> {
        if !self.has_time() {
            return None;
        }

        unsafe { Some(ffi::gst_date_time_get_hour(self.to_glib_none().0)) }
    }

    #[doc(alias = "get_microsecond")]
    #[doc(alias = "gst_date_time_get_microsecond")]
    pub fn microsecond(&self) -> Option<i32> {
        if !self.has_second() {
            return None;
        }

        unsafe { Some(ffi::gst_date_time_get_microsecond(self.to_glib_none().0)) }
    }

    #[doc(alias = "get_minute")]
    #[doc(alias = "gst_date_time_get_minute")]
    pub fn minute(&self) -> Option<i32> {
        if !self.has_time() {
            return None;
        }

        unsafe { Some(ffi::gst_date_time_get_minute(self.to_glib_none().0)) }
    }

    #[doc(alias = "get_month")]
    #[doc(alias = "gst_date_time_get_month")]
    pub fn month(&self) -> Option<i32> {
        if !self.has_month() {
            return None;
        }

        unsafe { Some(ffi::gst_date_time_get_month(self.to_glib_none().0)) }
    }

    #[doc(alias = "get_second")]
    #[doc(alias = "gst_date_time_get_second")]
    pub fn second(&self) -> Option<i32> {
        if !self.has_second() {
            return None;
        }

        unsafe { Some(ffi::gst_date_time_get_second(self.to_glib_none().0)) }
    }

    #[doc(alias = "get_time_zone_offset")]
    #[doc(alias = "gst_date_time_get_time_zone_offset")]
    pub fn time_zone_offset(&self) -> Option<f32> {
        if !self.has_time() {
            return None;
        }

        unsafe {
            Some(ffi::gst_date_time_get_time_zone_offset(
                self.to_glib_none().0,
            ))
        }
    }

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
                .and_then(|d| d.to_utc())
                .map(|d| d.into())
        } else {
            // It would be cheaper to build a `glib::DateTime` direcly, unfortunetaly
            // this would require using `glib::TimeZone::new_offset` which is feature-gated
            // to `glib/v2_58`. So we need to build a new `gst::DateTime` with `0f64`
            // and then discard seconds again
            DateTime::new(
                self.time_zone_offset(),
                self.year(),
                self.month(),
                self.day(),
                self.hour(),
                self.minute(),
                Some(0.0),
            )
            .and_then(|d| d.to_g_date_time())
            .and_then(|d| d.to_utc())
            .and_then(|d| {
                DateTime::new(
                    None, // UTC TZ offset
                    d.year(),
                    Some(d.month()),
                    Some(d.day_of_month()),
                    Some(d.hour()),
                    Some(d.minute()),
                    None, // No second
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
        #[allow(clippy::unnecessary_wraps)]
        #[doc(alias = "get_cmp")]
        fn cmp(delta: i32) -> Option<cmp::Ordering> {
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

        let year_delta = self_norm.year() - other_norm.year();
        if year_delta != 0 {
            return cmp(year_delta);
        }

        // Same year

        if !self.has_month() && !other.has_month() {
            // Nothing left to compare
            return cmp(year_delta);
        }

        if !(self.has_month() && other.has_month()) {
            // One has month, the other doesn't => can't compare (note 1)
            return None;
        }

        let month_delta = self_norm.month().unwrap() - other_norm.month().unwrap();
        if month_delta != 0 {
            return cmp(month_delta);
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

        let day_delta = self_norm.day().unwrap() - other_norm.day().unwrap();
        if day_delta != 0 {
            return cmp(day_delta);
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

        let hour_delta = self_norm.hour().unwrap() - other_norm.hour().unwrap();
        if hour_delta != 0 {
            return cmp(hour_delta);
        }

        let minute_delta = self_norm.minute().unwrap() - other_norm.minute().unwrap();
        if minute_delta != 0 {
            return cmp(minute_delta);
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
        let second_delta = self_norm.second().unwrap() - other_norm.second().unwrap();
        if second_delta != 0 {
            return cmp(second_delta);
        }

        cmp(self_norm.microsecond().unwrap() - other_norm.microsecond().unwrap())
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
            debug_struct.field("year", &self.year());
        }
        if self.has_month() {
            debug_struct.field("month", &self.month());
        }
        if self.has_day() {
            debug_struct.field("day", &self.day());
        }
        if self.has_time() {
            debug_struct.field("hour", &self.hour());
            debug_struct.field("minute", &self.minute());

            if self.has_second() {
                debug_struct.field("second", &self.second());
                debug_struct.field("microsecond", &self.microsecond());
            }

            debug_struct.field("tz_offset", &self.time_zone_offset());
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

impl<'a> From<&'a glib::DateTime> for DateTime {
    fn from(v: &'a glib::DateTime) -> DateTime {
        skip_assert_initialized!();
        DateTime::from_g_date_time(v)
    }
}

impl From<glib::DateTime> for DateTime {
    fn from(v: glib::DateTime) -> DateTime {
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
        crate::init().unwrap();

        // Hour offset
        let utc_date_time = DateTime::new(2f32, 2019, 8, 20, 20, 9, 42.123_456f64)
            .unwrap()
            .to_utc()
            .unwrap();
        assert_eq!(utc_date_time.year(), 2019);
        assert_eq!(utc_date_time.month().unwrap(), 8);
        assert_eq!(utc_date_time.day().unwrap(), 20);
        assert_eq!(utc_date_time.hour().unwrap(), 18);
        assert_eq!(utc_date_time.minute().unwrap(), 9);
        assert_eq!(utc_date_time.second().unwrap(), 42);
        assert_eq!(utc_date_time.microsecond().unwrap(), 123_456);

        // Year, month, day and hour offset
        let utc_date_time = DateTime::new(2f32, 2019, 1, 1, 0, 0, 42.123_456f64)
            .unwrap()
            .to_utc()
            .unwrap();
        assert_eq!(utc_date_time.year(), 2018);
        assert_eq!(utc_date_time.month().unwrap(), 12);
        assert_eq!(utc_date_time.day().unwrap(), 31);
        assert_eq!(utc_date_time.hour().unwrap(), 22);
        assert_eq!(utc_date_time.minute().unwrap(), 0);
        assert_eq!(utc_date_time.second().unwrap(), 42);
        assert_eq!(utc_date_time.microsecond().unwrap(), 123_456);

        // Date without an hour (which implies no TZ)
        let utc_date_time = DateTime::new_ymd(2019, 1, 1).unwrap().to_utc().unwrap();
        assert_eq!(utc_date_time.year(), 2019);
        assert_eq!(utc_date_time.month().unwrap(), 1);
        assert_eq!(utc_date_time.day().unwrap(), 1);
        assert!(!utc_date_time.has_time());
        assert!(!utc_date_time.has_second());

        // Date without seconds
        let utc_date_time = DateTime::new(2f32, 2018, 5, 28, 16, 6, None)
            .unwrap()
            .to_utc()
            .unwrap();
        assert_eq!(utc_date_time.year(), 2018);
        assert_eq!(utc_date_time.month().unwrap(), 5);
        assert_eq!(utc_date_time.day().unwrap(), 28);
        assert_eq!(utc_date_time.hour().unwrap(), 14);
        assert_eq!(utc_date_time.minute().unwrap(), 6);
        assert!(!utc_date_time.has_second());
    }

    #[test]
    fn test_partial_ord() {
        crate::init().unwrap();

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
        crate::init().unwrap();

        assert_eq!(
            DateTime::new(2f32, 2018, 5, 28, 16, 6, 42.123_456f64).unwrap(),
            DateTime::new(2f32, 2018, 5, 28, 16, 6, 42.123_456f64).unwrap()
        );

        assert_eq!(
            DateTime::new(2f32, 2018, 5, 28, 16, 6, 0f64).unwrap(),
            DateTime::new(2f32, 2018, 5, 28, 16, 6, 0f64).unwrap()
        );

        assert_eq!(
            DateTime::new(2f32, 2018, 5, 28, 16, 6, None).unwrap(),
            DateTime::new(2f32, 2018, 5, 28, 16, 6, None).unwrap()
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
            DateTime::new(2f32, 2018, 5, 28, 16, 6, None).unwrap()
        );

        assert_ne!(
            DateTime::new(2f32, 2018, 5, 28, 16, 6, None).unwrap(),
            DateTime::new_ym(2018, 5).unwrap()
        );

        assert_ne!(
            DateTime::new(2f32, 2018, 5, 28, 16, 6, None).unwrap(),
            DateTime::new_y(2018).unwrap()
        );
    }
}
