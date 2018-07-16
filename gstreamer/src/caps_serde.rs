// Copyright (C) 2018 François Laignel <fengalin@free.fr>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, Serializer, SerializeSeq};

use std::fmt;

use Caps;
use CapsRef;
use Structure;

impl<'a> Serialize for CapsRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let iter = self.iter();
        let size = iter.size_hint().0;
        if size > 0 {
            let mut seq = serializer.serialize_seq(Some(size))?;
            for structure in iter {
                seq.serialize_element(structure)?;
            }
            seq.end()
        } else {
            let seq = serializer.serialize_seq(None)?;
            seq.end()
        }
    }
}

impl<'a> Serialize for Caps {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_ref().serialize(serializer)
    }
}

struct CapsVisitor;
impl<'de> Visitor<'de> for CapsVisitor {
    type Value = Caps;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence of `Structure`s")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut caps = Caps::new_empty();
        {
            let caps = caps.get_mut().unwrap();
            while let Some(structure) = seq.next_element::<Structure>()? {
                caps.append_structure(structure);
            }
        }
        Ok(caps)
    }
}

impl<'de> Deserialize<'de> for Caps {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_seq(CapsVisitor)
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

        let res = ron::ser::to_string_pretty(&caps, pretty_config);
        assert_eq!(
            Ok(
                concat!(
                    "[",
                    "    (\"foo/bar\", [",
                    "        (\"int\", \"i32\", 12),",
                    "        (\"bool\", \"bool\", true),",
                    "        (\"string\", \"String\", \"bla\"),",
                    "        (\"fraction\", \"Fraction\", (1, 2)),",
                    "        (\"array\", \"Array\", [",
                    "            (\"i32\", 1),",
                    "            (\"i32\", 2),",
                    "        ]),",
                    "    ]),",
                    "]"
                )
                    .to_owned()
            ),
            res,
        );
    }

    #[test]
    fn test_deserialize() {
        use Structure;

        ::init().unwrap();

        let caps_ron = r#"
            [
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
            ]"#;
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
}
