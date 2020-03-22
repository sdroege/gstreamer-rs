// Copyright (C) 2018 François Laignel <fengalin@free.fr>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::de;
use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use Format;
use FormattedSegment;
use FormattedValue;
use GenericFormattedValue;
use Segment;
use SegmentFlags;
use SpecificFormattedValue;

#[derive(Serialize, Deserialize)]
struct FormattedSegmentSerde {
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

impl<T: FormattedValue> Serialize for FormattedSegment<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let fmt_seg = unsafe {
            FormattedSegmentSerde {
                flags: self.get_flags(),
                rate: self.get_rate(),
                applied_rate: self.get_applied_rate(),
                format: self.get_format(),
                base: self.get_base().to_raw_value(),
                offset: self.get_offset().to_raw_value(),
                start: self.get_start().to_raw_value(),
                stop: self.get_stop().to_raw_value(),
                time: self.get_time().to_raw_value(),
                position: self.get_position().to_raw_value(),
                duration: self.get_duration().to_raw_value(),
            }
        };
        fmt_seg.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Segment {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        FormattedSegmentSerde::deserialize(deserializer).map(|fmt_seg_de| {
            let mut segment = Self::new();
            segment.set_flags(fmt_seg_de.flags);
            segment.set_rate(fmt_seg_de.rate);
            segment.set_applied_rate(fmt_seg_de.applied_rate);
            segment.set_format(fmt_seg_de.format);
            segment.set_base(GenericFormattedValue::new(
                fmt_seg_de.format,
                fmt_seg_de.base,
            ));
            segment.set_offset(GenericFormattedValue::new(
                fmt_seg_de.format,
                fmt_seg_de.offset,
            ));
            segment.set_start(GenericFormattedValue::new(
                fmt_seg_de.format,
                fmt_seg_de.start,
            ));
            segment.set_stop(GenericFormattedValue::new(
                fmt_seg_de.format,
                fmt_seg_de.stop,
            ));
            segment.set_time(GenericFormattedValue::new(
                fmt_seg_de.format,
                fmt_seg_de.time,
            ));
            segment.set_position(GenericFormattedValue::new(
                fmt_seg_de.format,
                fmt_seg_de.position,
            ));
            segment.set_duration(GenericFormattedValue::new(
                fmt_seg_de.format,
                fmt_seg_de.duration,
            ));

            segment
        })
    }
}

impl<'de, T: FormattedValue + SpecificFormattedValue> Deserialize<'de> for FormattedSegment<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        Segment::deserialize(deserializer).and_then(|segment| {
            segment.downcast::<T>().map_err(|segment| {
                de::Error::custom(format!(
                    "failed to convert segment with format {:?} to {:?}",
                    segment.get_format(),
                    T::get_default_format(),
                ))
            })
        })
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
            )
            .to_owned()),
            res,
        );
    }

    #[test]
    fn test_deserialize_segment() {
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
        assert!((segment.get_rate() - 1f64).abs() < std::f64::EPSILON);
        assert!((segment.get_applied_rate() - 0.9f64).abs() < std::f64::EPSILON);
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

    #[test]
    fn test_deserialize_formatted() {
        use format::Time;
        use FormattedSegment;

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

        let fmt_seg: FormattedSegment<Time> = ron::de::from_str(segment_ron).unwrap();
        assert_eq!(
            fmt_seg.get_flags(),
            SegmentFlags::RESET | SegmentFlags::SEGMENT
        );
        assert!((fmt_seg.get_rate() - 1f64).abs() < std::f64::EPSILON);
        assert!((fmt_seg.get_applied_rate() - 0.9f64).abs() < std::f64::EPSILON);
        assert_eq!(fmt_seg.get_format(), Format::Time);
        assert_eq!(fmt_seg.get_base(), ClockTime::from_nseconds(123));
        assert_eq!(fmt_seg.get_offset(), ClockTime::from_nseconds(42));
        assert_eq!(fmt_seg.get_start(), ClockTime::from_nseconds(1024));
        assert_eq!(fmt_seg.get_stop(), ClockTime::from_nseconds(2048));
        assert_eq!(fmt_seg.get_time(), ClockTime::from_nseconds(1042));
        assert_eq!(fmt_seg.get_position(), ClockTime::from_nseconds(256));
        assert_eq!(fmt_seg.get_duration(), ClockTime::none());
    }

    #[test]
    fn test_serde_roundtrip() {
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
        let segment_se = ron::ser::to_string(&segment).unwrap();

        let segment_de: Segment = ron::de::from_str(segment_se.as_str()).unwrap();
        assert_eq!(segment_de.get_flags(), segment.get_flags());
        assert!((segment_de.get_rate() - segment.get_rate()).abs() < std::f64::EPSILON);
        assert!(
            (segment_de.get_applied_rate() - segment.get_applied_rate()).abs() < std::f64::EPSILON
        );
        assert_eq!(segment_de.get_format(), segment.get_format());
        assert_eq!(segment_de.get_base(), segment.get_base());
        assert_eq!(segment_de.get_offset(), segment.get_offset());
        assert_eq!(segment_de.get_start(), segment.get_start());
        assert_eq!(segment_de.get_stop(), segment.get_stop());
        assert_eq!(segment_de.get_time(), segment.get_time());
        assert_eq!(segment_de.get_position(), segment.get_position());
        assert_eq!(segment_de.get_duration(), segment.get_duration());
    }
}
