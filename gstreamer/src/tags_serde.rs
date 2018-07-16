// Copyright (C) 2018 François Laignel <fengalin@free.fr>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib;
use glib::translate::{ToGlibPtr, from_glib, from_glib_none};
use glib::{ToValue, Value};
use gobject_ffi;

use serde::de;
use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser;
use serde::ser::{Serialize, Serializer, SerializeSeq, SerializeTuple};

use std::ffi::CStr;
use std::ffi::CString;
use std::fmt;

use DateTime;
use Sample;
use miniobject::MiniObject;
use tags::*;
use value_serde::{DATE_TIME_OTHER_TYPE_ID, SAMPLE_OTHER_TYPE_ID};

struct TagSer<'a>(&'a str, Value);

macro_rules! ser_tag (
    ($tag_ser:ident, $ser:ident, $t:ty) => (
        ser_value!($tag_ser.1, $tag_ser.0, $t, |tag_name, value| {
            let mut tup = $ser.serialize_tuple(2)?;
            tup.serialize_element(tag_name)?;
            tup.serialize_element(&value)?;
            tup.end()
        })
    );
);

impl<'a> Serialize for TagSer<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.1.type_() {
            glib::Type::F64 => ser_tag!(self, serializer, f64),
            glib::Type::String => ser_tag!(self, serializer, String),
            glib::Type::U32 => ser_tag!(self, serializer, u32),
            glib::Type::U64 => ser_tag!(self, serializer, u64),
            glib::Type::Other(type_id) => {
                if *DATE_TIME_OTHER_TYPE_ID == type_id {
                    ser_tag!(self, serializer, DateTime)
                } else if *SAMPLE_OTHER_TYPE_ID == type_id {
                    ser_tag!(self, serializer, Sample)
                } else {
                    Err(
                        ser::Error::custom(
                            format!("unimplemented Tag serialization for type {}",
                                glib::Type::Other(type_id),
                            )
                        )
                    )
                }
            }
            type_ => {
                Err(
                    ser::Error::custom(
                        format!("unimplemented Tag serialization for type {}", type_)
                    )
                )
            }
        }
    }
}

impl Serialize for TagListRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let tag_count = unsafe { ffi::gst_tag_list_n_tags(self.as_ptr()) };
        if tag_count > 0 {
            let mut seq = serializer.serialize_seq(Some(tag_count as usize))?;
            for name_index in 0..tag_count {
                unsafe {
                    let tag_name = ffi::gst_tag_list_nth_tag_name(self.as_ptr(), name_index as u32);
                    let tag_size = ffi::gst_tag_list_get_tag_size(self.as_ptr(), tag_name);
                    for tag_index in 0..tag_size {
                        let value = ffi::gst_tag_list_get_value_index(
                            self.as_ptr(),
                            tag_name,
                            tag_index,
                        );

                        if !value.is_null() {
                            let tag_name = CStr::from_ptr(tag_name)
                                .to_str()
                                .map_err(|_| {
                                    ser::Error::custom(
                                        format!(
                                            "invalid UTF-8 characters in Tag name {:?}",
                                            tag_name,
                                        )
                                    )
                                })?;
                            seq.serialize_element(
                                &TagSer(
                                    tag_name,
                                    from_glib_none(value as *mut gobject_ffi::GValue),
                                )
                            )?
                        }
                    }
                }
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

struct TagDe(CString, Value);
struct TagVisitor;

macro_rules! de_tag_value(
    ($tag_name:expr, $seq:expr, $t:ty) => (
        {
            let value = de_value!("Tag", $tag_name, $seq, $t);
            TagDe($tag_name, value)
        }
    );
);

impl<'de> Visitor<'de> for TagVisitor {
    type Value = TagDe;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a tuple of 2 elements (name: String, value: Tag value type)")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let name = CString::new(
                seq
                    .next_element::<String>()
                    .map_err(|err| de::Error::custom(
                        format!("Error reading Tag name. {:?}", err)
                    ))?
                    .ok_or(de::Error::custom("Expected a value for Tag name"))?
                    .as_str()
            ).unwrap();

        unsafe {
            let type_: glib::Type = from_glib(ffi::gst_tag_get_type(name.as_ptr() as *const i8));
            let tag_de = match type_ {
                glib::Type::F64 => de_tag_value!(name, seq, f64),
                glib::Type::String => de_tag_value!(name, seq, String),
                glib::Type::U32 => de_tag_value!(name, seq, u32),
                glib::Type::U64 => de_tag_value!(name, seq, u64),
                glib::Type::Other(type_id) => {
                    if *DATE_TIME_OTHER_TYPE_ID == type_id {
                        de_tag_value!(name, seq, DateTime)
                    } else if *SAMPLE_OTHER_TYPE_ID == type_id {
                        de_tag_value!(name, seq, Sample)
                    } else {
                        return Err(
                            de::Error::custom(
                                format!(
                                    "unimplemented deserialization for Tag {:?} with type `{}`",
                                    name,
                                    glib::Type::Other(type_id),
                                ),
                            )
                        );
                    }
                }
                type_ => {
                    return Err(
                        de::Error::custom(
                            format!(
                                "unimplemented deserialization for Tag {:?} with type `{}`",
                                name,
                                type_,
                            ),
                        )
                    );
                }
            };

            Ok(tag_de)
        }
    }
}

impl<'de> Deserialize<'de> for TagDe {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_tuple(2, TagVisitor{}) // 2 items in the tuple: (name, value)
    }
}

struct TagListVisitor;
impl<'de> Visitor<'de> for TagListVisitor {
    type Value = TagList;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence of Tags")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let tag_list = TagList::new();
        while let Some(tag_de) = seq.next_element::<TagDe>()? {
            unsafe {
                ffi::gst_tag_list_add_value(
                    tag_list.as_mut_ptr(),
                    ffi::GST_TAG_MERGE_APPEND,
                    tag_de.0.as_ptr(),
                    tag_de.1.to_glib_none().0,
                );
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

    #[test]
    fn test_serialize() {
        use Buffer;
        use GenericFormattedValue;
        use TagMergeMode;
        use Sample;

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
                Sample::new::<GenericFormattedValue>(
                    Some(&buffer),
                    None,
                    None,
                    None,
                )
            };
            tags.add::<Image>(&sample, TagMergeMode::Append); // Sample
        }

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        let res = ron::ser::to_string_pretty(&tags, pretty_config);
        assert_eq!(
            Ok(
                concat!(
                    "[",
                    "    (\"title\", \"a title\"),",
                    "    (\"title\", \"another title\"),",
                    "    (\"duration\", 120000000000),",
                    "    (\"bitrate\", 96000),",
                    "    (\"replaygain-track-gain\", 1),",
                    "    (\"datetime\", (",
                    "        tz_offset: 2,",
                    "        y: 2018,",
                    "        m: 5,",
                    "        d: 28,",
                    "        h: 16,",
                    "        mn: 6,",
                    "        s: 42,",
                    "        us: 841000,",
                    "    )),",
                    "    (\"image\", (",
                    "        buffer: Some((",
                    "            pts: None,",
                    "            dts: None,",
                    "            duration: None,",
                    "            offset: 0,",
                    "            offset_end: 0,",
                    "            flags: (",
                    "                bits: 0,",
                    "            ),",
                    "            buffer: \"AQIDBA==\",",
                    "        )),",
                    "        buffer_list: None,",
                    "        caps: None,",
                    "        segment: Some((",
                    "            flags: (",
                    "                bits: 0,",
                    "            ),",
                    "            rate: 1,",
                    "            applied_rate: 1,",
                    "            format: Time,",
                    "            base: 0,",
                    "            offset: 0,",
                    "            start: 0,",
                    "            stop: -1,",
                    "            time: 0,",
                    "            position: 0,",
                    "            duration: -1,",
                    "        )),",
                    "        info: None,",
                    "    )),",
                    "]",
                ).to_owned()
            ),
            res,
        );
    }

    #[test]
    fn test_deserialize() {
        extern crate serde_json;

        ::init().unwrap();

        let tag_list_ron = r#"
            [
                ("title", "a title"),
                ("title", "another title"),
                ("duration", 120000000000),
                ("bitrate", 96000),
                ("replaygain-track-gain", 1),
                ("datetime", (
                    tz_offset: 2,
                    y: 2018,
                    m: 5,
                    d: 28,
                    h: 16,
                    mn: 6,
                    s: 42,
                    us: 841000,
                )),
                ("image", (
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
                ))
            ]
        "#;
        let tags: TagList = ron::de::from_str(tag_list_ron).unwrap();
        assert_eq!(tags.get_index::<Title>(0).unwrap().get(), Some("a title"));
        assert_eq!(tags.get_index::<Title>(1).unwrap().get(), Some("another title"));
        assert_eq!(tags.get_index::<Duration>(0).unwrap().get(), Some(::SECOND * 120));
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
                ["title", "a title"],
                ["title", "another title"],
                ["duration", 120000000000],
                ["bitrate", 96000],
                ["replaygain-track-gain", 1.0],
                ["datetime",{"tz_offset":2.0,"y":2018,"m":5,"d":28,"h":16,"mn":6,"s":42,"us":841000}],
                ["image",{"buffer":{"pts":null,"dts":null,"duration":null,"offset":0,"offset_end":0,"flags":{"bits":0},"buffer":[1,2,3,4]},"buffer_list":null,"caps":null,"segment":null,"info":null}]
            ]
        "#;
        let tags: TagList = serde_json::from_str(tag_json).unwrap();
        assert_eq!(tags.get_index::<Title>(0).unwrap().get(), Some("a title"));
        assert_eq!(tags.get_index::<Title>(1).unwrap().get(), Some("another title"));
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
}
