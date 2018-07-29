// Copyright (C) 2018 François Laignel <fengalin@free.fr>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib;
use glib::translate::{from_glib, ToGlibPtr};
use glib::{SendValue, ToValue};

use serde::de;
use serde::de::{Deserialize, DeserializeSeed, Deserializer, SeqAccess, Visitor};
use serde::ser;
use serde::ser::{Serialize, SerializeSeq, SerializeTuple, Serializer};

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use tags::*;
use value_serde::{DATE_TIME_OTHER_TYPE_ID, SAMPLE_OTHER_TYPE_ID};
use DateTime;
use Sample;
use TagMergeMode;

macro_rules! ser_tag (
    ($value:ident, $seq:ident, $t:ty) => (
        ser_value!($value, $t, |_, value| {
            $seq.serialize_element(&value)
        })
    );
);

// serialize trait is only available for `&self`, but we need to mutate the iterator
struct TagValuesSer<'a>(Rc<RefCell<GenericTagIterator<'a>>>);
impl<'a> TagValuesSer<'a> {
    fn from(tags_ser: &TagsSer<'a>) -> Self {
        TagValuesSer(Rc::clone(&tags_ser.1))
    }
}

impl<'a> Serialize for TagValuesSer<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut tag_iter = self.0.borrow_mut();
        let mut seq = serializer.serialize_seq(tag_iter.size_hint().1)?;
        while let Some(value) = tag_iter.next() {
            match value.type_() {
                glib::Type::F64 => ser_tag!(value, seq, f64),
                glib::Type::String => ser_tag!(value, seq, String),
                glib::Type::U32 => ser_tag!(value, seq, u32),
                glib::Type::U64 => ser_tag!(value, seq, u64),
                glib::Type::Other(type_id) => {
                    if *DATE_TIME_OTHER_TYPE_ID == type_id {
                        ser_tag!(value, seq, DateTime)
                    } else if *SAMPLE_OTHER_TYPE_ID == type_id {
                        ser_tag!(value, seq, Sample)
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

struct TagsSer<'a>(&'a str, Rc<RefCell<GenericTagIterator<'a>>>);
impl<'a> TagsSer<'a> {
    fn new(name: &'a str, tag_iter: GenericTagIterator<'a>) -> Self {
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

impl Serialize for TagListRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let tag_count = self.n_tags();
        if tag_count > 0 {
            let mut seq = serializer.serialize_seq(Some(tag_count as usize))?;
            let tag_list_iter = self.iter_tag_list();
            for (tag_name, tag_iter) in tag_list_iter {
                seq.serialize_element(&TagsSer::new(tag_name, tag_iter))?;
            }
            seq.end()
        } else if tag_count == 0 {
            let seq = serializer.serialize_seq(None)?;
            seq.end()
        } else {
            Err(ser::Error::custom("tag count < 0"))
        }
    }
}

impl Serialize for TagList {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_ref().serialize(serializer)
    }
}

macro_rules! de_tag_value(
    ($tag_name:expr, $seq:expr, $t:ty) => (
        de_send_value!("Tag", $tag_name, $seq, $t)
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
            from_glib(ffi::gst_tag_get_type(tag_name.0))
        };

        loop {
            let tag_value = match tag_type {
                glib::Type::F64 => de_tag_value!(self.0, seq, f64),
                glib::Type::String => de_tag_value!(self.0, seq, String),
                glib::Type::U32 => de_tag_value!(self.0, seq, u32),
                glib::Type::U64 => de_tag_value!(self.0, seq, u64),
                glib::Type::Other(type_id) => {
                    if *DATE_TIME_OTHER_TYPE_ID == type_id {
                        de_tag_value!(self.0, seq, DateTime)
                    } else if *SAMPLE_OTHER_TYPE_ID == type_id {
                        de_tag_value!(self.0, seq, Sample)
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
            .ok_or(de::Error::custom("Expected a name for the `Tag` name"))?;
        seq.next_element_seed(TagValues(name.as_str(), self.0))?
            .ok_or(de::Error::custom("Expected a seq of values for the `Tag`"))
    }
}

impl<'de, 'a> DeserializeSeed<'de> for TagValuesTuple<'a> {
    type Value = ();

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        deserializer.deserialize_tuple(2, TagValuesTupleVisitor(self.0))
    }
}

struct TagListVisitor;
impl<'de> Visitor<'de> for TagListVisitor {
    type Value = TagList;

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
        Ok(tag_list)
    }
}

impl<'de> Deserialize<'de> for TagList {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_seq(TagListVisitor)
    }
}

#[cfg(test)]
mod tests {
    extern crate ron;

    use tags::*;
    use Buffer;
    use GenericFormattedValue;
    use Sample;
    use TagMergeMode;

    #[test]
    fn test_serialize() {
        ::init().unwrap();

        let mut tags = TagList::new();
        assert_eq!(tags.to_string(), "taglist;");
        {
            let tags = tags.get_mut().unwrap();
            tags.add::<Title>(&"a title", TagMergeMode::Append); // String
            tags.add::<Title>(&"another title", TagMergeMode::Append); // String
            tags.add::<Duration>(&(::SECOND * 120).into(), TagMergeMode::Append); // u64
            tags.add::<Bitrate>(&96_000, TagMergeMode::Append); // u32
            tags.add::<TrackGain>(&1f64, TagMergeMode::Append); // f64
            tags.add::<DateTime>(
                &::DateTime::new(2f32, 2018, 5, 28, 16, 6, 42.841f64),
                TagMergeMode::Append,
            ); // DateTime

            let sample = {
                let mut buffer = Buffer::from_slice(vec![1, 2, 3, 4]).unwrap();
                {
                    let buffer = buffer.get_mut().unwrap();
                    buffer.set_offset(0);
                    buffer.set_offset_end(0);
                }
                Sample::new::<GenericFormattedValue>(Some(&buffer), None, None, None)
            };
            tags.add::<Image>(&sample, TagMergeMode::Append); // Sample
        }

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        let res = ron::ser::to_string_pretty(&tags, pretty_config);
        assert_eq!(
            Ok(concat!(
                "[",
                "    (\"title\", [",
                "        \"a title\",",
                "        \"another title\",",
                "    ]),",
                "    (\"duration\", [",
                "        120000000000,",
                "    ]),",
                "    (\"bitrate\", [",
                "        96000,",
                "    ]),",
                "    (\"replaygain-track-gain\", [",
                "        1,",
                "    ]),",
                "    (\"datetime\", [",
                "        (",
                "            tz_offset: 2,",
                "            y: 2018,",
                "            m: 5,",
                "            d: 28,",
                "            h: 16,",
                "            mn: 6,",
                "            s: 42,",
                "            us: 841000,",
                "        ),",
                "    ]),",
                "    (\"image\", [",
                "        (",
                "            buffer: Some((",
                "                pts: None,",
                "                dts: None,",
                "                duration: None,",
                "                offset: 0,",
                "                offset_end: 0,",
                "                flags: (",
                "                    bits: 0,",
                "                ),",
                "                buffer: \"AQIDBA==\",",
                "            )),",
                "            buffer_list: None,",
                "            caps: None,",
                "            segment: Some((",
                "                flags: (",
                "                    bits: 0,",
                "                ),",
                "                rate: 1,",
                "                applied_rate: 1,",
                "                format: Time,",
                "                base: 0,",
                "                offset: 0,",
                "                start: 0,",
                "                stop: -1,",
                "                time: 0,",
                "                position: 0,",
                "                duration: -1,",
                "            )),",
                "            info: None,",
                "        ),",
                "    ]),",
                "]",
            ).to_owned()),
            res,
        );
    }

    #[test]
    fn test_deserialize() {
        extern crate serde_json;

        ::init().unwrap();

        let tag_list_ron = r#"
            [
                ("title", [
                    "a title",
                    "another title",
                ]),
                ("duration", [120000000000]),
                ("bitrate", [96000]),
                ("replaygain-track-gain", [1]),
                ("datetime", [
                    (
                        tz_offset: 2,
                        y: 2018,
                        m: 5,
                        d: 28,
                        h: 16,
                        mn: 6,
                        s: 42,
                        us: 841000,
                    ),
                ]),
                ("image", [
                    (
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
                    ),
                ])
            ]
        "#;
        let tags: TagList = ron::de::from_str(tag_list_ron).unwrap();
        assert_eq!(tags.get_index::<Title>(0).unwrap().get(), Some("a title"));
        assert_eq!(
            tags.get_index::<Title>(1).unwrap().get(),
            Some("another title")
        );
        assert_eq!(
            tags.get_index::<Duration>(0).unwrap().get(),
            Some(::SECOND * 120)
        );
        assert_eq!(tags.get_index::<Bitrate>(0).unwrap().get(), Some(96_000));
        assert_eq!(tags.get_index::<TrackGain>(0).unwrap().get(), Some(1f64));
        let datetime = tags.get_index::<DateTime>(0).unwrap().get().unwrap();
        assert_eq!(datetime.get_year(), 2018);
        assert_eq!(datetime.get_microsecond(), 841_000);
        let sample = tags.get_index::<Image>(0).unwrap().get().unwrap();
        let buffer = sample.get_buffer().unwrap();
        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());
        }

        let tag_json = r#"
            [
                ["title", ["a title", "another title"]],
                ["duration", [120000000000]],
                ["bitrate", [96000]],
                ["replaygain-track-gain", [1.0]],
                ["datetime",[{"tz_offset":2.0,"y":2018,"m":5,"d":28,"h":16,"mn":6,"s":42,"us":841000}]],
                ["image",[{"buffer":{"pts":null,"dts":null,"duration":null,"offset":0,"offset_end":0,"flags":{"bits":0},"buffer":[1,2,3,4]},"buffer_list":null,"caps":null,"segment":null,"info":null}]]
            ]
        "#;
        let tags: TagList = serde_json::from_str(tag_json).unwrap();
        assert_eq!(tags.get_index::<Title>(0).unwrap().get(), Some("a title"));
        assert_eq!(
            tags.get_index::<Title>(1).unwrap().get(),
            Some("another title")
        );
        assert_eq!(tags.get_index::<Bitrate>(0).unwrap().get(), Some(96_000));
        assert_eq!(tags.get_index::<TrackGain>(0).unwrap().get(), Some(1f64));
        let datetime = tags.get_index::<DateTime>(0).unwrap().get().unwrap();
        assert_eq!(datetime.get_month(), 5);
        assert_eq!(datetime.get_hour(), 16);
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
            tags.add::<Title>(&"a title", TagMergeMode::Append); // String
            tags.add::<Title>(&"another title", TagMergeMode::Append); // String
            tags.add::<Duration>(&(::SECOND * 120).into(), TagMergeMode::Append); // u64
            tags.add::<Bitrate>(&96_000, TagMergeMode::Append); // u32
            tags.add::<TrackGain>(&1f64, TagMergeMode::Append); // f64
            tags.add::<DateTime>(
                &::DateTime::new(2f32, 2018, 5, 28, 16, 6, 42.841f64),
                TagMergeMode::Append,
            ); // DateTime

            let sample = {
                let mut buffer = Buffer::from_slice(vec![1, 2, 3, 4]).unwrap();
                {
                    let buffer = buffer.get_mut().unwrap();
                    buffer.set_offset(0);
                    buffer.set_offset_end(0);
                }
                Sample::new::<GenericFormattedValue>(Some(&buffer), None, None, None)
            };
            tags.add::<Image>(&sample, TagMergeMode::Append); // Sample
        }
        let tags_ser = ron::ser::to_string(&tags).unwrap();

        let tags_de: TagList = ron::de::from_str(tags_ser.as_str()).unwrap();
        assert_eq!(
            tags_de.get_index::<Title>(0).unwrap().get(),
            tags.get_index::<Title>(0).unwrap().get(),
        );
        assert_eq!(
            tags_de.get_index::<Title>(1).unwrap().get(),
            tags.get_index::<Title>(1).unwrap().get(),
        );
        assert_eq!(
            tags_de.get_index::<Duration>(0).unwrap().get(),
            tags.get_index::<Duration>(0).unwrap().get(),
        );
        assert_eq!(
            tags_de.get_index::<Bitrate>(0).unwrap().get(),
            tags.get_index::<Bitrate>(0).unwrap().get(),
        );
        assert_eq!(
            tags_de.get_index::<TrackGain>(0).unwrap().get(),
            tags.get_index::<TrackGain>(0).unwrap().get(),
        );
        let datetime = tags.get_index::<DateTime>(0).unwrap().get().unwrap();
        assert_eq!(datetime.get_year(), 2018);
        assert_eq!(datetime.get_microsecond(), 841_000);
        let sample = tags.get_index::<Image>(0).unwrap().get().unwrap();
        let buffer = sample.get_buffer().unwrap();
        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());
        }
    }
}
