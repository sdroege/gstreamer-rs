// Take a look at the license at the top of the repository in the LICENSE file.

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::Buffer;
use crate::BufferList;
use crate::Caps;
use crate::Sample;
use crate::SampleRef;
use crate::Segment;
use crate::Structure;

impl Serialize for SampleRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut sample = serializer.serialize_struct("Sample", 5)?;
        sample.serialize_field("buffer", &self.buffer())?;
        sample.serialize_field("buffer_list", &self.buffer_list())?;
        sample.serialize_field("caps", &self.caps())?;
        sample.serialize_field("segment", &self.segment())?;
        sample.serialize_field("info", &self.info())?;
        sample.end()
    }
}

impl Serialize for Sample {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_ref().serialize(serializer)
    }
}

#[derive(serde::Deserialize)]
struct SampleDe {
    buffer: Option<Buffer>,
    buffer_list: Option<BufferList>,
    caps: Option<Caps>,
    segment: Option<Segment>,
    info: Option<Structure>,
}

impl From<SampleDe> for Sample {
    fn from(buf_de: SampleDe) -> Self {
        skip_assert_initialized!();
        let mut builder = Sample::builder();

        if let Some(buffer) = buf_de.buffer.as_ref() {
            builder = builder.buffer(buffer);
        }

        if let Some(buffer_list) = buf_de.buffer_list.as_ref() {
            builder = builder.buffer_list(buffer_list);
        }

        if let Some(caps) = buf_de.caps.as_ref() {
            builder = builder.caps(caps);
        }

        if let Some(segment) = buf_de.segment.as_ref() {
            builder = builder.segment(segment);
        }

        if let Some(info) = buf_de.info {
            builder = builder.info(info);
        }

        builder.build()
    }
}

impl<'de> Deserialize<'de> for Sample {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        SampleDe::deserialize(deserializer).map(|sample_de| sample_de.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::Buffer;
    use crate::Caps;
    use crate::ClockTime;
    use crate::Format;
    use crate::GenericFormattedValue;
    use crate::Sample;
    use crate::Segment;
    use crate::SegmentFlags;
    use crate::Structure;

    #[test]
    fn test_serialize() {
        crate::init().unwrap();

        let pretty_config = ron::ser::PrettyConfig::new().new_line("".to_string());

        let sample = {
            let mut buffer = Buffer::from_slice(vec![1, 2, 3, 4]);
            {
                let buffer = buffer.get_mut().unwrap();
                buffer.set_pts(Some(ClockTime::NSECOND));
                buffer.set_offset(0);
                buffer.set_offset_end(4);
                buffer.set_duration(Some(4 * ClockTime::NSECOND));
            }

            let caps = Caps::builder("sample/caps")
                .field("int", 12)
                .field("bool", true)
                .build();

            let mut segment = Segment::new();
            segment.set_flags(SegmentFlags::RESET | SegmentFlags::SEGMENT);
            segment.set_rate(1f64);
            segment.set_applied_rate(0.9f64);
            segment.set_format(Format::Time);
            segment.set_base(GenericFormattedValue::from(ClockTime::from_nseconds(123)));
            segment.set_offset(GenericFormattedValue::from(ClockTime::from_nseconds(42)));
            segment.set_start(GenericFormattedValue::from(ClockTime::from_nseconds(1024)));
            segment.set_stop(GenericFormattedValue::from(ClockTime::from_nseconds(2048)));
            segment.set_time(GenericFormattedValue::from(ClockTime::from_nseconds(1042)));
            segment.set_position(GenericFormattedValue::from(ClockTime::from_nseconds(256)));
            segment.set_duration(GenericFormattedValue::from(ClockTime::NONE));

            let info = Structure::builder("sample.info")
                .field("f3", 123i32)
                .build();

            Sample::builder()
                .buffer(&buffer)
                .caps(&caps)
                .segment(&segment)
                .info(info)
                .build()
        };

        let res = ron::ser::to_string_pretty(&sample, pretty_config.clone());
        assert_eq!(
            Ok(concat!(
                "(",
                "    buffer: Some((",
                "        pts: Some(1),",
                "        dts: None,",
                "        duration: Some(4),",
                "        offset: 0,",
                "        offset_end: 4,",
                "        flags: \"\",",
                "        buffer: \"AQIDBA==\",",
                "    )),",
                "    buffer_list: None,",
                "    caps: Some(Some([",
                "        ((\"sample/caps\", [",
                "            (\"int\", \"i32\", 12),",
                "            (\"bool\", \"bool\", true),",
                "        ]), None),",
                "    ])),",
                "    segment: Some((",
                "        flags: \"reset+segment\",",
                "        rate: 1,",
                "        applied_rate: 0.9,",
                "        format: Time,",
                "        base: 123,",
                "        offset: 42,",
                "        start: 1024,",
                "        stop: 2048,",
                "        time: 1042,",
                "        position: 256,",
                "        duration: -1,",
                "    )),",
                "    info: Some((\"sample.info\", [",
                "        (\"f3\", \"i32\", 123),",
                "    ])),",
                ")"
            )
            .to_owned()),
            res
        );

        let sample = {
            let mut buffer = Buffer::from_slice(vec![1, 2, 3, 4]);
            {
                let buffer = buffer.get_mut().unwrap();
                buffer.set_pts(Some(ClockTime::NSECOND));
                buffer.set_offset(0);
                buffer.set_offset_end(4);
                buffer.set_duration(Some(4 * ClockTime::NSECOND));
            }
            Sample::builder().buffer(&buffer).build()
        };

        // `Sample`'s `Segment` is allocated in GStreamer 1.x, should be fixed in version 2.0

        let res = ron::ser::to_string_pretty(&sample, pretty_config);
        assert_eq!(
            Ok(concat!(
                "(",
                "    buffer: Some((",
                "        pts: Some(1),",
                "        dts: None,",
                "        duration: Some(4),",
                "        offset: 0,",
                "        offset_end: 4,",
                "        flags: \"\",",
                "        buffer: \"AQIDBA==\",",
                "    )),",
                "    buffer_list: None,",
                "    caps: None,",
                "    segment: Some((",
                "        flags: \"\",",
                "        rate: 1,",
                "        applied_rate: 1,",
                "        format: Time,",
                "        base: 0,",
                "        offset: 0,",
                "        start: 0,",
                "        stop: -1,",
                "        time: 0,",
                "        position: 0,",
                "        duration: -1,",
                "    )),",
                "    info: None,",
                ")"
            )
            .to_owned()),
            res
        );
    }

    #[test]
    fn test_deserialize() {
        crate::init().unwrap();

        let buffer_ron = r#"
            (
                buffer: Some((
                    pts: Some(1),
                    dts: None,
                    duration: Some(4),
                    offset: 0,
                    offset_end: 4,
                    flags: "",
                    buffer: "AQIDBA==",
                )),
                buffer_list: None,
                caps: Some(Some([
                    (("sample/caps", [
                        ("int", "i32", 12),
                        ("bool", "bool", true),
                    ]), None),
                ])),
                segment: Some((
                    flags: "",
                    rate: 1,
                    applied_rate: 0.9,
                    format: Time,
                    base: 123,
                    offset: 42,
                    start: 1024,
                    stop: 2048,
                    time: 1042,
                    position: 256,
                    duration: -1,
                )),
                info: Some(("sample.info", [
                    ("f3", "i32", 123),
                ])),
            )"#;
        let sample: Sample = ron::de::from_str(buffer_ron).unwrap();
        let buffer = sample.buffer().unwrap();
        assert_eq!(buffer.pts(), Some(ClockTime::NSECOND));
        assert_eq!(buffer.offset_end(), 4);
        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());
        }
        assert!(sample.buffer_list().is_none());
        assert!(sample.caps().is_some());
        assert!(sample.segment().is_some());
        assert!(sample.info().is_some());

        let buffer_ron = r#"
            (
                buffer: None,
                buffer_list: Some([
                    (
                        pts: Some(1),
                        dts: None,
                        duration: Some(4),
                        offset: 0,
                        offset_end: 4,
                        flags: "",
                        buffer: "AQIDBA==",
                    ),
                ]),
                caps: None,
                segment: None,
                info: None,
            )"#;
        let sample: Sample = ron::de::from_str(buffer_ron).unwrap();
        assert!(sample.buffer().is_none());
        assert!(sample.buffer_list().is_some());
        assert!(sample.caps().is_none());
        // Not true in GStreamer 1.x, should be fixed in version 2.0
        //assert!(sample.get_segment().is_none());
        assert!(sample.info().is_none());
    }

    #[test]
    fn test_roundrip() {
        crate::init().unwrap();

        // Segment present
        let sample = {
            let mut buffer = Buffer::from_slice(vec![1, 2, 3, 4]);
            {
                let buffer = buffer.get_mut().unwrap();
                buffer.set_pts(Some(ClockTime::NSECOND));
                buffer.set_offset(0);
                buffer.set_offset_end(4);
                buffer.set_duration(Some(4 * ClockTime::NSECOND));
            }

            let caps = Caps::builder("sample/caps")
                .field("int", 12)
                .field("bool", true)
                .build();

            let mut segment = Segment::new();
            segment.set_flags(SegmentFlags::RESET | SegmentFlags::SEGMENT);
            segment.set_rate(1f64);
            segment.set_applied_rate(0.9f64);
            segment.set_format(Format::Time);
            segment.set_base(GenericFormattedValue::from(ClockTime::from_nseconds(123)));
            segment.set_offset(GenericFormattedValue::from(ClockTime::from_nseconds(42)));
            segment.set_start(GenericFormattedValue::from(ClockTime::from_nseconds(1024)));
            segment.set_stop(GenericFormattedValue::from(ClockTime::from_nseconds(2048)));
            segment.set_time(GenericFormattedValue::from(ClockTime::from_nseconds(1042)));
            segment.set_position(GenericFormattedValue::from(ClockTime::from_nseconds(256)));
            segment.set_duration(GenericFormattedValue::from(ClockTime::NONE));

            let info = Structure::builder("sample.info")
                .field("f3", 123i32)
                .build();

            Sample::builder()
                .buffer(&buffer)
                .caps(&caps)
                .segment(&segment)
                .info(info)
                .build()
        };
        let sample_ser = ron::ser::to_string(&sample).unwrap();
        let sample_de: Sample = ron::de::from_str(sample_ser.as_str()).unwrap();
        let buffer_de = sample_de.buffer().unwrap();
        assert_eq!(buffer_de.pts(), Some(ClockTime::NSECOND));
        assert_eq!(buffer_de.offset_end(), 4);
        {
            let data = buffer_de.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());
        }
        assert!(sample_de.buffer_list().is_none());
        assert!(sample_de.caps().is_some());
        assert!(sample_de.segment().is_some());
        assert!(sample_de.info().is_some());
    }
}
