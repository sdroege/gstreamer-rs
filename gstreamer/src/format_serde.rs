// Take a look at the license at the top of the repository in the LICENSE file.

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use crate::format::{Buffers, Bytes, Default, Percent, Undefined};

// Manual implementation for some types that would otherwise yield representations such as:
// "Default((Some(42)))"
macro_rules! impl_ser_de(
    ($t:ident) => {
        impl Serialize for $t {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.0.serialize(serializer)
            }
        }


        impl<'de> Deserialize<'de> for $t {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
	        skip_assert_initialized!();
                u64::deserialize(deserializer).map($t)
            }
        }
    }
);

impl_ser_de!(Buffers);
impl_ser_de!(Bytes);
impl_ser_de!(Default);

impl Serialize for Undefined {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Undefined {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        i64::deserialize(deserializer).map(Undefined)
    }
}

impl Serialize for Percent {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Percent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        u32::deserialize(deserializer).map(Percent)
    }
}

#[cfg(test)]
mod tests {
    use crate::format::{Buffers, Bytes, Default, Percent, Undefined};
    use crate::ClockTime;
    use crate::Format;
    use crate::GenericFormattedValue;

    #[test]
    fn test_serialize() {
        crate::init().unwrap();

        let pretty_config = ron::ser::PrettyConfig::new().new_line("".to_string());

        let value = GenericFormattedValue::from(Undefined(42));
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Undefined(42)".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Undefined\":42}".to_owned(), res);

        let value = GenericFormattedValue::from(Default(42));
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Default(Some(42))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Default\":42}".to_owned(), res);

        let value = GenericFormattedValue::from(Option::<Default>::None);
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Default(None)".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Default\":null}".to_owned(), res);

        let value = GenericFormattedValue::from(Bytes(42));
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Bytes(Some(42))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Bytes\":42}".to_owned(), res);

        let value = GenericFormattedValue::from(ClockTime::from_nseconds(42_123_456_789));
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Time(Some(42123456789))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Time\":42123456789}".to_owned(), res);

        let value = GenericFormattedValue::from(Buffers(42));
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Buffers(Some(42))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Buffers\":42}".to_owned(), res);

        let value = GenericFormattedValue::from(Percent::try_from(0.42).unwrap());
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Percent(Some(4200))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Percent\":4200}".to_owned(), res);

        let value = GenericFormattedValue::Other(Format::Percent, 42);
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Other(Percent, 42)".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Other\":[\"Percent\",42]}".to_owned(), res);

        let value = GenericFormattedValue::Other(Format::__Unknown(7), 42);
        let res = ron::ser::to_string_pretty(&value, pretty_config);
        assert_eq!(Ok("Other(__Unknown(7), 42)".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Other\":[{\"__Unknown\":7},42]}".to_owned(), res);
    }

    #[test]
    fn test_deserialize() {
        crate::init().unwrap();

        let value_ron = "Default(Some(42))";
        let value_de: GenericFormattedValue = ron::de::from_str(value_ron).unwrap();
        assert_eq!(value_de, GenericFormattedValue::from(Default(42)));

        let value_json = "{\"Default\":42}";
        let value_de: GenericFormattedValue = serde_json::from_str(value_json).unwrap();
        assert_eq!(value_de, GenericFormattedValue::from(Default(42)));

        let value_ron = "Other(Percent, 42)";
        let value_de: GenericFormattedValue = ron::de::from_str(value_ron).unwrap();
        assert_eq!(value_de, GenericFormattedValue::Other(Format::Percent, 42));

        let value_json = "{\"Other\":[\"Percent\",42]}";
        let value_de: GenericFormattedValue = serde_json::from_str(value_json).unwrap();
        assert_eq!(value_de, GenericFormattedValue::Other(Format::Percent, 42));
    }

    #[test]
    fn test_serde_roundtrip() {
        crate::init().unwrap();

        macro_rules! test_roundrip(
            ($value:expr) => {
                let value_ser = ron::ser::to_string(&$value).unwrap();
                let value_de: GenericFormattedValue = ron::de::from_str(value_ser.as_str()).unwrap();
                assert_eq!(value_de, $value);
            }
        );

        test_roundrip!(GenericFormattedValue::Undefined(Undefined::from(42)));
        test_roundrip!(GenericFormattedValue::from(Default(42)));
        test_roundrip!(GenericFormattedValue::from(Bytes(42)));
        test_roundrip!(GenericFormattedValue::from(ClockTime::from_nseconds(
            42_123_456_789
        )));
        test_roundrip!(GenericFormattedValue::from(Buffers(42)));
        test_roundrip!(GenericFormattedValue::from(
            Percent::try_from(0.42).unwrap()
        ));
        test_roundrip!(GenericFormattedValue::Other(Format::Percent, 42));
        test_roundrip!(GenericFormattedValue::Other(Format::__Unknown(7), 42));
    }
}
