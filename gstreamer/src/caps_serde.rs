// Copyright (C) 2018 François Laignel <fengalin@free.fr>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::de;
use serde::de::{Deserialize, Deserializer, EnumAccess, SeqAccess, VariantAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, SerializeTuple, Serializer};

use std::fmt;

use Caps;
use CapsFeatures;
use CapsFeaturesRef;
use CapsRef;
use Structure;
use StructureRef;

enum CapsVariantKinds {
    Any,
    Empty,
    Some,
}

const CAPS_VARIANT_ANY_ID: u32 = 0;
const CAPS_VARIANT_ANY_STR: &str = "Any";
const CAPS_VARIANT_EMPTY_ID: u32 = 1;
const CAPS_VARIANT_EMPTY_STR: &str = "Empty";
const CAPS_VARIANT_SOME_ID: u32 = 2;
const CAPS_VARIANT_SOME_STR: &str = "Some";

const CAPS_VARIANT_NAMES: &[&str] = &[
    &CAPS_VARIANT_ANY_STR,
    &CAPS_VARIANT_EMPTY_STR,
    &CAPS_VARIANT_SOME_STR,
];

struct CapsItemSe<'a>(&'a StructureRef, Option<&'a CapsFeaturesRef>);
impl<'a> Serialize for CapsItemSe<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut tup = serializer.serialize_tuple(2)?;
        tup.serialize_element(self.0)?;
        tup.serialize_element(&self.1)?;
        tup.end()
    }
}

struct CapsForIterSe<'a>(&'a CapsRef);
impl<'a> Serialize for CapsForIterSe<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let iter = self.0.iter_with_features();
        let size = iter.size_hint().0;
        if size > 0 {
            let mut seq = serializer.serialize_seq(Some(size))?;
            for (structure, features) in iter {
                let features = if !features.is_any()
                    && features.is_equal(::CAPS_FEATURES_MEMORY_SYSTEM_MEMORY.as_ref())
                {
                    None
                } else {
                    Some(features)
                };
                seq.serialize_element(&CapsItemSe(structure, features))?;
            }
            seq.end()
        } else {
            let seq = serializer.serialize_seq(None)?;
            seq.end()
        }
    }
}

impl Serialize for CapsRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.is_any() {
            serializer.serialize_unit_variant(
                stringify!(Caps),
                CAPS_VARIANT_ANY_ID,
                CAPS_VARIANT_ANY_STR,
            )
        } else if self.is_empty() {
            serializer.serialize_unit_variant(
                stringify!(Caps),
                CAPS_VARIANT_EMPTY_ID,
                CAPS_VARIANT_EMPTY_STR,
            )
        } else {
            serializer.serialize_newtype_variant(
                stringify!(Caps),
                CAPS_VARIANT_SOME_ID,
                CAPS_VARIANT_SOME_STR,
                &CapsForIterSe(&self),
            )
        }
    }
}

impl Serialize for Caps {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_ref().serialize(serializer)
    }
}

struct CapsItemDe(Structure, Option<CapsFeatures>);

struct CapsItemVisitor;
impl<'de> Visitor<'de> for CapsItemVisitor {
    type Value = CapsItemDe;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a tuple `(Structure, Option<CapsFeature>)`")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let structure = seq
            .next_element::<Structure>()?
            .ok_or_else(|| de::Error::custom("Expected a `Structure` for `Caps` item"))?;
        let features_option = seq.next_element::<Option<CapsFeatures>>()?.ok_or_else(|| {
            de::Error::custom("Expected an `Option<CapsFeature>` for `Caps` item")
        })?;

        Ok(CapsItemDe(structure, features_option))
    }
}

impl<'de> Deserialize<'de> for CapsItemDe {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<CapsItemDe, D::Error> {
        skip_assert_initialized!();
        deserializer.deserialize_tuple(2, CapsItemVisitor)
    }
}

struct CapsSome(Caps);

struct CapsSomeVisitor;
impl<'de> Visitor<'de> for CapsSomeVisitor {
    type Value = CapsSome;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence of `(Structure, Option<CapsFeature>)`")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut caps = Caps::new_empty();
        {
            let caps = caps.get_mut().unwrap();
            while let Some(caps_item) = seq.next_element::<CapsItemDe>()? {
                caps.append_structure_full(caps_item.0, caps_item.1);
            }
        }
        Ok(CapsSome(caps))
    }
}

impl<'de> Deserialize<'de> for CapsSome {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<CapsSome, D::Error> {
        skip_assert_initialized!();
        deserializer.deserialize_seq(CapsSomeVisitor)
    }
}

struct CapsVariantKindsVisitor;
impl<'de> Visitor<'de> for CapsVariantKindsVisitor {
    type Value = CapsVariantKinds;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Caps variant kind (`Any`, `None` or `Some`)")
    }

    fn visit_u32<E: de::Error>(self, value: u32) -> Result<Self::Value, E> {
        match value {
            CAPS_VARIANT_ANY_ID => Ok(CapsVariantKinds::Any),
            CAPS_VARIANT_EMPTY_ID => Ok(CapsVariantKinds::Empty),
            CAPS_VARIANT_SOME_ID => Ok(CapsVariantKinds::Some),
            _ => Err(de::Error::invalid_value(
                de::Unexpected::Unsigned(u64::from(value)),
                &self,
            )),
        }
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        match value {
            CAPS_VARIANT_ANY_STR => Ok(CapsVariantKinds::Any),
            CAPS_VARIANT_EMPTY_STR => Ok(CapsVariantKinds::Empty),
            CAPS_VARIANT_SOME_STR => Ok(CapsVariantKinds::Some),
            _ => Err(de::Error::unknown_variant(value, CAPS_VARIANT_NAMES)),
        }
    }
}

impl<'de> Deserialize<'de> for CapsVariantKinds {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        deserializer.deserialize_identifier(CapsVariantKindsVisitor)
    }
}

struct CapsVisitor;
impl<'de> Visitor<'de> for CapsVisitor {
    type Value = Caps;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Caps enum (`Any`, `None` or `Some()`)")
    }

    fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
        let res = match data.variant()? {
            (CapsVariantKinds::Any, _v) => Caps::new_any(),
            (CapsVariantKinds::Empty, _v) => Caps::new_empty(),
            (CapsVariantKinds::Some, v) => v
                .newtype_variant::<CapsSome>()
                .map(|caps_some| caps_some.0)?,
        };

        Ok(res)
    }
}

impl<'de> Deserialize<'de> for Caps {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        deserializer.deserialize_enum(stringify!(Caps), CAPS_VARIANT_NAMES, CapsVisitor)
    }
}

#[cfg(test)]
mod tests {
    extern crate ron;

    use Array;
    use Caps;
    use CapsFeatures;
    use Fraction;

    #[test]
    fn test_serialize() {
        ::init().unwrap();

        let caps = Caps::builder("foo/bar")
            .field("int", &12)
            .field("bool", &true)
            .field("string", &"bla")
            .field("fraction", &Fraction::new(1, 2))
            .field("array", &Array::new(&[&1, &2]))
            .build();

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        let res = ron::ser::to_string_pretty(&caps, pretty_config);
        assert_eq!(
            Ok(concat!(
                "Some([",
                "    ((\"foo/bar\", [",
                "        (\"int\", \"i32\", 12),",
                "        (\"bool\", \"bool\", true),",
                "        (\"string\", \"String\", Some(\"bla\")),",
                "        (\"fraction\", \"Fraction\", (1, 2)),",
                "        (\"array\", \"Array\", [",
                "            (\"i32\", 1),",
                "            (\"i32\", 2),",
                "        ]),",
                "    ]), None),",
                "])"
            )
            .to_owned()),
            res,
        );

        let caps = Caps::builder("foo/bar")
            .field("int", &12)
            .field("bool", &true)
            .field("string", &"bla")
            .field("fraction", &Fraction::new(1, 2))
            .field("array", &Array::new(&[&1, &2]))
            .features(&["foo:bar", "foo:baz"])
            .build();

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        let res = ron::ser::to_string_pretty(&caps, pretty_config);
        assert_eq!(
            Ok(concat!(
                "Some([",
                "    ((\"foo/bar\", [",
                "        (\"int\", \"i32\", 12),",
                "        (\"bool\", \"bool\", true),",
                "        (\"string\", \"String\", Some(\"bla\")),",
                "        (\"fraction\", \"Fraction\", (1, 2)),",
                "        (\"array\", \"Array\", [",
                "            (\"i32\", 1),",
                "            (\"i32\", 2),",
                "        ]),",
                "    ]), Some(Some([",
                "        \"foo:bar\",",
                "        \"foo:baz\",",
                "    ]))),",
                "])"
            )
            .to_owned()),
            res,
        );

        let caps = Caps::builder("foo/bar")
            .field("int", &12)
            .field("bool", &true)
            .field("string", &"bla")
            .field("fraction", &Fraction::new(1, 2))
            .field("array", &Array::new(&[&1, &2]))
            .any_features()
            .build();

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        let res = ron::ser::to_string_pretty(&caps, pretty_config.clone());
        assert_eq!(
            Ok(concat!(
                "Some([",
                "    ((\"foo/bar\", [",
                "        (\"int\", \"i32\", 12),",
                "        (\"bool\", \"bool\", true),",
                "        (\"string\", \"String\", Some(\"bla\")),",
                "        (\"fraction\", \"Fraction\", (1, 2)),",
                "        (\"array\", \"Array\", [",
                "            (\"i32\", 1),",
                "            (\"i32\", 2),",
                "        ]),",
                "    ]), Some(Any)),",
                "])"
            )
            .to_owned()),
            res,
        );

        let caps_any = Caps::new_any();
        let res = ron::ser::to_string_pretty(&caps_any, pretty_config.clone());
        assert_eq!(Ok("Any".to_owned()), res);

        let caps_empty = Caps::new_empty();
        let res = ron::ser::to_string_pretty(&caps_empty, pretty_config);
        assert_eq!(Ok("Empty".to_owned()), res);
    }

    #[test]
    fn test_deserialize() {
        use Structure;

        ::init().unwrap();

        let caps_ron = "Any";
        let caps: Caps = ron::de::from_str(caps_ron).unwrap();
        assert!(caps.is_any());

        let caps_ron = "Empty";
        let caps: Caps = ron::de::from_str(caps_ron).unwrap();
        assert!(caps.is_empty());

        let caps_ron = r#"
            Some([
                (
                    ("foo/bar", [
                        ("int", "i32", 12),
                        ("bool", "bool", true),
                        ("string", "String", Some("bla")),
                        ("fraction", "Fraction", (1, 2)),
                        ("array", "Array", [
                            ("i32", 1),
                            ("i32", 2),
                        ]),
                    ]),
                    None,
                ),
            ])"#;
        let caps: Caps = ron::de::from_str(caps_ron).unwrap();
        let s = caps.get_structure(0).unwrap();
        assert_eq!(
            s,
            Structure::new(
                "foo/bar",
                &[
                    ("int", &12),
                    ("bool", &true),
                    ("string", &"bla"),
                    ("fraction", &Fraction::new(1, 2)),
                    ("array", &Array::new(&[&1, &2])),
                ],
            )
            .as_ref()
        );

        let caps_ron = r#"
            Some([
                (
                    ("foo/bar", [
                        ("int", "i32", 12),
                        ("bool", "bool", true),
                        ("string", "String", None),
                        ("fraction", "Fraction", (1, 2)),
                        ("array", "Array", [
                            ("i32", 1),
                            ("i32", 2),
                        ]),
                    ]),
                    Some(Some(["foo:bar", "foo:baz"])),
                ),
            ])"#;
        let caps: Caps = ron::de::from_str(caps_ron).unwrap();
        let s = caps.get_structure(0).unwrap();
        let str_none: Option<&str> = None;
        assert_eq!(
            s,
            Structure::new(
                "foo/bar",
                &[
                    ("int", &12),
                    ("bool", &true),
                    ("string", &str_none),
                    ("fraction", &Fraction::new(1, 2)),
                    ("array", &Array::new(&[&1, &2])),
                ],
            )
            .as_ref()
        );
        let f = caps.get_features(0).unwrap();
        assert!(f.is_equal(CapsFeatures::new(&["foo:bar", "foo:baz"]).as_ref()));

        let caps_ron = r#"
            Some([
                (
                    ("foo/bar", [
                        ("int", "i32", 12),
                        ("bool", "bool", true),
                        ("string", "String", Some("bla")),
                        ("fraction", "Fraction", (1, 2)),
                        ("array", "Array", [
                            ("i32", 1),
                            ("i32", 2),
                        ]),
                    ]),
                    Some(Any),
                ),
            ])"#;
        let caps: Caps = ron::de::from_str(caps_ron).unwrap();
        let s = caps.get_structure(0).unwrap();
        assert_eq!(
            s,
            Structure::new(
                "foo/bar",
                &[
                    ("int", &12),
                    ("bool", &true),
                    ("string", &"bla"),
                    ("fraction", &Fraction::new(1, 2)),
                    ("array", &Array::new(&[&1, &2])),
                ],
            )
            .as_ref()
        );
        let f = caps.get_features(0).unwrap();
        assert!(f.is_any());
    }

    #[test]
    fn test_serde_roundtrip() {
        ::init().unwrap();

        let caps = Caps::new_any();
        let caps_ser = ron::ser::to_string(&caps).unwrap();
        let caps_de: Caps = ron::de::from_str(caps_ser.as_str()).unwrap();
        assert!(caps_de.is_any());

        let caps = Caps::new_empty();
        let caps_ser = ron::ser::to_string(&caps).unwrap();
        let caps_de: Caps = ron::de::from_str(caps_ser.as_str()).unwrap();
        assert!(caps_de.is_empty());

        let caps = Caps::builder("foo/bar")
            .field("int", &12)
            .field("bool", &true)
            .field("string", &"bla")
            .field("fraction", &Fraction::new(1, 2))
            .field("array", &Array::new(&[&1, &2]))
            .build();
        let caps_ser = ron::ser::to_string(&caps).unwrap();
        let caps_de: Caps = ron::de::from_str(caps_ser.as_str()).unwrap();
        assert!(caps_de.is_strictly_equal(&caps));

        let caps = Caps::builder("foo/bar")
            .field("int", &12)
            .field("bool", &true)
            .field("string", &"bla")
            .field("fraction", &Fraction::new(1, 2))
            .field("array", &Array::new(&[&1, &2]))
            .features(&["foo:bar", "foo:baz"])
            .build();
        let caps_ser = ron::ser::to_string(&caps).unwrap();
        let caps_de: Caps = ron::de::from_str(caps_ser.as_str()).unwrap();
        assert!(caps_de.is_strictly_equal(&caps));

        let caps = Caps::builder("foo/bar")
            .field("int", &12)
            .field("bool", &true)
            .field("string", &"bla")
            .field("fraction", &Fraction::new(1, 2))
            .field("array", &Array::new(&[&1, &2]))
            .any_features()
            .build();
        let caps_ser = ron::ser::to_string(&caps).unwrap();
        let caps_de: Caps = ron::de::from_str(caps_ser.as_str()).unwrap();
        assert!(caps_de.is_strictly_equal(&caps));
    }
}
