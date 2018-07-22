// Copyright (C) 2018 François Laignel <fengalin@free.fr>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use DateTime;

impl<'a> Serialize for DateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut datetime = serializer.serialize_struct("DateTime", 8)?;
        datetime.serialize_field("tz_offset", &self.get_time_zone_offset())?;
        datetime.serialize_field("y", &self.get_year())?;
        datetime.serialize_field("m", &self.get_month())?;
        datetime.serialize_field("d", &self.get_day())?;
        datetime.serialize_field("h", &self.get_hour())?;
        datetime.serialize_field("mn", &self.get_minute())?;
        datetime.serialize_field("s", &self.get_second())?;
        datetime.serialize_field("us", &self.get_microsecond())?;
        datetime.end()
    }
}

#[derive(Deserialize)]
struct DateTimeDe {
    tz_offset: f32,
    y: i32,
    m: i32,
    d: i32,
    h: i32,
    mn: i32,
    s: i32,
    us: i32,
}

impl From<DateTimeDe> for DateTime {
    fn from(dt_de: DateTimeDe) -> Self {
        ::DateTime::new(
            dt_de.tz_offset,
            dt_de.y,
            dt_de.m,
            dt_de.d,
            dt_de.h,
            dt_de.mn,
            f64::from(dt_de.s) + f64::from(dt_de.us) / 1_000_000f64,
        )
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        DateTimeDe::deserialize(deserializer).and_then(|datetime_de| Ok(datetime_de.into()))
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

        let datetime = DateTime::new(2f32, 2018, 5, 28, 16, 6, 42.841f64);

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        let res = ron::ser::to_string_pretty(&datetime, pretty_config);
        assert_eq!(
            Ok(concat!(
                "(",
                "    tz_offset: 2,",
                "    y: 2018,",
                "    m: 5,",
                "    d: 28,",
                "    h: 16,",
                "    mn: 6,",
                "    s: 42,",
                "    us: 841000,",
                ")"
            ).to_owned()),
            res,
        );

        let res = serde_json::to_string(&datetime).unwrap();
        assert_eq!(
            "{\"tz_offset\":2.0,\"y\":2018,\"m\":5,\"d\":28,\"h\":16,\"mn\":6,\"s\":42,\"us\":841000}"
                .to_owned(),
            res,
        );
    }

    #[test]
    fn test_deserialize() {
        ::init().unwrap();

        let datetime_ron = r#"
            (
                tz_offset: 2,
                y: 2018,
                m: 5,
                d: 28,
                h: 16,
                mn: 6,
                s: 42,
                us: 841000,
            )
        "#;
        let datetime: DateTime = ron::de::from_str(datetime_ron).unwrap();
        assert_eq!(datetime.get_time_zone_offset(), 2f32);
        assert_eq!(datetime.get_year(), 2018);
        assert_eq!(datetime.get_month(), 5);
        assert_eq!(datetime.get_day(), 28);
        assert_eq!(datetime.get_hour(), 16);
        assert_eq!(datetime.get_minute(), 6);
        assert_eq!(datetime.get_second(), 42);
        assert_eq!(datetime.get_microsecond(), 841_000);

        let datetime_json = r#"
            {"tz_offset":2.0,"y":2018,"m":5,"d":28,"h":16,"mn":6,"s":42,"us":841000}
        "#;
        let datetime: DateTime = serde_json::from_str(datetime_json).unwrap();
        assert_eq!(datetime.get_time_zone_offset(), 2f32);
        assert_eq!(datetime.get_year(), 2018);
        assert_eq!(datetime.get_month(), 5);
        assert_eq!(datetime.get_day(), 28);
        assert_eq!(datetime.get_hour(), 16);
        assert_eq!(datetime.get_minute(), 6);
        assert_eq!(datetime.get_second(), 42);
        assert_eq!(datetime.get_microsecond(), 841_000);
    }
}
