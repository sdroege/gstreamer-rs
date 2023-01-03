// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{
    translate::{from_glib, ToGlibPtr},
    FlagsClass, StaticType, ToValue,
};
use gst::bitflags_serde_impl;

bitflags_serde_impl!(crate::AudioFlags);
bitflags_serde_impl!(crate::AudioFormatFlags);
bitflags_serde_impl!(crate::AudioPackFlags);

#[cfg(test)]
mod tests {
    macro_rules! check_serialize {
        ($flags:expr, $expected:expr) => {
            let actual = serde_json::to_string(&$flags).unwrap();
            assert_eq!(actual, $expected);
        };
    }

    macro_rules! check_deserialize {
        ($ty:ty, $expected:expr, $json:expr) => {
            let actual: $ty = serde_json::from_str(&$json).unwrap();
            assert_eq!(actual, $expected);
        };
    }

    macro_rules! check_roundtrip {
        ($ty:ty, $flags:expr) => {
            let json = serde_json::to_string(&$flags).unwrap();
            let deserialized: $ty = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, $flags);
        };
    }

    #[test]
    fn test_serialize() {
        gst::init().unwrap();

        check_serialize!(crate::AudioFlags::all(), "\"unpositioned\"");
        check_serialize!(
            crate::AudioFormatFlags::all(),
            "\"integer+float+signed+complex+unpack\""
        );
        check_serialize!(crate::AudioPackFlags::all(), "\"truncate-range\"");
    }

    #[test]
    fn test_deserialize() {
        gst::init().unwrap();

        check_deserialize!(
            crate::AudioFlags,
            crate::AudioFlags::all(),
            "\"unpositioned\""
        );
        check_deserialize!(
            crate::AudioFormatFlags,
            crate::AudioFormatFlags::all(),
            "\"integer+float+signed+complex+unpack\""
        );
        check_deserialize!(
            crate::AudioPackFlags,
            crate::AudioPackFlags::all(),
            "\"truncate-range\""
        );
    }

    #[test]
    fn test_serde_roundtrip() {
        gst::init().unwrap();

        check_roundtrip!(crate::AudioFlags, crate::AudioFlags::all());
        check_roundtrip!(crate::AudioFormatFlags, crate::AudioFormatFlags::all());
        check_roundtrip!(crate::AudioPackFlags, crate::AudioPackFlags::all());
    }
}
