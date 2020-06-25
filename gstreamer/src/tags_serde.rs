// Copyright (C) 2018 François Laignel <fengalin@free.fr>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib;
use glib::translate::{from_glib, ToGlibPtr};
use glib::{Date, SendValue, ToValue};
use gst_sys;

use serde::de;
use serde::de::{Deserialize, DeserializeSeed, Deserializer, SeqAccess, Visitor};
use serde::ser;
use serde::ser::{Serialize, SerializeSeq, SerializeStruct, SerializeTuple, Serializer};

use std::cell::RefCell;
use std::cmp;
use std::fmt;
use std::rc::Rc;

use date_time_serde;
use tags::{GenericTagIter, TagList, TagListRef};
use value_serde::{DATE_OTHER_TYPE_ID, DATE_TIME_OTHER_TYPE_ID, SAMPLE_OTHER_TYPE_ID};
use DateTime;
use Sample;
use TagMergeMode;
use TagScope;

macro_rules! ser_some_tag (
    ($value:ident, $seq:ident, $t:ty) => (
        ser_some_value!($value, $t, |_, value| {
            $seq.serialize_element(&value)
        })
    );
);
macro_rules! ser_opt_tag (
    ($value:ident, $seq:ident, $t:ty) => (
        ser_opt_value!($value, $t, |_, value| {
            $seq.serialize_element(&value)
        })
    );
);

// Note: unlike `Value`s, `Tag`s with  optional `Type` `String` & `Date` values are guarenteed
// to be Non-null and non-empty in the C API. See:
// https://gitlab.freedesktop.org/gstreamer/gstreamer/blob/d90d771a9a512381315f7694c3a50b152035f3cb/gst/gststructure.c#L810-853

// serialize trait is only available for `&self`, but we need to mutate the iterator
struct TagValuesSer<'a>(Rc<RefCell<GenericTagIter<'a>>>);
impl<'a> TagValuesSer<'a> {
    fn from(tags_ser: &TagsSer<'a>) -> Self {
        skip_assert_initialized!();
        TagValuesSer(Rc::clone(&tags_ser.1))
    }
}

impl<'a> Serialize for TagValuesSer<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use std::ops::DerefMut;

        let mut tag_iter = self.0.borrow_mut();
        let mut seq = serializer.serialize_seq(tag_iter.size_hint().1)?;
        for value in tag_iter.deref_mut() {
            match value.type_() {
                glib::Type::F64 => ser_some_tag!(value, seq, f64),
                glib::Type::String => {
                    // See above comment about `Tag`s with `String` values
                    ser_opt_value!(value, String, |_, value: Option<String>| {
                        seq.serialize_element(&value.expect("String tag ser"))
                    })
                }
                glib::Type::U32 => ser_some_tag!(value, seq, u32),
                glib::Type::U64 => ser_some_tag!(value, seq, u64),
                glib::Type::Other(type_id) => {
                    if *DATE_OTHER_TYPE_ID == type_id {
                        // See above comment about `Tag`s with `Date` values
                        ser_opt_value!(value, Date, |_, value: Option<Date>| {
                            // Need to wrap the `glib::Date` in new type `date_time_serde::Date` first
                            // See comment in `date_time_serde.rs`
                            seq.serialize_element(&date_time_serde::Date::from(
                                value.expect("Date tag ser"),
                            ))
                        })
                    } else if *DATE_TIME_OTHER_TYPE_ID == type_id {
                        ser_opt_tag!(value, seq, DateTime)
                    } else if *SAMPLE_OTHER_TYPE_ID == type_id {
                        ser_opt_tag!(value, seq, Sample)
                    } else {
                        Err(ser::Error::custom(format!(
                            "unimplemented `Tag` serialization for type {}",
                            glib::Type::Other(type_id),
                        )))
                    }
                }
                type_ => Err(ser::Error::custom(format!(
                    "unimplemented `Tag` serialization for type {}",
                    type_
                ))),
            }?;
        }
        seq.end()
    }
}

struct TagsSer<'a>(&'a str, Rc<RefCell<GenericTagIter<'a>>>);
impl<'a> TagsSer<'a> {
    fn new(name: &'a str, tag_iter: GenericTagIter<'a>) -> Self {
        skip_assert_initialized!();
        TagsSer(name, Rc::new(RefCell::new(tag_iter)))
    }
}

impl<'a> Serialize for TagsSer<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut tup = serializer.serialize_tuple(2)?;
        tup.serialize_element(self.0)?;
        tup.serialize_element(&TagValuesSer::from(&self))?;
        tup.end()
    }
}

struct TagListSer<'a>(&'a TagListRef);
impl<'a> Serialize for TagListSer<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let tag_count = self.0.n_tags();
        match tag_count.cmp(&0) {
            cmp::Ordering::Greater => {
                let mut seq = serializer.serialize_seq(Some(tag_count as usize))?;
                let tag_list_iter = self.0.iter_generic();
                for (tag_name, tag_iter) in tag_list_iter {
                    seq.serialize_element(&TagsSer::new(tag_name, tag_iter))?;
                }
                seq.end()
            }
            cmp::Ordering::Equal => {
                let seq = serializer.serialize_seq(None)?;
                seq.end()
            }
            cmp::Ordering::Less => Err(ser::Error::custom("tag count < 0")),
        }
    }
}

impl Serialize for TagListRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut tag_list = serializer.serialize_struct("TagList", 3)?;
        tag_list.serialize_field("scope", &self.get_scope())?;
        tag_list.serialize_field("tags", &TagListSer(self))?;
        tag_list.end()
    }
}

impl Serialize for TagList {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_ref().serialize(serializer)
    }
}

macro_rules! de_some_tag(
    ($tag_name:expr, $seq:expr, $t:ty) => (
        de_some_send_value!("Tag", $tag_name, $seq, $t)
    );
);
macro_rules! de_opt_tag(
    ($tag_name:expr, $seq:expr, $t:ty) => (
        de_opt_send_value!("Tag", $tag_name, $seq, $t)
    );
);

struct TagValues<'a>(&'a str, &'a mut TagListRef);

struct TagValuesVisitor<'a>(&'a str, &'a mut TagListRef);
impl<'de, 'a> Visitor<'de> for TagValuesVisitor<'a> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence of `Tag` values with the same type")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
        let tag_type: glib::Type = unsafe {
            let tag_name = self.0.to_glib_none();
            from_glib(gst_sys::gst_tag_get_type(tag_name.0))
        };

        loop {
            let tag_value = match tag_type {
                glib::Type::F64 => de_some_tag!(self.0, seq, f64),
                glib::Type::String => {
                    // See comment above `TagValuesSer` definition about `Tag`s with `String` values
                    de_some_tag!(self.0, seq, String)
                }
                glib::Type::U32 => de_some_tag!(self.0, seq, u32),
                glib::Type::U64 => de_some_tag!(self.0, seq, u64),
                glib::Type::Other(type_id) => {
                    if *DATE_OTHER_TYPE_ID == type_id {
                        // See comment above `TagValuesSer` definition about `Tag`s with `Date` values
                        // Need to deserialize as `date_time_serde::Date` new type
                        // See comment in `date_time_serde.rs`
                        de_send_value!("Tag", self.0, seq, date_time_serde::Date, Date)
                    } else if *DATE_TIME_OTHER_TYPE_ID == type_id {
                        de_opt_tag!(self.0, seq, DateTime)
                    } else if *SAMPLE_OTHER_TYPE_ID == type_id {
                        de_opt_tag!(self.0, seq, Sample)
                    } else {
                        return Err(de::Error::custom(format!(
                            "unimplemented deserialization for `Tag` {} with type `{}`",
                            self.0,
                            glib::Type::Other(type_id),
                        )));
                    }
                }
                type_ => {
                    return Err(de::Error::custom(format!(
                        "unimplemented deserialization for `Tag` {} with type `{}`",
                        self.0, type_,
                    )));
                }
            }?;

            match tag_value {
                Some(tag_value) => self
                    .1
                    .add_generic(self.0, &tag_value, TagMergeMode::Append)
                    .map_err(|_| {
                        de::Error::custom(format!("wrong value type for `Tag` {}", self.0))
                    })?,
                None => break,
            }
        }

        Ok(())
    }
}

impl<'de, 'a> DeserializeSeed<'de> for TagValues<'a> {
    type Value = ();

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        skip_assert_initialized!();
        deserializer.deserialize_seq(TagValuesVisitor(self.0, self.1))
    }
}

struct TagValuesTuple<'a>(&'a mut TagListRef);

struct TagValuesTupleVisitor<'a>(&'a mut TagListRef);
impl<'de, 'a> Visitor<'de> for TagValuesTupleVisitor<'a> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .write_str("a tuple (`Tag` name: `String`, seq. of `Tag` values with the same type)")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
        let name = seq
            .next_element::<String>()
            .map_err(|err| de::Error::custom(format!("Error reading Tag name. {:?}", err)))?
            .ok_or_else(|| de::Error::custom("Expected a name for the `Tag` name"))?;
        seq.next_element_seed(TagValues(name.as_str(), self.0))?
            .ok_or_else(|| de::Error::custom("Expected a seq of values for the `Tag`"))
    }
}

impl<'de, 'a> DeserializeSeed<'de> for TagValuesTuple<'a> {
    type Value = ();

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        skip_assert_initialized!();
        deserializer.deserialize_tuple(2, TagValuesTupleVisitor(self.0))
    }
}

struct TagsDe(TagList);

struct TagsVisitor;
impl<'de> Visitor<'de> for TagsVisitor {
    type Value = TagsDe;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence of `Tag`s")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut tag_list = TagList::new();
        {
            let tag_list = tag_list.get_mut().unwrap();
            while seq.next_element_seed(TagValuesTuple(tag_list))?.is_some() {
                // tags are added in the dedicated deserializers
            }
        }
        Ok(TagsDe(tag_list))
    }
}

impl<'de> Deserialize<'de> for TagsDe {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        deserializer.deserialize_seq(TagsVisitor)
    }
}

#[derive(Deserialize)]
struct TagListDe {
    scope: TagScope,
    tags: TagsDe,
}

impl From<TagListDe> for TagList {
    fn from(tag_list_de: TagListDe) -> Self {
        skip_assert_initialized!();
        let mut tag_list = tag_list_de.tags.0;
        tag_list.get_mut().unwrap().set_scope(tag_list_de.scope);

        tag_list
    }
}

impl<'de> Deserialize<'de> for TagList {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        TagListDe::deserialize(deserializer).map(|tag_list_de| tag_list_de.into())
    }
}

#[cfg(test)]
mod tests {
    extern crate ron;

    use tags::*;
    use Buffer;
    use Sample;
    use TagMergeMode;
    use TagScope;

    #[test]
    fn test_serialize() {
        ::init().unwrap();

        let mut tags = TagList::new();
        assert_eq!(tags.to_string(), "taglist;");
        {
            let tags = tags.get_mut().unwrap();
            tags.add::<Title>(&"a title", TagMergeMode::Append); // String
            tags.add::<Title>(&"another title", TagMergeMode::Append); // String
            tags.add::<Duration>(&(::SECOND * 120), TagMergeMode::Append); // u64
            tags.add::<Bitrate>(&96_000, TagMergeMode::Append); // u32
            tags.add::<TrackGain>(&1f64, TagMergeMode::Append); // f64
            tags.add::<Date>(
                &glib::Date::new_dmy(28, glib::DateMonth::May, 2018),
                TagMergeMode::Append,
            );
            tags.add::<DateTime>(
                &::DateTime::new_ymd(2018, 5, 28).unwrap(),
                TagMergeMode::Append,
            );

            let sample = {
                let mut buffer = Buffer::from_slice(vec![1, 2, 3, 4]);
                {
                    let buffer = buffer.get_mut().unwrap();
                    buffer.set_offset(0);
                    buffer.set_offset_end(0);
                }
                Sample::builder().buffer(&buffer).build()
            };
            tags.add::<Image>(&sample, TagMergeMode::Append); // Sample
        }

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        let res = ron::ser::to_string_pretty(&tags, pretty_config);
        assert_eq!(
            Ok(concat!(
                r#"("#,
                r#"    scope: Stream,"#,
                r#"    tags: ["#,
                r#"        ("title", ["#,
                r#"            "a title","#,
                r#"            "another title","#,
                r#"        ]),"#,
                r#"        ("duration", ["#,
                r#"            120000000000,"#,
                r#"        ]),"#,
                r#"        ("bitrate", ["#,
                r#"            96000,"#,
                r#"        ]),"#,
                r#"        ("replaygain-track-gain", ["#,
                r#"            1,"#,
                r#"        ]),"#,
                r#"        ("date", ["#,
                r#"            YMD(2018, 5, 28),"#,
                r#"        ]),"#,
                r#"        ("datetime", ["#,
                r#"            Some(YMD(2018, 5, 28)),"#,
                r#"        ]),"#,
                r#"        ("image", ["#,
                r#"            Some(("#,
                r#"                buffer: Some(("#,
                r#"                    pts: None,"#,
                r#"                    dts: None,"#,
                r#"                    duration: None,"#,
                r#"                    offset: 0,"#,
                r#"                    offset_end: 0,"#,
                r#"                    flags: ("#,
                r#"                        bits: 0,"#,
                r#"                    ),"#,
                r#"                    buffer: "AQIDBA==","#,
                r#"                )),"#,
                r#"                buffer_list: None,"#,
                r#"                caps: None,"#,
                r#"                segment: Some(("#,
                r#"                    flags: ("#,
                r#"                        bits: 0,"#,
                r#"                    ),"#,
                r#"                    rate: 1,"#,
                r#"                    applied_rate: 1,"#,
                r#"                    format: Time,"#,
                r#"                    base: 0,"#,
                r#"                    offset: 0,"#,
                r#"                    start: 0,"#,
                r#"                    stop: -1,"#,
                r#"                    time: 0,"#,
                r#"                    position: 0,"#,
                r#"                    duration: -1,"#,
                r#"                )),"#,
                r#"                info: None,"#,
                r#"            )),"#,
                r#"        ]),"#,
                r#"    ],"#,
                r#")"#,
            )
            .to_owned()),
            res,
        );
    }

    #[test]
    fn test_deserialize() {
        extern crate serde_json;

        ::init().unwrap();

        let tag_list_ron = r#"
            (
                scope: Global,
                tags: [
                    ("title", [
                        "a title",
                        "another title",
                    ]),
                    ("duration", [120000000000]),
                    ("bitrate", [96000]),
                    ("replaygain-track-gain", [1]),
                    ("date", [
                        YMD(2018, 5, 28),
                    ]),
                    ("datetime", [
                        Some(YMD(2018, 5, 28)),
                    ]),
                    ("image", [
                        Some((
                            buffer: Some((
                                pts: None,
                                dts: None,
                                duration: None,
                                offset: 0,
                                offset_end: 0,
                                flags: (
                                    bits: 0,
                                ),
                                buffer: "AQIDBA==",
                            )),
                            buffer_list: None,
                            caps: None,
                            segment: None,
                            info: None,
                        )),
                    ])
                ],
            )
        "#;
        let tags: TagList = ron::de::from_str(tag_list_ron).unwrap();
        assert_eq!(tags.get_scope(), TagScope::Global);

        assert_eq!(tags.get_index::<Title>(0).unwrap().get(), Some("a title"));
        assert_eq!(
            tags.get_index::<Title>(1).unwrap().get(),
            Some("another title")
        );
        assert_eq!(
            tags.get_index::<Duration>(0).unwrap().get_some(),
            ::SECOND * 120
        );
        assert_eq!(tags.get_index::<Bitrate>(0).unwrap().get_some(), 96_000);
        assert!(
            (tags.get_index::<TrackGain>(0).unwrap().get_some() - 1f64).abs() < std::f64::EPSILON
        );
        assert_eq!(
            tags.get_index::<Date>(0).unwrap().get().unwrap(),
            glib::Date::new_dmy(28, glib::DateMonth::May, 2018)
        );
        assert_eq!(
            tags.get_index::<DateTime>(0).unwrap().get().unwrap(),
            ::DateTime::new_ymd(2018, 5, 28).unwrap()
        );
        let sample = tags.get_index::<Image>(0).unwrap().get().unwrap();
        let buffer = sample.get_buffer().unwrap();
        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());
        }

        let tag_json = r#"
            {
                "scope":"Global",
                "tags":[
                    ["title", ["a title", "another title"]],
                    ["duration", [120000000000]],
                    ["bitrate", [96000]],
                    ["replaygain-track-gain", [1.0]],
                    ["date",[{"YMD":[2018,5,28]}]],
                    ["datetime",[{"YMD":[2018,5,28]}]],
                    ["image",[{"buffer":{"pts":null,"dts":null,"duration":null,"offset":0,"offset_end":0,"flags":{"bits":0},"buffer":[1,2,3,4]},"buffer_list":null,"caps":null,"segment":null,"info":null}]]
                ]
            }
        "#;
        let tags: TagList = serde_json::from_str(tag_json).unwrap();
        assert_eq!(tags.get_scope(), TagScope::Global);

        assert_eq!(tags.get_index::<Title>(0).unwrap().get(), Some("a title"));
        assert_eq!(
            tags.get_index::<Title>(1).unwrap().get(),
            Some("another title")
        );
        assert_eq!(tags.get_index::<Bitrate>(0).unwrap().get_some(), 96_000);
        assert!(
            (tags.get_index::<TrackGain>(0).unwrap().get_some() - 1f64).abs() < std::f64::EPSILON
        );
        assert_eq!(
            tags.get_index::<Date>(0).unwrap().get().unwrap(),
            glib::Date::new_dmy(28, glib::DateMonth::May, 2018)
        );
        assert_eq!(
            tags.get_index::<DateTime>(0).unwrap().get().unwrap(),
            ::DateTime::new_ymd(2018, 5, 28).unwrap()
        );
        let sample = tags.get_index::<Image>(0).unwrap().get().unwrap();
        let buffer = sample.get_buffer().unwrap();
        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());
        }
    }

    #[test]
    fn test_serde_roundtrip() {
        ::init().unwrap();

        let mut tags = TagList::new();
        assert_eq!(tags.to_string(), "taglist;");
        {
            let tags = tags.get_mut().unwrap();
            tags.set_scope(TagScope::Global);
            tags.add::<Title>(&"a title", TagMergeMode::Append); // String
            tags.add::<Title>(&"another title", TagMergeMode::Append); // String
            tags.add::<Duration>(&(::SECOND * 120), TagMergeMode::Append); // u64
            tags.add::<Bitrate>(&96_000, TagMergeMode::Append); // u32
            tags.add::<TrackGain>(&1f64, TagMergeMode::Append); // f64
            tags.add::<Date>(
                &glib::Date::new_dmy(28, glib::DateMonth::May, 2018),
                TagMergeMode::Append,
            );
            tags.add::<DateTime>(
                &::DateTime::new_ymd(2018, 5, 28).unwrap(),
                TagMergeMode::Append,
            );

            let sample = {
                let mut buffer = Buffer::from_slice(vec![1, 2, 3, 4]);
                {
                    let buffer = buffer.get_mut().unwrap();
                    buffer.set_offset(0);
                    buffer.set_offset_end(0);
                }
                Sample::builder().buffer(&buffer).build()
            };
            tags.add::<Image>(&sample, TagMergeMode::Append); // Sample
        }
        let tags_ser = ron::ser::to_string(&tags).unwrap();

        let tags_de: TagList = ron::de::from_str(tags_ser.as_str()).unwrap();
        assert_eq!(tags_de.get_scope(), TagScope::Global);

        assert_eq!(
            tags_de.get_index::<Title>(0).unwrap().get(),
            tags.get_index::<Title>(0).unwrap().get(),
        );
        assert_eq!(
            tags_de.get_index::<Title>(1).unwrap().get(),
            tags.get_index::<Title>(1).unwrap().get(),
        );
        assert_eq!(
            tags_de.get_index::<Duration>(0).unwrap().get_some(),
            tags.get_index::<Duration>(0).unwrap().get_some(),
        );
        assert_eq!(
            tags_de.get_index::<Bitrate>(0).unwrap().get_some(),
            tags.get_index::<Bitrate>(0).unwrap().get_some(),
        );
        assert!(
            (tags_de.get_index::<TrackGain>(0).unwrap().get_some()
                - tags.get_index::<TrackGain>(0).unwrap().get_some())
            .abs()
                < std::f64::EPSILON
        );
        assert_eq!(
            tags_de.get_index::<Date>(0).unwrap().get(),
            tags.get_index::<Date>(0).unwrap().get(),
        );
        assert_eq!(
            tags.get_index::<DateTime>(0).unwrap().get().unwrap(),
            ::DateTime::new_ymd(2018, 5, 28).unwrap()
        );
        let sample = tags.get_index::<Image>(0).unwrap().get().unwrap();
        let buffer = sample.get_buffer().unwrap();
        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());
        }
    }
}
