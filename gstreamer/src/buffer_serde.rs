// Copyright (C) 2018 François Laignel <fengalin@free.fr>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::de::{Deserialize, Deserializer};
use serde::ser;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde_bytes::{ByteBuf, Bytes};

use Buffer;
use BufferFlags;
use BufferRef;
use ClockTime;

// TODO: try `Either<ByteBuf, Bytes>` to merge the base reprensentations for ser and de
// while avoiding unneeded copy

impl<'a> Serialize for BufferRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut buffer = serializer.serialize_struct("Buffer", 6)?;
        buffer.serialize_field("pts", &self.get_pts())?;
        buffer.serialize_field("dts", &self.get_dts())?;
        buffer.serialize_field("duration", &self.get_duration())?;
        buffer.serialize_field("offset", &self.get_offset())?;
        buffer.serialize_field("offset_end", &self.get_offset_end())?;
        buffer.serialize_field("flags", &self.get_flags())?;
        {
            let data = self
                .map_readable()
                .map_err(|_| ser::Error::custom("Couldn't map `buffer` as readable"))?;
            buffer.serialize_field("buffer", &Bytes::new(data.as_slice()))?;
        }
        buffer.end()
    }
}

impl<'a> Serialize for Buffer {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_ref().serialize(serializer)
    }
}

#[derive(Deserialize)]
struct BufferDe {
    pts: ClockTime,
    dts: ClockTime,
    duration: ClockTime,
    offset: u64,
    offset_end: u64,
    flags: BufferFlags,
    buffer: ByteBuf,
}

impl From<BufferDe> for Buffer {
    fn from(buf_de: BufferDe) -> Self {
        skip_assert_initialized!();
        let mut buffer = Buffer::from_mut_slice(buf_de.buffer.to_vec());
        {
            let buffer = buffer.get_mut().unwrap();
            buffer.set_pts(buf_de.pts);
            buffer.set_dts(buf_de.dts);
            buffer.set_duration(buf_de.duration);
            buffer.set_offset(buf_de.offset);
            buffer.set_offset_end(buf_de.offset_end);
            buffer.set_flags(buf_de.flags);
        }
        buffer
    }
}

impl<'de> Deserialize<'de> for Buffer {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        BufferDe::deserialize(deserializer).map(|buffer_de| buffer_de.into())
    }
}

#[cfg(test)]
mod tests {
    extern crate ron;
    extern crate serde_json;

    use Buffer;
    use BufferFlags;

    #[test]
    fn test_serialize() {
        ::init().unwrap();

        let mut buffer = Buffer::from_slice(vec![1, 2, 3, 4]);
        {
            let buffer = buffer.get_mut().unwrap();
            buffer.set_pts(1.into());
            buffer.set_offset(3);
            buffer.set_offset_end(4);
            buffer.set_duration(5.into());
            buffer.set_flags(BufferFlags::LIVE | BufferFlags::DISCONT);
        }

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        let res = ron::ser::to_string_pretty(&buffer, pretty_config);
        assert_eq!(
            Ok(concat!(
                "(",
                "    pts: Some(1),",
                "    dts: None,",
                "    duration: Some(5),",
                "    offset: 3,",
                "    offset_end: 4,",
                "    flags: (",
                "        bits: 80,",
                "    ),",
                "    buffer: \"AQIDBA==\",",
                ")"
            )
            .to_owned()),
            res
        );

        let res = serde_json::to_string(&buffer).unwrap();
        assert_eq!(
            concat!(
                "{",
                "\"pts\":1,",
                "\"dts\":null,",
                "\"duration\":5,",
                "\"offset\":3,",
                "\"offset_end\":4,",
                "\"flags\":{\"bits\":80},",
                "\"buffer\":[1,2,3,4]",
                "}"
            )
            .to_owned(),
            res
        );
    }

    #[test]
    fn test_deserialize() {
        ::init().unwrap();

        let buffer_ron = r#"
            (
                pts: Some(1),
                dts: None,
                duration: Some(5),
                offset: 3,
                offset_end: 4,
                flags: (
                    bits: 80,
                ),
                buffer: "AQIDBA==",
            )
        "#;
        let buffer: Buffer = ron::de::from_str(buffer_ron).unwrap();
        assert_eq!(buffer.get_pts(), 1.into());
        assert_eq!(buffer.get_dts(), None.into());
        assert_eq!(buffer.get_offset(), 3);
        assert_eq!(buffer.get_offset_end(), 4);
        assert_eq!(buffer.get_duration(), 5.into());
        assert_eq!(buffer.get_flags(), BufferFlags::LIVE | BufferFlags::DISCONT);
        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());
        }

        let buffer_json = r#"
            {
                "pts":1,
                "dts":null,
                "duration":5,
                "offset":3,
                "offset_end":4,
                "flags":{"bits":80},
                "buffer":[1,2,3,4]
            }
        "#;
        let buffer: Buffer = serde_json::from_str(buffer_json).unwrap();
        assert_eq!(buffer.get_pts(), 1.into());
        assert_eq!(buffer.get_dts(), None.into());
        assert_eq!(buffer.get_offset(), 3);
        assert_eq!(buffer.get_offset_end(), 4);
        assert_eq!(buffer.get_duration(), 5.into());
        assert_eq!(buffer.get_flags(), BufferFlags::LIVE | BufferFlags::DISCONT);
        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());
        }
    }

    #[test]
    fn test_serde_roundtrip() {
        ::init().unwrap();

        let mut buffer = Buffer::from_slice(vec![1, 2, 3, 4]);
        {
            let buffer = buffer.get_mut().unwrap();
            buffer.set_pts(1.into());
            buffer.set_offset(3);
            buffer.set_offset_end(4);
            buffer.set_duration(5.into());
            buffer.set_flags(BufferFlags::LIVE | BufferFlags::DISCONT);
        }

        // Ron
        let buffer_ser = ron::ser::to_string(&buffer).unwrap();
        let buffer_de: Buffer = ron::de::from_str(buffer_ser.as_str()).unwrap();
        assert_eq!(buffer_de.get_pts(), buffer.get_pts());
        assert_eq!(buffer_de.get_dts(), buffer.get_dts());
        assert_eq!(buffer_de.get_offset(), buffer.get_offset());
        assert_eq!(buffer_de.get_offset_end(), buffer.get_offset_end());
        assert_eq!(buffer_de.get_duration(), buffer.get_duration());
        assert_eq!(buffer_de.get_flags(), buffer.get_flags());
        {
            let data = buffer_de.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());
        }
    }
}
