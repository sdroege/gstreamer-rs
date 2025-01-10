// Take a look at the license at the top of the repository in the LICENSE file.

use serde::{
    de::{Deserialize, Deserializer},
    ser,
    ser::{Serialize, SerializeStruct, Serializer},
};
use serde_bytes::{ByteBuf, Bytes};

use crate::{Buffer, BufferFlags, BufferRef, ClockTime};

// TODO: try `Either<ByteBuf, Bytes>` to merge the base representations for ser and de
// while avoiding unneeded copy

impl Serialize for BufferRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut buffer = serializer.serialize_struct("Buffer", 7)?;
        buffer.serialize_field("pts", &self.pts())?;
        buffer.serialize_field("dts", &self.dts())?;
        buffer.serialize_field("duration", &self.duration())?;
        buffer.serialize_field("offset", &self.offset())?;
        buffer.serialize_field("offset_end", &self.offset_end())?;
        buffer.serialize_field("flags", &self.flags())?;
        {
            let data = self
                .map_readable()
                .map_err(|_| ser::Error::custom("Couldn't map `buffer` as readable"))?;
            buffer.serialize_field("buffer", &Bytes::new(data.as_slice()))?;
        }
        buffer.end()
    }
}

impl Serialize for Buffer {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_ref().serialize(serializer)
    }
}

#[derive(serde::Deserialize)]
struct BufferDe {
    pts: Option<ClockTime>,
    dts: Option<ClockTime>,
    duration: Option<ClockTime>,
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
    use crate::{Buffer, BufferFlags, ClockTime};

    #[test]
    fn test_serialize() {
        crate::init().unwrap();

        let mut buffer = Buffer::from_slice(vec![1, 2, 3, 4]);
        {
            let buffer = buffer.get_mut().unwrap();
            buffer.set_pts(Some(ClockTime::NSECOND));
            buffer.set_offset(3);
            buffer.set_offset_end(4);
            buffer.set_duration(5 * ClockTime::NSECOND);
            buffer.set_flags(BufferFlags::LIVE | BufferFlags::DISCONT);
        }

        let pretty_config = ron::ser::PrettyConfig::new().new_line("".to_string());

        let res = ron::ser::to_string_pretty(&buffer, pretty_config);
        assert_eq!(
            Ok(concat!(
                "(",
                "    pts: Some(1),",
                "    dts: None,",
                "    duration: Some(5),",
                "    offset: 3,",
                "    offset_end: 4,",
                "    flags: \"live+discont\",",
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
                "\"flags\":\"live+discont\",",
                "\"buffer\":[1,2,3,4]",
                "}"
            )
            .to_owned(),
            res
        );
    }

    #[test]
    fn test_deserialize() {
        crate::init().unwrap();

        let buffer_ron = r#"
            (
                pts: Some(1),
                dts: None,
                duration: Some(5),
                offset: 3,
                offset_end: 4,
                flags: "live+discont",
                buffer: "AQIDBA==",
            )
        "#;
        let buffer: Buffer = ron::de::from_str(buffer_ron).unwrap();
        assert_eq!(buffer.pts(), Some(ClockTime::NSECOND));
        assert_eq!(buffer.dts(), crate::ClockTime::NONE);
        assert_eq!(buffer.offset(), 3);
        assert_eq!(buffer.offset_end(), 4);
        assert_eq!(buffer.duration(), Some(5 * ClockTime::NSECOND));
        assert_eq!(buffer.flags(), BufferFlags::LIVE | BufferFlags::DISCONT);
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
                "flags":"live+discont",
                "buffer":[1,2,3,4]
            }
        "#;
        let buffer: Buffer = serde_json::from_str(buffer_json).unwrap();
        assert_eq!(buffer.pts(), Some(ClockTime::NSECOND));
        assert_eq!(buffer.dts(), crate::ClockTime::NONE);
        assert_eq!(buffer.offset(), 3);
        assert_eq!(buffer.offset_end(), 4);
        assert_eq!(buffer.duration(), Some(5 * ClockTime::NSECOND));
        assert_eq!(buffer.flags(), BufferFlags::LIVE | BufferFlags::DISCONT);
        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());
        }
    }

    #[test]
    fn test_serde_roundtrip() {
        crate::init().unwrap();

        let mut buffer = Buffer::from_slice(vec![1, 2, 3, 4]);
        {
            let buffer = buffer.get_mut().unwrap();
            buffer.set_pts(Some(ClockTime::NSECOND));
            buffer.set_offset(3);
            buffer.set_offset_end(4);
            buffer.set_duration(5 * ClockTime::NSECOND);
            buffer.set_flags(BufferFlags::LIVE | BufferFlags::DISCONT);
        }

        // Ron
        let buffer_ser = ron::ser::to_string(&buffer).unwrap();
        let buffer_de: Buffer = ron::de::from_str(buffer_ser.as_str()).unwrap();
        assert_eq!(buffer_de.pts(), buffer.pts());
        assert_eq!(buffer_de.dts(), buffer.dts());
        assert_eq!(buffer_de.offset(), buffer.offset());
        assert_eq!(buffer_de.offset_end(), buffer.offset_end());
        assert_eq!(buffer_de.duration(), buffer.duration());
        assert_eq!(buffer_de.flags(), buffer.flags());
        {
            let data = buffer_de.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());
        }
    }
}
