// Copyright (C) 2018 François Laignel <fengalin@free.fr>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib;
use glib::{Date, StaticType, ToValue};

use num_rational::Rational32;

use serde::de;
use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser;
use serde::ser::{Serialize, SerializeTuple, Serializer};

use std::{fmt, mem};

use once_cell::sync::Lazy;

use Buffer;
use DateTime;
use Sample;

use date_time_serde;
use value::*;

fn get_other_type_id<T: StaticType>() -> usize {
    match T::static_type() {
        glib::Type::Other(type_id) => type_id,
        type_ => panic!("Expecting `Other` variant, found `{}`", type_),
    }
}

pub(crate) static ARRAY_OTHER_TYPE_ID: Lazy<usize> = Lazy::new(get_other_type_id::<Array>);
pub(crate) static BITMASK_OTHER_TYPE_ID: Lazy<usize> = Lazy::new(get_other_type_id::<Bitmask>);
pub(crate) static DATE_OTHER_TYPE_ID: Lazy<usize> = Lazy::new(get_other_type_id::<Date>);
pub(crate) static DATE_TIME_OTHER_TYPE_ID: Lazy<usize> = Lazy::new(get_other_type_id::<DateTime>);
pub(crate) static FRACTION_OTHER_TYPE_ID: Lazy<usize> = Lazy::new(get_other_type_id::<Fraction>);
pub(crate) static FRACTION_RANGE_OTHER_TYPE_ID: Lazy<usize> =
    Lazy::new(get_other_type_id::<FractionRange>);
pub(crate) static INT_RANGE_I32_OTHER_TYPE_ID: Lazy<usize> =
    Lazy::new(get_other_type_id::<IntRange<i32>>);
pub(crate) static INT_RANGE_I64_OTHER_TYPE_ID: Lazy<usize> =
    Lazy::new(get_other_type_id::<IntRange<i64>>);
pub(crate) static LIST_OTHER_TYPE_ID: Lazy<usize> = Lazy::new(get_other_type_id::<List>);
pub(crate) static SAMPLE_OTHER_TYPE_ID: Lazy<usize> = Lazy::new(get_other_type_id::<Sample>);
pub(crate) static BUFFER_OTHER_TYPE_ID: Lazy<usize> = Lazy::new(get_other_type_id::<Buffer>);

impl<'a> Serialize for Fraction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Fraction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        Rational32::deserialize(deserializer)
            .map(|rational| Fraction::new(*rational.numer(), *rational.denom()))
    }
}

macro_rules! ser_some_value (
    ($value:expr, $t:ty, $ser_closure:expr) => (
        {
            let value = $value.get_some::<$t>().expect("ser_some_value macro");
            $ser_closure(stringify!($t), value)
        }
    );
);
macro_rules! ser_opt_value (
    ($value:expr, $t:ty, $ser_closure:expr) => (
        {
            let value = $value.get::<$t>().expect("ser_opt_value macro");
            $ser_closure(stringify!($t), value)
        }
    );
);
macro_rules! ser_value (
    ($value:expr, $ser_closure:expr) => (
        match $value.type_() {
            glib::Type::I8 => ser_some_value!($value, i8, $ser_closure),
            glib::Type::U8 => ser_some_value!($value, u8, $ser_closure),
            glib::Type::Bool => ser_some_value!($value, bool, $ser_closure),
            glib::Type::I32 => ser_some_value!($value, i32, $ser_closure),
            glib::Type::U32 => ser_some_value!($value, u32, $ser_closure),
            glib::Type::I64 => ser_some_value!($value, i64, $ser_closure),
            glib::Type::U64 => ser_some_value!($value, u64, $ser_closure),
            glib::Type::F32 => ser_some_value!($value, f32, $ser_closure),
            glib::Type::F64 => ser_some_value!($value, f64, $ser_closure),
            glib::Type::String => ser_opt_value!($value, String, $ser_closure),
            glib::Type::Other(type_id) => {
                if *ARRAY_OTHER_TYPE_ID == type_id {
                    ser_some_value!($value, Array, $ser_closure)
                } else if *BITMASK_OTHER_TYPE_ID == type_id {
                    ser_some_value!($value, Bitmask, $ser_closure)
                } else if *DATE_OTHER_TYPE_ID == type_id {
                    ser_opt_value!($value, Date, |type_, value: Option<Date>| {
                        // Need to wrap the `glib::Date` in new type `date_time_serde::Date` first
                        // See comment in `date_time_serde.rs`
                        $ser_closure(type_, value.map(date_time_serde::Date::from))
                    })
                } else if *DATE_TIME_OTHER_TYPE_ID == type_id {
                    ser_opt_value!($value, DateTime, $ser_closure)
                } else if *FRACTION_OTHER_TYPE_ID == type_id {
                    ser_some_value!($value, Fraction, $ser_closure)
                } else if *FRACTION_RANGE_OTHER_TYPE_ID == type_id {
                    ser_some_value!($value, FractionRange, $ser_closure)
                } else if *INT_RANGE_I32_OTHER_TYPE_ID == type_id {
                    ser_some_value!($value, IntRange<i32>, $ser_closure)
                } else if *INT_RANGE_I64_OTHER_TYPE_ID == type_id {
                    ser_some_value!($value, IntRange<i64>, $ser_closure)
                } else if *LIST_OTHER_TYPE_ID == type_id {
                    ser_some_value!($value, List, $ser_closure)
                } else if *SAMPLE_OTHER_TYPE_ID == type_id {
                    ser_opt_value!($value, Sample, $ser_closure)
                } else if *BUFFER_OTHER_TYPE_ID == type_id {
                    ser_opt_value!($value, Buffer, $ser_closure)
                } else {
                    Err(
                        ser::Error::custom(
                            format!("unimplemented `Value` serialization for type {}",
                                glib::Type::Other(type_id),
                            )
                        )
                    )
                }
            }
            type_ => {
                Err(
                    ser::Error::custom(
                        format!("unimplemented `Value` serialization for type {}", type_)
                    )
                )
            }
        }
    );
);

#[repr(transparent)]
pub(crate) struct SendValue(glib::SendValue);
impl SendValue {
    pub(crate) fn from(send_value: glib::SendValue) -> Self {
        skip_assert_initialized!();
        SendValue(send_value)
    }
}

impl From<SendValue> for glib::SendValue {
    fn from(send_value: SendValue) -> Self {
        skip_assert_initialized!();
        send_value.0
    }
}

impl Serialize for SendValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        ser_value!(self.0, |type_, value| {
            let mut tup = serializer.serialize_tuple(2)?;
            tup.serialize_element(type_)?;
            tup.serialize_element(&value)?;
            tup.end()
        })
    }
}

macro_rules! impl_ser_send_value_collection (
    ($t:ident) => (
        impl<'a> Serialize for $t<'a> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let send_value_vec = unsafe {
                    &*(self.as_slice() as *const [glib::SendValue] as *const [SendValue])
                };
                send_value_vec.serialize(serializer)
            }
        }
    );
);

impl_ser_send_value_collection!(Array);
impl_ser_send_value_collection!(List);

macro_rules! de_some_send_value(
    ($type_name:expr, $seq:expr, $t:ty) => (
        de_some_send_value!("Value", $type_name, $seq, $t)
    );
    ($outer_type:expr, $type_name:expr, $seq:expr, $t:ty) => (
        de_send_value!($outer_type, $type_name, $seq, $t, $t)
    );
);
macro_rules! de_opt_send_value(
    ($type_name:expr, $seq:expr, $t:ty) => (
        de_opt_send_value!("Value", $type_name, $seq, $t)
    );
    ($outer_type:expr, $type_name:expr, $seq:expr, $t:ty) => (
        de_send_value!($outer_type, $type_name, $seq, Option<$t>, $t)
    );
);
macro_rules! de_send_value(
    ($outer_type:expr, $type_name:expr, $seq:expr, $elem_t:ty, $t:ty) => (
        Ok(match $seq.next_element::<$elem_t>()? {
            Some(base_value) => {
                Some(SendValue::from(base_value
                    .to_value()
                    .try_into_send_value::<$t>()
                    .map_err(|_|
                        de::Error::custom(format!(
                            "Failed to convert `{}` with type {:?} to `SendValue`",
                            $outer_type,
                            $type_name,
                        ))
                    )?
                ))
            }
            None => None
        })
    );
    ($type_name:expr, $seq:expr) => (
        match $type_name.as_str() {
            "i8" => de_some_send_value!($type_name, $seq, i8),
            "u8" => de_some_send_value!($type_name, $seq, u8),
            "bool" => de_some_send_value!($type_name, $seq, bool),
            "i32" => de_some_send_value!($type_name, $seq, i32),
            "u32" => de_some_send_value!($type_name, $seq, u32),
            "i64" => de_some_send_value!($type_name, $seq, i64),
            "u64" => de_some_send_value!($type_name, $seq, u64),
            "f32" => de_some_send_value!($type_name, $seq, f32),
            "f64" => de_some_send_value!($type_name, $seq, f64),
            "String" => de_opt_send_value!($type_name, $seq, String),
            "Array" => de_some_send_value!($type_name, $seq, Array),
            "Bitmask" => de_some_send_value!($type_name, $seq, Bitmask),
            "Date" => {
                // Need to deserialize as `date_time_serde::Date` new type
                // See comment in `date_time_serde.rs`
                de_send_value!(
                    "Value",
                    $type_name,
                    $seq,
                    Option<date_time_serde::Date>,
                    Date
                )
            }
            "DateTime" => de_opt_send_value!($type_name, $seq, DateTime),
            "Fraction" => de_some_send_value!($type_name, $seq, Fraction),
            "FractionRange" => de_some_send_value!($type_name, $seq, FractionRange),
            "IntRange<i32>" => de_some_send_value!($type_name, $seq, IntRange<i32>),
            "IntRange<i64>" => de_some_send_value!($type_name, $seq, IntRange<i64>),
            "Sample" => de_opt_send_value!($type_name, $seq, Sample),
            "Buffer" => de_opt_send_value!($type_name, $seq, Buffer),
            _ => return Err(
                de::Error::custom(
                    format!(
                        "unimplemented deserialization for `Value` with type `{}`",
                        $type_name,
                    ),
                )
            ),
        }
    );
);

struct SendValueVisitor;
impl<'de> Visitor<'de> for SendValueVisitor {
    type Value = SendValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a tuple: (name, value)")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let type_name = seq
            .next_element::<String>()?
            .ok_or_else(|| de::Error::custom("Expected a value for `Value` type"))?;
        let send_value = de_send_value!(type_name, seq)?
            .ok_or_else(|| de::Error::custom("Expected a value for `Value`"))?;
        Ok(send_value)
    }
}

impl<'de> Deserialize<'de> for SendValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        deserializer.deserialize_tuple(2, SendValueVisitor {})
    }
}

macro_rules! impl_de_send_value_collection (
    ($t:ident) => {
        impl<'a, 'de> Deserialize<'de> for $t<'a> {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                skip_assert_initialized!();
                let send_value_vec = Vec::<SendValue>::deserialize(deserializer)?;
                Ok($t::from_owned(unsafe{
                    mem::transmute::<Vec<SendValue>, Vec<glib::SendValue>>(send_value_vec)
                }))
            }
        }
    }
);

impl_de_send_value_collection!(Array);
impl_de_send_value_collection!(List);

#[cfg(test)]
mod tests {
    extern crate ron;
    extern crate serde_json;

    use Array;
    use Bitmask;
    use DateTime;
    use Fraction;
    use FractionRange;
    use IntRange;
    use List;

    use glib::{Date, DateMonth};

    #[test]
    fn test_serialize_simple() {
        ::init().unwrap();

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        // Fraction
        let fraction = Fraction::new(1, 3);

        let res = ron::ser::to_string_pretty(&fraction, pretty_config.clone());
        assert_eq!(Ok("(1, 3)".to_owned()), res);

        let res = serde_json::to_string(&fraction).unwrap();
        assert_eq!("[1,3]".to_owned(), res);

        // FractionRange
        let fraction_range = FractionRange::new(Fraction::new(1, 3), Fraction::new(1, 2));

        let res = ron::ser::to_string_pretty(&fraction_range, pretty_config.clone());
        assert_eq!(Ok("(    min: (1, 3),    max: (1, 2),)".to_owned()), res);

        let res = serde_json::to_string(&fraction_range).unwrap();
        assert_eq!(r#"{"min":[1,3],"max":[1,2]}"#.to_owned(), res);

        // IntRange
        let int_range = IntRange::<i32>::with_step(0, 42, 21);
        let res = ron::ser::to_string_pretty(&int_range, pretty_config.clone());
        assert_eq!(Ok("(    min: 0,    max: 42,    step: 21,)".to_owned()), res,);

        let res = serde_json::to_string(&int_range).unwrap();
        assert_eq!(r#"{"min":0,"max":42,"step":21}"#.to_owned(), res);

        // Bitmask
        let bitmask = Bitmask::new(1024 + 128 + 32);

        let res = ron::ser::to_string_pretty(&bitmask, pretty_config);
        assert_eq!(Ok("(1184)".to_owned()), res);

        let res = serde_json::to_string(&bitmask).unwrap();
        assert_eq!("1184".to_owned(), res);
    }

    #[test]
    fn test_serialize_collections() {
        use glib::value::ToValue;

        use Fraction;
        use List;

        ::init().unwrap();

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        // Array
        let value_13 = Fraction::new(1, 3).to_value();
        let send_value_13 = value_13.try_into_send_value::<Fraction>().unwrap();

        let value_12 = Fraction::new(1, 2).to_value();
        let send_value_12 = value_12.try_into_send_value::<Fraction>().unwrap();

        let value_str = "test str".to_value();
        let send_value_str = value_str.try_into_send_value::<String>().unwrap();

        let str_none: Option<&str> = None;
        let value_str_none = str_none.to_value();
        let send_value_str_none = value_str_none.try_into_send_value::<String>().unwrap();

        let value_date = Date::new_dmy(19, DateMonth::August, 2019).to_value();
        let send_value_date = value_date.try_into_send_value::<Date>().unwrap();

        let date_none: Option<Date> = None;
        let value_date_none = date_none.to_value();
        let send_value_date_none = value_date_none.try_into_send_value::<Date>().unwrap();

        let array = Array::new(&[
            &send_value_13,
            &send_value_12,
            &send_value_str,
            &send_value_str_none,
            &send_value_date,
            &send_value_date_none,
        ]);

        let res = ron::ser::to_string_pretty(&array, pretty_config.clone());
        assert_eq!(
            Ok(concat!(
                r#"["#,
                r#"    ("Fraction", (1, 3)),"#,
                r#"    ("Fraction", (1, 2)),"#,
                r#"    ("String", Some("test str")),"#,
                r#"    ("String", None),"#,
                r#"    ("Date", Some(YMD(2019, 8, 19))),"#,
                r#"    ("Date", None),"#,
                r#"]"#
            )
            .to_owned()),
            res,
        );

        let res = serde_json::to_string(&array).unwrap();
        assert_eq!(
            r#"[["Fraction",[1,3]],["Fraction",[1,2]],["String","test str"],["String",null],["Date",{"YMD":[2019,8,19]}],["Date",null]]"#
                .to_owned(),
            res
        );

        // List
        let value_12 = Fraction::new(1, 2).to_value();
        let send_value_12 = value_12.try_into_send_value::<Fraction>().unwrap();

        let value_str = "test str".to_value();
        let send_value_str = value_str.try_into_send_value::<String>().unwrap();

        let str_none: Option<&str> = None;
        let value_str_none = str_none.to_value();
        let send_value_str_none = value_str_none.try_into_send_value::<String>().unwrap();

        let value_date_time = DateTime::new(2f32, 2019, 8, 19, 13, 34, 42f64)
            .unwrap()
            .to_value();
        let send_value_date_time = value_date_time.try_into_send_value::<DateTime>().unwrap();

        let date_time_none: Option<DateTime> = None;
        let value_date_time_none = date_time_none.to_value();
        let send_value_date_time_none = value_date_time_none
            .try_into_send_value::<DateTime>()
            .unwrap();

        let list = List::new(&[
            &send_value_12,
            &send_value_str,
            &send_value_str_none,
            &send_value_date_time,
            &send_value_date_time_none,
        ]);

        let res = ron::ser::to_string_pretty(&list, pretty_config);
        assert_eq!(
            Ok(concat!(
                r#"["#,
                r#"    ("Fraction", (1, 2)),"#,
                r#"    ("String", Some("test str")),"#,
                r#"    ("String", None),"#,
                r#"    ("DateTime", Some(YMDhmsTz(2019, 8, 19, 13, 34, 42, 2))),"#,
                r#"    ("DateTime", None),"#,
                r#"]"#
            )
            .to_owned()),
            res,
        );
    }

    #[test]
    fn test_deserialize_simple() {
        ::init().unwrap();

        // Fraction
        let fraction_ron = "(1, 3)";
        let fraction: Fraction = ron::de::from_str(fraction_ron).unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &3);

        let fraction_json = "[1,3]";
        let fraction: Fraction = serde_json::from_str(fraction_json).unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &3);

        // FractionRange
        let fraction_range_ron = "(min: (1, 3), max: (1, 2))";
        let fraction_range: FractionRange = ron::de::from_str(fraction_range_ron).unwrap();
        assert_eq!(fraction_range.min().0.denom(), &3);
        assert_eq!(fraction_range.max().0.denom(), &2);

        let fraction_range_json = r#"{"min":[1,3],"max":[1,2]}"#;
        let fraction_range: FractionRange = serde_json::from_str(fraction_range_json).unwrap();
        assert_eq!(fraction_range.min().0.denom(), &3);
        assert_eq!(fraction_range.max().0.denom(), &2);

        // IntRange
        let int_range_ron = "(min: 0, max: 42, step: 21)";
        let int_range: IntRange<i32> = ron::de::from_str(int_range_ron).unwrap();
        assert_eq!(int_range.min(), 0);
        assert_eq!(int_range.max(), 42);
        assert_eq!(int_range.step(), 21);

        let int_range_json = r#"{"min":0,"max":42,"step":21}"#;
        let int_range: IntRange<i32> = serde_json::from_str(int_range_json).unwrap();
        assert_eq!(int_range.min(), 0);
        assert_eq!(int_range.max(), 42);
        assert_eq!(int_range.step(), 21);

        // Bitmask
        let bitmask_ref = Bitmask::new(1024 + 128 + 32);

        let bitmask_ron = "(1184)";
        let bitmask: Bitmask = ron::de::from_str(bitmask_ron).unwrap();
        assert_eq!(bitmask_ref, bitmask);

        let bitmask_json = "1184";
        let bitmask: Bitmask = serde_json::from_str(bitmask_json).unwrap();
        assert_eq!(bitmask_ref, bitmask);
    }

    #[test]
    fn test_serde_roundtrip_simple() {
        ::init().unwrap();

        // Fraction
        let fraction = Fraction::new(1, 3);
        let fraction_ser = ron::ser::to_string(&fraction).unwrap();
        let fraction_de: Fraction = ron::de::from_str(fraction_ser.as_str()).unwrap();
        assert_eq!(fraction_de.0.numer(), fraction.0.numer());
        assert_eq!(fraction_de.0.denom(), fraction.0.denom());

        // FractionRange
        let fraction_range = FractionRange::new(Fraction::new(1, 3), Fraction::new(1, 2));
        let fraction_range_ser = ron::ser::to_string(&fraction_range).unwrap();
        let fraction_range_de: FractionRange =
            ron::de::from_str(fraction_range_ser.as_str()).unwrap();
        assert_eq!(
            fraction_range_de.min().0.denom(),
            fraction_range.min().0.denom()
        );
        assert_eq!(
            fraction_range_de.max().0.denom(),
            fraction_range.max().0.denom()
        );

        // IntRange
        let int_range = IntRange::<i32>::with_step(0, 42, 21);
        let int_range_ser = ron::ser::to_string(&int_range).unwrap();
        let int_range_de: IntRange<i32> = ron::de::from_str(int_range_ser.as_str()).unwrap();
        assert_eq!(int_range_de.min(), int_range.min());
        assert_eq!(int_range_de.max(), int_range.max());
        assert_eq!(int_range_de.step(), int_range.step());

        // Bitmask
        let bitmask = Bitmask::new(1024 + 128 + 32);
        let bitmask_ser = ron::ser::to_string(&bitmask).unwrap();
        let bitmask_de: Bitmask = ron::de::from_str(bitmask_ser.as_str()).unwrap();
        assert_eq!(bitmask_de, bitmask);
    }

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn test_deserialize_collections() {
        ::init().unwrap();

        // Array
        let array_ron = r#"[
                ("Fraction", (1, 3)),
                ("Fraction", (1, 2)),
                ("String", Some("test str")),
                ("String", None),
                ("Date", Some(YMD(2019, 8, 19))),
                ("Date", None),
            ]"#;
        let array: Array = ron::de::from_str(array_ron).unwrap();
        let slice = array.as_slice();
        assert_eq!(6, slice.len());

        let fraction = slice[0].get::<Fraction>().expect("slice[0]").unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &3);

        let fraction = slice[1].get::<Fraction>().expect("slice[1]").unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &2);

        assert_eq!(
            "test str".to_owned(),
            slice[2].get::<String>().expect("slice[2]").unwrap()
        );

        assert!(slice[3].get::<String>().expect("slice[3]").is_none());

        assert_eq!(
            Date::new_dmy(19, DateMonth::August, 2019),
            slice[4].get::<Date>().expect("slice[4]").unwrap()
        );

        assert!(slice[5].get::<Date>().expect("slice[5]").is_none());

        let array_json = r#"[["Fraction",[1,3]],["Fraction",[1,2]],["String","test str"],["String",null],["Date",{"YMD":[2019,8,19]}],["Date",null]]"#;
        let array: Array = serde_json::from_str(array_json).unwrap();
        let slice = array.as_slice();
        assert_eq!(6, slice.len());

        let fraction = slice[0].get::<Fraction>().expect("slice[0]").unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &3);

        let fraction = slice[1].get::<Fraction>().expect("slice[1]").unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &2);

        assert_eq!(
            "test str".to_owned(),
            slice[2].get::<String>().expect("slice[2]").unwrap()
        );

        assert!(slice[3].get::<String>().expect("slice[3]").is_none());

        assert_eq!(
            Date::new_dmy(19, DateMonth::August, 2019),
            slice[4].get::<Date>().expect("slice[4]").unwrap()
        );

        assert!(slice[5].get::<Date>().expect("slice[5]").is_none());

        // List
        let list_ron = r#"[
                ("Fraction", (1, 2)),
                ("String", Some("test str")),
                ("String", None),
                ("DateTime", Some(YMDhmsTz(2019, 8, 19, 13, 34, 42, 2))),
                ("DateTime", None),
            ]"#;
        let list: List = ron::de::from_str(list_ron).unwrap();
        let slice = list.as_slice();
        assert_eq!(5, slice.len());

        let fraction = slice[0].get::<Fraction>().expect("slice[0]").unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &2);

        assert_eq!(
            "test str".to_owned(),
            slice[1].get::<String>().expect("slice[1]").unwrap()
        );

        assert!(slice[2].get::<String>().expect("slice[2]").is_none());

        assert_eq!(
            DateTime::new(2f32, 2019, 8, 19, 13, 34, 42f64).unwrap(),
            slice[3].get::<DateTime>().expect("slice[3]").unwrap()
        );

        assert!(slice[4].get::<DateTime>().expect("slice[4]").is_none());
    }

    #[test]
    fn test_serde_roundtrip_collection() {
        use glib::value::ToValue;

        ::init().unwrap();

        // Array
        let value_13 = Fraction::new(1, 3).to_value();
        let send_value_13 = value_13.try_into_send_value::<Fraction>().unwrap();
        let value_12 = Fraction::new(1, 2).to_value();
        let send_value_12 = value_12.try_into_send_value::<Fraction>().unwrap();
        let value_str = "test str".to_value();
        let send_value_str = value_str.try_into_send_value::<String>().unwrap();
        let str_none: Option<&str> = None;
        let value_str_none = str_none.to_value();
        let send_value_str_none = value_str_none.try_into_send_value::<String>().unwrap();
        let value_date = Date::new_dmy(19, DateMonth::August, 2019).to_value();
        let send_value_date = value_date.try_into_send_value::<Date>().unwrap();
        let date_none: Option<Date> = None;
        let value_date_none = date_none.to_value();
        let send_value_date_none = value_date_none.try_into_send_value::<Date>().unwrap();
        let array = Array::new(&[
            &send_value_13,
            &send_value_12,
            &send_value_str,
            &send_value_str_none,
            &send_value_date,
            &send_value_date_none,
        ]);
        let array_ser = ron::ser::to_string(&array).unwrap();

        let array_de: Array = ron::de::from_str(array_ser.as_str()).unwrap();
        let slice_de = array_de.as_slice();
        let slice = array.as_slice();
        assert_eq!(slice_de.len(), slice.len());

        let fraction_de = slice_de[0].get::<Fraction>().expect("slice_de[0]").unwrap();
        let fraction = slice[0].get::<Fraction>().expect("slice[0]").unwrap();
        assert_eq!(fraction_de.0.numer(), fraction.0.numer());
        assert_eq!(fraction_de.0.denom(), fraction.0.denom());

        let fraction_de = slice_de[1].get::<Fraction>().expect("slice_de[1]").unwrap();
        let fraction = slice[1].get::<Fraction>().expect("slice[1]").unwrap();
        assert_eq!(fraction_de.0.numer(), fraction.0.numer());
        assert_eq!(fraction.0.denom(), fraction.0.denom());

        assert_eq!(
            slice_de[2].get::<String>().expect("slice_de[2]").unwrap(),
            slice[2].get::<String>().expect("slice[2]").unwrap()
        );

        assert!(slice[3].get::<String>().expect("slice[3]").is_none());

        assert_eq!(
            slice_de[4].get::<Date>().expect("slice_de[4]").unwrap(),
            slice[4].get::<Date>().expect("slice[4]").unwrap()
        );

        assert!(slice[5].get::<Date>().expect("slice[5]").is_none());

        // List
        let value_12 = Fraction::new(1, 2).to_value();
        let send_value_12 = value_12.try_into_send_value::<Fraction>().unwrap();
        let value_str = "test str".to_value();
        let send_value_str = value_str.try_into_send_value::<String>().unwrap();
        let str_none: Option<&str> = None;
        let value_str_none = str_none.to_value();
        let send_value_str_none = value_str_none.try_into_send_value::<String>().unwrap();
        let value_date_time = DateTime::new(2f32, 2019, 8, 19, 13, 34, 42f64)
            .unwrap()
            .to_value();
        let send_value_date_time = value_date_time.try_into_send_value::<DateTime>().unwrap();
        let date_time_none: Option<DateTime> = None;
        let value_date_time_none = date_time_none.to_value();
        let send_value_date_time_none = value_date_time_none
            .try_into_send_value::<DateTime>()
            .unwrap();
        let list = List::new(&[
            &send_value_12,
            &send_value_str,
            &send_value_str_none,
            &send_value_date_time,
            &send_value_date_time_none,
        ]);
        let list_ser = ron::ser::to_string(&list).unwrap();

        let list_de: List = ron::de::from_str(list_ser.as_str()).unwrap();
        let slice_de = list_de.as_slice();
        let slice = list.as_slice();
        assert_eq!(slice_de.len(), slice.len());

        let fraction_de = slice_de[0].get::<Fraction>().expect("slice_de[0]").unwrap();
        let fraction = slice[0].get::<Fraction>().expect("slice[0]").unwrap();
        assert_eq!(fraction_de.0.numer(), fraction.0.numer());
        assert_eq!(fraction_de.0.denom(), fraction.0.denom());

        assert_eq!(
            slice_de[1].get::<String>().expect("slice_de[1]").unwrap(),
            slice[1].get::<String>().expect("slice[1]").unwrap()
        );

        assert!(slice[2].get::<String>().expect("slice[2]").is_none());

        assert_eq!(
            slice_de[3].get::<DateTime>().expect("slice_de[3]").unwrap(),
            slice[3].get::<DateTime>().expect("slice[3]").unwrap()
        );

        assert!(slice[4].get::<DateTime>().expect("slice[4]").is_none());
    }
}
