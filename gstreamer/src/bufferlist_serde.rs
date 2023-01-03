// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use serde::{
    de::{Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, SerializeSeq, Serializer},
};

use crate::{Buffer, BufferList, BufferListRef};

impl Serialize for BufferListRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let iter = self.iter();
        let (remaining, _) = iter.size_hint();
        if remaining > 0 {
            let mut seq = serializer.serialize_seq(Some(remaining))?;
            for buffer in iter {
                seq.serialize_element(buffer)?;
            }
            seq.end()
        } else {
            let seq = serializer.serialize_seq(None)?;
            seq.end()
        }
    }
}

impl Serialize for BufferList {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_ref().serialize(serializer)
    }
}

struct BufferListVisitor;
impl<'de> Visitor<'de> for BufferListVisitor {
    type Value = BufferList;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence of Buffers")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut buffer_list = BufferList::new();
        {
            let buffer_list = buffer_list.get_mut().unwrap();
            while let Some(buffer) = seq.next_element::<Buffer>()? {
                buffer_list.add(buffer);
            }
        }
        Ok(buffer_list)
    }
}

impl<'de> Deserialize<'de> for BufferList {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        deserializer.deserialize_seq(BufferListVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::{BufferList, ClockTime};

    #[test]
    fn test_serialize() {
        use crate::Buffer;

        crate::init().unwrap();

        let mut buffer_list = BufferList::new();
        {
            let buffer_list = buffer_list.get_mut().unwrap();

            let mut buffer = Buffer::from_slice(vec![1, 2, 3, 4]);
            {
                let buffer = buffer.get_mut().unwrap();
                buffer.set_pts(ClockTime::NSECOND);
                buffer.set_offset(0);
                buffer.set_offset_end(4);
                buffer.set_duration(4 * ClockTime::NSECOND);
            }
            buffer_list.add(buffer);

            let mut buffer = Buffer::from_slice(vec![5, 6]);
            {
                let buffer = buffer.get_mut().unwrap();
                buffer.set_pts(5 * ClockTime::NSECOND);
                buffer.set_offset(4);
                buffer.set_offset_end(6);
                buffer.set_duration(2 * ClockTime::NSECOND);
            }
            buffer_list.add(buffer);
        }

        let pretty_config = ron::ser::PrettyConfig::new().new_line("".to_string());

        let res = ron::ser::to_string_pretty(&buffer_list, pretty_config);
        assert_eq!(
            Ok(concat!(
                "[",
                "    (",
                "        pts: Some(1),",
                "        dts: None,",
                "        duration: Some(4),",
                "        offset: 0,",
                "        offset_end: 4,",
                "        flags: \"\",",
                "        buffer: \"AQIDBA==\",",
                "    ),",
                "    (",
                "        pts: Some(5),",
                "        dts: None,",
                "        duration: Some(2),",
                "        offset: 4,",
                "        offset_end: 6,",
                "        flags: \"\",",
                "        buffer: \"BQY=\",",
                "    ),",
                "]"
            )
            .to_owned()),
            res,
        );
    }

    #[test]
    fn test_deserialize() {
        crate::init().unwrap();

        let buffer_list_ron = r#"
            [
                (
                    pts: Some(1),
                    dts: None,
                    duration: Some(4),
                    offset: 0,
                    offset_end: 4,
                    flags: "",
                    buffer: "AQIDBA==",
                ),
                (
                    pts: Some(5),
                    dts: None,
                    duration: Some(2),
                    offset: 4,
                    offset_end: 6,
                    flags: "",
                    buffer: "BQY=",
                ),
            ]
        "#;

        let buffer_list: BufferList = ron::de::from_str(buffer_list_ron).unwrap();
        let mut iter = buffer_list.iter();
        let buffer = iter.next().unwrap();
        assert_eq!(buffer.pts(), Some(ClockTime::NSECOND));
        assert_eq!(buffer.dts(), None);
        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());
        }

        let buffer = iter.next().unwrap();
        assert_eq!(buffer.pts(), Some(5 * ClockTime::NSECOND));
        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![5, 6].as_slice());
        }
    }

    #[test]
    fn test_serde_roundtrip() {
        use crate::Buffer;

        crate::init().unwrap();

        let mut buffer_list = BufferList::new();
        {
            let buffer_list = buffer_list.get_mut().unwrap();

            let mut buffer = Buffer::from_slice(vec![1, 2, 3, 4]);
            {
                let buffer = buffer.get_mut().unwrap();
                buffer.set_pts(ClockTime::NSECOND);
                buffer.set_offset(0);
                buffer.set_offset_end(4);
                buffer.set_duration(4 * ClockTime::NSECOND);
            }
            buffer_list.add(buffer);

            let mut buffer = Buffer::from_slice(vec![5, 6]);
            {
                let buffer = buffer.get_mut().unwrap();
                buffer.set_pts(5 * ClockTime::NSECOND);
                buffer.set_offset(4);
                buffer.set_offset_end(6);
                buffer.set_duration(2 * ClockTime::NSECOND);
            }
            buffer_list.add(buffer);
        }
        let buffer_list_ser = ron::ser::to_string(&buffer_list).unwrap();

        let buffer_list: BufferList = ron::de::from_str(buffer_list_ser.as_str()).unwrap();
        let mut iter = buffer_list.iter();
        let buffer = iter.next().unwrap();
        assert_eq!(buffer.pts(), Some(ClockTime::NSECOND));
        assert_eq!(buffer.dts(), None);
        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());
        }

        let buffer = iter.next().unwrap();
        assert_eq!(buffer.pts(), Some(5 * ClockTime::NSECOND));
        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![5, 6].as_slice());
        }
    }
}
