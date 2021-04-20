// Take a look at the license at the top of the repository in the LICENSE file.

use serde::de;
use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use crate::Format;
use crate::FormattedSegment;
use crate::FormattedValue;
use crate::GenericFormattedValue;
use crate::Segment;
use crate::SegmentFlags;
use crate::SpecificFormattedValue;

#[derive(serde::Serialize, serde::Deserialize)]
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
                flags: self.flags(),
                rate: self.rate(),
                applied_rate: self.applied_rate(),
                format: self.format(),
                base: self.base().to_raw_value(),
                offset: self.offset().to_raw_value(),
                start: self.start().to_raw_value(),
                stop: self.stop().to_raw_value(),
                time: self.time().to_raw_value(),
                position: self.position().to_raw_value(),
                duration: self.duration().to_raw_value(),
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
                    segment.format(),
                    T::default_format(),
                ))
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ClockTime;
    use crate::Format;
    use crate::GenericFormattedValue;
    use crate::Segment;
    use crate::SegmentFlags;

    #[test]
    fn test_serialize() {
        crate::init().unwrap();

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

        let pretty_config = ron::ser::PrettyConfig::new().with_new_line("".to_string());

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
        crate::init().unwrap();

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
        assert_eq!(segment.flags(), SegmentFlags::RESET | SegmentFlags::SEGMENT);
        assert!((segment.rate() - 1f64).abs() < std::f64::EPSILON);
        assert!((segment.applied_rate() - 0.9f64).abs() < std::f64::EPSILON);
        assert_eq!(segment.format(), Format::Time);
        assert_eq!(
            segment.base(),
            GenericFormattedValue::Time(ClockTime::from_nseconds(123))
        );
        assert_eq!(
            segment.offset(),
            GenericFormattedValue::Time(ClockTime::from_nseconds(42))
        );
        assert_eq!(
            segment.start(),
            GenericFormattedValue::Time(ClockTime::from_nseconds(1024))
        );
        assert_eq!(
            segment.stop(),
            GenericFormattedValue::Time(ClockTime::from_nseconds(2048))
        );
        assert_eq!(
            segment.time(),
            GenericFormattedValue::Time(ClockTime::from_nseconds(1042))
        );
        assert_eq!(
            segment.position(),
            GenericFormattedValue::Time(ClockTime::from_nseconds(256))
        );
        assert_eq!(
            segment.duration(),
            GenericFormattedValue::Time(ClockTime::none())
        );
    }

    #[test]
    fn test_deserialize_formatted() {
        use crate::format::Time;
        use crate::FormattedSegment;

        crate::init().unwrap();

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
        assert_eq!(fmt_seg.flags(), SegmentFlags::RESET | SegmentFlags::SEGMENT);
        assert!((fmt_seg.rate() - 1f64).abs() < std::f64::EPSILON);
        assert!((fmt_seg.applied_rate() - 0.9f64).abs() < std::f64::EPSILON);
        assert_eq!(fmt_seg.format(), Format::Time);
        assert_eq!(fmt_seg.base(), ClockTime::from_nseconds(123));
        assert_eq!(fmt_seg.offset(), ClockTime::from_nseconds(42));
        assert_eq!(fmt_seg.start(), ClockTime::from_nseconds(1024));
        assert_eq!(fmt_seg.stop(), ClockTime::from_nseconds(2048));
        assert_eq!(fmt_seg.time(), ClockTime::from_nseconds(1042));
        assert_eq!(fmt_seg.position(), ClockTime::from_nseconds(256));
        assert_eq!(fmt_seg.duration(), ClockTime::none());
    }

    #[test]
    fn test_serde_roundtrip() {
        crate::init().unwrap();

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
        assert_eq!(segment_de.flags(), segment.flags());
        assert!((segment_de.rate() - segment.rate()).abs() < std::f64::EPSILON);
        assert!((segment_de.applied_rate() - segment.applied_rate()).abs() < std::f64::EPSILON);
        assert_eq!(segment_de.format(), segment.format());
        assert_eq!(segment_de.base(), segment.base());
        assert_eq!(segment_de.offset(), segment.offset());
        assert_eq!(segment_de.start(), segment.start());
        assert_eq!(segment_de.stop(), segment.stop());
        assert_eq!(segment_de.time(), segment.time());
        assert_eq!(segment_de.position(), segment.position());
        assert_eq!(segment_de.duration(), segment.duration());
    }
}
