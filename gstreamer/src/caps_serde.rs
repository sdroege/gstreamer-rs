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

// `CapsFeature` is not available in `gstreamer-rs` yet
struct CapsItemSe<'a>(&'a StructureRef);
impl<'a> Serialize for CapsItemSe<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut tup = serializer.serialize_tuple(2)?;
        tup.serialize_element(self.0)?;
        // `CapsFeature` is not available in `gstreamer-rs` yet
        // Fake the type for now and use `None` as a value
        tup.serialize_element::<Option<Structure>>(&None)?;
        tup.end()
    }
}

struct CapsForIterSe<'a>(&'a CapsRef);
impl<'a> Serialize for CapsForIterSe<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let iter = self.0.iter();
        let size = iter.size_hint().0;
        if size > 0 {
            let mut seq = serializer.serialize_seq(Some(size))?;
            for structure in iter {
                seq.serialize_element(&CapsItemSe(structure))?;
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

// `CapsFeature` is not available in `gstreamer-rs` yet
struct CapsItemDe(Structure);
impl From<CapsItemDe> for Structure {
    fn from(caps_item: CapsItemDe) -> Structure {
        caps_item.0
    }
}

struct CapsItemVisitor;
impl<'de> Visitor<'de> for CapsItemVisitor {
    type Value = CapsItemDe;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a tuple `(Structure, Option<CapsFeature>)`")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let structure = seq
            .next_element::<Structure>()?
            .ok_or(de::Error::custom("Expected a `Structure` for `Caps` item"))?;
        // `CapsFeature` is not available in `gstreamer-rs` yet
        // Fake the type for now and expect `None` as a value
        let feature_option = seq
            .next_element::<Option<Structure>>()?
            .ok_or(de::Error::custom(
                "Expected an `Option<CapsFeature>` for `Caps` item",
            ))?;
        if feature_option.is_some() {
            Err(de::Error::custom(
                "Found a value for `CapsFeature`, expected `None` (not implemented yet)",
            ))
        } else {
            Ok(CapsItemDe(structure))
        }
    }
}

impl<'de> Deserialize<'de> for CapsItemDe {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<CapsItemDe, D::Error> {
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
                caps.append_structure(caps_item.into());
            }
        }
        Ok(CapsSome(caps))
    }
}

impl<'de> Deserialize<'de> for CapsSome {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<CapsSome, D::Error> {
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
                de::Unexpected::Unsigned(value as u64),
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
        deserializer.deserialize_enum(stringify!(Caps), CAPS_VARIANT_NAMES, CapsVisitor)
    }
}

#[cfg(test)]
mod tests {
    extern crate ron;

    use Array;
    use Caps;
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

        let res = ron::ser::to_string_pretty(&caps, pretty_config.clone());
        assert_eq!(
            Ok(concat!(
                "Some([",
                "    ((\"foo/bar\", [",
                "        (\"int\", \"i32\", 12),",
                "        (\"bool\", \"bool\", true),",
                "        (\"string\", \"String\", \"bla\"),",
                "        (\"fraction\", \"Fraction\", (1, 2)),",
                "        (\"array\", \"Array\", [",
                "            (\"i32\", 1),",
                "            (\"i32\", 2),",
                "        ]),",
                "    ]), None),",
                "])"
            ).to_owned()),
            res,
        );

        let caps_any = Caps::new_any();
        let res = ron::ser::to_string_pretty(&caps_any, pretty_config.clone());
        assert_eq!(Ok("Any".to_owned()), res);

        let caps_empty = Caps::new_empty();
        let res = ron::ser::to_string_pretty(&caps_empty, pretty_config.clone());
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
                        ("string", "String", "bla"),
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
            ).as_ref()
        );
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
    }
}
