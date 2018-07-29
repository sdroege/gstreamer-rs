// Copyright (C) 2018 François Laignel <fengalin@free.fr>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use format::{Buffers, Bytes, Default};

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
                Option::<u64>::deserialize(deserializer).map(|value| $t(value))
            }
        }
    }
);

impl_ser_de!(Buffers);
impl_ser_de!(Bytes);
impl_ser_de!(Default);

#[cfg(test)]
mod tests {
    extern crate ron;
    extern crate serde_json;

    use format::{Buffers, Bytes, Default};
    use ClockTime;
    use Format;
    use GenericFormattedValue;

    #[test]
    fn test_serialize() {
        ::init().unwrap();

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        let value = GenericFormattedValue::Undefined(42);
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Undefined(42)".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Undefined\":42}".to_owned(), res);

        let value = GenericFormattedValue::Default(Default(Some(42)));
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Default(Some(42))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Default\":42}".to_owned(), res);

        let value = GenericFormattedValue::Default(Default(None));
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Default(None)".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Default\":null}".to_owned(), res);

        let value = GenericFormattedValue::Bytes(Bytes(Some(42)));
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Bytes(Some(42))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Bytes\":42}".to_owned(), res);

        let value = GenericFormattedValue::Time(ClockTime::from_nseconds(42_123_456_789));
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Time(Some(42123456789))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Time\":42123456789}".to_owned(), res);

        let value = GenericFormattedValue::Buffers(Buffers(Some(42)));
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Buffers(Some(42))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Buffers\":42}".to_owned(), res);

        let value = GenericFormattedValue::Percent(Some(42));
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Percent(Some(42))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Percent\":42}".to_owned(), res);

        let value = GenericFormattedValue::Other(Format::Percent, 42);
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Other(Percent, 42)".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Other\":[\"Percent\",42]}".to_owned(), res);

        let value = GenericFormattedValue::Other(Format::__Unknown(7), 42);
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Other(__Unknown(7), 42)".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Other\":[{\"__Unknown\":7},42]}".to_owned(), res);
    }

    #[test]
    fn test_deserialize() {
        ::init().unwrap();

        let value_ron = "Default(Some(42))";
        let value_de: GenericFormattedValue = ron::de::from_str(value_ron).unwrap();
        assert_eq!(value_de, GenericFormattedValue::Default(Default(Some(42))));

        let value_json = "{\"Default\":42}";
        let value_de: GenericFormattedValue = serde_json::from_str(value_json).unwrap();
        assert_eq!(value_de, GenericFormattedValue::Default(Default(Some(42))));

        let value_ron = "Other(Percent, 42)";
        let value_de: GenericFormattedValue = ron::de::from_str(value_ron).unwrap();
        assert_eq!(value_de, GenericFormattedValue::Other(Format::Percent, 42));

        let value_json = "{\"Other\":[\"Percent\",42]}";
        let value_de: GenericFormattedValue = serde_json::from_str(value_json).unwrap();
        assert_eq!(value_de, GenericFormattedValue::Other(Format::Percent, 42));
    }

    #[test]
    fn test_serde_roundtrip() {
        ::init().unwrap();

        macro_rules! test_roundrip(
            ($value:expr) => {
                let value_ser = ron::ser::to_string(&$value).unwrap();
                let value_de: GenericFormattedValue = ron::de::from_str(value_ser.as_str()).unwrap();
                assert_eq!(value_de, $value);
            }
        );

        test_roundrip!(GenericFormattedValue::Undefined(42));
        test_roundrip!(GenericFormattedValue::Default(Default(Some(42))));
        test_roundrip!(GenericFormattedValue::Bytes(Bytes(Some(42))));
        test_roundrip!(GenericFormattedValue::Time(ClockTime::from_nseconds(
            42_123_456_789
        )));
        test_roundrip!(GenericFormattedValue::Buffers(Buffers(Some(42))));
        test_roundrip!(GenericFormattedValue::Percent(Some(42)));
        test_roundrip!(GenericFormattedValue::Other(Format::Percent, 42));
        test_roundrip!(GenericFormattedValue::Other(Format::__Unknown(7), 42));
    }
}
