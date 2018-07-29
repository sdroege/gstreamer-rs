// Copyright (C) 2018 François Laignel <fengalin@free.fr>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::de::{Deserialize, Deserializer};
use serde::ser;
use serde::ser::{Serialize, Serializer};
use DateTime;

#[derive(Serialize, Deserialize)]
enum DateTimeVariants {
    Y(i32),
    YM(i32, i32),
    YMD(i32, i32, i32),
    YMDhmTz(i32, i32, i32, i32, i32, f32),
    YMDhmsTz(i32, i32, i32, i32, i32, f64, f32),
}

impl<'a> Serialize for DateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let variant = if self.has_second() {
            DateTimeVariants::YMDhmsTz(
                self.get_year(),
                self.get_month(),
                self.get_day(),
                self.get_hour(),
                self.get_minute(),
                (self.get_second() as f64) + (self.get_microsecond() as f64) / 1_000_000f64,
                self.get_time_zone_offset(),
            )
        } else if self.has_time() {
            DateTimeVariants::YMDhmTz(
                self.get_year(),
                self.get_month(),
                self.get_day(),
                self.get_hour(),
                self.get_minute(),
                self.get_time_zone_offset(),
            )
        } else if self.has_day() {
            DateTimeVariants::YMD(self.get_year(), self.get_month(), self.get_day())
        } else if self.has_month() {
            DateTimeVariants::YM(self.get_year(), self.get_month())
        } else if self.has_year() {
            DateTimeVariants::Y(self.get_year())
        } else {
            return Err(ser::Error::custom(format!(
                "no parts could be found in `DateTime` {}",
                self,
            )));
        };

        variant.serialize(serializer)
    }
}

impl From<DateTimeVariants> for DateTime {
    fn from(dt_variant: DateTimeVariants) -> Self {
        match dt_variant {
            DateTimeVariants::Y(y) => DateTime::new_y(y),
            DateTimeVariants::YM(y, m) => DateTime::new_ym(y, m),
            DateTimeVariants::YMD(y, m, d) => DateTime::new_ymd(y, m, d),
            DateTimeVariants::YMDhmTz(y, m, d, h, mn, tz) => {
                DateTime::new(tz, y, m, d, h, mn, -1f64)
            }
            DateTimeVariants::YMDhmsTz(y, m, d, h, mn, s, tz) => {
                DateTime::new(tz, y, m, d, h, mn, s)
            }
        }
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        DateTimeVariants::deserialize(deserializer).map(|dt_variant| dt_variant.into())
    }
}

#[cfg(test)]
mod tests {
    extern crate ron;
    extern crate serde_json;

    use DateTime;

    #[test]
    fn test_serialize() {
        ::init().unwrap();

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        let datetime = DateTime::new(2f32, 2018, 5, 28, 16, 6, 42.123_456f64);
        let res = ron::ser::to_string_pretty(&datetime, pretty_config.clone());
        assert_eq!(
            Ok("YMDhmsTz(2018, 5, 28, 16, 6, 42.123456, 2)".to_owned()),
            res,
        );

        let res = serde_json::to_string(&datetime).unwrap();
        assert_eq!(
            "{\"YMDhmsTz\":[2018,5,28,16,6,42.123456,2.0]}".to_owned(),
            res
        );

        let datetime = DateTime::new(2f32, 2018, 5, 28, 16, 6, -1f64);
        let res = ron::ser::to_string_pretty(&datetime, pretty_config.clone());
        assert_eq!(Ok("YMDhmTz(2018, 5, 28, 16, 6, 2)".to_owned()), res,);

        let datetime = DateTime::new_ymd(2018, 5, 28);
        let res = ron::ser::to_string_pretty(&datetime, pretty_config.clone());
        assert_eq!(Ok("YMD(2018, 5, 28)".to_owned()), res);

        let datetime = DateTime::new_ym(2018, 5);
        let res = ron::ser::to_string_pretty(&datetime, pretty_config.clone());
        assert_eq!(Ok("YM(2018, 5)".to_owned()), res);

        let datetime = DateTime::new_y(2018);
        let res = ron::ser::to_string_pretty(&datetime, pretty_config.clone());
        assert_eq!(Ok("Y(2018)".to_owned()), res);
    }

    #[test]
    fn test_deserialize() {
        ::init().unwrap();

        let datetime_ron = "YMDhmsTz(2018, 5, 28, 16, 6, 42.123456, 2)";
        let datetime_de: DateTime = ron::de::from_str(datetime_ron).unwrap();
        assert_eq!(datetime_de.get_time_zone_offset(), 2f32);
        assert_eq!(datetime_de.get_year(), 2018);
        assert_eq!(datetime_de.get_month(), 5);
        assert_eq!(datetime_de.get_day(), 28);
        assert_eq!(datetime_de.get_hour(), 16);
        assert_eq!(datetime_de.get_minute(), 6);
        assert_eq!(datetime_de.get_second(), 42);
        assert_eq!(datetime_de.get_microsecond(), 123_456);

        let datetime_json = "{\"YMDhmsTz\":[2018,5,28,16,6,42.123456,2.0]}";
        let datetime_de: DateTime = serde_json::from_str(datetime_json).unwrap();
        assert_eq!(datetime_de.get_time_zone_offset(), 2f32);
        assert_eq!(datetime_de.get_year(), 2018);
        assert_eq!(datetime_de.get_month(), 5);
        assert_eq!(datetime_de.get_day(), 28);
        assert_eq!(datetime_de.get_hour(), 16);
        assert_eq!(datetime_de.get_minute(), 6);
        assert_eq!(datetime_de.get_second(), 42);
        assert_eq!(datetime_de.get_microsecond(), 123_456);

        let datetime_ron = "YMDhmTz(2018, 5, 28, 16, 6, 2)";
        let datetime_de: DateTime = ron::de::from_str(datetime_ron).unwrap();
        assert!(datetime_de.has_time());
        assert!(!datetime_de.has_second());
        assert_eq!(datetime_de.get_time_zone_offset(), 2f32);
        assert_eq!(datetime_de.get_year(), 2018);
        assert_eq!(datetime_de.get_month(), 5);
        assert_eq!(datetime_de.get_day(), 28);
        assert_eq!(datetime_de.get_hour(), 16);
        assert_eq!(datetime_de.get_minute(), 6);

        let datetime_ron = "YMD(2018, 5, 28)";
        let datetime_de: DateTime = ron::de::from_str(datetime_ron).unwrap();
        assert!(datetime_de.has_day());
        assert!(!datetime_de.has_time());
        assert_eq!(datetime_de.get_year(), 2018);
        assert_eq!(datetime_de.get_month(), 5);
        assert_eq!(datetime_de.get_day(), 28);

        let datetime_ron = "YM(2018, 5)";
        let datetime_de: DateTime = ron::de::from_str(datetime_ron).unwrap();
        assert!(datetime_de.has_month());
        assert!(!datetime_de.has_day());
        assert_eq!(datetime_de.get_year(), 2018);
        assert_eq!(datetime_de.get_month(), 5);

        let datetime_ron = "Y(2018)";
        let datetime_de: DateTime = ron::de::from_str(datetime_ron).unwrap();
        assert!(datetime_de.has_year());
        assert!(!datetime_de.has_month());
        assert_eq!(datetime_de.get_year(), 2018);
    }

    #[test]
    fn test_serde_roundtrip() {
        ::init().unwrap();

        let datetime = DateTime::new(2f32, 2018, 5, 28, 16, 6, 42.123_456f64);
        let datetime_ser = ron::ser::to_string(&datetime).unwrap();
        let datetime_de: DateTime = ron::de::from_str(datetime_ser.as_str()).unwrap();
        assert_eq!(
            datetime_de.get_time_zone_offset(),
            datetime.get_time_zone_offset()
        );
        assert_eq!(datetime_de.get_year(), datetime.get_year());
        assert_eq!(datetime_de.get_month(), datetime.get_month());
        assert_eq!(datetime_de.get_day(), datetime.get_day());
        assert_eq!(datetime_de.get_hour(), datetime.get_hour());
        assert_eq!(datetime_de.get_minute(), datetime.get_minute());
        assert_eq!(datetime_de.get_second(), datetime.get_second());
        assert_eq!(datetime_de.get_microsecond(), datetime.get_microsecond());

        let datetime = DateTime::new(2f32, 2018, 5, 28, 16, 6, -1f64);
        let datetime_ser = ron::ser::to_string(&datetime).unwrap();
        let datetime_de: DateTime = ron::de::from_str(datetime_ser.as_str()).unwrap();
        assert!(datetime_de.has_time());
        assert!(!datetime_de.has_second());
        assert_eq!(
            datetime_de.get_time_zone_offset(),
            datetime.get_time_zone_offset()
        );
        assert_eq!(datetime_de.get_year(), datetime.get_year());
        assert_eq!(datetime_de.get_month(), datetime.get_month());
        assert_eq!(datetime_de.get_day(), datetime.get_day());
        assert_eq!(datetime_de.get_hour(), datetime.get_hour());
        assert_eq!(datetime_de.get_minute(), datetime.get_minute());

        let datetime = DateTime::new_ymd(2018, 5, 28);
        let datetime_ser = ron::ser::to_string(&datetime).unwrap();
        let datetime_de: DateTime = ron::de::from_str(datetime_ser.as_str()).unwrap();
        assert!(datetime_de.has_day());
        assert!(!datetime_de.has_time());
        assert_eq!(datetime_de.get_year(), datetime.get_year());
        assert_eq!(datetime_de.get_month(), datetime.get_month());
        assert_eq!(datetime_de.get_day(), datetime.get_day());

        let datetime = DateTime::new_ym(2018, 5);
        let datetime_ser = ron::ser::to_string(&datetime).unwrap();
        let datetime_de: DateTime = ron::de::from_str(datetime_ser.as_str()).unwrap();
        assert!(datetime_de.has_month());
        assert!(!datetime_de.has_day());
        assert_eq!(datetime_de.get_year(), datetime.get_year());
        assert_eq!(datetime_de.get_month(), datetime.get_month());

        let datetime = DateTime::new_y(2018);
        let datetime_ser = ron::ser::to_string(&datetime).unwrap();
        let datetime_de: DateTime = ron::de::from_str(datetime_ser.as_str()).unwrap();
        assert!(datetime_de.has_year());
        assert!(!datetime_de.has_month());
        assert_eq!(datetime_de.get_year(), datetime.get_year());
    }
}
