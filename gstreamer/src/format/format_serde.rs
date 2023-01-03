// Take a look at the license at the top of the repository in the LICENSE file.

use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

use crate::format::{Buffers, Bytes, Default, Other, Percent, Undefined};

// FIXME: the ser/de impl assumed `GenericFormattedValue` was always used.
// When serializing a `SpecificFormattedValue`, we loose the type and only
// serialize the inner value in parenthesis.
// Manual implementation for some types that would otherwise yield representations such as:
// "Default(Some((42)))"
macro_rules! impl_serde(
    ($t:ident, $inner:ty) => {
        impl Serialize for $t {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use std::ops::Deref;
                self.deref().serialize(serializer)
            }
        }

        impl<'de> Deserialize<'de> for $t {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
	            skip_assert_initialized!();
                <$inner>::deserialize(deserializer)
                    .and_then(|value| {
                        $t::try_from(value).map_err(|_| {
                            use serde::de::{Error, Unexpected};
                            D::Error::invalid_value(
                                Unexpected::Unsigned(value.into()),
                                &concat!("valid ", stringify!($t)),
                            )
                        })
                    })
            }
        }
    }
);

impl_serde!(Buffers, u64);
impl_serde!(Bytes, u64);
impl_serde!(Default, u64);
impl_serde!(Other, u64);
impl_serde!(Percent, u32);

impl Serialize for Undefined {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use std::ops::Deref;
        self.deref().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Undefined {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        i64::deserialize(deserializer).map(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        format::{prelude::*, Default, Other, Undefined},
        ClockTime, Format, GenericFormattedValue,
    };

    #[test]
    fn test_serialize() {
        let pretty_config = ron::ser::PrettyConfig::new().new_line("".to_string());

        let value = GenericFormattedValue::from(Undefined::from(42));
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Undefined(42)".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Undefined\":42}".to_owned(), res);

        let value = GenericFormattedValue::from(42.default_format());
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Default(Some(42))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Default\":42}".to_owned(), res);

        let value = GenericFormattedValue::from(Default::NONE);
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Default(None)".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Default\":null}".to_owned(), res);

        let value = GenericFormattedValue::from(42.bytes());
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Bytes(Some(42))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Bytes\":42}".to_owned(), res);

        let value = GenericFormattedValue::from(ClockTime::from_nseconds(42_123_456_789));
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Time(Some(42123456789))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Time\":42123456789}".to_owned(), res);

        let value = GenericFormattedValue::from(42.buffers());
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Buffers(Some(42))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Buffers\":42}".to_owned(), res);

        let percent = 0.42.percent_ratio();
        let value = GenericFormattedValue::from(percent);
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Percent(Some(420000))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Percent\":420000}".to_owned(), res);

        let other = Other::try_from(42).ok();
        let value = GenericFormattedValue::Other(Format::Default, other);
        let res = ron::ser::to_string_pretty(&value, pretty_config.clone());
        assert_eq!(Ok("Other(Default, Some(42))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Other\":[\"Default\",42]}".to_owned(), res);

        let value = GenericFormattedValue::new(Format::__Unknown(7), 42);
        let res = ron::ser::to_string_pretty(&value, pretty_config);
        assert_eq!(Ok("Other(__Unknown(7), Some(42))".to_owned()), res);
        let res = serde_json::to_string(&value).unwrap();
        assert_eq!("{\"Other\":[{\"__Unknown\":7},42]}".to_owned(), res);
    }

    #[test]
    fn test_deserialize() {
        let value_ron = "Default(Some(42))";
        let value_de: GenericFormattedValue = ron::de::from_str(value_ron).unwrap();
        assert_eq!(value_de, GenericFormattedValue::from(42.default_format()));

        let value_json = "{\"Default\":42}";
        let value_de: GenericFormattedValue = serde_json::from_str(value_json).unwrap();
        assert_eq!(value_de, GenericFormattedValue::from(Default::from_u64(42)));

        let value_ron = "Other(Buffers, Some(42))";
        let gfv_value = GenericFormattedValue::Other(Format::Buffers, Some(42 * Other::ONE));

        let value_de: GenericFormattedValue = ron::de::from_str(value_ron).unwrap();
        assert_eq!(value_de, gfv_value);

        let value_json = "{\"Other\":[\"Buffers\",42]}";
        let value_de: GenericFormattedValue = serde_json::from_str(value_json).unwrap();
        assert_eq!(value_de, gfv_value);
    }

    #[test]
    fn test_serde_roundtrip() {
        macro_rules! test_roundrip(
            ($value:expr) => {
                let value_ser = ron::ser::to_string(&$value).unwrap();
                let value_de: GenericFormattedValue = ron::de::from_str(value_ser.as_str()).unwrap();
                assert_eq!(value_de, $value);
            }
        );

        test_roundrip!(GenericFormattedValue::Undefined(Undefined::from(42)));
        test_roundrip!(GenericFormattedValue::from(Default::from_u64(42)));
        test_roundrip!(GenericFormattedValue::from(42.bytes()));
        test_roundrip!(GenericFormattedValue::from(ClockTime::from_nseconds(
            42_123_456_789
        )));
        test_roundrip!(GenericFormattedValue::from(42.buffers()));
        test_roundrip!(GenericFormattedValue::from(42.percent()));
        let gfv_value = GenericFormattedValue::Other(Format::Default, Other::try_from(42).ok());
        test_roundrip!(gfv_value);
        test_roundrip!(GenericFormattedValue::new(Format::__Unknown(7), 42));
    }
}
