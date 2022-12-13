// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::upper_case_acronyms)]

use glib::translate::{FromGlib, IntoGlib};
use glib::value::{ToValue, ToValueOptional};
use glib::StaticType;

use crate::DateTime;
use serde::de::{Deserialize, Deserializer, Error};
use serde::ser;
use serde::ser::{Serialize, Serializer};

#[derive(serde::Serialize, serde::Deserialize)]
enum DateTimeVariants {
    Y(i32),
    YM(i32, i32),
    YMD(i32, i32, i32),
    YMDhmTz(i32, i32, i32, i32, i32, f32),
    YMDhmsTz(i32, i32, i32, i32, i32, f64, f32),
}

// Note: ser / de for `glib::Date` should be implemented in the `glib` crate
// However, there is no `serde` feature in `glib` right now. The limitation is that
// `Date` fields can only be ser / de when they are used in `Value`s (which implies
// `Array`s, `List`s, `Structure` fields and `Tag`s)
pub(crate) struct Date(glib::Date);

impl From<glib::Date> for Date {
    fn from(glib_date: glib::Date) -> Self {
        skip_assert_initialized!();
        Date(glib_date)
    }
}

impl ToValue for Date {
    fn to_value(&self) -> glib::Value {
        self.0.to_value()
    }

    fn value_type(&self) -> glib::Type {
        glib::Date::static_type()
    }
}

impl ToValueOptional for Date {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        s.map(|s| &s.0).to_value()
    }
}

impl StaticType for Date {
    fn static_type() -> glib::Type {
        glib::Date::static_type()
    }
}

impl From<Date> for glib::Value {
    fn from(v: Date) -> glib::Value {
        skip_assert_initialized!();
        v.0.into()
    }
}

impl Serialize for Date {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        DateTimeVariants::YMD(
            self.0.year() as i32,
            self.0.month().into_glib(),
            self.0.day() as i32,
        )
        .serialize(serializer)
    }
}

impl Serialize for DateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let variant = if self.has_second() {
            DateTimeVariants::YMDhmsTz(
                self.year(),
                self.month().unwrap(),
                self.day().unwrap(),
                self.hour().unwrap(),
                self.minute().unwrap(),
                f64::from(self.second().unwrap())
                    + f64::from(self.microsecond().unwrap()) / 1_000_000f64,
                self.time_zone_offset().unwrap(),
            )
        } else if self.has_time() {
            DateTimeVariants::YMDhmTz(
                self.year(),
                self.month().unwrap(),
                self.day().unwrap(),
                self.hour().unwrap(),
                self.minute().unwrap(),
                self.time_zone_offset().unwrap(),
            )
        } else if self.has_day() {
            DateTimeVariants::YMD(self.year(), self.month().unwrap(), self.day().unwrap())
        } else if self.has_month() {
            DateTimeVariants::YM(self.year(), self.month().unwrap())
        } else if self.has_year() {
            DateTimeVariants::Y(self.year())
        } else {
            return Err(ser::Error::custom(format!(
                "no parts could be found in `DateTime` {}",
                self,
            )));
        };

        variant.serialize(serializer)
    }
}

impl TryFrom<DateTimeVariants> for Date {
    type Error = glib::BoolError;

    fn try_from(dt_variant: DateTimeVariants) -> Result<Self, Self::Error> {
        skip_assert_initialized!();
        match dt_variant {
            DateTimeVariants::YMD(y, m, d) => {
                let month = unsafe { glib::DateMonth::from_glib(m) };
                if let glib::DateMonth::__Unknown(_) = month {
                    return Err(glib::bool_error!("Out of range `month` for `Date`"));
                }

                Ok(Date(glib::Date::from_dmy(
                    d.try_into()
                        .map_err(|_| glib::bool_error!("Out of range `day` for `Date`"))?,
                    month,
                    y.try_into()
                        .map_err(|_| glib::bool_error!("Out of range `year` for `Date`"))?,
                )?))
            }
            _ => Err(glib::bool_error!(
                "Incompatible variant for `Date` (expecting \"YMD\")"
            )),
        }
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        DateTimeVariants::deserialize(deserializer)
            .and_then(|dt_variant| dt_variant.try_into().map_err(D::Error::custom))
    }
}

#[allow(clippy::many_single_char_names)]
impl TryFrom<DateTimeVariants> for DateTime {
    type Error = glib::BoolError;

    fn try_from(dt_variant: DateTimeVariants) -> Result<Self, Self::Error> {
        skip_assert_initialized!();
        match dt_variant {
            DateTimeVariants::Y(y) => DateTime::from_y(y),
            DateTimeVariants::YM(y, m) => DateTime::from_ym(y, m),
            DateTimeVariants::YMD(y, m, d) => DateTime::from_ymd(y, m, d),
            DateTimeVariants::YMDhmTz(y, m, d, h, mn, tz) => {
                DateTime::new(tz, y, m, d, h, mn, None)
            }
            DateTimeVariants::YMDhmsTz(y, m, d, h, mn, s, tz) => {
                DateTime::new(tz, y, m, d, h, mn, s)
            }
        }
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        DateTimeVariants::deserialize(deserializer)
            .and_then(|dt_variant| dt_variant.try_into().map_err(D::Error::custom))
    }
}

#[cfg(test)]
mod tests {
    use crate::DateTime;

    #[test]
    fn test_serialize() {
        crate::init().unwrap();

        let pretty_config = ron::ser::PrettyConfig::new().new_line("".to_string());

        let datetime = DateTime::new(2f32, 2018, 5, 28, 16, 6, 42.123_456f64).unwrap();
        let res = ron::ser::to_string_pretty(&datetime, pretty_config.clone());
        assert_eq!(
            Ok("YMDhmsTz(2018, 5, 28, 16, 6, 42.123456, 2.0)".to_owned()),
            res,
        );

        let res = serde_json::to_string(&datetime).unwrap();
        assert_eq!(
            r#"{"YMDhmsTz":[2018,5,28,16,6,42.123456,2.0]}"#.to_owned(),
            res
        );

        let datetime = DateTime::new(2f32, 2018, 5, 28, 16, 6, None).unwrap();
        let res = ron::ser::to_string_pretty(&datetime, pretty_config.clone());
        assert_eq!(Ok("YMDhmTz(2018, 5, 28, 16, 6, 2.0)".to_owned()), res,);

        let datetime = DateTime::from_ymd(2018, 5, 28).unwrap();
        let res = ron::ser::to_string_pretty(&datetime, pretty_config.clone());
        assert_eq!(Ok("YMD(2018, 5, 28)".to_owned()), res);

        let datetime = DateTime::from_ym(2018, 5).unwrap();
        let res = ron::ser::to_string_pretty(&datetime, pretty_config.clone());
        assert_eq!(Ok("YM(2018, 5)".to_owned()), res);

        let datetime = DateTime::from_y(2018).unwrap();
        let res = ron::ser::to_string_pretty(&datetime, pretty_config);
        assert_eq!(Ok("Y(2018)".to_owned()), res);
    }

    #[test]
    fn test_deserialize() {
        crate::init().unwrap();

        let datetime_ron = "YMDhmsTz(2018, 5, 28, 16, 6, 42.123456, 2)";
        let datetime_de: DateTime = ron::de::from_str(datetime_ron).unwrap();
        assert_eq!(
            datetime_de,
            DateTime::new(2f32, 2018, 5, 28, 16, 6, 42.123_456f64).unwrap()
        );

        let datetime_json = r#"{"YMDhmsTz":[2018,5,28,16,6,42.123456,2.0]}"#;
        let datetime_de: DateTime = serde_json::from_str(datetime_json).unwrap();
        assert_eq!(
            datetime_de,
            DateTime::new(2f32, 2018, 5, 28, 16, 6, 42.123_456f64).unwrap()
        );

        let datetime_ron = "YMDhmTz(2018, 5, 28, 16, 6, 2)";
        let datetime_de: DateTime = ron::de::from_str(datetime_ron).unwrap();
        assert_eq!(
            datetime_de,
            DateTime::new(2f32, 2018, 5, 28, 16, 6, None).unwrap()
        );

        let datetime_ron = "YMD(2018, 5, 28)";
        let datetime_de: DateTime = ron::de::from_str(datetime_ron).unwrap();
        assert_eq!(datetime_de, DateTime::from_ymd(2018, 5, 28).unwrap());

        let datetime_ron = "YM(2018, 5)";
        let datetime_de: DateTime = ron::de::from_str(datetime_ron).unwrap();
        assert_eq!(datetime_de, DateTime::from_ym(2018, 5).unwrap());

        let datetime_ron = "Y(2018)";
        let datetime_de: DateTime = ron::de::from_str(datetime_ron).unwrap();
        assert_eq!(datetime_de, DateTime::from_y(2018).unwrap());
    }

    #[test]
    fn test_serde_roundtrip() {
        crate::init().unwrap();

        let datetime = DateTime::new(2f32, 2018, 5, 28, 16, 6, 42.123_456f64).unwrap();
        let datetime_ser = ron::ser::to_string(&datetime).unwrap();
        let datetime_de: DateTime = ron::de::from_str(datetime_ser.as_str()).unwrap();
        assert_eq!(datetime_de, datetime);

        let datetime = DateTime::new(2f32, 2018, 5, 28, 16, 6, None).unwrap();
        let datetime_ser = ron::ser::to_string(&datetime).unwrap();
        let datetime_de: DateTime = ron::de::from_str(datetime_ser.as_str()).unwrap();
        assert_eq!(datetime_de, datetime);

        let datetime = DateTime::from_ymd(2018, 5, 28).unwrap();
        let datetime_ser = ron::ser::to_string(&datetime).unwrap();
        let datetime_de: DateTime = ron::de::from_str(datetime_ser.as_str()).unwrap();
        assert_eq!(datetime_de, datetime);

        let datetime = DateTime::from_ym(2018, 5).unwrap();
        let datetime_ser = ron::ser::to_string(&datetime).unwrap();
        let datetime_de: DateTime = ron::de::from_str(datetime_ser.as_str()).unwrap();
        assert_eq!(datetime_de, datetime);

        let datetime = DateTime::from_y(2018).unwrap();
        let datetime_ser = ron::ser::to_string(&datetime).unwrap();
        let datetime_de: DateTime = ron::de::from_str(datetime_ser.as_str()).unwrap();
        assert_eq!(datetime_de, datetime);
    }
}
