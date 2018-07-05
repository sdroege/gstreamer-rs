// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib;
use glib::translate::*;
use std::{cmp, fmt};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Default)]
pub struct ClockTime(pub Option<u64>);

impl ClockTime {
    pub fn hours(&self) -> Option<u64> {
        (*self / ::SECOND / 60 / 60).0
    }

    pub fn minutes(&self) -> Option<u64> {
        (*self / ::SECOND / 60).0
    }

    pub fn seconds(&self) -> Option<u64> {
        (*self / ::SECOND).0
    }

    pub fn mseconds(&self) -> Option<u64> {
        (*self / ::MSECOND).0
    }

    pub fn useconds(&self) -> Option<u64> {
        (*self / ::USECOND).0
    }

    pub fn nseconds(&self) -> Option<u64> {
        (*self / ::NSECOND).0
    }

    pub fn nanoseconds(&self) -> Option<u64> {
        self.0
    }

    pub fn from_seconds(seconds: u64) -> ClockTime {
        seconds * ::SECOND
    }

    pub fn from_mseconds(mseconds: u64) -> ClockTime {
        mseconds * ::MSECOND
    }

    pub fn from_useconds(useconds: u64) -> ClockTime {
        useconds * ::USECOND
    }

    pub fn from_nseconds(nseconds: u64) -> ClockTime {
        nseconds * ::NSECOND
    }

    pub fn none() -> ClockTime {
        ClockTime(None)
    }
}

impl fmt::Display for ClockTime {
    #[cfg_attr(feature = "cargo-clippy", allow(many_single_char_names))]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let precision = f.precision().unwrap_or(9);
        // TODO: Could also check width and pad the hours as needed

        let (h, m, s, ns) = match self.0 {
            Some(v) => {
                let mut s = v / 1_000_000_000;
                let mut m = s / 60;
                let h = m / 60;
                s %= 60;
                m %= 60;
                let ns = v % 1_000_000_000;

                (h, m, s, ns)
            }
            None => (99, 99, 99, 999_999_999),
        };

        if precision == 0 {
            f.write_fmt(format_args!("{:02}:{:02}:{:02}", h, m, s))
        } else {
            let mut divisor = 1;
            let precision = cmp::min(precision, 9);
            for _ in 0..(9 - precision) {
                divisor *= 10;
            }

            f.write_fmt(format_args!(
                "{:02}:{:02}:{:02}.{:0width$}",
                h,
                m,
                s,
                ns / divisor,
                width = precision
            ))
        }
    }
}

#[cfg(feature = "ser_de")]
pub(crate) mod serde {
    use serde::de;
    use serde::de::{Deserialize, Deserializer, Visitor};
    use serde::ser::{Serialize, Serializer};

    use std::fmt;

    use ClockTime;

    impl<'a> Serialize for ClockTime {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self.nanoseconds() {
                Some(ref value) => serializer.serialize_some(value),
                None => serializer.serialize_none(),
            }
        }
    }

    struct ClockTimeVisitor;
    impl<'de> Visitor<'de> for ClockTimeVisitor {
        type Value = ClockTime;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an optional u64 ClockTime with ns precision")
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            u64::deserialize(deserializer)
                .and_then(|value| Ok(ClockTime::from_nseconds(value)))
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(ClockTime(None))
        }
    }

    impl<'de> Deserialize<'de> for ClockTime {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_option(ClockTimeVisitor)
        }
    }
}

#[doc(hidden)]
impl ToGlib for ClockTime {
    type GlibType = ffi::GstClockTime;

    fn to_glib(&self) -> ffi::GstClockTime {
        match self.0 {
            None => ffi::GST_CLOCK_TIME_NONE,
            Some(v) => v,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstClockTime> for ClockTime {
    fn from_glib(value: ffi::GstClockTime) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GST_CLOCK_TIME_NONE => ClockTime(None),
            value => ClockTime(Some(value)),
        }
    }
}

#[doc(hidden)]
impl<'a> glib::value::FromValueOptional<'a> for ClockTime {
    unsafe fn from_value_optional(value: &'a glib::Value) -> Option<Self> {
        <u64 as glib::value::FromValueOptional>::from_value_optional(value)
            .map(ClockTime::from_glib)
    }
}

#[doc(hidden)]
impl<'a> glib::value::FromValue<'a> for ClockTime {
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        ClockTime::from_glib(<u64 as glib::value::FromValue>::from_value(value))
    }
}

#[doc(hidden)]
impl glib::value::SetValue for ClockTime {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        <u64 as glib::value::SetValue>::set_value(value, &this.to_glib());
    }
}

#[doc(hidden)]
impl glib::StaticType for ClockTime {
    fn static_type() -> glib::Type {
        <u64 as glib::StaticType>::static_type()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "ser_de")]
    #[test]
    fn test_serialize() {
        extern crate ron;
        extern crate serde_json;

        use ClockTime;

        ::init().unwrap();

        // Some
        let clocktime = ClockTime::from_nseconds(42_123_456_789);

        // don't use newlines
        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        let res = ron::ser::to_string_pretty(&clocktime, pretty_config.clone());
        assert_eq!(Ok("Some(42123456789)".to_owned()), res);

        let res = serde_json::to_string(&clocktime).unwrap();
        assert_eq!("42123456789".to_owned(), res);

        // None
        let clocktime = ClockTime(None);

        let res = ron::ser::to_string_pretty(&clocktime, pretty_config);
        assert_eq!(Ok("None".to_owned()), res);

        let res = serde_json::to_string(&clocktime).unwrap();
        assert_eq!("null".to_owned(), res);
    }

    #[cfg(feature = "ser_de")]
    #[test]
    fn test_deserialize() {
        extern crate ron;
        extern crate serde_json;

        use ClockTime;

        ::init().unwrap();

        // Some
        let clocktime_ron = "Some(42123456789)";
        let clocktime: ClockTime = ron::de::from_str(clocktime_ron).unwrap();
        assert_eq!(clocktime.seconds(), Some(42));
        assert_eq!(clocktime.mseconds(), Some(42_123));
        assert_eq!(clocktime.useconds(), Some(42_123_456));
        assert_eq!(clocktime.nseconds(), Some(42_123_456_789));

        let clocktime_json = "42123456789";
        let clocktime: ClockTime = serde_json::from_str(clocktime_json).unwrap();
        assert_eq!(clocktime.seconds(), Some(42));
        assert_eq!(clocktime.mseconds(), Some(42_123));
        assert_eq!(clocktime.useconds(), Some(42_123_456));
        assert_eq!(clocktime.nseconds(), Some(42_123_456_789));

        // None
        let clocktime_ron = "None";
        let clocktime: ClockTime = ron::de::from_str(clocktime_ron).unwrap();
        assert_eq!(clocktime.nseconds(), None);

        let clocktime_json = "null";
        let clocktime: ClockTime = serde_json::from_str(clocktime_json).unwrap();
        assert_eq!(clocktime.nseconds(), None);
    }
}
