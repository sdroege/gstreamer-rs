// Copyright (C) 2018 François Laignel <fengalin@free.fr>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::de;
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};

use std::fmt;

use ClockTime;

impl<'a> Serialize for ClockTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.nanoseconds() {
            Some(ref value) => serializer.serialize_some(value),
            None => serializer.serialize_none(),
        }
    }
}

struct ClockTimeVisitor;
impl<'de> Visitor<'de> for ClockTimeVisitor {
    type Value = ClockTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an optional u64 ClockTime with ns precision")
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        u64::deserialize(deserializer).map(ClockTime::from_nseconds)
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(ClockTime(None))
    }
}

impl<'de> Deserialize<'de> for ClockTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        deserializer.deserialize_option(ClockTimeVisitor)
    }
}

#[cfg(test)]
mod tests {
    extern crate ron;
    extern crate serde_json;

    use ClockTime;

    #[test]
    fn test_serialize() {
        ::init().unwrap();

        // Some
        let clocktime = ClockTime::from_nseconds(42_123_456_789);

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        let res = ron::ser::to_string_pretty(&clocktime, pretty_config.clone());
        assert_eq!(Ok("Some(42123456789)".to_owned()), res);

        let res = serde_json::to_string(&clocktime).unwrap();
        assert_eq!("42123456789".to_owned(), res);

        // None
        let clocktime = ClockTime(None);

        let res = ron::ser::to_string_pretty(&clocktime, pretty_config);
        assert_eq!(Ok("None".to_owned()), res);

        let res = serde_json::to_string(&clocktime).unwrap();
        assert_eq!("null".to_owned(), res);
    }

    #[test]
    fn test_deserialize() {
        ::init().unwrap();

        // Some
        let clocktime_ron = "Some(42123456789)";
        let clocktime: ClockTime = ron::de::from_str(clocktime_ron).unwrap();
        assert_eq!(clocktime.seconds(), Some(42));
        assert_eq!(clocktime.mseconds(), Some(42_123));
        assert_eq!(clocktime.useconds(), Some(42_123_456));
        assert_eq!(clocktime.nseconds(), Some(42_123_456_789));

        let clocktime_json = "42123456789";
        let clocktime: ClockTime = serde_json::from_str(clocktime_json).unwrap();
        assert_eq!(clocktime.seconds(), Some(42));
        assert_eq!(clocktime.mseconds(), Some(42_123));
        assert_eq!(clocktime.useconds(), Some(42_123_456));
        assert_eq!(clocktime.nseconds(), Some(42_123_456_789));

        // None
        let clocktime_ron = "None";
        let clocktime: ClockTime = ron::de::from_str(clocktime_ron).unwrap();
        assert_eq!(clocktime.nseconds(), None);

        let clocktime_json = "null";
        let clocktime: ClockTime = serde_json::from_str(clocktime_json).unwrap();
        assert_eq!(clocktime.nseconds(), None);
    }

    #[test]
    fn test_serde_roundtrip() {
        ::init().unwrap();

        // Some
        let clocktime = ClockTime::from_nseconds(42_123_456_789);
        let clocktime_ser = ron::ser::to_string(&clocktime).unwrap();
        let clocktime: ClockTime = ron::de::from_str(clocktime_ser.as_str()).unwrap();
        assert_eq!(clocktime.seconds(), Some(42));
        assert_eq!(clocktime.mseconds(), Some(42_123));
        assert_eq!(clocktime.useconds(), Some(42_123_456));
        assert_eq!(clocktime.nseconds(), Some(42_123_456_789));

        // None
        let clocktime = ClockTime(None);
        let clocktime_ser = ron::ser::to_string(&clocktime).unwrap();
        let clocktime: ClockTime = ron::de::from_str(clocktime_ser.as_str()).unwrap();
        assert_eq!(clocktime.nseconds(), None);
    }
}
