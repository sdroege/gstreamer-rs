// Copyright (C) 2018 François Laignel <fengalin@free.fr>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, SerializeStruct, Serializer};

use Format;
use GenericFormattedValue;
use Segment;
use SegmentFlags;

impl<'a> Serialize for Segment {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut datetime = serializer.serialize_struct("Segment", 11)?;
        datetime.serialize_field("flags", &self.get_flags())?;
        datetime.serialize_field("rate", &self.get_rate())?;
        datetime.serialize_field("applied_rate", &self.get_applied_rate())?;
        datetime.serialize_field("format", &self.get_format())?;
        datetime.serialize_field("base", &self.get_base().get_value())?;
        datetime.serialize_field("offset", &self.get_offset().get_value())?;
        datetime.serialize_field("start", &self.get_start().get_value())?;
        datetime.serialize_field("stop", &self.get_stop().get_value())?;
        datetime.serialize_field("time", &self.get_time().get_value())?;
        datetime.serialize_field("position", &self.get_position().get_value())?;
        datetime.serialize_field("duration", &self.get_duration().get_value())?;
        datetime.end()
    }
}

#[derive(Deserialize)]
struct SegmentDe {
    flags: SegmentFlags,
    rate: f64,
    applied_rate: f64,
    format: Format,
    base: i64,
    offset: i64,
    start: i64,
    stop: i64,
    time: i64,
    position: i64,
    duration: i64,
}

impl From<SegmentDe> for Segment {
    fn from(segment_de: SegmentDe) -> Self {
        let mut segment = Segment::new();
        segment.set_flags(segment_de.flags);
        segment.set_rate(segment_de.rate);
        segment.set_applied_rate(segment_de.applied_rate);
        segment.set_format(segment_de.format);
        segment.set_base(GenericFormattedValue::new(
            segment_de.format,
            segment_de.base,
        ));
        segment.set_offset(GenericFormattedValue::new(
            segment_de.format,
            segment_de.offset,
        ));
        segment.set_start(GenericFormattedValue::new(
            segment_de.format,
            segment_de.start,
        ));
        segment.set_stop(GenericFormattedValue::new(
            segment_de.format,
            segment_de.stop,
        ));
        segment.set_time(GenericFormattedValue::new(
            segment_de.format,
            segment_de.time,
        ));
        segment.set_position(GenericFormattedValue::new(
            segment_de.format,
            segment_de.position,
        ));
        segment.set_duration(GenericFormattedValue::new(
            segment_de.format,
            segment_de.duration,
        ));

        segment
    }
}

impl<'de> Deserialize<'de> for Segment {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        SegmentDe::deserialize(deserializer).and_then(|segment_de| Ok(segment_de.into()))
    }
}

#[cfg(test)]
mod tests {
    extern crate ron;

    use ClockTime;
    use Format;
    use GenericFormattedValue;
    use Segment;
    use SegmentFlags;

    #[test]
    fn test_serialize() {
        ::init().unwrap();

        let mut segment = Segment::new();
        segment.set_flags(SegmentFlags::RESET | SegmentFlags::SEGMENT);
        segment.set_rate(1f64);
        segment.set_applied_rate(0.9f64);
        segment.set_format(Format::Time);
        segment.set_base(GenericFormattedValue::Time(ClockTime::from_nseconds(123)));
        segment.set_offset(GenericFormattedValue::Time(ClockTime::from_nseconds(42)));
        segment.set_start(GenericFormattedValue::Time(ClockTime::from_nseconds(1024)));
        segment.set_stop(GenericFormattedValue::Time(ClockTime::from_nseconds(2048)));
        segment.set_time(GenericFormattedValue::Time(ClockTime::from_nseconds(1042)));
        segment.set_position(GenericFormattedValue::Time(ClockTime::from_nseconds(256)));
        segment.set_duration(GenericFormattedValue::Time(ClockTime::none()));

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        let res = ron::ser::to_string_pretty(&segment, pretty_config);
        assert_eq!(
            Ok(concat!(
                "(",
                "    flags: (",
                "        bits: 9,",
                "    ),",
                "    rate: 1,",
                "    applied_rate: 0.9,",
                "    format: Time,",
                "    base: 123,",
                "    offset: 42,",
                "    start: 1024,",
                "    stop: 2048,",
                "    time: 1042,",
                "    position: 256,",
                "    duration: -1,",
                ")"
            ).to_owned()),
            res,
        );
    }

    #[test]
    fn test_deserialize() {
        ::init().unwrap();

        let segment_ron = r#"
            (
                flags: (
                    bits: 9,
                ),
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
            )
        "#;

        let segment: Segment = ron::de::from_str(segment_ron).unwrap();
        assert_eq!(
            segment.get_flags(),
            SegmentFlags::RESET | SegmentFlags::SEGMENT
        );
        assert_eq!(segment.get_rate(), 1f64);
        assert_eq!(segment.get_applied_rate(), 0.9f64);
        assert_eq!(segment.get_format(), Format::Time);
        assert_eq!(
            segment.get_base(),
            GenericFormattedValue::Time(ClockTime::from_nseconds(123))
        );
        assert_eq!(
            segment.get_offset(),
            GenericFormattedValue::Time(ClockTime::from_nseconds(42))
        );
        assert_eq!(
            segment.get_start(),
            GenericFormattedValue::Time(ClockTime::from_nseconds(1024))
        );
        assert_eq!(
            segment.get_stop(),
            GenericFormattedValue::Time(ClockTime::from_nseconds(2048))
        );
        assert_eq!(
            segment.get_time(),
            GenericFormattedValue::Time(ClockTime::from_nseconds(1042))
        );
        assert_eq!(
            segment.get_position(),
            GenericFormattedValue::Time(ClockTime::from_nseconds(256))
        );
        assert_eq!(
            segment.get_duration(),
            GenericFormattedValue::Time(ClockTime::none())
        );
    }
}
