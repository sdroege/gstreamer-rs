// Take a look at the license at the top of the repository in the LICENSE file.

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use crate::ClockTime;

impl<'a> Serialize for ClockTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ClockTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        u64::deserialize(deserializer).map(ClockTime::from_nseconds)
    }
}

#[cfg(test)]
mod tests {
    use crate::ClockTime;

    #[test]
    fn test_serialize() {
        crate::init().unwrap();

        // Some
        let clocktime = Some(ClockTime::from_nseconds(42_123_456_789));

        let pretty_config = ron::ser::PrettyConfig::new().with_new_line("".to_string());

        let res = ron::ser::to_string_pretty(&clocktime, pretty_config.clone());
        assert_eq!(Ok("Some(42123456789)".to_owned()), res);

        let res = serde_json::to_string(&clocktime).unwrap();
        assert_eq!("42123456789".to_owned(), res);

        // None
        let clocktime = ClockTime::NONE;

        let res = ron::ser::to_string_pretty(&clocktime, pretty_config);
        assert_eq!(Ok("None".to_owned()), res);

        let res = serde_json::to_string(&clocktime).unwrap();
        assert_eq!("null".to_owned(), res);
    }

    #[test]
    fn test_deserialize() {
        crate::init().unwrap();

        // Some
        let clocktime_ron = "Some(42123456789)";
        let clocktime: Option<ClockTime> = ron::de::from_str(clocktime_ron).unwrap();
        let clocktime = clocktime.unwrap();
        assert_eq!(clocktime.seconds(), 42);
        assert_eq!(clocktime.mseconds(), 42_123);
        assert_eq!(clocktime.useconds(), 42_123_456);
        assert_eq!(clocktime.nseconds(), 42_123_456_789);

        let clocktime_json = "42123456789";
        let clocktime: Option<ClockTime> = serde_json::from_str(clocktime_json).unwrap();
        let clocktime = clocktime.unwrap();
        assert_eq!(clocktime.seconds(), 42);
        assert_eq!(clocktime.mseconds(), 42_123);
        assert_eq!(clocktime.useconds(), 42_123_456);
        assert_eq!(clocktime.nseconds(), 42_123_456_789);

        // None
        let clocktime_ron = "None";
        let clocktime: Option<ClockTime> = ron::de::from_str(clocktime_ron).unwrap();
        assert!(clocktime.is_none());

        let clocktime_json = "null";
        let clocktime: Option<ClockTime> = serde_json::from_str(clocktime_json).unwrap();
        assert!(clocktime.is_none());
        assert!(clocktime.is_none());
    }

    #[test]
    fn test_serde_roundtrip() {
        crate::init().unwrap();

        // Direct
        let clocktime = ClockTime::from_nseconds(42_123_456_789);
        let clocktime_ser = ron::ser::to_string(&clocktime).unwrap();
        let clocktime: ClockTime = ron::de::from_str(clocktime_ser.as_str()).unwrap();
        assert_eq!(clocktime.seconds(), 42);
        assert_eq!(clocktime.mseconds(), 42_123);
        assert_eq!(clocktime.useconds(), 42_123_456);
        assert_eq!(clocktime.nseconds(), 42_123_456_789);

        // Some
        let clocktime = Some(ClockTime::from_nseconds(42_123_456_789));
        let clocktime_ser = ron::ser::to_string(&clocktime).unwrap();
        let clocktime: Option<ClockTime> = ron::de::from_str(clocktime_ser.as_str()).unwrap();
        let clocktime = clocktime.unwrap();
        assert_eq!(clocktime.seconds(), 42);
        assert_eq!(clocktime.mseconds(), 42_123);
        assert_eq!(clocktime.useconds(), 42_123_456);
        assert_eq!(clocktime.nseconds(), 42_123_456_789);

        // None
        let clocktime = ClockTime::NONE;
        let clocktime_ser = ron::ser::to_string(&clocktime).unwrap();
        let clocktime: Option<ClockTime> = ron::de::from_str(clocktime_ser.as_str()).unwrap();
        assert!(clocktime.is_none());
    }
}
